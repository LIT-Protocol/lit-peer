use crate::p2p_comms::web::chatter_server::chatter::{
    ConnectRequest, ConnectResponse, NodeRecord, NodeRecordResponse,
    chatter_service_server::ChatterService, chatter_service_server::ChatterServiceServer,
};
use crate::p2p_comms::web::internal::handle_node_share_set;
use crate::peers::peer_item::PeerItem;
use crate::peers::peer_state::models::PeerValidatorStatus;
use crate::tasks::chatter_sender::INTERNAL_CHATTER_PORT_OFFSET;
use crate::tss::common::models::NodeTransmissionEntry;
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use crate::tss::common::tss_state::TssState;
use crate::utils;
use crate::utils::attestation::create_attestation;
use crate::version;
use lit_blockchain::config::LitBlockchainConfig;
use lit_core::config::{LitConfig, ReloadableLitConfig};
use lit_node_common::config::LitNodeConfig;
use lit_observability::net::grpc;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio_stream::Stream;
use tonic::transport::Server;
use tonic::{self, Code, Status};
use tracing::{debug, error, info, instrument};
use xor_name::XorName;

#[allow(clippy::unwrap_used)]
pub mod chatter {
    tonic::include_proto!("chatter");
}

#[derive(Debug, Clone)]
pub struct ChatterServer {
    pub tss_state: Arc<TssState>,
    pub fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64>>,
}

#[derive(Debug, Clone)]
struct CloneableTcpListenerStream {
    inner: Arc<TcpListener>,
}

impl CloneableTcpListenerStream {
    fn new(listener: TcpListener) -> Self {
        Self {
            inner: Arc::new(listener),
        }
    }
}

impl Stream for CloneableTcpListenerStream {
    type Item = io::Result<tokio::net::TcpStream>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<io::Result<tokio::net::TcpStream>>> {
        // Use the Arc's as_ref() method to get a reference to the TcpListener
        let listener = self.inner.as_ref();
        match listener.poll_accept(cx) {
            Poll::Ready(Ok((stream, _))) => Poll::Ready(Some(Ok(stream))),
            Poll::Ready(Err(err)) => Poll::Ready(Some(Err(err))),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[tonic::async_trait]
impl ChatterService for ChatterServer {
    #[instrument(level = "debug", skip(self))]
    async fn exchange_record(
        &self,
        request: tonic::Request<NodeRecord>,
    ) -> tonic::Result<tonic::Response<NodeRecordResponse>, tonic::Status> {
        let peer_state = self.tss_state.peer_state.clone();
        let tx_round_sender = self.tss_state.tx_round_manager.clone();
        let sender = request.remote_addr();
        let remote_addr = match sender {
            Some(remote_addr) => remote_addr,
            None => {
                error!("Could not get remote address from sender: {:?}", sender);
                return Err(Status::new(
                    Code::Internal,
                    "Could not get remote address".to_string(),
                ));
            }
        };
        let req = request.into_inner();
        let header = if let Some(header) = req.header {
            header
        } else {
            return Ok(tonic::Response::new(NodeRecordResponse {
                ok: false,
                error: "No header provided".to_string(),
            }));
        };

        if req.messages.is_empty() {
            return Ok(tonic::Response::new(NodeRecordResponse {
                ok: false,
                error: "No message provided".to_string(),
            }));
        }

        let entry = match utils::serde_encrypt::deserialize_and_decrypt::<NodeTransmissionEntry>(
            peer_state.as_ref(),
            XorName::from_content(header.sender_id.as_bytes()),
            &req.messages[0].message,
        )
        .await
        {
            Ok(entry) => entry,
            Err(e) => {
                error!("Error deserializing and decrypting entry: {:?}", e);
                return Err(Status::new(
                    Code::Internal,
                    format!("Error deserializing and decrypting entry: {:?}", e),
                ));
            }
        };
        if let Err(e) = handle_node_share_set(
            &tx_round_sender,
            &self.fsm_worker_metadata,
            entry,
            remote_addr,
        )
        .await
        {
            error!("Error handling node share set: {:?}", e);
            return Err(Status::new(
                Code::Internal,
                format!("Error handling node share set: {:?}", e),
            ));
        }
        Ok(tonic::Response::new(NodeRecordResponse {
            ok: true,
            error: "".to_string(),
        }))
    }

    #[instrument(level = "debug", skip(self))]
    async fn peer_connect(
        &self,
        request: tonic::Request<ConnectRequest>,
    ) -> tonic::Result<tonic::Response<ConnectResponse>, tonic::Status> {
        let remote_addr = request.remote_addr();
        let connect_request = request.into_inner();
        let noonce = connect_request.noonce;
        let tss_state = self.tss_state.clone();
        let peer_state = tss_state.peer_state.clone();
        let cfg = self.tss_state.lit_config.clone();

        let private_key = match peer_state
            .lit_config
            .load_full()
            .blockchain_wallet_private_key_bytes(None)
        {
            Ok(private_key) => private_key,
            Err(e) => {
                error!("Error retrieving private key: {:?}", e);
                return Err(Status::new(
                    Code::Internal,
                    format!("Error retrieving private key: {:?}", e),
                ));
            }
        };

        let secret_key = match libsecp256k1::SecretKey::parse_slice(&private_key) {
            Ok(secret_key) => secret_key,
            Err(e) => {
                error!("Error parsing secret key: {:?}", e);
                return Err(Status::new(
                    Code::Internal,
                    format!("Error parsing secret key: {:?}", e),
                ));
            }
        };
        let public_key = libsecp256k1::PublicKey::from_secret_key(&secret_key);
        let mut peer_item = PeerItem {
            id: peer_state.id,
            public_key,
            node_address: peer_state.node_address(),
            sender_public_key: peer_state.comskeys.sender_public_key().to_bytes(),
            receiver_public_key: peer_state.comskeys.receiver_public_key().to_bytes(),
            staker_address: peer_state.staker_address,
            addr: tss_state.addr.clone(),
            status: PeerValidatorStatus::Unknown,
            attestation: None,
            version: version::get_version().to_string(),
        };

        if let Ok(at) = create_attestation(cfg.load_full(), &noonce, None).await {
            peer_item.attestation = Some(at);
        } else {
            #[cfg(not(feature = "testing"))]
            error!("Error creating attestation.");
        }

        let peer_item_json = match serde_json::to_string(&peer_item) {
            Ok(json) => json,
            Err(e) => {
                error!("Failed to serialize peer_item: {:?}", e);
                return Err(Status::new(
                    Code::Internal,
                    format!("Failed to serialize peer_item: {:?}", e),
                ));
            }
        };

        Ok(tonic::Response::new(ConnectResponse {
            peer_item: peer_item_json,
            error: "".to_string(),
        }))
    }
}

pub async fn launch_chatter_server(
    tss_state: Arc<TssState>,
    fsm_worker_metadata: Arc<dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64>>,
    reloadable_cfg: ReloadableLitConfig,
    mut quit_rx: broadcast::Receiver<bool>,
    _file_rx: mpsc::Receiver<bool>,
) {
    let addr = format!(
        "0.0.0.0:{}",
        tss_state.port + INTERNAL_CHATTER_PORT_OFFSET as u32
    );
    let socket_addr: SocketAddr = addr.parse().expect("Failed to parse address");
    let listener = TcpListener::bind(socket_addr)
        .await
        .expect("Failed to bind address");
    let incoming = CloneableTcpListenerStream::new(listener);
    let incoming_clone = incoming.clone();

    let (controller_tx, mut controller_rx) = broadcast::channel(1);
    let chatter_server = ChatterServer {
        tss_state: tss_state.clone(),
        fsm_worker_metadata: fsm_worker_metadata.clone(),
    };

    let cfg = reloadable_cfg.load_full();
    let builder = build_server(cfg).await;
    let chatter_service = builder
        .layer(grpc::tonic_middleware::MiddlewareLayer::new(
            grpc::TracingMiddleware,
        ))
        .layer(grpc::tonic_middleware::MiddlewareLayer::new(
            grpc::MetricsMiddleware::new(),
        ))
        .add_service(ChatterServiceServer::new(chatter_server.clone()));

    info!("Starting chatter server on {}", addr);
    let handle = tokio::spawn(async move {
        debug!("Chatter service task starting...");
        let result = chatter_service
            .serve_with_incoming_shutdown(incoming_clone, async move {
                controller_rx.recv().await.ok();
                info!("Chatter server shutdown signal received");
            })
            .await;
        debug!("Chatter service task completed with result: {:?}", result);
        result
    });

    tokio::select! {
        _ = quit_rx.recv() => {
            info!("Shutting down chatter server...");
            match controller_tx.send(true) {
                Ok(_) => {
                    info!("Chatter server shutdown signal sent...");
                }
                Err(e) => {
                    error!("Error sending shutdown signal: {:?}", e);
                }
            }
            info!("Chatter server shutdown successfully");
        }
    }
}

async fn build_server(cfg: Arc<LitConfig>) -> Server {
    let timeout = cfg.chatter_client_timeout().unwrap_or(30);
    let duration_timeout = Duration::from_secs(timeout);

    let mut server = Server::builder()
        .tcp_keepalive(Some(duration_timeout))
        .timeout(duration_timeout);

    if let Ok(Some(limit)) = cfg.grpc_server_concurrency_limit_per_connection() {
        server = server.concurrency_limit_per_connection(limit as usize);
    }

    server
}
