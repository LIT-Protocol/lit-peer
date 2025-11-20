use crate::peers::PeerState;
use crate::peers::peer_item::{PeerData, PeerItem};
use crate::peers::peer_state::models::PeerValidatorStatus;
use crate::utils::key_share_proof::KeyShareProofs;
use crate::utils::networking::get_web_addr_from_chain_info;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::MissedTickBehavior;

#[derive(Debug, Clone)]
pub enum PeerCheckerMessage {
    AddPeer(PeerItem),
    RemovePeer(String),
    GetPeers(flume::Sender<PeerData>),
}

/// polling interval for this thread
const PEER_CHECKER_INTERVAL_MS: u64 = 1000;
/// how often we attempt to check on a peer
const UPDATE_INTERVAL_MS: u64 = 12000;
/// how often we retry a failed connection to a peer
const RETRY_INTERVAL_MS: u64 = 3000;

/// This worker simply connects to all nodes on an interval, which will trigger a complaint if they are not online
pub async fn peer_checker_worker(
    mut quit_rx: mpsc::Receiver<bool>,
    peer_state: Arc<PeerState>,
    peer_checker_tx: flume::Sender<PeerCheckerMessage>,
    peer_checker_rx: flume::Receiver<PeerCheckerMessage>,
) {
    info!("Starting: tasks::peer_checker_worker");

    // Data about our connected peers;
    let mut peer_data = PeerData::default();
    let mut last_update: HashMap<String, Instant> = HashMap::new();
    let mut last_attempted_update: HashMap<String, Instant> = HashMap::new();

    // Possible FIXME:Should this be a config var pulled from chain
    let mut interval = tokio::time::interval(Duration::from_millis(PEER_CHECKER_INTERVAL_MS));

    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    interval.tick().await;

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
                info!("Shutting down: tasks::peer_checker_worker");
                break;
            }

            msg = peer_checker_rx.recv_async() => {
                let msg = match msg {
                    Ok(msg) => msg,
                    Err(e) => {
                        warn!("Error receiving peer checker message: {:?}", e);
                        continue;
                    }
                };
                match msg {
                    PeerCheckerMessage::AddPeer(pi) => {
                        trace!("Received AddPeer message for {}", pi.addr);
                        if let Err(e) = peer_data.insert(pi.clone()) {
                            warn!("Error inserting peer into peer data: {:?}", e);
                            continue;
                        }
                        last_update.insert(pi.addr.clone(), Instant::now());
                    }
                    PeerCheckerMessage::RemovePeer(addr) => {
                        trace!("Received RemovePeer message for {}", addr);
                        if let Err(e) = peer_data.remove_by_addr(&addr) {
                            warn!("Error removing peer from peer data: {:?}", e);
                            continue;
                        }
                        last_update.remove(&addr);
                        last_attempted_update.remove(&addr);
                    }
                    PeerCheckerMessage::GetPeers(tx) => {
                        // info!("Received GetPeers message");
                        if let Err(e) = tx.send(peer_data.clone()) {
                            warn!("Error sending peer data back to caller: {:?}", e);
                            continue;
                        }
                    }
                }
            }

            _= interval.tick() => {
                check_for_peer_updates(&peer_state, &peer_checker_tx, &mut last_attempted_update, &mut last_update).await;
            }
        }
    }

    info!("Stopped: tasks::peer_checker_worker");
}

async fn check_for_peer_updates(
    peer_state: &Arc<PeerState>,
    peer_checker_tx: &flume::Sender<PeerCheckerMessage>,
    last_attempted_update: &mut HashMap<String, Instant>,
    last_update: &mut HashMap<String, Instant>,
) {
    let validators = peer_state.validators_in_next_epoch_current_union();
    let mut validators_to_check = Vec::new();

    for validator in validators {
        let mut needs_update: bool = true;

        if let Some(last_attempt) = last_attempted_update.get(&validator.staker_address.to_string())
        {
            if last_attempt.elapsed() < Duration::from_millis(RETRY_INTERVAL_MS) {
                needs_update = false;
            }
        }

        // logic here is a bit different.  if we've never received an update from a peer, we don't wait to recheck - yes, it's potentially
        // checking more freqently, but we want to ensure we're getting updates from all peers.   This scenario is likely
        // to occur when we first start up nodes.
        match last_update.get(&validator.staker_address.to_string()) {
            Some(actual_last_update) => {
                if actual_last_update.elapsed() < Duration::from_millis(UPDATE_INTERVAL_MS) {
                    needs_update = false;
                }
            }
            None => {
                needs_update = true;
            }
        }

        if needs_update {
            last_attempted_update.insert(validator.staker_address.to_string(), Instant::now());
            validators_to_check.push(validator);
        }
    }

    if validators_to_check.is_empty() {
        return;
    }

    trace!(
        "Checking validators for updates: {}",
        validators_to_check
            .iter()
            .map(|v| v.staker_address.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    for validator in validators_to_check {
        let peer_state_for_spawning = peer_state.clone();
        let peer_checker_tx = peer_checker_tx.clone();
        tokio::spawn(async move {
            let addr = get_web_addr_from_chain_info(validator.ip, validator.port);
            let result = peer_state_for_spawning
                .connect_to_node(validator.clone())
                .await;
            if let Ok(node_info) = result {
                let pi = PeerItem::new(
                    &node_info.addr,
                    node_info.public_key,
                    node_info.node_address,
                    node_info.sender_public_key,
                    node_info.receiver_public_key,
                    node_info.staker_address,
                    PeerValidatorStatus::Unknown,
                    None,
                    node_info.version,
                    KeyShareProofs::default(),
                );
                if let Err(e) = peer_checker_tx
                    .send_async(PeerCheckerMessage::AddPeer(pi))
                    .await
                {
                    warn!("Error sending peer checker message: {:?}", e);
                };
            } else {
                let err_msg = result.err();
                if let Some(err_msg) = err_msg {
                    if err_msg.to_string().contains("network is paused") {
                        warn!("No response from node {}, but network is paused.", addr);
                    } else {
                        warn!("Error connecting to node: {:?}", err_msg);
                    }
                }
            }
        });
    }
}
