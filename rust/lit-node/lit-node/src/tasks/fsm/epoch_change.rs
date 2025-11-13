use crate::config::chain::CachedRootKey;
use crate::error::unexpected_err;
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tasks::fsm::utils::parse_epoch_number_from_dkg_id;
use crate::tss::common::dkg_type::DkgType;
use crate::tss::common::key_persistence::RECOVERY_DKG_EPOCH;
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use crate::tss::dkg::manager::DkgManager;
use ethers::types::U256;
use lit_core::error::Result;
use std::sync::Arc;
use tracing::instrument;

use super::utils::get_current_and_new_peer_addresses;
use super::utils::key_share_proofs_check;

// only log the epoch number field
#[instrument(level = "debug", skip(dkg_manager, fsm_worker_metadata))]
pub(crate) async fn perform_epoch_change(
    dkg_manager: &DkgManager,
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64>>,
    realm_id: u64,
    is_shadow: bool,
    epoch_number: U256,
) -> Result<Option<Vec<CachedRootKey>>> {
    struct EpochChangeResOrUpdateNeeded {
        pub epoch_change_res: Option<Option<Vec<CachedRootKey>>>,
        pub update_req: Option<u64>,
    }

    let peer_state = dkg_manager.tss_state.peer_state.clone();
    let cfg = dkg_manager.tss_state.lit_config.clone();

    // Derive the DKG ID.
    let mut fsm_worker_lifecycle_id = fsm_worker_metadata.get_lifecycle_id(realm_id);

    // We keep looping until we get a result from a completed epoch change operation.
    let mut latest_dkg_id = "".to_string();
    let mut abort_and_restart_count = 0;

    // We currently set the limit of aborts and restarts to be a high number to avoid infinite loops. This should never happen,
    // in theory, but we want to be safe. This will be removed as soon as we have implemented an improved strategy to synchronize
    // the DKG ID across our distributed network of nodes.
    while abort_and_restart_count < 10 {
        info!(
            "Performing epoch change for dkg_id: {} (abort_and_restart_count: {})",
            latest_dkg_id, abort_and_restart_count
        );

        // make sure peers are up to date, across potential abort + restarts.
        let (current_peers, new_peers) =
            match get_current_next_dkg_peers(dkg_manager, realm_id, is_shadow).await {
                Ok((current_peers, new_peers)) => (current_peers, new_peers),
                Err(e) => {
                    error!("Error in get_current_next_dkg_peers: {}", e);
                    return Err(e);
                }
            };

        let dkg_id = derive_dkg_id(realm_id, epoch_number, &new_peers, fsm_worker_lifecycle_id);
        latest_dkg_id = dkg_id.clone();

        // when you start with a shadow node, they are going to read the "original" key (from the src realm) ....
        let shadow_key_opts = match is_shadow {
            true => {
                trace!("Getting key epoch number for shadow realm");
                let base_realm_id = peer_state.realm_id();
                let base_epoch_number = peer_state.get_epoch(base_realm_id).await;

                let base_epoch_number = match base_epoch_number {
                    Ok(base_epoch_number) => base_epoch_number.1,
                    Err(e) => {
                        error!(
                            "Error in get_epoch for base epoch when shadow node is starting: {}",
                            e
                        );
                        continue;
                    }
                };

                trace!("Base epoch number: {}", base_epoch_number);
                (base_epoch_number.as_u64(), base_realm_id)
            }
            false => (epoch_number.as_u64(), realm_id),
        };

        let current_epoch = epoch_number.as_u64();

        let epoch_change_res_or_update_needed = tokio::select! {
            // We stop polling the other future as soon as `yield_until_update` returns, and
            // after we parse the lifecycle IDs.
            new_lifecycle_id = fsm_worker_metadata.yield_until_update(realm_id) => {
                let existing_lifecycle_id = fsm_worker_metadata.get_lifecycle_id(realm_id);
                info!("FSMWorkerMetadata is outdated, updating the lifecycle id from {} to {} in realm {}, aborting the current epoch change and restarting with the new updated lifecycle id", existing_lifecycle_id, new_lifecycle_id, realm_id);
                EpochChangeResOrUpdateNeeded {
                    epoch_change_res: None,
                    update_req: Some(new_lifecycle_id),
                }
            }

            res = dkg_manager.change_epoch(&latest_dkg_id, current_epoch, shadow_key_opts, realm_id, &current_peers, &new_peers) => {
                if res.is_ok() {
                    let epoch = match dkg_manager.dkg_type {
                        DkgType::RecoveryParty => RECOVERY_DKG_EPOCH,
                        DkgType::Standard => current_epoch + 1,
                    };

                    let lifecycle_id = fsm_worker_metadata.get_lifecycle_id(realm_id);
                    match key_share_proofs_check(&dkg_manager.tss_state, &res, &new_peers, &latest_dkg_id, realm_id, epoch, lifecycle_id).await {
                        Err(e) => {
                            error!("Key share proofs check failed in realm {}: {}", realm_id, e);
                            return Err(e);
                        },
                        Ok(()) => {
                            debug!("Key share proofs check passed for realm {}", realm_id);
                        }
                    }
                }
                EpochChangeResOrUpdateNeeded {
                    epoch_change_res: Some(res.inspect_err(|e| error!("DKG error: {}", e)).ok()),
                    update_req: None,
                }
            }
        };

        let (post_current_peers, post_new_peers) =
            match get_current_next_dkg_peers(dkg_manager, realm_id, is_shadow).await {
                Ok((current_peers, new_peers)) => (current_peers, new_peers),
                Err(e) => {
                    error!(
                        "Error in get_current_next_dkg_peers in realm {}: {}",
                        realm_id, e
                    );
                    return Err(e);
                }
            };

        if post_new_peers.0.len() != new_peers.0.len() {
            warn!(
                "The upcoming peer-set in realm {} for the {} epoch change has changed during DKG : {} / {}  | Details:  {} / {} ",
                realm_id,
                epoch_number,
                &new_peers.0.len(),
                &post_new_peers.0.len(),
                &new_peers.debug_addresses(),
                &post_new_peers.debug_addresses(),
            );
        }
        // If there is a result, we immediately return the result.
        if let Some(res) = epoch_change_res_or_update_needed.epoch_change_res {
            return Ok(res);
        }

        // If we are here, that means that we need to update the lifecycle ID and restart the epoch change.
        let new_lifecycle_id = match epoch_change_res_or_update_needed.update_req {
            Some(new_lifecycle_id) => new_lifecycle_id,
            None => {
                error!("epoch_change_res_or_update_needed.update_req is None");
                return Err(unexpected_err(
                    "epoch_change_res_or_update_needed.update_req is None",
                    None,
                ));
            }
        };

        fsm_worker_metadata.update_lifecycle_id(Some(new_lifecycle_id), realm_id);

        let existing_epoch_number = match parse_epoch_number_from_dkg_id(&dkg_id) {
            Ok(existing_epoch_number) => existing_epoch_number,
            Err(e) => {
                error!("Error in parse_epoch_number_from_dkg_id: {}", e);
                return Err(e);
            }
        };
        trace!(
            "existing_epoch_number in realm {}: {}",
            realm_id, existing_epoch_number
        );

        let previous_dkg_id = latest_dkg_id.clone();
        latest_dkg_id = derive_dkg_id(
            realm_id,
            existing_epoch_number,
            &new_peers,
            new_lifecycle_id,
        );
        debug!(
            "Previous dkg_id in realm {} was {}, new dkg_id is {}",
            realm_id, previous_dkg_id, latest_dkg_id
        );

        fsm_worker_lifecycle_id = new_lifecycle_id;

        abort_and_restart_count += 1;
    }

    // If we are here, that means that we have aborted and restarted the epoch change too many times. Just return a failure.
    error!("Aborted and restarted the epoch change too many times. Aborting the epoch change.");
    Err(unexpected_err(
        "Aborted and restarted the epoch change too many times. Aborting the epoch change.",
        None,
    ))
}

pub fn derive_dkg_id(
    realm_id: u64,
    epoch_number: U256,
    next_peers: &SimplePeerCollection,
    fsm_worker_lifecycle_id: <dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64> as FSMWorkerMetadata>::LifecycleId,
) -> String {
    format!(
        "EPOCH_DKG_{}_{}_{}_{}",
        epoch_number,
        fsm_worker_lifecycle_id,
        next_peers.hash(),
        realm_id
    )
}

pub async fn get_current_next_dkg_peers(
    dkg_manager: &DkgManager,
    realm_id: u64,
    is_shadow: bool,
) -> Result<(SimplePeerCollection, SimplePeerCollection)> {
    let peer_state = dkg_manager.tss_state.peer_state.clone();

    let (current_peers, new_peers) = if dkg_manager.dkg_type == DkgType::RecoveryParty {
        let recovery_dkg_peers = match peer_state.get_recovery_dkg_peers().await {
            Ok(recovery_dkg_peers) => recovery_dkg_peers,
            Err(e) => {
                error!("Error in getting Recovery DKG peers: {:?}", e);
                return Err(e);
            }
        };

        (SimplePeerCollection(vec![]), recovery_dkg_peers)
    } else {
        match get_current_and_new_peer_addresses(is_shadow, peer_state.clone()).await {
            Ok((current_peers, new_peers)) => (current_peers, new_peers),
            Err(e) => {
                error!("Error in get_peer_addresses: {}", e);
                return Err(e);
            }
        }
    };

    Ok((current_peers, new_peers))
}
