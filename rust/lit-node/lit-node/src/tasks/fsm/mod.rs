pub mod epoch_change;
pub mod node_fsm_worker;
pub mod utils;

use crate::config::chain::CachedRootKey;
use crate::error::unexpected_err;
use crate::node_state::{NodeState, State, Transition};
use crate::peers::{PeerState, peer_state::models::NetworkState};
use crate::tasks::fsm::epoch_change::perform_epoch_change;
use crate::tasks::fsm::utils::{
    check_version_compatibility, fsm_realm_id, get_current_and_new_peer_addresses,
};
use crate::tss::common::restore::{NodeRecoveryStatus, RestoreState, report_progress};
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use crate::tss::dkg::engine::DkgAfterRestoreData;
use crate::tss::dkg::{engine::DkgAfterRestore, manager::DkgManager};
use ethers::types::U256;
use lit_blockchain::contracts::pubkey_router::RootKey;
use lit_core::config::ReloadableLitConfig;
use lit_core::error::Result;
use lit_core::utils::binary::hex_to_bytes;
use lit_node_common::{
    client_state::ClientState,
    config::{
        CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT, CFG_KEY_RESTORE_LOG_INTERVAL_MS_DEFAULT,
        LitNodeConfig,
    },
};
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

    let mut root_keys: Vec<CachedRootKey> = Vec::new();
    let mut epoch_to_signal_ready = U256::from(0);
    let mut previous_included_epoch_number = U256::from(0); // Any initial value will work
    let mut previous_retries = U256::from(0);
    let interval_ms = cfg
        .load_full()
        .chain_polling_interval_ms()
        .unwrap_or(CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT) as u64;
    let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));

    let realm_id = fsm_realm_id(peer_state.clone(), is_shadow).await;

    // We're currently in a state where the FSM is doing many operations and is often
    // taking longer than the interval tick. Setting the missed tick behavior will ensure
    // we maintain the interval between ticks whenever we're able to process certain loops
    // faster and have missed many prior ticks.
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    let initial_network_state = peer_state
        .network_state(realm_id)
        .await
        .unwrap_or(NetworkState::Unknown);

    info!(
        "Starting: FSM polling (will try every {}s).  Shadow State: {}. Realm ID: {}, Current Network State: {:?}",
        interval.period().as_secs(),
        is_shadow,
        realm_id,
        initial_network_state
    );

    // Restore the key store if the config is set so. If the config did not set it,
    // assume new network initialization and do not attempt to restore.
    // We don't restore shadow nodes because they don't have a key store.
    if initial_network_state == NetworkState::Restore {
        info!("Entering restore state");
        report_progress(&cfg.load_full(), NodeRecoveryStatus::StartedInRestoreState).await;
        let restore_log_interval =
            cfg.load()
                .restore_log_interval()
                .unwrap_or(CFG_KEY_RESTORE_LOG_INTERVAL_MS_DEFAULT) as u64;
        let log_frequency_in_loop = restore_log_interval / interval_ms;
        let mut thick_counter = 0;
        info!("RestoreState: NetworkState is in Restore. Attempting to restore.");
        let staker_address = &peer_state.hex_staker_address();
        loop {
            if let Err(e) = restore_state
                .prepare_for_recovery(standard_dkg_manager.tss_state.clone())
                .await
            {
                error!("unable to prepare for recovery, {}", e.to_string());
                tokio::time::sleep(Duration::from_secs(log_frequency_in_loop)).await;
                continue;
            }
            break;
        }

        // Try to restore the key shares until all the key shares are restored.
        loop {
            // Check if we should quit, or continue.
            tokio::select! {
                _ = quit_rx.recv() => {
                    info!("RestoreState: Received signal, exiting");
                    return;
                }
                _ = interval.tick() => {
                    // Continue below.
                }
            }

            // Check that the blinders are set in the RestoreState.
            let blinders = restore_state.get_blinders();
            if !blinders.are_blinders_set() {
                info!("RestoreState: No blinders found. Skipping restore and looping.");
                continue;
            }

            let current_peer_id = match peer_state.peer_id_in_current_epoch() {
                Ok(peer_id) => peer_id,
                Err(e) => {
                    error!("Error getting peer_id_in_current_epoch: {}", e);
                    continue;
                }
            };

            // Try to restore the key shares under the read lock.
            let newly_recovered_key_shares = {
                let epoch = match peer_state.get_epoch(realm_id).await {
                    Ok(epoch) => epoch.1.as_u64(),
                    Err(e) => {
                        error!("Error getting epoch: {}", e);
                        continue;
                    }
                };

                match peer_state.peer_id_in_current_epoch() {
                    Ok(peer_id) => {
                        restore_state
                            .try_restore_key_shares(&peer_id, epoch, staker_address, realm_id)
                            .await
                    }
                    Err(e) => {
                        error!("Error getting peer_id_in_current_epoch: {}", e);
                        continue;
                    }
                }
            };

            // Mark these keys as recovered.
            // We are doing this separately because the process of recovering keys
            // is long, so we do it with a read lock. We take the write lock
            // only to set their boolean `restored` flags as true.
            restore_state
                .mark_keys_restored(&newly_recovered_key_shares)
                .await;

            thick_counter = match thick_counter >= log_frequency_in_loop {
                true => {
                    info!(
                        "RestoreState: Not all the root keys are restored yet. \
                          Looping again with network_state: {:?}",
                        peer_state.network_state(realm_id).await
                    );
                    restore_state.log().await;
                    0
                }
                false => thick_counter + 1,
            };

            // Check if the restoration is complete for this node.
            if restore_state.are_all_keys_restored().await {
                let cfg = cfg.load_full();
                info!("RestoreState: All the root keys are restored");
                if let Err(e) = restore_state
                    .report_recovered_peer_id(&cfg, current_peer_id)
                    .await
                {
                    error!(
                        "RestoredState: Failed to report the recovered peer id: {}",
                        e
                    );
                    // Try again
                    continue;
                }
                standard_dkg_manager
                    .tss_state
                    .set_threshold(restore_state.get_restored_threshold().await);
                let key_cache = match restore_state.pull_recovered_key_cache().await {
                    Ok(key_cache) => key_cache,
                    Err(e) => {
                        error!(
                            "RestoredState: Failed to retrieve restored keys and commitments: {}",
                            e
                        );
                        continue;
                    }
                };
                standard_dkg_manager.next_dkg_after_restore =
                    DkgAfterRestore::True(DkgAfterRestoreData {
                        peers: vec![],
                        key_cache,
                    });

                report_progress(&cfg, NodeRecoveryStatus::AllKeysAreRestored).await;
                break;
            }
            info!(
                "RestoreState: Not all the root keys are restored yet. Looping again with network_state: {:?}",
                peer_state.network_state(realm_id).await
            );

            // Check if the restoration is complete for the network.
            match peer_state.network_state(realm_id).await {
                Ok(NetworkState::Restore) | Ok(NetworkState::Paused) => {}
                Ok(s) => {
                    info!("RestoreState: network state changed: {:?}", s);
                    report_progress(
                        &cfg.load_full(),
                        NodeRecoveryStatus::AbandonedRecoveryDueToNetworkState,
                    )
                    .await;
                    // Remove the stored key cache if one exists since recovery is abandoned
                    standard_dkg_manager.next_dkg_after_restore.take();
                    break;
                }
                Err(_) => {}
            };

            // TODO: Gather all `newly_recovered_key_shares` over the previous loop
            // (not only within the last iteration, but all iterations). Check if
            // they are already registered as root keys in the staking contract.
            // If not, register them as root keys. Make sure they are added as
            // additional keys and do not override the existing root keys.
        }

        // The loop is over--the restoration is complete.
        // Clear the restore state and generate new blinders.
        info!("RestoreState: Clearing Restore State");
        restore_state.clear().await;

        // Wait until the network is active again
        loop {
            if let Ok(state) = peer_state.network_state(realm_id).await {
                if state != NetworkState::Restore && state != NetworkState::Paused {
                    let recovered_peer_ids = match restore_state
                        .pull_recovered_peer_ids(&cfg.load_full())
                        .await
                    {
                        Ok(ids) => ids,
                        Err(e) => {
                            error!(
                                "RestoredState: Failed to read the recovered peer ids: {}",
                                e
                            );
                            // Try again
                            continue;
                        }
                    };
                    let data = match standard_dkg_manager.next_dkg_after_restore.take() {
                        Some(mut data) => {
                            data.peers = recovered_peer_ids;
                            data
                        }
                        None => DkgAfterRestoreData {
                            peers: recovered_peer_ids,
                            ..Default::default()
                        },
                    };
                    standard_dkg_manager.next_dkg_after_restore = DkgAfterRestore::True(data);

                    info!("RestoreState: Exiting recovery code, starting the fsm loop.");
                    break;
                }
            }
        }
    }

    // initialize the node state
    let mut node_state = NodeState::new();
    node_state.next(Transition::Init);

    info!("Starting main FSM loop");
    // Main FSM Loop
    loop {
        // Check if we should quit, or continue with the next check of FSM.
        tokio::select! {
            _ = quit_rx.recv() => {
                info!("Stopped: PeerState and BlsState poller");
                return;
            }
            _ = interval.tick() => {
                // Pad the interval with a 2s sleep - if the loop took longer than the interval time, the next loop
                // will start immediately, which is not what we want. This is only a short-term fix, and we should
                // aim to make the loop lighter weight and faster.
                tokio::time::sleep(Duration::from_secs(2)).await;

                // Continue below.
            }
        }

        let realm_id = fsm_realm_id(peer_state.clone(), is_shadow).await;

        if realm_id == 0 {
            if !is_shadow {
                // don't log for shadow nodes, since they seldom occur.
                trace!("Node is not yet assigned to a realm.  Waiting for realm assignment.");
            }
            continue;
        }

        // Update worker metadata
        if is_shadow {
            fsm_worker_metadata.set_shadow_realm_id(realm_id);
        }

        // Update worker metadata
        fsm_worker_metadata.update_lifecycle_id(None, realm_id);

        let lifecycle_id = fsm_worker_metadata.get_lifecycle_id(realm_id);

        let (epoch_number, network_state, retries) =
            match get_fsm_state(&peer_state, &node_state, realm_id).await {
                Ok(fsm_state) => fsm_state,
                Err(e) => {
                    error!("Error in get_fsm_state: {}", e);
                    continue;
                }
            };

        // because the retry counter only increases in the staking contract
        // and is never reset to 0
        if previous_retries == U256::from(0) {
            previous_retries = retries;
        };

        // if we're paused, just do another loop.
        if network_state == NetworkState::Paused {
            info!("Network state is Paused.  Pausing FSM node state polling.");
            continue;
        }

        // If the network is Restoring, and we are not, this process quits.
        if network_state == NetworkState::Restore {
            std::process::exit(0);
        }

        // set epoch_to_signal_ready to the current epoch if uninitialized (aka == 0)
        if epoch_to_signal_ready == U256::from(0) {
            epoch_to_signal_ready = epoch_number;
        }

        // the flow is different if we're waiting to be part of the network, or an actual current member.
        match peer_state.part_of_validators_union() {
            // functions/transitions to check if we're not part of the current validators
            false => {
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
                        let _ =
                            get_current_and_new_peer_addresses(is_shadow, peer_state.clone()).await;
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
                        continue;
                    }
                }
            }

            // This is the main point for functions that deal with nodes that are part of the current active set.
            true => {
                // Locked & next epoch? As before
                let current_state = node_state.current_state();
                trace!(
                    "Epoch (prior/current): {} / {} - node state: {:?} - epoch_to_signal_ready: {} - network state: {:?}",
                    previous_included_epoch_number,
                    epoch_number,
                    current_state,
                    epoch_to_signal_ready,
                    network_state
                );

                match current_state {
                    State::Suspended => {
                        // if we're in the node set, and we're in the suspended state, we should move to Online
                        // so that we can participate in the epoch transition
                        node_state.next(Transition::Rejoin);
                    }
                    State::Active | State::Online => {
                        let is_node_running_compatible_version =
                            match check_version_compatibility(peer_state.clone()).await {
                                Ok(is_node_running_compatible_version) => {
                                    is_node_running_compatible_version
                                }
                                Err(e) => {
                                    error!("Error in check_version_compatibility: {}", e);
                                    continue;
                                }
                            };

                        match current_state {
                            State::Online => {
                                // The online validator node should do nothing if it is running a soon-to-be / already incompatible node version.
                                if !is_node_running_compatible_version {
                                    info!(
                                        "Node is running an incompatible version and will do nothing."
                                    );
                                    continue;
                                }
                            }
                            State::Active => {
                                // The active validator node should request to leave if it is running a soon-to-be / already incompatible node version.
                                if !is_node_running_compatible_version {
                                    info!(
                                        "Node is running an incompatible version. Requesting to leave the network."
                                    );
                                    if let Err(e) = peer_state.request_to_leave().await {
                                        error!("Error in request_to_leave: {}", e);
                                    }
                                    continue;
                                }
                            }
                            _ => {}
                        }

                        if epoch_number > previous_included_epoch_number {
                            if network_state == NetworkState::NextValidatorSetLocked {
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
                                        error!(
                                            "Failed to update chain data manager state: {:?}",
                                            e
                                        );
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
                        } else if retries > previous_retries {
                            // check if we should retry
                            // this happens if the node thinks it finished reshare, but the
                            // contract is forcing a retry because the node set changed after locking

                            // reset previous_included_epoch_number because it was incremented in anticipation of the epoch transitioning, but
                            // the epoch didn't transition, and instead we are retrying
                            if previous_included_epoch_number == epoch_number {
                                previous_included_epoch_number = epoch_number - U256::from(1);
                            }
                            previous_retries = retries;
                            debug!("Retrying: Staying in Active or Online state to retry");
                            continue;
                        }
                    }

                    State::PendingActive => {
                        //FIXME-> this is debugging work:
                        set_peer_state_debugging_data(peer_state.clone(), is_shadow, realm_id)
                            .await;

                        let sigst = peer_state.clone();
                        // don't do both signaling ready for next epoch - enough of threshold already did, and calling again will fail.
                        if epoch_number > epoch_to_signal_ready {
                            node_state.next(Transition::Complete);
                            trace!(
                                "Node state in realm {} (pending->complete) with epoch: {} and epoch to signal: {} : {:?}",
                                realm_id,
                                epoch_number,
                                epoch_to_signal_ready,
                                node_state.current_state()
                            );
                            continue;
                        }

                        if !peer_state
                            .peers_in_next_epoch()
                            .contains_address(&peer_state.addr)
                        {
                            trace!(
                                "This node is not in the next epoch (in realm {}).  Not signaling ready for next epoch.",
                                realm_id
                            );
                            continue;
                        }

                        // if we've loaded things up, and we have an epoch, vote for any keys.
                        check_root_key_voting(&mut root_keys, &cfg, &peer_state, epoch_number)
                            .await;

                        let res = sigst
                            .rpc_signal_ready_for_next_epoch(epoch_to_signal_ready, realm_id)
                            .await;

                        if res.is_err() {
                            error!("Failed to signal ready for next epoch! Error: {:?}.", res);
                            // if we failed to signal ready here, then it's likely because
                            // the transition failed, and we should retry
                            if retries > previous_retries {
                                // reset previous_included_epoch_number because it was incremented in anticipation of the epoch transitioning, but
                                // the epoch didn't transition, and instead we are retrying
                                if previous_included_epoch_number == epoch_number {
                                    previous_included_epoch_number = epoch_number - U256::from(1);
                                }
                                previous_retries = retries;
                                node_state.next(Transition::Retry);
                                debug!(
                                    "Retrying: Moving from PendingActive to Active for realm {}",
                                    realm_id
                                );
                                continue;
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
                                    continue;
                                }
                            };

                            if !is_ready {
                                info!(
                                    "Signal ready vote in realm {} was cancelled by the network switching state. No longer moving to complete.",
                                    realm_id
                                );
                                continue;
                            }

                            node_state.next(Transition::Complete);
                            trace!(
                                "Node state in realm {} (pending->complete) after signal ready for epoch {} : {:?}",
                                realm_id,
                                epoch_number,
                                node_state.current_state()
                            );
                        }
                    }

                    State::Locked => {
                        previous_included_epoch_number = epoch_number;
                        epoch_to_signal_ready = epoch_number;

                        // We don't do a backup in the first epoch as there's nothing to back up
                        let not_first_epoch = epoch_number > U256::from(1);

                        // Perform a DKG only when the backup party in the nextState has changed which happens when `registerNewBackupParty` is called
                        let backup_party_not_empty = match peer_state.backup_party_not_empty().await
                        {
                            Ok(backup_party_not_empty) => backup_party_not_empty,
                            Err(e) => {
                                error!("Failed to call getNextBackupPartyMembers w/ err {:?}", e);
                                continue;
                            }
                        };

                        let is_recovery_dkg_completed =
                            match peer_state.is_recovery_dkg_registered().await {
                                Ok(is_recovery_dkg_completed) => is_recovery_dkg_completed,
                                Err(e) => {
                                    error!("Failed to call isRecoveryDkgCompleted w/ err {:?}", e);
                                    continue;
                                }
                            };

                        let potentially_participate_in_recover_dkg =
                            not_first_epoch && backup_party_not_empty && !is_recovery_dkg_completed;

                        if potentially_participate_in_recover_dkg {
                            match peer_state.set_recovery_dkg_member().await {
                                Ok(_) => info!("called setMemberForDkg"),
                                Err(e) => {
                                    error!("Fail to set member for recovery DKG w/ err {:?}", e);
                                    continue;
                                }
                            };

                            // Check whether we're participating in the Recovery DKG
                            let do_recovery_dkg = match peer_state.is_node_mapped().await {
                                Ok(do_recovery_dkg) => do_recovery_dkg,
                                Err(e) => {
                                    error!("Failed to call for isNodeForDkg w/ err {:?}", e);
                                    continue;
                                }
                            };

                            if do_recovery_dkg {
                                info!("Doing Recovery DKG!");

                                let recovery_keys_result = perform_epoch_change(
                                    &recovery_dkg_manager,
                                    fsm_worker_metadata.clone(),
                                    realm_id,
                                    is_shadow,
                                    epoch_number,
                                )
                                .await;

                                let recovery_keys_result = match recovery_keys_result {
                                    Ok(recovery_keys_result) => recovery_keys_result,
                                    Err(e) => {
                                        error!("Error in perform_epoch_change: {}", e);
                                        continue;
                                    }
                                };

                                // NOTE: We can't continue until it's registered on chain as other nodes won't know that the Recovery DKG is completed so they should keep on waiting
                                let recovery_keys_result = match recovery_keys_result {
                                    Some(recovery_keys) => recovery_keys,
                                    None => {
                                        debug!(
                                            "recovery_keys_result == None in realm: {}",
                                            realm_id
                                        );
                                        continue;
                                    }
                                };

                                // NOTE?: Assume ECDSA DKG pass when BLS passes just like Standard DKG
                                peer_state
                                    .register_recovery_keys(recovery_keys_result)
                                    .await;
                            }
                        }

                        let is_recovery_dkg_completed = match peer_state
                            .is_recovery_dkg_registered()
                            .await
                        {
                            Ok(is_recovery_dkg_completed) => is_recovery_dkg_completed,
                            Err(e) => {
                                error!(
                                    "Can't proceed to Standard DKG, failed to call isRecoveryDkgCompleted w/ err {:?}",
                                    e
                                );
                                continue;
                            }
                        };

                        // Don't perform Standard DKG until the Recovery DKG is successful
                        if not_first_epoch && backup_party_not_empty && !is_recovery_dkg_completed {
                            info!(
                                "Recovery DKG isn't completed. Checking again in the next FSM interation"
                            );
                            continue;
                        }

                        // if we start again, and this in the initial epoch, we need to clear the root_keys
                        root_keys.clear();
                        let epoch_change_results = perform_epoch_change(
                            &standard_dkg_manager,
                            fsm_worker_metadata.clone(),
                            realm_id,
                            is_shadow,
                            epoch_number,
                        )
                        .await;

                        let root_keys_result = match epoch_change_results {
                            Ok(root_keys_result) => root_keys_result,
                            Err(e) => {
                                error!("Error in perform_epoch_change: {}", e);
                                continue;
                            }
                        };

                        let root_keys_result = match root_keys_result {
                            Some(root_keys) => root_keys,
                            None => {
                                debug!("root_keys_result == None for realm {}", realm_id);
                                continue;
                            }
                        };

                        root_keys.extend(root_keys_result); // NOTE?: Assume all DKGs pass when one passes
                        // if we've gone through the nodes and appended all the keys (if required), we can signal ready.

                        standard_dkg_manager.next_dkg_after_restore = DkgAfterRestore::False;
                        client_state.rotate_identity_keys();
                        node_state.next(Transition::Complete);
                        debug!("Node state (t:complete) : {:?}", node_state.current_state());
                    }

                    // unhandled?
                    state => {
                        debug!(
                            "Part_of_validators_union; but unhandled node_state: {:?}",
                            state
                        );
                        continue;
                    }
                }

                //FIXME-> this is debugging work:
                set_peer_state_debugging_data(peer_state.clone(), is_shadow, realm_id).await;

                // attempt to lock validators if we're in a state that can do so
                if network_state == NetworkState::Active || network_state == NetworkState::Unlocked
                {
                    peer_state
                        .rpc_lock_validators_for_next_epoch(realm_id)
                        .await;
                }

                // attempt to advance the epoch if we're in a state that can do so
                if network_state == NetworkState::ReadyForNextEpoch {
                    let r_epoch = peer_state.rpc_advance_epoch().await;
                }
            }
        }
    }
}

pub async fn get_fsm_state(
    peer_state: &Arc<PeerState>,
    node_state: &NodeState,
    realm_id: u64,
) -> Result<(U256, NetworkState, U256)> {
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

    Ok((epoch_number, network_state, retries))
}

pub async fn check_root_key_voting(
    root_keys: &mut Vec<CachedRootKey>,
    cfg: &ReloadableLitConfig,
    peer_state: &Arc<PeerState>,
    epoch_number: U256,
) {
    if !root_keys.is_empty() && epoch_number >= U256::from(1) {
        match vote_for_root_pubkeys(cfg, root_keys.clone(), peer_state).await {
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
    pubkeys: Vec<CachedRootKey>,
    peer_state: &Arc<PeerState>,
) -> Result<bool> {
    use crate::pkp::utils::vote_for_root_key;
    use ethers::core::types::Bytes;

    info!("incoming root pubkeys: {:?}", pubkeys);

    let mut root_keys: Vec<RootKey> = Vec::new();

    for dkg_root_key in pubkeys {
        let pk_bytes = hex_to_bytes(&dkg_root_key.public_key)?;
        let rootkey = RootKey {
            pubkey: Bytes::from(pk_bytes),
            key_type: dkg_root_key.curve_type.into(),
        };
        root_keys.push(rootkey);
    }

    info!("Root Keys to vote for: {:?}", root_keys);
    vote_for_root_key(&cfg.load_full(), root_keys, peer_state).await
}

// FIXME: This is debugging work.  Remove when done.
pub async fn set_peer_state_debugging_data(
    peer_state: Arc<PeerState>,
    is_shadow: bool,
    realm_id: u64,
) {
    let next_peers = match is_shadow {
        true => peer_state.peers_in_next_shadow_epoch(),
        false => peer_state.peers_in_next_epoch(),
    };

    let mut m1 = "Validators signaled ready (".to_string();
    let mut m2 = "".to_string();

    for v in next_peers.active_peers().0 {
        m1.push_str(&format!("{},", v.socket_address));
        let is_peer_ready_for_next_epoch = match peer_state
            .get_ready_signal(realm_id, v.staker_address)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                error!("Error in get_ready_signal: {}", e);
                return;
            }
        };
        m2.push_str(&format!("{},", is_peer_ready_for_next_epoch,));
    }
    trace!("{}): {}", m1, m2);

    let current_validator_count_for_consensus = match peer_state
        .current_validator_count_for_consensus(realm_id)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            error!("Error in current_validator_count_for_consensus: {}", e);
            return;
        }
    };

    let next_validator_count_for_consensus = match peer_state
        .next_validator_count_for_consensus(realm_id)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            error!("Error in next_validator_count_for_consensus: {}", e);
            return;
        }
    };

    let count_of_validators_ready_for_next_epoch = match peer_state
        .get_count_of_validators_ready_for_next_epoch(realm_id)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            error!(
                "Error in get_count_of_validators_ready_for_next_epoch: {}",
                e
            );
            return;
        }
    };

    trace!(
        "Req'd consensus for realm {} (current/next) : {:?}/{:?} - Validators ready for next epoch {:?}",
        realm_id,
        current_validator_count_for_consensus,
        next_validator_count_for_consensus,
        count_of_validators_ready_for_next_epoch,
    );
}
