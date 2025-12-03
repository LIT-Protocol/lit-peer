use super::super::PeerState;
use super::models::NetworkState;
use crate::error::{EC, Result, unexpected_err_code};
use crate::tasks::presign_manager::models::PresignMessage;
use ethers::providers::StreamExt;
use lit_blockchain::contracts::staking::StakingEvents;
use rocket::serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;

#[allow(dead_code)]
impl PeerState {
    pub async fn listen_for_events(
        self: &Arc<Self>,
        mut quit_rx: mpsc::Receiver<bool>,
    ) -> Result<()> {
        let events = self.staking_contract.events();
        let mut stream = events
            .stream()
            .await
            .map_err(|e| unexpected_err_code(e, EC::NodeUnknownError, None))?;

        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    info!("Received quit signal, exiting event listener");
                    break;
                }

                event = stream.next() => {
                    if let Some(event) = event {
                        match event {
                            Ok(log) => {
                                trace!("New event in event listener: {:?}", log);
                                match log.clone() {
                                    StakingEvents::StateChangedFilter(state_changed_event) => {
                                        let state = NetworkState::from(state_changed_event.new_state);
                                        if state == NetworkState::Unknown {
                                            error!("Unknown state retrieved from the staking contract");
                                            continue;
                                        }
                                        debug!("New state: {:?}", state);
                                        if state == NetworkState::NextValidatorSetLocked {
                                            // update peers
                                            // this will log any errors so we can skip the error handling here
                                        } else if state == NetworkState::Active {
                                            // update chain data manager state
                                            match self.chain_data_config_manager.set_peer_and_epoch_data_from_chain().await {
                                                Ok(_) => {
                                                    trace!("Updated chain data manager state");
                                                }
                                                Err(e) => {
                                                    error!("Failed to update chain data manager state: {:?}", e);
                                                }
                                            }
                                        }
                                    }
                                    StakingEvents::RequestToJoin1Filter(request_to_join_event) => {
                                        // update chain data manager state
                                        match self.chain_data_config_manager.set_peer_and_epoch_data_from_chain().await {
                                            Ok(_) => {
                                                trace!("Updated chain data manager state");
                                            }
                                            Err(e) => {
                                                error!("Failed to update chain data manager state: {:?}", e);
                                            }
                                        }
                                    }

                                    StakingEvents::RequestToLeaveFilter(request_to_leave_event) => {
                                        // update chain data manager state
                                        match self.chain_data_config_manager.set_peer_and_epoch_data_from_chain().await {
                                            Ok(_) => {
                                                trace!("Updated chain data manager state");
                                            }
                                            Err(e) => {
                                                error!("Failed to update chain data manager state: {:?}", e);
                                            }
                                        }
                                    }
                                    StakingEvents::ValidatorKickedFromNextEpochFilter(
                                        validator_kicked_event,
                                    ) => {
                                        debug!("ValidatorKickedFromNextEpoch event");
                                        // try to rejoin if kicked
                                        if validator_kicked_event.staker == self.staker_address &&  self.auto_join {
                                            match self.request_to_join().await {
                                                Ok(_) => trace!("Auto requested to rejoin the network"),
                                                Err(e) => error!("Error in request_to_join: {}", e),
                                            }
                                        }
                                    }
                                    StakingEvents::ValidatorRejoinedNextEpochFilter(
                                        validator_kicked_event,
                                    ) => {
                                        debug!("ValidatorRejoinedNextEpoch event");
                                        // update peers
                                    }
                                    StakingEvents::VersionRequirementsUpdatedFilter(
                                        version_requirements_updated_event,
                                    ) => {
                                        debug!("VersionRequirementsUpdated event");
                                        // update chain data manager state
                                        match self.chain_data_config_manager.set_version_requirements(Some((
                                            version_requirements_updated_event.index,
                                            version_requirements_updated_event.version,
                                        ))).await {
                                            Ok(_) => {
                                                trace!("Updated version requirements");
                                            }
                                            Err(e) => {
                                                error!("Failed to update version requirements: {:?}", e);
                                            }
                                        }
                                    }
                                    StakingEvents::ConfigSetFilter(
                                        global_config_set_event,
                                    ) => {
                                        debug!("Global Config event");
                                        // update CDM state
                                        if let Err(e) = self.chain_data_config_manager.set_all_config_from_chain().await {
                                            error!("Failed to update chain data manager state: {:?}", e);
                                        }
                                    }

                                    StakingEvents::CountOfflinePhaseDataFilter(data_type) => {
                                        debug!("CountOfflinePhaseData event: {:?}", data_type);
                                        let _r = self.ps_tx.send_async(PresignMessage::Count).await;
                                    }
                                    StakingEvents::ClearOfflinePhaseDataFilter(data_type) => {
                                        debug!("ClearOfflinePhaseData event: {:?}", data_type);
                                        let _r = self.ps_tx.send_async(PresignMessage::Clear).await;
                                    }
                                    _ => {}

                                }
                            }
                            Err(e) => {
                                error!("Error in event listener: {:?}", e)
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerValidatorStatus {
    Entering, // Not in current, but in locked next
    Exiting,  // in current, but not in locked next
    Survivor, // in both
    Unknown,
}
