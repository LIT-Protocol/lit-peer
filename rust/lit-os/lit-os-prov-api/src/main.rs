#[macro_use]
extern crate rocket;

use error::{Result, unexpected_err};
use lit_observability::opentelemetry_sdk::logs::LoggerProvider;
use lit_observability::opentelemetry_sdk::metrics::SdkMeterProvider;

use std::sync::Arc;

use lit_core::config::LitConfig;
use lit_observability::opentelemetry::{KeyValue, global};
use lit_observability::opentelemetry_sdk::{Resource, trace as sdktrace};
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use tracing::error;
use tracing_subscriber::util::SubscriberInitExt;

use lit_api_core::{Engine, Launcher};
use lit_blockchain::resolver::contract::ContractResolver;

use crate::config::load_cfg;
use crate::error::PKG_NAME;

pub(crate) mod api;
pub mod config;
pub mod error;
pub(crate) mod release;

pub fn main() {
    // Init config
    let cfg = load_cfg().expect("failed to load config");

    // Init observability
    let observability_rt = tokio::runtime::Runtime::new()
        .map_err(|e| unexpected_err(e, Some("failed to create Observability Runtime: {:?}".into())))
        .expect("failed to create Observability Runtime");

    let (metrics_provider, logger_provider) = observability_rt
        .block_on(async { init_observability(cfg.load().as_ref()).await })
        .expect("failed to init observability");

    // Init resolver
    let resolver = Arc::new(
        ContractResolver::try_from(cfg.load().as_ref())
            .expect("failed to construct ContractResolver"),
    );

    // Init rocket
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch].into_iter().map(From::from).collect(),
        )
        .allow_credentials(true)
        .to_cors()
        .expect("CORS failed to build");

    Engine::new(move || {
        let cfg = cfg.clone();
        let resolver = resolver.clone();
        let cors = cors.clone();

        Box::pin(async move {
            Launcher::try_new(cfg.clone(), None)
                .expect("failed to construct rocket launcher")
                .mount("/", api::status::routes())
                .mount("/", api::attest::routes())
                .mount("/release", api::release::routes())
                .attach(cors.clone())
                .manage(cfg)
                .manage(resolver)
        })
    })
    .start();

    shutdown_observability(metrics_provider, logger_provider);
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
