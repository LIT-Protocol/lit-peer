use std::sync::Arc;

use futures::future::join_all;
use lit_core::config::LitConfig;

use error::{Result, unexpected_err};

use lit_observability::opentelemetry::{KeyValue, global};
use lit_observability::opentelemetry_sdk::logs::LoggerProvider;
use lit_observability::opentelemetry_sdk::metrics::SdkMeterProvider;
use lit_observability::opentelemetry_sdk::{Resource, trace as sdktrace};
use tracing::error;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::LitLoggingServiceConfig;
use crate::error::PKG_NAME;

pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod metrics;

fn main() -> Result<()> {
    let cfg = Arc::new(<LitConfig as LitLoggingServiceConfig>::must_new());

    let (quit_tx, _quit_rx) = tokio::sync::broadcast::channel(1);
    let quit_rx_otel_service = quit_tx.subscribe();

    let otel_service_rt =
        tokio::runtime::Runtime::new().expect("failed to create otel service runtime");

    let join_handles = vec![otel_service_rt.spawn(async move {
        lit_logging_service::start_otel_service(None, quit_rx_otel_service).await
    })];

    // Init observability - this MUST occur after we have spawned the tasks to bring up the GRPC server.
    let observability_rt = tokio::runtime::Runtime::new().map_err(|e| {
        unexpected_err(e, Some("failed to create Observability Runtime: {:?}".into()))
    })?;
    let (metrics_provider, logger_provider) =
        observability_rt.block_on(async { init_observability(&cfg).await })?;

    let main_rt = tokio::runtime::Runtime::new()
        .map_err(|e| unexpected_err(e, Some("failed to create Main Runtime: {:?}".into())))?;

    main_rt.block_on(async {
        join_all(join_handles).await;
    });

    // Shutdown tasks
    let _ = quit_tx.send(true);

    shutdown_observability(metrics_provider, logger_provider);

    Ok(())
}

/// NOTE: Run in a dedicated runtime, to avoid blocking a runtime that is needed to perform tasks.
async fn init_observability(cfg: &LitConfig) -> Result<(SdkMeterProvider, LoggerProvider)> {
    lit_logging::set_panic_hook();

    let otel_resource = Resource::new(vec![KeyValue::new(
        lit_observability::opentelemetry_semantic_conventions::resource::SERVICE_NAME,
        PKG_NAME,
    )]);

    let (tracing_provider, metrics_provider, subscriber, logger_provider) =
        lit_observability::create_providers(
            cfg,
            otel_resource.clone(),
            sdktrace::Config::default().with_resource(otel_resource.clone()),
            #[cfg(feature = "proxy-collector")]
            PKG_NAME,
        )
        .await
        .map_err(|e| unexpected_err(e, Some("failed to create OTEL providers: {:?}".into())))?;

    // Set globals
    global::set_tracer_provider(tracing_provider);
    global::set_meter_provider(metrics_provider.clone());
    subscriber.init();

    Ok((metrics_provider, logger_provider))
}

fn shutdown_observability(metrics_provider: SdkMeterProvider, logger_provider: LoggerProvider) {
    global::shutdown_tracer_provider();
    if let Err(e) = metrics_provider.shutdown() {
        error!("Failed to shutdown metrics provider: {:?}", e);
    }
    if let Err(e) = logger_provider.shutdown() {
        error!("Failed to shutdown logger provider: {:?}", e);
    }
}
