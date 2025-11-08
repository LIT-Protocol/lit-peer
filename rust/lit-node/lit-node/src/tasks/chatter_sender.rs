use crate::error::Result;
use crate::networking::grpc::client::ChatterClientFactory;
use crate::p2p_comms::web::chatter_server::chatter::chatter_service_client::ChatterServiceClient;
use crate::p2p_comms::web::chatter_server::chatter::{
    NodeRecord, NodeRecordHeader, NodeRecordMessage, NodeRecordResponse,
};
use crate::peers::PeerState;
use crate::peers::peer_reviewer::{Issue, PeerComplaint};
use crate::tss::common::models::NodeTransmissionDetails;
use crate::utils;
use crate::utils::tracing::inject_tracing_metadata;
use lit_core::config::{LitConfig, ReloadableLitConfig};
use lit_node_common::config::{CFG_KEY_CHATTER_CLIENT_TIMEOUT_SECS_DEFAULT, LitNodeConfig};
use lit_observability::channels::TracedReceiver;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;
use tonic::Response;
use tonic::{Code, Request, Status, transport::Channel};
use tracing::{Instrument, instrument};
use url::Url;

pub static INTERNAL_CHATTER_PORT_OFFSET: u16 = 19608;

pub async fn chatter_sender_worker(
    mut quit_rx: tokio::sync::broadcast::Receiver<bool>,
    reloadable_lit_config: ReloadableLitConfig,
    peer_state: Arc<PeerState>,
    rx_node_transmission_details: TracedReceiver<NodeTransmissionDetails>,
) {
    let lit_config = reloadable_lit_config.load_full();
    // The prefix is always http for grpc now
    let prefix = "http://";
    info!("Starting: tasks::chatter_sender_worker");

    let timeout = lit_config
        .signing_round_timeout()
        .unwrap_or(CFG_KEY_CHATTER_CLIENT_TIMEOUT_SECS_DEFAULT);

    let mut heartbeat = tokio::time::interval(Duration::from_secs(timeout as u64));
    heartbeat.tick().await;

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
                info!("Shutting down: tasks::chatter_sender_worker");
                break;
            }
            recv_res = rx_node_transmission_details.recv_async() => {
                let (msg, span) = match recv_res {
                    Ok(chan_msg_and_span) => chan_msg_and_span,
                    Err(e) => {
                        error!("Error receiving NodeTransmissionDetails: {}", e);
                        continue;
                    }
                };
                let transmission_details = msg.data().to_owned();
                let peer_state = peer_state.clone();
                let peer_addr = transmission_details.dest_peer.socket_address.clone();
                let dest_url = match Url::parse(format!(
                    "{}{}/",
                    prefix,
                    peer_addr
                ).as_str()) {
                    Ok(url) => url,
                    Err(e) => {
                        error!("Error parsing peer url: {}", e);
                        continue;
                    }
                };
                let lit_config = lit_config.clone();
                tokio::spawn(async move {
                    // Only try twice to send the message
                    for i in 0..2 {

                        let client = match peer_state.client_grpc_channels.create_or_get_connection(
                            &peer_addr,
                            || create_client(dest_url.clone(), lit_config.clone(), &peer_state, &transmission_details)
                        ).await {
                            Ok(value) => {
                                value
                            }
                            Err(e) => continue
                        };

                        let response = match send_chatter(&transmission_details.clone(), &peer_state, client).await {
                            Ok(res) => res.into_inner(),
                            Err(e) => {
                                if i == 0 {
                                    error!("Error sending chatter to {}: {}", peer_addr, e);
                                    peer_state.client_grpc_channels.remove_connection(&peer_addr).await;
                                    trace!("Resending chatter to {} with new client", peer_addr);
                                    continue;
                                }

                                error!("Error resending chatter to {:?}: {}", peer_addr.clone(), e);
                                peer_state.client_grpc_channels.remove_connection(&peer_addr).await;
                                match e.code() {
                                    Code::Cancelled | Code::DeadlineExceeded | Code::Unavailable => {
                                        // Complain
                                        warn!("Peer {:?} is unresponsive. Complaining.", transmission_details.dest_peer);
                                        let complainer = peer_state.addr.clone();
                                        let complaint_channel = peer_state.complaint_channel.clone();
                                        if let Err(e) = complaint_channel
                                            .send_async(PeerComplaint {
                                                complainer,
                                                issue: Issue::Unresponsive,
                                                peer_node_staker_address: transmission_details.dest_peer.staker_address,
                                                peer_node_socket_address: transmission_details
                                                    .dest_peer
                                                    .socket_address
                                                    .clone(),
                                            })
                                            .await
                                        {
                                            error!("Failed to send complaint to complaint_channel: {:?}", e);
                                        }
                                    }
                                    _ => {}
                                }
                                error!(
                                    "Problem sending chatter for round {} from node #{} to node #{}  ({:?}): {:?}.",
                                    transmission_details.round, &transmission_details.node_transmission_entry.src_peer_id, transmission_details.node_transmission_entry.dest_peer_id, transmission_details.dest_peer, e
                                );
                                return;
                            }
                        };
                        if response.ok {
                            break;
                        } else {
                            error!(
                                "Peer responded with failure for chatter to {:?}: {}",
                                transmission_details.dest_peer, response.error,
                            );
                        }
                    }
                }.instrument(span));
            }
            _ = heartbeat.tick() => {
                // prune old connections
                let test_timeout = Duration::from_secs(5);
                let mut remove = Vec::new();
                let mut tasks = JoinSet::new();
                let keys = peer_state.client_grpc_channels.get_addresses().await;
                for peer_addr in keys {
                     // very lightweight check without making a gRPC call
                    let addr: SocketAddr = match peer_addr.parse() {
                        Ok(a) => a,
                        Err(e) => {
                            error!("Unable to parse peer addr {} - pruning", e);
                            remove.push(peer_addr);
                            continue;
                        }
                    };
                    tasks.spawn(async move {
                        let res = tokio::time::timeout(
                            test_timeout,
                            tokio::net::TcpStream::connect(&addr)
                        ).await;
                        (peer_addr, res.is_ok())
                    });
                }

                let results = tasks.join_all().await;

                remove.append(&mut results.into_iter().filter_map(|(peer_addr, valid_connection)|{
                    if !valid_connection {
                        Some(peer_addr)
                    } else {
                        None
                    }
                }).collect::<Vec<_>>());
                trace!("pruning grpc clients at {:?}", remove);
                peer_state.client_grpc_channels.remove_connections(&remove).await;
            }
        }
    }
}

#[instrument(level = "debug", name = "send_direct", skip_all)]
async fn send_chatter(
    transmission_details: &NodeTransmissionDetails,
    peer_state: &PeerState,
    mut client: ChatterServiceClient<Channel>,
) -> std::result::Result<Response<NodeRecordResponse>, Status> {
    let dest_addr_full = transmission_details.dest_peer.socket_address.clone();
    let round = &transmission_details.round;
    let dest_peer_id = transmission_details.dest_peer.peer_id;
    let data = transmission_details.node_transmission_entry.clone();

    // TODO: add the header and footer to be integrity checked in encrypt_and_serialize
    let encrypted_entry =
        utils::serde_encrypt::encrypt_and_serialize(peer_state, &dest_addr_full, &data)
            .map_err(|e| Status::internal(e.to_string()))?;

    let mut request = Request::new(NodeRecord {
        header: Some(NodeRecordHeader {
            sender_id: peer_state.addr.clone(),
            metadata: vec![],
        }),
        messages: vec![NodeRecordMessage {
            version: crate::version::NODE_RECORD_MESSAGE_VERSION.to_string(),
            message: encrypted_entry,
        }],
        footer: vec![],
    });
    request.set_timeout(Duration::from_secs(
        CFG_KEY_CHATTER_CLIENT_TIMEOUT_SECS_DEFAULT as u64,
    ));

    // Add trace context to the metadata for distributed tracing.
    inject_tracing_metadata(request.metadata_mut());

    client.exchange_record(request).await
}

#[instrument(level = "debug", skip_all)]
async fn create_client(
    dest_url: Url,
    lit_config: Arc<LitConfig>,
    peer_state: &Arc<PeerState>,
    transmission_details: &NodeTransmissionDetails,
) -> Result<ChatterServiceClient<Channel>> {
    match ChatterClientFactory::new_client(dest_url, lit_config).await {
        Ok(client) => Ok(client),
        Err(e) => {
            warn!(
                "Peer {:?} is unresponsive. Complaining.",
                transmission_details.dest_peer.socket_address
            );
            let complainer = peer_state.addr.clone();
            let complaint_channel = peer_state.complaint_channel.clone();
            if let Err(e) = complaint_channel
                .send_async(PeerComplaint {
                    complainer,
                    issue: Issue::Unresponsive,
                    peer_node_staker_address: transmission_details.dest_peer.staker_address,
                    peer_node_socket_address: transmission_details.dest_peer.socket_address.clone(),
                })
                .await
            {
                error!("Failed to send complaint to channel: {:?}", e);
            }
            Err(e)
        }
    }
}
