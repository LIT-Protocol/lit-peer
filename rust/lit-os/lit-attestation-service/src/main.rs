use std::sync::Arc;

use config::LitAttestationServiceConfig;
use error::{PKG_NAME, Result, unexpected_err};
use lit_core::config::LitConfig;
use lit_observability::opentelemetry::{KeyValue, global};
use lit_observability::opentelemetry_sdk::{
    Resource, logs::LoggerProvider, metrics::SdkMeterProvider, trace as sdktrace,
};
use tracing::error;
use tracing_subscriber::util::SubscriberInitExt;

mod config;
mod error;

fn main() -> Result<()> {
    let cfg = Arc::new(<LitConfig as LitAttestationServiceConfig>::must_new());

    let observability_rt = tokio::runtime::Runtime::new().map_err(|e| {
        unexpected_err(e, Some("failed to create Observability Runtime: {:?}".into()))
    })?;

    let (metrics_provider, logger_provider) =
        observability_rt.block_on(async { init_observability(&cfg).await })?;

    let main_rt = tokio::runtime::Runtime::new()
        .map_err(|e| unexpected_err(e, Some("failed to create Main Runtime: {:?}".into())))?;

    if let Err(e) =
        main_rt.block_on(async { lit_attestation_service::start(cfg.clone(), None).await })
    {
        error!("Attestation service error: {:?}", e);
    }

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
