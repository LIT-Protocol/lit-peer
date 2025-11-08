use crate::config::chain::CachedRootKey;
use crate::error::unexpected_err;
use crate::node_state::{NodeState, State, Transition};
use crate::p2p_comms::CommsManager;
use crate::peers::peer_reviewer::{Issue, PeerComplaint};
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::peers::{PeerState, peer_state::models::NetworkState};
use crate::tss::common::dkg_type::DkgType;
use crate::tss::common::key_persistence::RECOVERY_DKG_EPOCH;
use crate::tss::common::restore::{NodeRecoveryStatus, RestoreState, report_progress};
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use crate::tss::common::tss_state::TssState;
use crate::tss::dkg::engine::DkgAfterRestoreData;
use crate::tss::dkg::{engine::DkgAfterRestore, manager::DkgManager};
use crate::utils::key_share_proof::{
    KeyShareProofs, compute_key_share_proofs, verify_key_share_proofs,
};
use crate::version::get_version;
use ethers::types::U256;
use lit_blockchain::contracts::pubkey_router::RootKey;
use lit_blockchain::contracts::staking::Version;
use lit_core::config::ReloadableLitConfig;
use lit_core::error::{Result, Unexpected};
use lit_core::utils::binary::hex_to_bytes;
use lit_node_common::{
    client_state::ClientState,
    config::{
        CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT, CFG_KEY_RESTORE_LOG_INTERVAL_MS_DEFAULT,
        LitNodeConfig,
    },
};
use lit_node_core::CurveType;
use lit_node_core::PeerId;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::instrument;

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

    // really more pertinent for nodes coming online with the network already up.
    if let Err(e) = peer_state.connect_to_validators_union().await {
        error!("Error in connect_to_validators_union: {}", e);
    }

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
            match peer_state.connect_to_validators_union().await {
                Ok(_) => {}
                Err(e) => {
                    error!("Error in connect_to_validators_union: {}", e);
                }
            }
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
                            match peer_state.connect_to_validators_union().await {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Error in connect_to_validators_union: {}", e);
                                }
                            }
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

// only log the epoch number field
#[instrument(level = "debug", skip(dkg_manager, fsm_worker_metadata))]
async fn perform_epoch_change(
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

pub async fn get_current_and_new_peer_addresses(
    is_shadow: bool,
    peer_state: Arc<PeerState>,
) -> Result<(SimplePeerCollection, SimplePeerCollection)> {
    peer_state.connect_to_validators_union().await?;

    let (current_peers, new_peers) = if is_shadow {
        (
            peer_state.peers_in_current_shadow_epoch(),
            peer_state.peers_in_next_shadow_epoch(),
        )
    } else {
        (
            peer_state.peers(),
            peer_state.peers_in_next_epoch().active_peers(),
        )
    };

    let shadow_text = if is_shadow { "shadow" } else { "main" };
    let realm_id = match is_shadow {
        false => peer_state.realm_id(),
        true => peer_state.shadow_realm_id(),
    };

    debug!(
        "Current/new peers for {} realm {} epoch change: ( {}/{} )  {} / {} ",
        shadow_text,
        realm_id,
        &current_peers.0.len(),
        &new_peers.0.len(),
        &current_peers.debug_addresses(),
        &new_peers.debug_addresses(),
    );

    debug!(
        "Validators in realm {} for next epoch are locked: {} ",
        peer_state.peers_in_next_epoch().realm_id()?,
        peer_state
            .validators_for_next_epoch_locked(realm_id)
            .await?,
    );

    Ok((current_peers, new_peers))
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

fn parse_epoch_number_from_dkg_id<T>(dkg_id: T) -> Result<U256>
where
    T: AsRef<str>,
{
    let dkg_id = dkg_id.as_ref();
    let epoch_number = dkg_id
        .split('_')
        .nth(2)
        .expect_or_err("Failed to parse epoch number")?;
    let epoch_number_u128 = epoch_number
        .parse::<u128>()
        .expect_or_err("Failed to parse epoch number as u128")?;
    Ok(U256::from(epoch_number_u128))
}

async fn check_version_compatibility(peer_state: Arc<PeerState>) -> Result<bool> {
    let min_valid_version = peer_state
        .chain_data_config_manager
        .get_min_version_requirement()
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get min version requirement".into())))?;
    let max_valid_version = peer_state
        .chain_data_config_manager
        .get_max_version_requirement()
        .await
        .map_err(|e| unexpected_err(e, Some("Could not get max version requirement".into())))?;
    is_compatible_version(
        &get_version().to_string(),
        min_valid_version,
        max_valid_version,
    )
}

fn is_compatible_version(
    version: &str,
    min_valid_version: Version,
    max_valid_version: Version,
) -> Result<bool> {
    trace!(
        "Checking version compatibility: version: {}, min_valid_version: {:?}, max_valid_version: {:?}",
        version, min_valid_version, max_valid_version
    );

    // Parse version (e.g. "0.2.14"), otherwise known as NODE_VERSION_UNMARKED!
    let version_parts = version.split('.').collect::<Vec<&str>>();
    if version_parts.len() != 3 {
        return Err(unexpected_err(
            format!("Invalid version: {}", version),
            None,
        ));
    }
    let curr_major = U256::from_dec_str(version_parts[0]).map_err(|e| unexpected_err(e, None))?;
    let curr_minor = U256::from_dec_str(version_parts[1]).map_err(|e| unexpected_err(e, None))?;
    let curr_patch = U256::from_dec_str(version_parts[2]).map_err(|e| unexpected_err(e, None))?;

    // If the min_valid_version is set to default values, that means the version is not set on-chain, so we should not check against
    // the minimum version requirement.
    if min_valid_version != Version::default()
        && (curr_major < min_valid_version.major
            || (curr_major == min_valid_version.major && curr_minor < min_valid_version.minor)
            || (curr_major == min_valid_version.major
                && curr_minor == min_valid_version.minor
                && curr_patch < min_valid_version.patch))
    {
        return Ok(false);
    }

    // If the max_valid_version is set to default values, that means the version is not set on-chain, so we should not check against
    // the maximum version requirement.
    if max_valid_version != Version::default()
        && (curr_major > max_valid_version.major
            || (curr_major == max_valid_version.major && curr_minor > max_valid_version.minor)
            || (curr_major == max_valid_version.major
                && curr_minor == max_valid_version.minor
                && curr_patch > max_valid_version.patch))
    {
        return Ok(false);
    }

    Ok(true)
}

async fn fsm_realm_id(peer_state: Arc<PeerState>, is_shadow: bool) -> u64 {
    if is_shadow {
        peer_state.shadow_realm_id()
    } else {
        peer_state.realm_id()
    }
}

async fn key_share_proofs_check(
    tss_state: &Arc<TssState>,
    root_key_res: &Result<Vec<CachedRootKey>>,
    peers: &SimplePeerCollection,
    latest_dkg_id: &str,
    realm_id: u64,
    epoch: u64,
    lifecycle_id: u64,
) -> Result<()> {
    if !peers.contains_address(&tss_state.addr) {
        trace!("Peer not in next epoch, skipping key share proofs check");
        return Ok(()); // no need to compute key share proofs
    }

    let mut root_keys = Vec::new();
    if let Ok(rk) = root_key_res {
        if !rk.is_empty() {
            root_keys = rk.clone();
        }
    }
    if root_keys.is_empty() {
        root_keys = tss_state.chain_data_config_manager.root_keys();
    }

    trace!("Root keys for key share proofs: {:?}", root_keys);
    let mut root_keys_map = HashMap::<CurveType, Vec<String>>::with_capacity(root_keys.len());
    for root_key in root_keys {
        root_keys_map
            .entry(root_key.curve_type)
            .and_modify(|v| v.push(root_key.public_key.clone()))
            .or_insert(vec![root_key.public_key.clone()]);
    }

    let noonce = format!("{}-{}", epoch, lifecycle_id);
    trace!("Key share proofs nonce signed: {}", noonce);

    let proofs = compute_key_share_proofs(
        &noonce,
        &root_keys_map,
        &tss_state.addr,
        peers,
        realm_id,
        epoch,
    )
    .await?;
    trace!("Key share proofs generated");

    let txn_prefix = format!(
        "KEYSHAREPROOFS_{}-{}_1_{}_{}",
        epoch,
        lifecycle_id,
        peers.hash(),
        realm_id
    );

    let cm = CommsManager::new_with_peers(tss_state, &txn_prefix, peers, "10").await?;

    let received: Vec<(PeerId, KeyShareProofs)> = cm.broadcast_and_collect(&proofs).await?;
    trace!("Received key share proofs: {}", received.len());

    let mut any_failed = false;
    for (peer_id, key_share_proofs) in received {
        trace!(
            "Key share proofs for peer: {} - {}",
            peer_id,
            key_share_proofs.proofs.len()
        );
        let peer = peers.peer_by_id(&peer_id)?;
        let res = verify_key_share_proofs(
            &root_keys_map,
            &noonce,
            &tss_state.addr,
            &peer.socket_address,
            &tss_state.peer_state.hex_staker_address(),
            &key_share_proofs,
            peers,
            epoch,
            realm_id,
        )
        .await?;

        for (curve, result) in res {
            if result.is_err() {
                if !any_failed {
                    any_failed = true;
                    error!(
                        "Key share proof verification failed for peer {} - curve {}: {:?} - complaining",
                        peer.socket_address, curve, result
                    );
                    tss_state
                        .peer_state
                        .complaint_channel
                        .send_async(PeerComplaint {
                            complainer: tss_state.peer_state.addr.clone(),
                            issue: Issue::KeyShareValidationFailure(curve),
                            peer_node_staker_address: peer.staker_address,
                            peer_node_socket_address: peer.socket_address.clone(),
                        })
                        .await
                        .map_err(|e| unexpected_err(e, Some("Unable to complain".to_string())))?;
                } else {
                    error!(
                        "Key share proof verification failed for peer {} - curve {}: {:?} - already complainted for this DKG.",
                        peer.socket_address, curve, result
                    );
                }
            }
        }
    }
    if any_failed {
        return Err(unexpected_err(
            "Key share proof verification failed".to_string(),
            None,
        ));
    }
    trace!("Valid key share proofs");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::is_compatible_version;
    use crate::tasks::{fsm::parse_epoch_number_from_dkg_id, utils::parse_version};
    use crate::version::get_unmarked_version;
    use lit_blockchain::contracts::staking::Version;

    struct TestCase {
        node_version: String,
        min_valid_version: Version,
        max_valid_version: Version,
        expected_result: bool,
    }

    #[test]
    fn test_version_compatibility() {
        let test_cases = get_version_compability_test_cases();
        for (i, test_case) in test_cases.iter().enumerate() {
            let min_valid_version = test_case.min_valid_version.clone();
            let max_valid_version = test_case.max_valid_version.clone();
            let result = is_compatible_version(
                &test_case.node_version,
                min_valid_version,
                max_valid_version,
            )
            .expect("Failed to check version compatibility");
            assert_eq!(
                result,
                test_case.expected_result,
                "Test case {} failed",
                i + 1
            );
        }
    }

    fn get_version_compability_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            // Test patch version
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.2.13").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.2.15").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.2.13").expect("Unable to parse version"),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.2.13").expect("Unable to parse version"),
                max_valid_version: parse_version("0.2.15").expect("Unable to parse version"),
                expected_result: true,
            },
            // Test minor version
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.1.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.3.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.1.14").expect("Unable to parse version"),
                expected_result: false,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.3.14").expect("Unable to parse version"),
                expected_result: true,
            },
            TestCase {
                node_version: get_unmarked_version().to_string(),
                min_valid_version: parse_version("0.1.14").expect("Unable to parse version"),
                max_valid_version: parse_version("0.3.14").expect("Unable to parse version"),
                expected_result: true,
            },
            // Test major version
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: true,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: parse_version("2.2.14").expect("Unable to parse version"),
                max_valid_version: Version::default(),
                expected_result: false,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                expected_result: false,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: Version::default(),
                max_valid_version: parse_version("2.2.14").expect("Unable to parse version"),
                expected_result: true,
            },
            TestCase {
                node_version: "1.2.14".to_string(),
                min_valid_version: parse_version("0.2.14").expect("Unable to parse version"),
                max_valid_version: parse_version("2.2.14").expect("Unable to parse version"),
                expected_result: true,
            },
        ]
    }

    #[test]
    fn test_parse_epoch_number() {
        let epoch_number = parse_epoch_number_from_dkg_id("EPOCH_DKG_10_151")
            .expect("Failed to parse epoch number");
        assert_eq!(epoch_number.as_usize(), 10usize);
    }
}
