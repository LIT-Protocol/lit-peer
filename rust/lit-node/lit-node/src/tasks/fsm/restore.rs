use crate::peers::peer_state::models::NetworkState;
use crate::tasks::fsm::utils::fsm_realm_id;
use crate::tss::common::restore::{NodeRecoveryStatus, RestoreState, report_progress};
use crate::tss::dkg::engine::DkgAfterRestoreData;
use crate::tss::dkg::{engine::DkgAfterRestore, manager::DkgManager};
use lit_node_common::config::{
    CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT, CFG_KEY_RESTORE_LOG_INTERVAL_MS_DEFAULT,
    LitNodeConfig,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

// Main FSM worker thread.
#[allow(clippy::too_many_arguments)]
pub async fn do_network_restore(
    quit_rx: &mut mpsc::Receiver<bool>,
    is_shadow: bool,
    restore_state: &Arc<RestoreState>,
    standard_dkg_manager: &mut DkgManager,
) {
    let peer_state = standard_dkg_manager.tss_state.peer_state.clone();

    let cfg = standard_dkg_manager.tss_state.lit_config.clone();

    let interval_ms = cfg
        .load_full()
        .chain_polling_interval_ms()
        .unwrap_or(CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT) as u64;
    let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));

    let realm_id = fsm_realm_id(&peer_state, is_shadow).await;

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

    info!("Entering restore state");
    report_progress(&cfg.load_full(), NodeRecoveryStatus::StartedInRestoreState).await;
    let restore_log_interval = cfg
        .load()
        .restore_log_interval()
        .unwrap_or(CFG_KEY_RESTORE_LOG_INTERVAL_MS_DEFAULT) as u64;
    let log_frequency_in_loop = restore_log_interval / interval_ms;
    let mut tick_counter = 0;
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

        tick_counter = match tick_counter >= log_frequency_in_loop {
            true => {
                info!(
                    "RestoreState: Not all the root keys are restored yet. \
                          Looping again with network_state: {:?}",
                    peer_state.network_state(realm_id).await
                );
                restore_state.log().await;
                0
            }
            false => tick_counter + 1,
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
        if let Ok(state) = peer_state.network_state(realm_id).await
            && state != NetworkState::Restore
            && state != NetworkState::Paused
        {
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
