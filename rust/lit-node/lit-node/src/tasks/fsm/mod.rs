pub mod epoch_change;
pub mod fsm_worker;
pub mod node_fsm_worker;
pub mod restore;
pub mod utils;

use crate::config::chain::CachedRootKey;
use crate::error::unexpected_err;
use crate::node_state::{NodeState, State, Transition};
use crate::peers::{PeerState, peer_state::models::NetworkState};
use crate::tasks::fsm::epoch_change::perform_epoch_change;
use crate::tasks::fsm::restore::do_network_restore;
use crate::tasks::fsm::utils::{
    check_version_compatibility, fsm_realm_id, get_current_and_new_peer_addresses,
};
use crate::tss::common::restore::RestoreState;
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use crate::tss::dkg::{engine::DkgAfterRestore, manager::DkgManager};
use ethers::types::U256;
use lit_blockchain::contracts::pubkey_router::RootKey;
use lit_core::config::ReloadableLitConfig;
use lit_core::error::Result;
use lit_core::utils::binary::hex_to_bytes;
use lit_node_common::{
    client_state::ClientState,
    config::{CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT, LitNodeConfig},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

// Main FSM worker thread.
#[allow(clippy::too_many_arguments)]
pub async fn node_fsm_worker(
    mut quit_rx: mpsc::Receiver<bool>,
    is_shadow: bool,
    restore_state: Arc<RestoreState>,
    client_state: Arc<ClientState>,
    recovery_dkg_manager: DkgManager,
    mut standard_dkg_manager: DkgManager,
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64>>,
) {
    let peer_state = standard_dkg_manager.tss_state.peer_state.clone();
    let cfg = standard_dkg_manager.tss_state.lit_config.clone();

    let mut root_keys = HashMap::<String, Vec<CachedRootKey>>::new();
    let mut epoch_to_signal_ready = U256::from(0);
    let mut previous_included_epoch_number = U256::from(0); // Any initial value will work
    let mut previous_retries = U256::from(0);
    let interval_ms = cfg
        .load_full()
        .chain_polling_interval_ms()
        .unwrap_or(CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT) as u64;

    // These are the state changing variables that are used throughout the FSM loop.
    let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));
    let realm_id = fsm_realm_id(&peer_state, is_shadow).await;
    // initialize the node state
    let mut node_state = NodeState::new();
    node_state.next(Transition::Init);

    // We're currently in a state where the FSM is doing many operations and is often taking longer than the interval tick.
    // Setting the missed tick behavior will ensure we maintain the interval between ticks whenever we're able to process
    // certain loops faster and have missed many prior ticks.
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    let initial_network_state = peer_state.network_state_or_unknown(realm_id).await;

    if !is_shadow {
        info!("Checking if we need to do a network restore...");
        // Restore the key store if the config is set so. If the config did not set it, assume new network initialization and do not attempt to restore.
        // We don't restore shadow nodes because they don't have a key store.
        if initial_network_state == NetworkState::Restore {
            do_network_restore(
                &mut quit_rx,
                is_shadow,
                &restore_state,
                &mut standard_dkg_manager,
            )
            .await;
        }
    }

    info!(
        "Starting: FSM polling (will try every {}s).  Shadow State: {is_shadow}. Realm ID: {realm_id}, Current Network State: {:?}",
        interval.period().as_secs(),
        initial_network_state
    );

    // Main FSM Loop
    loop {
        // Check if we should quit, or continue with the next check of FSM.
        tokio::select! {
            _ = quit_rx.recv() => {
                info!("Stopped: PeerState and BlsState poller");
                return;
            }
            _ = interval.tick() => {
                // Continue below.
            }
        }

        // get basic FSM state information and check if we can proceed with this loop.
        let (realm_id, epoch_number, network_state, retries, can_proceed) =
            match get_fsm_state(&peer_state, &node_state, is_shadow, &fsm_worker_metadata).await {
                Ok(fsm_state) => fsm_state,
                Err(e) => {
                    warn!("get_fsm_state failed: {}", e);
                    continue;
                }
            };

        // if we're not able to proceed, based on some checks while getting the fsm state, just do another loop.
        if !can_proceed {
            continue;
        }

        // If we're not part of the validators union - attempt to join, do some logging and restart our loop.
        if !peer_state.part_of_validators_union() {
            handle_not_part_of_validators_union(
                &peer_state,
                &mut node_state,
                is_shadow,
                epoch_number,
                previous_included_epoch_number,
                realm_id,
            )
            .await;
            continue;
        }

        // set epoch_to_signal_ready to the current epoch if uninitialized (aka == 0)
        if epoch_to_signal_ready == U256::from(0) {
            epoch_to_signal_ready = epoch_number;
        }

        // because the retry counter only increases in the staking contract and is never reset to 0
        if previous_retries == U256::from(0) {
            previous_retries = retries;
        };

        // This is the main point for functions that deal with nodes that are part of the current active set.
        let current_state = node_state.current_state();
        trace!(
            "Epoch (prior/current): {previous_included_epoch_number} / {epoch_number} - node state: {:?} - epoch_to_signal_ready: {epoch_to_signal_ready} - network state: {:?}",
            current_state, network_state
        );

        // based on the current node state, perform the appropriate action.
        match current_state {
            // if we're in the node set, and we're in the suspended state, we should move to Online
            // so that we can participate in the epoch transition
            State::Suspended => {
                node_state.next(Transition::Rejoin);
            }

            // This is "normal processing state" for the node.  Handle a few different scenarios here.
            State::Active | State::Online => {
                // Check to ensure the nodes are still running compatible versions.
                if !verify_version_compatibiliy(&peer_state, current_state).await {
                    continue;
                }

                // if the epoch seems to have jumped, we need to figure out why and handle it.
                if epoch_number > previous_included_epoch_number {
                    // this could be the state if we haven't checked the chain - check it and continue.
                    if network_state == NetworkState::NextValidatorSetLocked {
                        wait_on_next_validator_set_locked(
                            &peer_state,
                            &mut node_state,
                            current_state,
                        )
                        .await;
                    }
                } else if retries > previous_retries {
                    // this happens if the node thinks it finished reshare, but the node set changed after locking
                    // reset previous_included_epoch_number because it was incremented in anticipation of the epoch transitioning, but
                    if previous_included_epoch_number == epoch_number {
                        previous_included_epoch_number = epoch_number - U256::from(1);
                    }
                    previous_retries = retries;
                    debug!("Retrying: Staying in Active or Online state to retry");
                    continue;
                }
            }

            // this is the state we're in after the DKG for the epoch transition has completed, but we haven't voted for keys or signaled ready yet.
            State::PendingActive => {
                // certain cases don't require signaling ( we're late, or not returning!)
                if !need_to_signal_ready(
                    &peer_state,
                    &mut node_state,
                    realm_id,
                    epoch_number,
                    epoch_to_signal_ready,
                )
                .await
                {
                    continue;
                }

                // if we've loaded things up, and we have an epoch, vote for any keys.
                check_root_key_voting(&mut root_keys, &cfg, &peer_state, epoch_number).await;

                // Attempt to signal ready for next epoch and check if we can continue, or loop again.
                if !can_continue_after_signal_ready(
                    &peer_state,
                    realm_id,
                    epoch_number,
                    epoch_to_signal_ready,
                    retries,
                    &mut previous_retries,
                    &mut previous_included_epoch_number,
                    &mut node_state,
                )
                .await
                {
                    continue;
                }
            }

            State::Locked => {
                // Perform a DKG only when the backup party in the nextState has changed which happens when `registerNewBackupParty` is called
                if !check_recovery_dkg_complete(
                    &peer_state,
                    epoch_number,
                    &recovery_dkg_manager,
                    fsm_worker_metadata.clone(),
                    is_shadow,
                    realm_id,
                )
                .await
                {
                    debug!("Recovery DKG is not complete. Not performing Standard DKG.");
                    continue;
                }

                // Clear the root keys and set the previous included epoch number and epoch to signal ready
                previous_included_epoch_number = epoch_number;
                epoch_to_signal_ready = epoch_number;

                // Attempt to perform the epoch change
                let epoch_change_results = perform_epoch_change(
                    &standard_dkg_manager,
                    fsm_worker_metadata.clone(),
                    realm_id,
                    is_shadow,
                    epoch_number,
                )
                .await;

                // Get the root keys from the epoch change results
                root_keys = match epoch_change_results {
                    Some(root_keys) => root_keys,
                    None => {
                        debug!("root_keys_result == None for realm {}", realm_id);
                        continue;
                    }
                };

                standard_dkg_manager.next_dkg_after_restore = DkgAfterRestore::False;
                client_state.rotate_identity_keys();
                node_state.next(Transition::Complete);
                debug!("Node state (t:complete) : {:?}", node_state.current_state());
            }

            // unhandled?
            state => {
                debug!("Unhandled node state (part of union): {:?}", state);
                continue;
            }
        }

        // attempt to lock validators if we're in a state that can do so
        if network_state == NetworkState::Active || network_state == NetworkState::Unlocked {
            peer_state
                .rpc_lock_validators_for_next_epoch(realm_id)
                .await;
        }

        // attempt to advance the epoch if we're in a state that can do so
        if network_state == NetworkState::ReadyForNextEpoch {
            peer_state.rpc_advance_epoch().await
        }
    }
}

// Verify that the node is running a compatible version.
async fn verify_version_compatibiliy(peer_state: &Arc<PeerState>, current_state: State) -> bool {
    let is_node_running_compatible_version =
        match check_version_compatibility(peer_state.clone()).await {
            Ok(is_node_running_compatible_version) => is_node_running_compatible_version,
            Err(e) => {
                error!("Error in check_version_compatibility: {}", e);
                return false;
            }
        };

    match current_state {
        State::Online => {
            // The online validator node should do nothing if it is running a soon-to-be / already incompatible node version.
            if !is_node_running_compatible_version {
                info!("Node is running an incompatible version and will do nothing.");
                return false;
            }
        }
        State::Active => {
            // The active validator node should request to leave if it is running a soon-to-be / already incompatible node version.
            if !is_node_running_compatible_version {
                info!("Node is running an incompatible version. Requesting to leave.");
                if let Err(e) = peer_state.request_to_leave().await {
                    error!("Error in request_to_leave: {}", e);
                }
                return false;
            }
        }
        _ => {}
    }

    true
}

#[allow(clippy::too_many_arguments)]
async fn can_continue_after_signal_ready(
    peer_state: &Arc<PeerState>,
    realm_id: u64,
    epoch_number: U256,
    epoch_to_signal_ready: U256,
    retries: U256,
    previous_retries: &mut U256,
    previous_included_epoch_number: &mut U256,
    node_state: &mut NodeState,
) -> bool {
    let res = peer_state
        .rpc_signal_ready_for_next_epoch(epoch_to_signal_ready, realm_id)
        .await;

    if res.is_err() {
        error!("Failed to signal ready for next epoch! Error: {:?}.", res);
        // if we failed to signal ready here, then it's likely because the transition failed, and we should retry
        if retries > *previous_retries {
            // reset previous_included_epoch_number because it was incremented in anticipation of the epoch transitioning, but
            // the epoch didn't transition, and instead we are retrying
            if *previous_included_epoch_number == epoch_number {
                *previous_included_epoch_number = epoch_number - U256::from(1);
            }
            *previous_retries = retries;
            node_state.next(Transition::Retry);
            debug!(
                "Retrying: Moving from PendingActive to Active for realm {}",
                realm_id
            );
            return false;
        }
    } else {
        // since this takes awhile, we're going to check if our signal ready vote was canceled by the network switching state.
        tokio::time::sleep(Duration::from_secs(2)).await;
        let is_ready = match peer_state
            .get_ready_signal(realm_id, peer_state.staker_address)
            .await
        {
            Ok(is_ready) => is_ready,
            Err(e) => {
                error!("Error in get_ready_signal: {}", e);
                return false;
            }
        };

        if !is_ready {
            info!(
                "Signal ready vote in realm {realm_id} was cancelled by the network switching state. No longer moving to complete.",
            );
            return false;
        }

        node_state.next(Transition::Complete);
        trace!(
            "Node state in realm {realm_id} (pending->complete) after signal ready for epoch {epoch_number} : {:?}",
            node_state.current_state()
        );
    }

    true
}

async fn wait_on_next_validator_set_locked(
    peer_state: &Arc<PeerState>,
    node_state: &mut NodeState,
    current_state: State,
) {
    // Here we have detected a chain state update. Let's forcibly update the CDM before we transition.
    // This is important because we need to make sure that the CDM is up to date before we start doing epoch change
    // operations in the Locked state.
    match peer_state
        .chain_data_config_manager
        .set_peer_and_epoch_data_from_chain()
        .await
    {
        Ok(_) => {
            trace!("Updated chain data manager state");
        }
        Err(e) => {
            error!("Failed to update chain data manager state: {:?}", e);
        }
    }
    if current_state == State::Active {
        node_state.next(Transition::Incumbent);
        debug!("Moving from Active node to Locked!");
    } else {
        node_state.next(Transition::Selected);
        debug!("Moving from Online node to Locked!");
    }
}

async fn need_to_signal_ready(
    peer_state: &Arc<PeerState>,
    node_state: &mut NodeState,
    realm_id: u64,
    epoch_number: U256,
    epoch_to_signal_ready: U256,
) -> bool {
    // don't do both signaling ready for next epoch - enough of threshold already did, and calling again will fail.
    if epoch_number > epoch_to_signal_ready {
        node_state.next(Transition::Complete);
        trace!(
            "Node state in realm {realm_id} (pending->complete) with epoch: {epoch_number} and epoch to signal: {epoch_to_signal_ready} : {:?}",
            node_state.current_state()
        );
        return false;
    }

    if !peer_state
        .peers_in_next_epoch()
        .contains_address(&peer_state.addr)
    {
        trace!(
            "This node is not in the next epoch (in realm {realm_id}).  Not signaling ready for next epoch.",
        );
        return false;
    }

    true
}

pub async fn get_fsm_state(
    peer_state: &Arc<PeerState>,
    node_state: &NodeState,
    is_shadow: bool,
    fsm_worker_metadata: &Arc<dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64>>,
) -> Result<(u64, U256, NetworkState, U256, bool)> {
    let realm_id = fsm_realm_id(peer_state, is_shadow).await;
    // if we're not yet assigned to a realm, just do another loop.
    if realm_id == 0 {
        return Ok((
            0,
            U256::from(0),
            NetworkState::Unknown,
            U256::from(0),
            false,
        ));
    }

    // Update worker metadata  - merely a convenient place to update the metadata.
    if is_shadow {
        fsm_worker_metadata.set_shadow_realm_id(realm_id);
    };
    fsm_worker_metadata.update_lifecycle_id(None, realm_id);

    let network_state = peer_state
        .network_state(realm_id)
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get network_state".into())))?;

    // FIXME: Check for errors.
    let epoch = peer_state
        .get_epoch(realm_id)
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get epoch".into())))?;
    let epoch_number = epoch.1;
    let retries = epoch.3;

    let block_number = peer_state
        .get_block_number()
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get block_number".into())))?;

    let peers_in_epoch = peer_state.peers();

    debug!(
        "Block: {} Epoch: {}, Network state: {:?}, Node state: {:?}, Retries: {:?}, Peers: {:?} ",
        block_number,
        epoch_number,
        network_state,
        node_state.current_state(),
        retries,
        peers_in_epoch.debug_addresses(),
    );

    // if we're paused, just do another loop.
    if network_state == NetworkState::Paused {
        info!("Network state is Paused.  Pausing FSM node state polling.");
        return Ok((realm_id, epoch_number, network_state, retries, false));
    }

    Ok((realm_id, epoch_number, network_state, retries, true))
}

pub async fn check_root_key_voting(
    root_keys: &mut HashMap<String, Vec<CachedRootKey>>,
    cfg: &ReloadableLitConfig,
    peer_state: &Arc<PeerState>,
    epoch_number: U256,
) {
    if !root_keys.is_empty() && epoch_number >= U256::from(1) {
        match vote_for_root_pubkeys(cfg, root_keys, peer_state).await {
            Ok(result) => {
                if !result {
                    info!("vote_for_root_pubkeys returned false");
                    return;
                }

                root_keys.clear();
            }
            Err(e) => {
                warn!(
                    "Current Validators: {:?}",
                    peer_state.validators_in_current_epoch()
                );
            }
        }
    }
}

pub async fn vote_for_root_pubkeys(
    cfg: &ReloadableLitConfig,
    pubkeys: &HashMap<String, Vec<CachedRootKey>>,
    peer_state: &Arc<PeerState>,
) -> Result<bool> {
    use crate::pkp::utils::vote_for_root_key;
    use ethers::core::types::Bytes;

    info!("incoming root pubkeys: {:?}", pubkeys);

    let mut res = true;
    for (key_set_id, dkg_root_key) in pubkeys {
        let mut root_keys: Vec<RootKey> = Vec::with_capacity(dkg_root_key.len());
        for key in dkg_root_key {
            let pk_bytes = hex_to_bytes(&key.public_key)?;
            let rootkey = RootKey {
                pubkey: Bytes::from(pk_bytes),
                key_type: key.curve_type.into(),
            };
            root_keys.push(rootkey);
        }
        info!("Root Keys to vote for: {:?}", root_keys);
        res &= vote_for_root_key(&cfg.load_full(), key_set_id, root_keys, peer_state).await?;
    }
    Ok(res)
}

pub async fn handle_not_part_of_validators_union(
    peer_state: &Arc<PeerState>,
    node_state: &mut NodeState,
    is_shadow: bool,
    epoch_number: U256,
    previous_included_epoch_number: U256,
    realm_id: u64,
) {
    let current_state = { node_state.current_state() };
    trace!(
        "Not part_of_validators_union; node_state: {:?}",
        current_state
    );

    match current_state {
        State::Online => {
            info!(
                "Node is online, but not part of the current validators.  Waiting for next epoch to join.  Preloading Peer State Data."
            );
            let _ = get_current_and_new_peer_addresses(is_shadow, peer_state.clone()).await;
            // requesting to auto rejoin if online but not part of the current validators
            if peer_state.auto_join {
                match peer_state.request_to_join().await {
                    Ok(_) => info!("Auto requested to join the network"),
                    Err(e) => error!("Error in request_to_join: {}", e),
                }
            }
        }

        State::Active | State::PendingActive | State::Locked => {
            if let Err(e) = peer_state.validators_in_active_state(realm_id).await {
                error!("Error in handle_if_validators_in_active_state: {}", e);
            }
            node_state.next(Transition::Leave)
        }
        State::Suspended => {
            if epoch_number > previous_included_epoch_number + 1 {
                node_state.next(Transition::Rejoin)
            }
        }
        // unhandled?
        state => {
            trace!(
                "Not part_of_validators_union; unhandled node_state: {:?}",
                state
            );
        }
    }
}

// The recovery DKG is only performed in the first epoch and when the backup party is not empty
async fn check_recovery_dkg_complete(
    peer_state: &Arc<PeerState>,
    epoch_number: U256,
    recovery_dkg_manager: &DkgManager,
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64>>,
    is_shadow: bool,
    realm_id: u64,
) -> bool {
    // We don't do a backup in the first epoch as there's nothing to back up
    let not_first_epoch = epoch_number > U256::from(1);

    let backup_party_not_empty = match peer_state.backup_party_not_empty().await {
        Ok(backup_party_not_empty) => backup_party_not_empty,
        Err(e) => {
            error!("Failed to call getNextBackupPartyMembers w/ err {:?}", e);
            return false;
        }
    };

    let is_recovery_dkg_completed = match peer_state.is_recovery_dkg_registered().await {
        Ok(is_recovery_dkg_completed) => is_recovery_dkg_completed,
        Err(e) => {
            error!("Failed to call isRecoveryDkgCompleted w/ err {:?}", e);
            return false;
        }
    };

    let potentially_participate_in_recover_dkg =
        not_first_epoch && backup_party_not_empty && !is_recovery_dkg_completed;

    if potentially_participate_in_recover_dkg {
        match peer_state.set_recovery_dkg_member().await {
            Ok(_) => info!("called setMemberForDkg"),
            Err(e) => {
                error!("Fail to set member for recovery DKG w/ err {:?}", e);
                return false;
            }
        };

        // Check whether we're participating in the Recovery DKG
        let do_recovery_dkg = match peer_state.is_node_mapped().await {
            Ok(do_recovery_dkg) => do_recovery_dkg,
            Err(e) => {
                error!("Failed to call for isNodeForDkg w/ err {:?}", e);
                return false;
            }
        };

        if do_recovery_dkg {
            info!("Doing Recovery DKG!");

            let recovery_keys_result = perform_epoch_change(
                recovery_dkg_manager,
                fsm_worker_metadata.clone(),
                realm_id,
                is_shadow,
                epoch_number,
            )
            .await;

            // NOTE: We can't continue until it's registered on chain as other nodes won't know that the Recovery DKG is completed so they should keep on waiting
            let recovery_keys_result = match recovery_keys_result {
                Some(recovery_keys) => recovery_keys,
                None => {
                    debug!("recovery_keys_result == None in realm: {}", realm_id);
                    return false;
                }
            };

            // NOTE?: Assume ECDSA DKG pass when BLS passes just like Standard DKG
            peer_state
                .register_recovery_keys(recovery_keys_result)
                .await;
        }
    }

    let is_recovery_dkg_completed = match peer_state.is_recovery_dkg_registered().await {
        Ok(is_recovery_dkg_completed) => is_recovery_dkg_completed,
        Err(e) => {
            error!(
                "Can't proceed to Standard DKG, failed to call isRecoveryDkgCompleted w/ err {:?}",
                e
            );
            return false;
        }
    };

    // Don't perform Standard DKG until the Recovery DKG is successful
    if not_first_epoch && backup_party_not_empty && !is_recovery_dkg_completed {
        info!("Recovery DKG isn't completed. Checking again in the next FSM iteration");
        return false;
    }

    true
}
