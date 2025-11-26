#![allow(unused_variables)]

extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate clap;

use crate::error::{EC, unexpected_err_code};
use crate::models::AuthContextCacheExpiry;
use crate::p2p_comms::web::chatter_server::launch_chatter_server;
use crate::payment::delegated_usage::DelegatedUsageDB;
use crate::peers::PeerState;
use crate::tasks::chatter_sender::chatter_sender_worker;
use crate::tasks::fsm::fsm_worker::CounterBasedFSMWorkerMetadata;
use crate::tasks::presign_manager::models::PresignManager;
use crate::tss::common::{
    restore::RestoreState, traits::fsm_worker_metadata::FSMWorkerMetadata, tss_state,
};
use config::chain::ChainDataConfigManager;
use error::{Result, unexpected_err};
use ethers::types::U256;
use lit_api_core::config::LitApiConfig;
use lit_api_core::context::{HEADER_KEY_X_CORRELATION_ID, HEADER_KEY_X_REQUEST_ID};
use lit_api_core::error::ApiError;
use lit_api_core::observability::MetricsFairings;
use lit_api_core::{Engine, Launcher};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;
use lit_core::utils::unix::raise_fd_limit;
use lit_node::error::PKG_NAME;
use lit_node::version;
use lit_node_common::client_state::ClientState;
use lit_node_common::config::LitNodeConfig;
use lit_node_common::config::load_cfg;
use lit_observability::channels::new_traced_unbounded_channel;
#[cfg(feature = "testing")]
use lit_observability::logging::simple_file_logging_subscriber;
#[cfg(not(feature = "testing"))]
use lit_observability::logging::simple_logging_subscriber;
use lit_observability::opentelemetry::{KeyValue, global};
use lit_observability::opentelemetry_sdk::logs::LoggerProvider;
use lit_observability::opentelemetry_sdk::metrics::SdkMeterProvider;
use lit_observability::opentelemetry_sdk::propagation::TraceContextPropagator;
use lit_observability::opentelemetry_sdk::{Resource, trace as sdktrace};
use moka::future::Cache;
use rocket::fairing::AdHoc;
use rocket::http::{Header, Status};
use rocket::response::status;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::{RwLock, mpsc::channel};
use tracing::error;
use tracing_subscriber::util::SubscriberInitExt;

use crate::peers::grpc_client_pool::GrpcClientPool;
use crate::utils::web::default_http_client;
use rocket::Request;
use rocket::serde::json::Value;
use rocket_cors::AllowedOrigins;

pub mod common;
mod config;
mod endpoints;
mod metrics;
mod models;
mod networking;
mod p2p_comms;
mod peers;
mod siwe_db;
mod utils {
    pub mod attestation;
    pub mod consensus;
    pub mod contract;
    pub mod cose_keys;
    pub mod encoding;
    pub mod eth;
    pub mod future;
    pub mod key_share_proof;
    pub mod networking;
    pub mod rocket;
    pub mod serde_encrypt;
    pub mod siwe;
    pub mod tracing;
    pub mod traits;
    #[allow(dead_code)]
    pub mod web;
}

pub mod access_control;
#[allow(dead_code)]
pub mod auth;
pub mod error;
#[cfg(feature = "lit-actions")]
pub mod functions;
pub mod jwt;
mod node_state;
pub mod payment;
pub mod pkp;
pub mod services;
pub mod tasks;
#[cfg(test)]
mod tests;
pub mod tss;

pub mod client_session;
pub mod git_info;
// mod test_nodes;

#[catch(422)]
fn bad_input_data_catcher(status: Status, req: &Request<'_>) -> status::Custom<Value> {
    let request_id = req
        .headers()
        .get_one(HEADER_KEY_X_REQUEST_ID)
        .unwrap_or("unknown_req_id")
        .to_string();
    let correlation_id = req
        .headers()
        .get_one(HEADER_KEY_X_CORRELATION_ID)
        .unwrap_or("unknown_correlation_id")
        .to_string();
    error!(
        "{}: Bad data input for request id: {}, correlation id: {}",
        status,
        request_id.as_str(),
        correlation_id.as_str()
    );
    unexpected_err_code("caught rocket 422: bad input error", EC::NodeBadInput, None).handle()
}

#[catch(500)]
fn internal_server_error_catcher(status: Status, req: &Request<'_>) -> status::Custom<Value> {
    let request_id = req
        .headers()
        .get_one(HEADER_KEY_X_REQUEST_ID)
        .unwrap_or("unknown_req_id")
        .to_string();
    let correlation_id = req
        .headers()
        .get_one(HEADER_KEY_X_CORRELATION_ID)
        .unwrap_or("unknown_correlation_id")
        .to_string();
    error!(
        "{}: Internal server error for request id: {}, correlation id: {}",
        status,
        request_id.as_str(),
        correlation_id.as_str()
    );
    unexpected_err_code(
        "caught rocket 500: internal server error",
        EC::NodeSystemFault,
        None,
    )
    .handle()
}

pub fn main() {
    raise_fd_limit();

    // When starting an internal lit_actions server for testing, we need to
    // init the V8 platform on the parent thread that will spawn V8 isolates
    // to avoid crashing the node process.
    #[cfg(feature = "lit-actions-server")]
    lit_actions_server::init_v8();

    // Load config
    let cfg = load_cfg().expect("failed to load LitConfig");
    let addr = cfg
        .load()
        .external_addr()
        .expect("failed to load external_addr");
    let port = cfg
        .load()
        .external_port()
        .expect("failed to load external_port");
    let domain = cfg.load().api_domain().expect("failed to load api_domain");

    let observability_rt = tokio::runtime::Runtime::new()
        .map_err(|e| {
            unexpected_err(
                e,
                Some("failed to create Observability Runtime: {:?}".into()),
            )
        })
        .expect("failed to create runtime");

    let observability_providers = observability_rt
        .block_on(async { init_observability(cfg.load().as_ref(), port, domain).await })
        .expect("failed to init observability");

    tracing::info!("Starting lit_node on port {}", port);

    // create a node_state directory if it doesn't exist
    let node_state_dir = "node_state";
    if !std::path::Path::new(node_state_dir).exists() {
        std::fs::create_dir(node_state_dir).expect("failed to create node_state directory");
    }

    // Load contract resolver
    let resolver = Arc::new(
        ContractResolver::try_from(cfg.load().as_ref()).expect("failed to load ContractResolver"),
    );

    siwe_db::db::db_initial_setup(port).expect("Initial SQLite db setup failed");

    // Since we use the same settings everywhere, we just create one and reuse it. From the docs.rs
    // The Client holds a connection pool internally, so it is advised that you create one and reuse it.
    // You do not have to wrap the Client in an Rc or Arc to reuse it, because it already uses an Arc internally.
    let http_client = default_http_client();

    let client_state = Arc::new(ClientState::default());

    let (pr_tx, pr_rx) = new_traced_unbounded_channel();

    let local_rt = tokio::runtime::Builder::new_multi_thread()
        .thread_name("comms-tasks")
        .worker_threads(32 * num_cpus::get_physical())
        .enable_all()
        .build()
        .expect("create tokio runtime");

    let (peer_checker_tx, peer_checker_rx) = flume::unbounded();

    let chain_data_manager = Arc::new(local_rt.block_on(ChainDataConfigManager::new(
        cfg.clone(),
        peer_checker_tx.clone(),
    )));

    let (ps_tx, ps_rx) = flume::unbounded();

    let peer_state = Arc::new(
        local_rt
            .block_on(PeerState::new(
                addr,
                pr_tx.clone(),
                Arc::new(cfg.clone()),
                chain_data_manager.clone(),
                ps_tx.clone(),
                peer_checker_tx.clone(),
            ))
            .expect("failed to create PeerState"),
    );

    let (tss_state, rx_round_manager, rx_batch_manager) = tss_state::TssState::init(
        peer_state,
        Arc::new(cfg.clone()),
        chain_data_manager.clone(),
    )
    .expect("Error initializing tss state");

    let delegation_usage_db = Arc::new(DelegatedUsageDB::default_with_chain_data_config_manager(
        chain_data_manager.clone(),
    ));
    let cache: Cache<String, crate::models::AuthMethodResponse> = Cache::builder()
        .max_capacity(100_000)
        .expire_after(AuthContextCacheExpiry)
        .build();

    let eth_blockhash_cache = Arc::new(siwe_db::rpc::EthBlockhashCache {
        blockhash: RwLock::new("0x".to_owned()),
        timestamp: RwLock::new(U256::zero()),
    });

    let auth_context_cache = Arc::new(models::AuthContextCache {
        auth_contexts: cache,
    });

    // 1gb max capacity
    let ipfs_cache: Cache<String, Arc<String>> = Cache::builder()
        .weigher(|_key, value: &Arc<String>| -> u32 { value.len().try_into().unwrap_or(u32::MAX) })
        .max_capacity(1024 * 1024 * 1024)
        .build();

    let allowlist_cache = Arc::new(models::AllowlistCache {
        entries: RwLock::new(HashMap::new()),
    });

    let restore_state = RestoreState::new();
    let restore_state = Arc::new(restore_state);

    let payment_tracker = Arc::new(payment::payment_tracker::PaymentTracker::default());

    let fsm_worker_metadata: Arc<
        dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64>,
    > = Arc::new(CounterBasedFSMWorkerMetadata::new());

    let (quit_tx, quit_rx) = broadcast::channel(1);
    let quit_rx_ps = quit_tx.subscribe();
    let quit_rx_grpc_server = quit_tx.subscribe();
    let peer_state_clone = tss_state.peer_state.clone();
    let presign_cfg = cfg.clone();
    let (file_tx, file_rx) = channel(1);
    let tss_state_comms = tss_state.clone();
    let tss_state_ps = tss_state.clone();
    let fsm_worker_metadata_grpc = fsm_worker_metadata.clone();

    local_rt.spawn(async move {
        let mut presign_manager = PresignManager::new(ps_rx, ps_tx, tss_state_ps);
        presign_manager.listen(quit_rx_ps, presign_cfg).await;
    });
    local_rt.spawn(chatter_sender_worker(
        quit_rx,
        cfg.clone(),
        peer_state_clone,
        rx_batch_manager,
    ));
    local_rt.spawn(launch_chatter_server(
        tss_state_comms,
        fsm_worker_metadata_grpc,
        cfg.clone(),
        quit_rx_grpc_server,
        file_rx,
    ));

    let action_store = local_rt.block_on(async {
        let db_path = format!("node_state/actions_{port}.db");
        functions::ActionStore::new(&db_path) // or new_in_memory() for file-less SQLite
            .await
            .expect("failed to create action store")
    });

    let t = tasks::launch(
        eth_blockhash_cache.clone(),
        pr_tx,
        pr_rx,
        auth_context_cache.clone(),
        tss_state.clone(),
        restore_state.clone(),
        tss_state.tx_round_manager.clone(),
        rx_round_manager,
        fsm_worker_metadata.clone(),
        payment_tracker.clone(),
        ipfs_cache.clone(),
        action_store.clone(),
        client_state.clone(),
        http_client.clone(),
        peer_checker_tx.clone(),
        peer_checker_rx,
    )
    .expect("failed to launch tasks");

    let mut allowed_methods: HashSet<rocket_cors::Method> = HashSet::new();
    allowed_methods.insert(
        rocket_cors::Method::from_str("Get").expect("failed to parse 'GET' for CORS method"),
    );
    allowed_methods.insert(
        rocket_cors::Method::from_str("Post").expect("failed to parse 'POST' for CORS method"),
    );
    allowed_methods.insert(
        rocket_cors::Method::from_str("Patch").expect("failed to parse 'PATCH' for CORS method"),
    );

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS failed to build");

    Engine::new(move || {
        let cfg = cfg.clone();
        let resolver = resolver.clone();
        let cors = cors.clone();
        let tss_state = tss_state.clone();
        let peer_state = tss_state.peer_state.clone();
        let delegation_usage_db = delegation_usage_db.clone();
        let auth_context_cache = auth_context_cache.clone();
        let eth_blockhash_cache = eth_blockhash_cache.clone();
        let allowlist_cache = allowlist_cache.clone();
        let tx_round_sender = tss_state.tx_round_manager.clone();
        let restore_state = restore_state.clone();
        let fsm_worker_metadata = fsm_worker_metadata.clone();
        let payment_tracker = payment_tracker.clone();
        let file_tx_clone = file_tx.clone();
        let ipfs_cache = ipfs_cache.clone();
        let action_store = action_store.clone();
        let client_state = client_state.clone();

        let all_routes = [
            endpoints::versions::initial::routes(),
            endpoints::versions::v1::routes(),
            endpoints::versions::v2::routes(),
        ]
        .concat();

        let metrics_fairings = MetricsFairings::new(all_routes);

        let http_cache_clone = http_client.clone();
        Box::pin(async move {
            #[allow(unused_mut)]
            let mut l = Launcher::try_new(cfg.clone(), Some(file_tx_clone))
                .expect("failed to construct rocket launcher")
                // include the initial routes with the v0 launch that have no versioning
                .mount("/", endpoints::versions::initial::routes())
                // include the v1 routes
                .mount("/", endpoints::versions::v1::routes())
                // include the v2 routes
                .mount("/", endpoints::versions::v2::routes())
                .attach(cors)
                .attach(AdHoc::on_response("Version Header", |_, resp| {
                    Box::pin(async move {
                        resp.set_header(Header::new(
                            "X-Lit-Node-Version",
                            version::get_version().to_string(),
                        ));
                    })
                }))
                .attach(metrics_fairings)
                .attach(crate::utils::rocket::privacy_mode::privacy_mode_fairing())
                .manage(cfg.clone())
                .manage(resolver)
                .manage(tss_state.peer_state.clone())
                .manage(tx_round_sender)
                .manage(delegation_usage_db)
                .manage(auth_context_cache)
                .manage(eth_blockhash_cache)
                .manage(allowlist_cache)
                .manage(fsm_worker_metadata)
                .manage(ipfs_cache)
                .manage(action_store)
                .manage(http_cache_clone)
                .manage(GrpcClientPool::<tonic::transport::Channel>::new(
                    cfg.load_full(),
                ))
                .register(
                    "/",
                    catchers![bad_input_data_catcher, internal_server_error_catcher],
                )
                .manage(tss_state)
                .manage(client_state)
                .manage(restore_state)
                .manage(payment_tracker);

            l
        })
    })
    .start();

    // Shutdown tasks
    let _ = quit_tx.send(true);
    t.shutdown();

    observability_providers.shutdown();
}

/// NOTE: Run in a dedicated runtime, to avoid blocking a runtime that is needed to perform tasks.
async fn init_observability(
    cfg: &LitConfig,
    port: u16,
    external_ip: String,
) -> Result<ObservabilityProviders> {
    lit_logging::set_panic_hook();

    if !cfg.enable_observability_export()? {
        #[cfg(not(feature = "testing"))]
        simple_logging_subscriber(cfg, Some(format!("{} -", port)))?.init();

        #[cfg(feature = "testing")]
        simple_file_logging_subscriber(cfg, Some(format!("{} -", port)))?.init();

        return Ok(ObservabilityProviders::default());
    }

    let otel_resource = Resource::new(vec![
        KeyValue::new(
            lit_observability::opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            PKG_NAME,
        ),
        KeyValue::new(
            lit_observability::opentelemetry_semantic_conventions::resource::URL_DOMAIN,
            external_ip,
        ),
        KeyValue::new(
            lit_observability::opentelemetry_semantic_conventions::resource::URL_PORT,
            port.to_string(),
        ),
        KeyValue::new(
            "lit.subnet.id",
            cfg.subnet_id().expect("failed to load subnet_id"),
        ),
    ]);

    let (tracing_provider, metrics_provider, subscriber, logger_provider) =
        lit_observability::create_providers(
            cfg,
            otel_resource.clone(),
            sdktrace::Config::default().with_resource(otel_resource.clone()),
        )
        .await
        .map_err(|e| unexpected_err(e, Some("failed to create OTEL providers: {:?}".into())))?;

    // Add privacy mode layer to disable tracing when privacy_mode is enabled
    // The privacy mode layer checks thread-local state set by the fairing
    use tracing_subscriber::layer::SubscriberExt;
    let subscriber = subscriber.with(crate::utils::rocket::privacy_mode::PrivacyModeLayer);

    // Set globals
    global::set_text_map_propagator(TraceContextPropagator::new());
    global::set_tracer_provider(tracing_provider);
    global::set_meter_provider(metrics_provider.clone());
    subscriber.init();

    Ok(ObservabilityProviders::new(
        metrics_provider,
        logger_provider,
    ))
}

#[derive(Default)]
struct ObservabilityProviders {
    meter_provider: Option<SdkMeterProvider>,
    logger_provider: Option<LoggerProvider>,
}

impl ObservabilityProviders {
    fn new(meter_provider: SdkMeterProvider, logger_provider: LoggerProvider) -> Self {
        Self {
            meter_provider: Some(meter_provider),
            logger_provider: Some(logger_provider),
        }
    }

    fn shutdown(self) {
        if let Some(meter_provider) = self.meter_provider {
            if let Err(e) = meter_provider.shutdown() {
                error!("Failed to shutdown metrics provider: {:?}", e);
            }
        }
        if let Some(logger_provider) = self.logger_provider {
            if let Err(e) = logger_provider.shutdown() {
                error!("Failed to shutdown logger provider: {:?}", e);
            }
        }
    }
}
