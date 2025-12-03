use crate::config::chain::CachedRootKey;
use crate::models::KeySetConfig;
use crate::peers::PeerState;
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tasks::fsm::utils::parse_epoch_number_from_dkg_id;
use crate::tss::common::curve_state::CurveState;
use crate::tss::common::dkg_type::DkgType;
use crate::tss::common::key_persistence::RECOVERY_DKG_EPOCH;
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use crate::tss::dkg::manager::DkgManager;
use crate::version::DataVersionReader;
use ethers::types::U256;
use lit_core::error::Result;
use lit_node_core::CurveType;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;

use super::utils::get_current_and_new_peer_addresses;
use super::utils::key_share_proofs_check;

struct EpochChangeResOrUpdateNeeded {
    pub epoch_change_res: Option<Option<HashMap<String, Vec<CachedRootKey>>>>,
    pub update_req: Option<u64>,
}

// only log the epoch number field
#[instrument(level = "debug", skip(dkg_manager, fsm_worker_metadata))]
pub(crate) async fn perform_epoch_change(
    dkg_manager: &DkgManager,
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64>>,
    realm_id: u64,
    is_shadow: bool,
    epoch_number: U256,
) -> Option<HashMap<String, Vec<CachedRootKey>>> {
    let peer_state = dkg_manager.tss_state.peer_state.clone();
    let mut fsm_worker_lifecycle_id = fsm_worker_metadata.get_lifecycle_id(realm_id);
    let mut latest_dkg_id = "".to_string();
    // We keep looping until we get a result from a completed epoch change operation.
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
            match get_dkg_peers_and_keysets(dkg_manager, realm_id, is_shadow).await {
                Ok((current_peers, new_peers)) => (current_peers, new_peers),
                Err(e) => {
                    warn!("get_current_next_dkg_peers failed: {}", e);
                    return None;
                }
            };

        let dkg_id = derive_dkg_id(realm_id, epoch_number, &new_peers, fsm_worker_lifecycle_id);
        latest_dkg_id = dkg_id.clone();

        // when you start with a shadow node, they are going to read the "original" key (from the src realm) ....
        let shadow_key_opts =
            get_shadow_key_opts(&peer_state, is_shadow, epoch_number, realm_id).await;
        if shadow_key_opts.0 == 0 && is_shadow {
            warn!(
                "Shadow realm is not ready yet, aborting the epoch change attempt #{}.",
                abort_and_restart_count
            );
            continue;
        }

        let current_epoch = epoch_number.as_u64();

        // If we are going to generate a new keyset, we need to ensure that the threshold of an existing keyset is not changed.
        // The exception to this is when the current peer set is not yet populated, in which case we can create one or more new keysets.
        // If the current peer set is not empty and not equivalent to the new peer set, we need to skip the DKG creation for this epoch.
        // let (key_sets_to_use, current_peers_to_use) = if empty_key_sets.is_empty() {
        //     (keysets, current_peers)
        // } else {
        //     if current_peers != new_peers && !current_peers.is_empty() {
        //         warn!(
        //             "When creating a new set of root keys, current peers should be empty or equivalent to new peers.  Skipping the DKG creation for this epoch."
        //         );
        //         (full_key_sets, current_peers)
        //     } else {
        //         (empty_key_sets, SimplePeerCollection(vec![]))
        //     }
        // };

        let (existing_key_sets, new_key_sets) = match get_existing_and_new_key_sets(
            peer_state.clone(),
        )
        .await
        {
            Ok((existing_key_sets, new_key_sets)) => (existing_key_sets, new_key_sets),
            Err(e) => {
                warn!(
                    "Unable to get existing and new key sets when performing epoch change in realm {}: {}",
                    realm_id, e
                );
                return None;
            }
        };

        trace!("New/existing key sets: {:?} / {:?}", new_key_sets.iter().map(|ks| ks.identifier.clone()).collect::<Vec<_>>(), existing_key_sets.iter().map(|ks| ks.identifier.clone()).collect::<Vec<_>>());

        // start by processing the epoch change for the new key sets
        let mut epoch_change_res_or_update_needed_for_new_keys = None;
        if !new_key_sets.is_empty() {
            trace!("Processing epoch change for new key sets");
            if current_peers != new_peers && !current_peers.is_empty() {
                warn!(
                    "When creating a new set of root keys, current peers should be empty or equivalent to new peers.  DKG will not be performed until the keyset is removed or the current peer set is equivalent to the new peer set."
                );
                return None;
            }
            let empty_peers = SimplePeerCollection(vec![]);
            epoch_change_res_or_update_needed_for_new_keys = match process_epoch_for_key_set(
                dkg_manager,
                fsm_worker_metadata.clone(),
                realm_id,
                is_shadow,
                &new_key_sets,
                &latest_dkg_id,
                current_epoch,
                shadow_key_opts,
                &empty_peers,
                &new_peers,
                None,
            )
            .await
            {
                Ok(result) => Some(result),
                Err(e) => {
                    warn!(
                        "Unable to process epoch change for new key sets when performing epoch change in realm {}: {}",
                        realm_id, e
                    );
                    return None;
                }
            };
        }

        trace!("Processing epoch change for existing key sets");
        let epoch_change_res_or_update_needed = match process_epoch_for_key_set(
            dkg_manager,
            fsm_worker_metadata.clone(),
            realm_id,
            is_shadow,
            &existing_key_sets,
            &latest_dkg_id,
            current_epoch,
            shadow_key_opts,
            &current_peers,
            &new_peers,
            epoch_change_res_or_update_needed_for_new_keys,
        )
        .await
        {
            Ok(result) => result,
            Err(e) => {
                warn!(
                    "Unable to process epoch change for existing key sets when performing epoch change in realm {}: {}",
                    realm_id, e
                );
                return None;
            }
        };

        let (post_current_peers, post_new_peers) =
            match get_dkg_peers_and_keysets(dkg_manager, realm_id, is_shadow).await {
                Ok((current_peers, new_peers)) => (current_peers, new_peers),
                Err(e) => {
                    error!(
                        "Error in get_current_next_dkg_peers in realm {}: {}",
                        realm_id, e
                    );
                    return None;
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
            return res;
        }

        // If we are here, that means that we need to update the lifecycle ID and restart the epoch change.
        let new_lifecycle_id = match epoch_change_res_or_update_needed.update_req {
            Some(new_lifecycle_id) => new_lifecycle_id,
            None => {
                error!("epoch_change_res_or_update_needed.update_req is None");
                return None;
            }
        };

        fsm_worker_metadata.update_lifecycle_id(Some(new_lifecycle_id), realm_id);

        let existing_epoch_number = match parse_epoch_number_from_dkg_id(&dkg_id) {
            Ok(existing_epoch_number) => existing_epoch_number,
            Err(e) => {
                error!("Error in parse_epoch_number_from_dkg_id: {}", e);
                return None;
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
    None
}

async fn process_epoch_for_key_set(
    dkg_manager: &DkgManager,
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64>>,
    realm_id: u64,
    is_shadow: bool,
    key_sets: &Vec<KeySetConfig>,
    latest_dkg_id: &str,
    current_epoch: u64,
    shadow_key_opts: (u64, u64),
    current_peers: &SimplePeerCollection,
    new_peers: &SimplePeerCollection,
    existing_epoch_change_res_or_update_needed: Option<EpochChangeResOrUpdateNeeded>,
) -> Result<EpochChangeResOrUpdateNeeded> {
    let existing_keys = match existing_epoch_change_res_or_update_needed {
        Some(existing_epoch_change_res_or_update_needed) => {
            match existing_epoch_change_res_or_update_needed.epoch_change_res {
                Some(existing_keys) => existing_keys,
                None => None,
            }
        }
        None => None,
    };

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

        res = dkg_manager.change_epoch(&latest_dkg_id, current_epoch, shadow_key_opts, realm_id, &current_peers, &new_peers, &key_sets) => {
            match res {
                Ok(res) => {
                let epoch = match dkg_manager.dkg_type {
                    DkgType::RecoveryParty => RECOVERY_DKG_EPOCH,
                    DkgType::Standard => current_epoch + 1,
                };

                let lifecycle_id = fsm_worker_metadata.get_lifecycle_id(realm_id);
                if false {
                    match key_share_proofs_check(&dkg_manager.tss_state, &res, &new_peers, &latest_dkg_id, realm_id, epoch, lifecycle_id).await {
                        Err(e) => {
                            warn!("Key share proofs check failed in realm {}: {}", realm_id, e);
                            return Err(e);
                        },
                        Ok(()) => {
                            debug!("Key share proofs check passed for realm {}", realm_id);
                        }
                    }
                }
                let mut res = res;
                if let Some(existing_keys) = existing_keys {
                    res.extend(existing_keys);
                }

                EpochChangeResOrUpdateNeeded {
                    epoch_change_res: Some(Some(res)),
                    update_req: None,
                }
                }
                Err(e) => {
                    error!("DKG error: {:?}", e);
                    return Err(e);
                }
            }

        }
    };

    Ok(epoch_change_res_or_update_needed)
}

async fn get_shadow_key_opts(
    peer_state: &PeerState,
    is_shadow: bool,
    epoch_number: U256,
    realm_id: u64,
) -> (u64, u64) {
    if is_shadow {
        let base_realm_id = peer_state.shadow_realm_id();
        let base_epoch_number = peer_state.get_epoch(base_realm_id).await;

        let base_epoch_number = match base_epoch_number {
            Ok(base_epoch_number) => base_epoch_number.1,
            Err(e) => {
                warn!(
                    "get_epoch failed for base epoch when shadow node is starting: {}",
                    e
                );
                return (0, 0);
            }
        };
        trace!("Base epoch number: {}", base_epoch_number);
        (base_epoch_number.as_u64(), base_realm_id)
    } else {
        (epoch_number.as_u64(), realm_id)
    }
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

pub async fn get_existing_and_new_key_sets(
    peer_state: Arc<PeerState>,
) -> Result<(Vec<KeySetConfig>, Vec<KeySetConfig>)> {
    // if there are any key sets that are empty, we need to generate new root keys for them.
    // we'll skip doing a regular DKG for already generated root keys / key sets during this epoch change.
    let cdm = &peer_state.chain_data_config_manager;
    let keysets = DataVersionReader::read_field_unchecked(&cdm.key_sets, |key_sets| {
        key_sets.values().cloned().collect::<Vec<_>>()
    });

    let mut new_key_sets = Vec::new();
    let mut existing_key_sets = Vec::new();
    for keyset in &keysets {
        let curve_type = CurveType::K256; // should inspect the keyset and determine the curve type
        let key_set_identifier = Some(keyset.identifier.clone());
        let curve_state = CurveState::new(peer_state.clone(), curve_type, key_set_identifier);
        let root_keys = curve_state.root_keys();

        // we assume that if some keys are present, then all keys are present.
        match root_keys {
            Ok(root_keys) => {
                if root_keys.is_empty() {
                    new_key_sets.push(keyset.clone());
                } else {
                    existing_key_sets.push(keyset.clone());
                }
            }
            Err(e) => {
                // this is temporary until we have a proper way to get the root keys from the chain.
                warn!("Error in getting root keys, thus key set {} will be treated as a new key set: {}", keyset.identifier, e);
                new_key_sets.push(keyset.clone());
            }
        }
    }

    trace!(
        "Full key sets: {:?}",
        existing_key_sets
            .iter()
            .map(|ks| ks.identifier.clone())
            .collect::<Vec<_>>()
    );
    trace!(
        "Empty key sets: {:?}",
        new_key_sets
            .iter()
            .map(|ks| ks.identifier.clone())
            .collect::<Vec<_>>()
    );
    Ok((existing_key_sets, new_key_sets))
}

pub async fn get_dkg_peers_and_keysets(
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
