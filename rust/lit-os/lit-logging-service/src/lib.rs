use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::{fs, sync::Arc};

use error::PKG_NAME;
use handler::grpc::OTELGrpcService;
use lit_core::config::LitConfig;
use lit_logging::config::LitLoggingConfig;
use lit_observability::net::grpc;
use opentelemetry_proto::tonic::collector::logs::v1::logs_service_server::LogsServiceServer;
use opentelemetry_proto::tonic::collector::metrics::v1::metrics_service_server::MetricsServiceServer;
use opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::TraceServiceServer;
use sd_notify::NotifyState;
use service::otel::OTELService;
use tokio::net::UnixListener;
use tokio::sync::broadcast;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tracing::warn;

use crate::config::LitLoggingServiceConfig;

pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod metrics;
pub(crate) mod service;

pub async fn start_otel_service(
    grpc_socket_path: Option<PathBuf>, mut quit_rx: broadcast::Receiver<bool>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cfg = Arc::new(<LitConfig as LitLoggingServiceConfig>::must_new());

    let grpc_socket_path =
        grpc_socket_path.unwrap_or_else(|| cfg.logging_service_grpc_socket_path());
    if grpc_socket_path.exists() {
        fs::remove_file(&grpc_socket_path).unwrap_or_else(|_| {
            panic!("Unable to remove existing socket: {:?}", &grpc_socket_path)
        });
    }

    let t_socket_path = grpc_socket_path.clone();
    thread::spawn(move || {
        for _ in 0..100 {
            if t_socket_path.exists() {
                break;
            }

            thread::sleep(Duration::from_millis(10));
        }

        if t_socket_path.exists() {
            if let Err(e) = sd_notify::notify(true, &[NotifyState::Ready]) {
                warn!(error = ?e, "failed to send systemd notify");
            }
        } else {
            warn!("gave up waiting for socket to appear, not sending systemd notify");
        }
    });

    // Init services
    let otel_service =
        OTELService::new().start(&cfg).unwrap_or_else(|_| panic!("Unable to start otel service"));

    let grpc_server = OTELGrpcService { otel_svc: Arc::new(otel_service) };
    let uds = UnixListener::bind(grpc_socket_path)?;
    let uds_stream = UnixListenerStream::new(uds);

    tokio::select! {
        _ = Server::builder()
        .layer(grpc::tonic_middleware::MiddlewareLayer::new(grpc::MetricsMiddleware::new_with_options(PKG_NAME.to_owned())))
        .add_service(TraceServiceServer::new(grpc_server.clone()))
        .add_service(MetricsServiceServer::new(grpc_server.clone()))
        .add_service(LogsServiceServer::new(grpc_server))
        .serve_with_incoming(uds_stream) => {}
        _ = quit_rx.recv() => {}
    }

    Ok(())
}
