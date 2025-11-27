use crate::p2p_comms::comms::channels::{deregister_comms_channel, register_comms_channel};
use crate::peers::peer_reviewer::{Issue, PeerComplaint};
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tss::common::models::{NodeWaitParams, RoundData};
use crate::tss::common::peer_communication::PeerCommunicationChecker;
use crate::{
    error::{Result, unexpected_err},
    tss::common::models::RoundsShareSet,
};
use flume::Sender;
use lit_node_core::PeerId;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tracing::{Instrument, info_span, instrument};

#[instrument(level = "debug", skip_all, fields(round = params.round))]
pub async fn node_share_await(
    params: NodeWaitParams,
    tx_round_sender: Arc<Sender<RoundData>>,
    peers: &SimplePeerCollection,
    expected_items_from: &SimplePeerCollection,
) -> Result<Vec<(PeerId, Vec<u8>)>> {
    let start = Instant::now();

    let channels = match params.channels {
        Some(channels) => channels,
        None => register_comms_channel(tx_round_sender.clone(), &params.txn_prefix, &params.round)
            .await
            .map_err(|e| unexpected_err(e, Some("Error registering comms channel".into())))?,
    };

    let mut remaining_peer_ids = expected_items_from.clone();

    trace!(
        "Node: share_id: {} node_share_await() round {}",
        &params.peer_id, &params.round
    );
    let mut recvd_ans: Vec<(PeerId, Vec<u8>)> = Vec::new();

    // Intentionally keeping PeerCommunicationChecker simple without doing the job of `recvd_ans`.
    let mut peer_communication_checker = PeerCommunicationChecker::new(
        &expected_items_from
            .0
            .iter()
            .map(|p| p.peer_id)
            .collect::<Vec<PeerId>>(),
    );

    // spawn a task to send abort messages after a timeout
    let (from_peer_id, channel_id, timeout) =
        (params.peer_id, params.txn_prefix.clone(), params.timeout);

    // set a timeout for the round
    let timeout_task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(timeout)).await;
        let sent = channels.tx.send(RoundsShareSet {
            key: "abort".to_string(),
            value: "".to_string().as_bytes().to_vec(),
            from_peer_id,
            channel_id,
            retry: 0,
            created: SystemTime::now(),
        });
    });

    'waiting_loop: while let Ok(msg) = channels
        .rx
        .recv_async()
        .instrument(info_span!("node_share_await.channels.rx.recv_async"))
        .await
    {
        if msg.key == "abort" {
            let recvd_ans_idx: Vec<PeerId> = recvd_ans.iter().map(|x| x.0).collect();
            deregister_comms_channel(tx_round_sender.clone(), &params.txn_prefix, &params.round)
                .await;

            let this_peer = peers.peer_by_id(&params.peer_id).expect("Peer not found");
            let recvd_peers = peers.peers_by_id(&recvd_ans_idx);

            // When we poll, we'll still return an error with details, but we won't log it immediately. Higher level code will decide whether to display it or not.
            if !params.poll {
                error!(
                    "Timeout waiting for all nodes to respond to round {} for channel {}.  We (node #{}) received {} of {} responses from : {:?}.  Missing responses from: {:?}",
                    params.round,
                    params.txn_prefix,
                    this_peer.socket_address,
                    recvd_ans.len(),
                    expected_items_from.0.len(),
                    recvd_peers.debug_addresses(),
                    remaining_peer_ids.debug_addresses()
                );
            }
            let txn_id = params.txn_prefix.clone();
            // Only trigger nonparticipation complaints if we are doing a DKG
            // note that if we can't connect via GRPC to the remote node, a complaint would still occur well before this timeout.
            // given that GRPC channels are held, that would probably be a valid scenario
            if txn_id.contains("DKG") {
                match peers.peer_by_id(&params.peer_id) {
                    Ok(complainer) => {
                        for peer in
                            peer_communication_checker.peers_not_communicated_with_self_yet()
                        {
                            match peers.peer_by_id(peer) {
                                Ok(peer) => {
                                    let complaint = PeerComplaint {
                                        complainer: complainer.socket_address.clone(),
                                        issue: Issue::NonParticipation,
                                        peer_node_staker_address: peer.staker_address,
                                        peer_node_socket_address: peer.socket_address.clone(),
                                    };
                                    if let Err(e) = params.tx_pr.send_async(complaint).await {
                                        debug!(
                                            "Error sending complaint to PeerReviewer worker: {:?}",
                                            e
                                        );
                                        continue;
                                    }
                                }
                                Err(e) => {
                                    debug!("Error getting peer at share index: {:?}", e);
                                    continue;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        debug!("Error getting peer at share index: {:?}", e);
                        continue;
                    }
                }
            }

            return Err(unexpected_err(
                format!(
                    "Timeout waiting for all nodes to respond to round {} for channel {}.  We (node #{}) received {} of {} responses from : {:?}.  Missing responses from: {:?}",
                    params.round,
                    params.txn_prefix,
                    this_peer.socket_address,
                    recvd_ans.len(),
                    expected_items_from.0.len(),
                    recvd_peers.debug_addresses(),
                    remaining_peer_ids.debug_addresses()
                ),
                None,
            ));
        }
        trace!(
            "Node: {} node_share_await() round {} received from {}",
            params.peer_id, params.round, &msg.from_peer_id
        );

        match remaining_peer_ids.peer_by_id(&msg.from_peer_id) {
            Ok(peer) => {
                recvd_ans.push((msg.from_peer_id, msg.value.clone()));
                remaining_peer_ids = remaining_peer_ids.all_peers_except(&peer.socket_address);
            }
            Err(e) => {
                let peer_ip = match peers.peer_by_id(&params.peer_id) {
                    Ok(peer) => peer.socket_address,
                    Err(e) => "'unknown'".to_string(),
                };

                let sender_peer_ip = match peers.peer_by_id(&msg.from_peer_id) {
                    Ok(peer) => peer.socket_address,
                    Err(e) => "'unknown'".to_string(),
                };

                error!(
                    "Node {} received a message from a peer that was not expected {}",
                    peer_ip, sender_peer_ip
                );
            }
        }

        peer_communication_checker.mark_peer_as_communicated_with(&msg.from_peer_id);

        if remaining_peer_ids.is_empty() {
            break 'waiting_loop;
        }
        // if recvd_ans.len() == expected_items_from.len() {
        //     break 'waiting_loop;
        // }

        // optionally exit early.
        if let Some(exit_on_qty_recvd) = params.exit_on_qty_recvd
            && recvd_ans.len() >= exit_on_qty_recvd
        {
            break 'waiting_loop;
        };
    }

    // kill the task
    timeout_task.abort();
    deregister_comms_channel(tx_round_sender.clone(), &params.txn_prefix, &params.round).await;
    recvd_ans.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(recvd_ans)
}
