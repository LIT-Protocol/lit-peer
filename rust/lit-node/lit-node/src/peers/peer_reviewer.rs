use crate::metrics;
use ethers::types::{Address, Bytes, U256};
use lit_blockchain::util::decode_revert;
use lit_core::error::Unexpected;
use lit_observability::channels::TracedReceiver;
use lit_observability::opentelemetry::KeyValue;
use moka::Expiry;
use moka::future::Cache;
use moka::notification::RemovalCause;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{Instrument, instrument};

use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::ReloadableLitConfig;

use crate::config::chain::ChainDataConfigManager;
use crate::error::Result;
use crate::peers::peer_state::models::NetworkState;
use lit_node_core::CurveType;

use super::PeerState;

// keep this updated with the max Issue value below
pub const MAX_COMPLAINT_REASON_VALUE: u8 = 4;

#[derive(Debug)]
pub enum Issue {
    /// This is when a peer is not responding by sending packets back over the network.
    Unresponsive,
    /// This is when a peer is not participating in a protocol.
    NonParticipation,
    /// This is when peer data returned from their handshake does not match the data on chain.
    IncorrectInfo,
    Error {
        err: anyhow::Error,
    },
    /// This is when a peer's key share fails validation.
    KeyShareValidationFailure(CurveType),
}

impl Issue {
    pub fn value(&self) -> u8 {
        match *self {
            Issue::Unresponsive => 1,
            Issue::NonParticipation => 2,
            Issue::IncorrectInfo => 3,
            Issue::KeyShareValidationFailure(_) => 4,
            _ => 5,
        }
    }
}

impl PartialEq for Issue {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

#[derive(Debug)]
pub struct PeerComplaint {
    pub complainer: String,
    pub issue: Issue,
    pub peer_node_staker_address: Address,
    pub peer_node_socket_address: String,
}

#[derive(Debug)]
/// PeerComplaints tracks all the complaints against a single peer for various reasons.
pub struct PeerComplaintsTracker {
    complaint_reason_to_count: Cache<u8, ExpiringCounter>,
}

#[derive(Debug, Clone)]
struct ExpiringCounter {
    counter: u64,
    expiry_after_create_secs: u64,
}

pub struct PeerReviewer {
    rx: TracedReceiver<PeerComplaint>,

    /// A map of peer keys to their respective complaints tracker.
    peer_key_to_complaints_tracker: HashMap<String, PeerComplaintsTracker>,

    config: Arc<ReloadableLitConfig>,
    chain_data_manager: Arc<ChainDataConfigManager>,
    peer_state: Arc<PeerState>,
}

struct ComplaintExpiry;

impl Expiry<u8, ExpiringCounter> for ComplaintExpiry {
    fn expire_after_create(
        &self,
        key: &u8,
        value: &ExpiringCounter,
        created_at: std::time::Instant,
    ) -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs(
            value.expiry_after_create_secs,
        ))
    }
}

// TODO: We should be implementing a proper rolling window for each complaint reason.
impl PeerReviewer {
    pub fn new(rx: TracedReceiver<PeerComplaint>, peer_state: Arc<PeerState>) -> PeerReviewer {
        PeerReviewer {
            rx,
            peer_key_to_complaints_tracker: HashMap::new(),
            config: peer_state.lit_config.clone(),
            chain_data_manager: peer_state.chain_data_config_manager.clone(),
            peer_state,
        }
    }

    // Should be spawned with a channel, with tx handles given to all tasks that need to complain
    pub async fn receive_complaints(&mut self, mut quit_rx: mpsc::Receiver<bool>) {
        // Start a timer and clear the issue counter every interval seconds
        info!("Starting: PeerReviewer::receive_complaints");

        trace!("Getting interval limit");
        let config = self.config.load_full();
        let resolver =
            ContractResolver::try_from(config.as_ref()).expect("failed to load ContractResolver");

        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    break;
                }
                Ok((chan_msg, span)) = self.rx.recv_async() => {
                    let complaint = chan_msg.data();
                    info!("Received complaint ({complaint:?})");
                    if let Err(e) = self.remember_complaint(complaint).instrument(span).await {
                        error!("Failed to remember complaint: {:?}", e);
                    }
                }
            }
        }

        info!("Stopped: PeerReviewer::receive_complaints");
    }

    /// If any of the complaints have reached the threshold for that corresponding reason, escalate the complaint.
    pub async fn remember_complaint(&mut self, complaint: &PeerComplaint) -> Result<()> {
        let realm_id = self.peer_state.realm_id();
        let network_state = match self.peer_state.network_state(realm_id).await {
            Ok(state) => state,
            Err(e) => {
                error!(
                    "Error getting network state in peer_checker_worker: {:?}",
                    e
                );
                return Err(e);
            }
        };
        if network_state == NetworkState::Restore || network_state == NetworkState::Paused {
            info!("Network state is {:?}. Skipping complaint.", network_state);
            return Ok(()); // don't complain about peers if we are restoring or paused
        }

        let peer_key = complaint.peer_node_staker_address;
        let peer_key_address = complaint.peer_node_socket_address.clone();
        let peer_complaints_tracker = match self
            .peer_key_to_complaints_tracker
            .get_mut(&peer_key.to_string())
        {
            Some(tracker) => tracker,
            None => {
                let eviction_listener = move |key: Arc<u8>, _value, cause: RemovalCause| {
                    trace!("Evicted key {key}. Cause: {cause:?}");
                    metrics::counter::add_one(
                        metrics::complaint::ComplaintMetrics::ComplaintCacheEvicted,
                        &[
                            KeyValue::new(
                                metrics::complaint::ATTRIBUTE_COMPLAINT_REASON,
                                format!("{:?}", key),
                            ),
                            KeyValue::new(
                                metrics::complaint::ATTRIBUTE_EVICTION_CAUSE,
                                format!("{:?}", cause),
                            ),
                            KeyValue::new(
                                metrics::complaint::ATTRIBUTE_PEER_KEY,
                                peer_key.to_string(),
                            ),
                        ],
                    );
                };
                let tracker = PeerComplaintsTracker {
                    complaint_reason_to_count: Cache::builder()
                        .expire_after(ComplaintExpiry)
                        .eviction_listener(eviction_listener)
                        .build(),
                };
                self.peer_key_to_complaints_tracker
                    .insert(peer_key.to_string(), tracker);
                self.peer_key_to_complaints_tracker
                    .get_mut(&peer_key.to_string())
                    .expect_or_err("Failed to insert tracker")?
            }
        };
        let cdm_clone = self.chain_data_manager.clone();
        let peer_key_entry = peer_complaints_tracker
            .complaint_reason_to_count
            .entry(complaint.issue.value())
            .and_upsert_with(|maybe_entry| async {
                match maybe_entry {
                    Some(entry) => {
                        // The entry exists, increment the value by 1.
                        let expiring_counter = entry.into_value();
                        ExpiringCounter {
                            counter: expiring_counter.counter.saturating_add(1),
                            expiry_after_create_secs: expiring_counter.expiry_after_create_secs,
                        }
                    }
                    None => {
                        // The entry does not exist, insert a new value of 1 with the latest expiry.
                        ExpiringCounter {
                            counter: 1,
                            expiry_after_create_secs: get_complaint_reason_interval_secs(
                                cdm_clone,
                                complaint.issue.value(),
                            )
                            .await,
                        }
                    }
                }
            })
            .await;
        let number_of_complaints = peer_key_entry.value();

        let complaint_reason_tolerance = {
            let complaint_config = self
                .chain_data_manager
                .complaint_reason_to_config
                .get(&U256::from(complaint.issue.value()))
                .await
                .expect_or_err(format!(
                    "No config found for complaint reason number {} which contains {:?}",
                    complaint.issue.value(),
                    complaint.issue
                ))?;
            complaint_config.tolerance
        };

        info!(
            "Remembering complaint #{:?} for peer {:?} ({}).  Tolerance is {:?}",
            number_of_complaints.counter, peer_key, peer_key_address, complaint_reason_tolerance
        );

        metrics::counter::add_one(
            metrics::complaint::ComplaintMetrics::ComplaintRemembered,
            &[
                KeyValue::new(
                    metrics::complaint::ATTRIBUTE_COMPLAINT_REASON,
                    format!("{:?}", complaint.issue.value()),
                ),
                KeyValue::new(metrics::complaint::ATTRIBUTE_PEER_KEY, peer_key.to_string()),
            ],
        );
        if number_of_complaints.counter >= complaint_reason_tolerance {
            self.escalate_complaint(complaint).await;
        }

        Ok(())
    }

    // post to blockchain
    #[instrument(level = "debug", skip(self))]
    pub async fn escalate_complaint(&self, complaint: &PeerComplaint) {
        info!("Escalating complaint ({:?})", complaint);
        let config = self.config.load_full();
        let resolver =
            ContractResolver::try_from(config.as_ref()).expect("failed to load ContractResolver");

        match self
            .peer_state
            .staking_contract
            .kick_validator_in_next_epoch(
                complaint.peer_node_staker_address,
                U256::from(complaint.issue.value()),
                Bytes::from(vec![]),
            )
            .send()
            .await
        {
            Ok(_) => {
                warn!("voted to kick peer. Final complaint was: {:?}", complaint);
                metrics::counter::add_one(
                    metrics::complaint::ComplaintMetrics::VotedToKick,
                    &[
                        KeyValue::new(
                            metrics::complaint::ATTRIBUTE_COMPLAINT_REASON,
                            format!("{:?}", complaint.issue.value()),
                        ),
                        KeyValue::new(
                            metrics::complaint::ATTRIBUTE_PEER_KEY,
                            complaint.peer_node_staker_address.to_string(),
                        ),
                    ],
                );
            }
            // NOTE: the below is trace because inactive nodes also kickvote, but the TX will revert which is intended
            Err(e) => trace!(
                "failed to vote to kick peer w/ err {:?}. Final complaint was {:?}",
                decode_revert(&e, self.peer_state.staking_contract.abi()),
                complaint
            ),
        }
    }
}

async fn get_complaint_reason_interval_secs(
    chain_data_manager: Arc<ChainDataConfigManager>,
    reason: u8,
) -> u64 {
    match chain_data_manager
        .complaint_reason_to_config
        .get(&U256::from(reason))
        .await
    {
        Some(complaint_config) => complaint_config.interval_secs,
        None => {
            error!("Complaint config not found - using default expiry of 60s.");
            60
        }
    }
}

// commented out until fixed or replaced
// #[cfg(test)]
// mod tests {
//     use std::path::PathBuf;

//     use ethers::types::Address;
//     use lit_core::config::{LitConfigBuilder, ReloadableLitConfig};
//     use tokio::sync::mpsc;

//     use super::{Issue, PeerComplaint, PeerReviewer};

//     fn generate_test_complaint(address: Address) -> PeerComplaint {
//         PeerComplaint {
//             complainer: String::from("complainer"),
//             issue: Issue::Unresponsive,
//             peer_node_staker_address: address,
//         }
//     }

//     fn generate_test_config() -> ReloadableLitConfig {
//         ReloadableLitConfig::new(|| {
//             let system_path = PathBuf::from("default");
//             let system_path_str = system_path.to_str().unwrap();

//             let cfg = LitConfigBuilder::new_with_paths(
//                 None,
//                 Some("/tmp/fake/nope".to_string()),
//                 system_path_str,
//                 "/tmp/fake/nope",
//             )
//             .build()
//             .expect("failed to load config");

//             Ok(cfg)
//         })
//         .unwrap()
//     }

//     #[tokio::test]
//     async fn test_receive_complaints() {
//         // create channel for passing complaint
//         let (pr_tx, pr_rx) = mpsc::channel(3);
//         let (quit_tx, quit_rx) = mpsc::channel(3);

//         // create new peerreviewer
//         let mut peer_reviewer = PeerReviewer::new(pr_rx, generate_test_config());

//         let res = tokio::time::timeout(
//             std::time::Duration::from_secs(1),
//             peer_reviewer.receive_complaints(quit_rx),
//         );

//         let num_requests: u64 = 3;
//         let addr = Address::random();
//         // pass message in channel
//         for i in 0..num_requests {
//             let send_complaint = pr_tx.send(generate_test_complaint(addr.clone())).await;
//             assert!(send_complaint.is_ok());
//         }

//         let _ = res.await;

//         // check received
//         let count = *peer_reviewer.counter.get(&addr.to_string()).unwrap();
//         assert!(count == num_requests);
//     }
// }
