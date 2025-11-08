use crate::auth::auth_material::JsonAuthSigExtended;
use crate::client_session::ClientSessionHandler;
use crate::endpoints::{admin, pkp, web_client};
use crate::functions::ActionStore;
use crate::models;
use crate::payment::delegated_usage::DelegatedUsageDB;
use crate::payment::{payed_endpoint::PayedEndpoint, payment_tracker::PaymentTracker};
use crate::peers::grpc_client_pool::GrpcClientPool;
use crate::tss::common::{restore::restore_state::RestoreState, tss_state::TssState};
use crate::utils::rocket::guards::RequestHeaders;
use crate::utils::web::with_timeout;
use lit_api_core::context::{Tracer, Tracing};
use lit_api_core::error::ApiError;
use lit_core::config::ReloadableLitConfig;
use lit_node_common::client_state::ClientState;
use lit_node_core::request::EncryptionSignRequest;
use lit_node_core::response::GenericResponse;
use lit_node_core::{EndpointVersion, request};
use lit_sdk::EncryptedPayload;
use moka::future::Cache;
use rocket::response::status;
use rocket::serde::json::{Json, Value, serde_json::json};
use rocket::{Route, State};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[allow(dead_code)]
pub(crate) fn routes() -> Vec<Route> {
    routes![
        admin_get_key_backup,
        admin_get_blinders,
        admin_set_blinders,
        sign_session_key,
        encryption_sign,
        pkp_sign,
        execute_function,
        get_job_status,
    ]
}

#[post(
    "/web/sign_session_key/v2",
    format = "json",
    data = "<json_sign_session_key_request>"
)]
#[instrument(level = "debug", name = "POST /web/sign_session_key/v2", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn sign_session_key(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    delegation_usage_db: Option<&State<Arc<DelegatedUsageDB>>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    http_client: &State<reqwest::Client>,
    cfg: &State<ReloadableLitConfig>,
    client_state: &State<Arc<ClientState>>,
    payment_tracker: &State<Arc<PaymentTracker>>,
    json_sign_session_key_request: Json<
        EncryptedPayload<lit_node_core::request::JsonSignSessionKeyRequestV2>,
    >,
    tracing: Tracing,
    request_headers: RequestHeaders<'_>,
) -> status::Custom<Value> {
    payment_tracker.register_usage(&PayedEndpoint::SignSessionKey);

    let (json_sign_session_key_request, client_session) =
        match client_state.json_decrypt_to_session(&json_sign_session_key_request) {
            Ok(request) => request,
            Err(e) => {
                let handle = e.handle();
                return status::Custom(
                    handle.0,
                    json!(GenericResponse::err_and_data_json(
                        "can't decrypt".to_string(),
                        handle.1
                    )),
                );
            }
        };
    let client_session = Arc::new(client_session);

    let call_result = with_timeout(
        &cfg.load_full(),
        None,
        Some(client_session.clone()),
        async move {
            web_client::sign_session_key(
                remote_addr,
                tss_state,
                auth_context_cache,
                ipfs_cache,
                cfg,
                client_state,
                json_sign_session_key_request,
                client_session,
                request_headers,
                EndpointVersion::V2,
                delegation_usage_db,
                payment_tracker,
                tracing.correlation_id().to_owned(),
                http_client,
            )
            .await
        },
    )
    .await;

    payment_tracker.deregister_usage(&PayedEndpoint::SignSessionKey);

    call_result
}

#[allow(clippy::too_many_arguments)]
#[post(
    "/web/encryption/sign/v2",
    format = "json",
    data = "<encryption_sign_request>"
)]
#[instrument(level = "debug", name = "POST /web/encryption/sign/v2", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
pub(crate) async fn encryption_sign(
    session: &State<Arc<TssState>>,
    payment_tracker: &State<Arc<PaymentTracker>>,
    delegation_usage_db: &State<Arc<DelegatedUsageDB>>,
    remote_addr: SocketAddr,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    client_state: &State<Arc<ClientState>>,
    http_client: &State<reqwest::Client>,
    encryption_sign_request: Json<EncryptedPayload<EncryptionSignRequest>>,
    tracing: Tracing,
) -> status::Custom<Value> {
    // TODO: Uncomment this once we support EIP1271 SessionSig as we currently only support EIP1271 AuthSig
    // // We can only accept a SessionSig in this endpoint
    // match encryption_sign_request.auth_sig.get_auth_type() {
    //     Ok(auth_material_type) => {
    //         if auth_material_type != &AuthMaterialType::SessionSig {
    //             return validation_err_code(
    //                 "Can't provide AuthSig for encryption_sign. \
    //                     You have to provide a SessionSig.",
    //                 EC::NodeCannotProvideAuthSigForEndpoint,
    //                 None,
    //             )
    //             .handle();
    //         }
    //     }
    //     Err(e) => return e.handle(),
    // };

    payment_tracker.register_usage(&PayedEndpoint::EncryptionSign);

    let (encryption_sign_request, client_session) =
        match client_state.json_decrypt_to_session(&encryption_sign_request) {
            Ok(request) => request,
            Err(e) => {
                let handle = e.handle();
                let msg = GenericResponse::err_and_data_json("can't decrypt".to_string(), handle.1);
                return status::Custom(handle.0, json!(msg));
            }
        };
    let client_session = Arc::new(client_session);

    let call_result = with_timeout(
        &cfg.load_full(),
        None,
        Some(client_session.clone()),
        async move {
            web_client::encryption_sign(
                session,
                remote_addr,
                Some(delegation_usage_db),
                ipfs_cache,
                cfg,
                client_state,
                encryption_sign_request,
                client_session,
                payment_tracker,
                EndpointVersion::V2,
                tracing.correlation_id().to_owned(),
                http_client,
            )
            .await
        },
    )
    .await;

    payment_tracker.deregister_usage(&PayedEndpoint::EncryptionSign);

    call_result
}

#[cfg(feature = "lit-actions")]
#[post("/web/execute/v2", format = "json", data = "<json_execution_request>")]
#[instrument(level = "debug", name = "POST /web/execute/v2", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn execute_function(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    delegation_usage_db: &State<Arc<DelegatedUsageDB>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    http_client: &State<reqwest::Client>,
    payment_tracker: &State<Arc<PaymentTracker>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    client_state: &State<Arc<ClientState>>,
    client_grpc_connections: &State<GrpcClientPool<tonic::transport::Channel>>,
    json_execution_request: Json<EncryptedPayload<request::JsonExecutionRequest>>,
    tracing: Tracing,
    request_headers: RequestHeaders<'_>,
    action_store: &State<ActionStore>,
) -> status::Custom<Value> {
    payment_tracker.register_usage(&PayedEndpoint::LitAction);

    let (json_execution_request, client_session) =
        match client_state.json_decrypt_to_session(&json_execution_request) {
            Ok(request) => request,
            Err(e) => {
                let handle = e.handle();
                return status::Custom(
                    handle.0,
                    json!(GenericResponse::err_and_data_json(
                        "can't decrypt".to_string(),
                        handle.1
                    )),
                );
            }
        };
    let client_session = Arc::new(client_session);

    let actions_config = tss_state.chain_data_config_manager.get_actions_config();

    let call_result = with_timeout(
        &cfg.load_full(),
        Some(actions_config.timeout_ms),
        Some(client_session.clone()),
        async move {
            web_client::execute_function(
                remote_addr,
                tss_state,
                auth_context_cache,
                Some(delegation_usage_db),
                cfg,
                client_grpc_connections,
                allowlist_cache,
                ipfs_cache,
                client_state,
                json_execution_request,
                client_session,
                request_headers,
                payment_tracker,
                EndpointVersion::V2,
                tracing.correlation_id().to_owned(),
                action_store,
                http_client,
            )
            .await
        },
    )
    .await;

    payment_tracker.deregister_usage(&PayedEndpoint::LitAction);

    call_result
}

#[cfg(feature = "lit-actions")]
#[post("/web/job_status/v2", format = "json", data = "<job_status_request>")]
#[instrument(level = "debug", name = "POST /web/job_status/v2", skip_all, ret)]
pub(crate) async fn get_job_status(
    job_status_request: Json<EncryptedPayload<models::JsonJobStatusRequest>>,
    action_store: &State<ActionStore>,
    tss_state: &State<Arc<TssState>>,
    cfg: &State<ReloadableLitConfig>,
    client_state: &State<Arc<ClientState>>,
) -> status::Custom<Value> {
    let (job_status_request, client_session) =
        match client_state.json_decrypt_to_session(&job_status_request) {
            Ok(request) => request,
            Err(e) => {
                let handle = e.handle();
                return status::Custom(
                    handle.0,
                    json!(GenericResponse::err_and_data_json(
                        "can't decrypt".to_string(),
                        handle.1
                    )),
                );
            }
        };
    let client_session = Arc::new(client_session);

    with_timeout(
        &cfg.load_full(),
        None,
        Some(client_session.clone()),
        async move {
            web_client::get_job_status(
                job_status_request,
                client_session,
                action_store,
                tss_state,
                cfg,
                client_state,
            )
            .await
        },
    )
    .await
}

#[post(
    "/web/pkp/sign/v2",
    format = "json",
    data = "<json_pkp_signing_request>"
)]
#[instrument(level = "debug", name = "POST /web/pkp/sign/v2", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn pkp_sign(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    delegation_usage_db: &State<Arc<DelegatedUsageDB>>,
    payment_tracker: &State<Arc<PaymentTracker>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    client_state: &State<Arc<ClientState>>,
    json_pkp_signing_request: Json<EncryptedPayload<request::JsonPKPSigningRequest>>,
    tracing: Tracing,
    http_client: &State<reqwest::Client>,
) -> status::Custom<Value> {
    payment_tracker.register_usage(&PayedEndpoint::PkpSign);

    let (json_pkp_signing_request, client_session) =
        match client_state.json_decrypt_to_session(&json_pkp_signing_request) {
            Ok(json_pkp_signing_request) => json_pkp_signing_request,
            Err(e) => {
                let handle = e.handle();
                let msg = GenericResponse::err_and_data_json("can't decrypt".to_string(), handle.1);
                return status::Custom(handle.0, json!(msg));
            }
        };
    let client_session = Arc::new(client_session);

    let call_result = with_timeout(
        &cfg.load_full(),
        None,
        Some(client_session.clone()),
        async move {
            pkp::pkp_sign(
                remote_addr,
                tss_state,
                auth_context_cache,
                Some(delegation_usage_db),
                cfg,
                allowlist_cache,
                client_state,
                json_pkp_signing_request,
                client_session,
                payment_tracker,
                EndpointVersion::V2,
                tracing.correlation_id().to_owned(),
                http_client,
            )
            .await
        },
    )
    .await;

    payment_tracker.deregister_usage(&PayedEndpoint::PkpSign);

    call_result
}

#[post("/web/admin/get_blinders/v2", format = "json", data = "<auth>")]
#[instrument(
    level = "trace",
    name = "POST /web/admin/get_blinders/v2",
    skip_all,
    ret
)]
pub(crate) async fn admin_get_blinders(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    auth: Json<JsonAuthSigExtended>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        admin::endpoints::admin_get_blinders(cfg, restore_state, auth.0).await
    })
    .await
}

#[post("/web/admin/set_blinders/v2", format = "json", data = "<data>")]
#[instrument(
    level = "trace",
    name = "POST /web/admin/set_blinders/v2",
    skip_all,
    ret
)]
pub async fn admin_set_blinders(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    data: Json<lit_sdk::admin::SetBlindersData>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        admin::endpoints::admin_set_blinders(cfg, restore_state, &data.auth_sig, &data.blinders)
            .await
    })
    .await
}

#[post("/web/admin/get_key_backup/v2", format = "json", data = "<data>")]
#[instrument(
    level = "trace",
    name = "POST /web/admin/set_blinders/v2",
    skip_all,
    ret
)]
pub async fn admin_get_key_backup(
    cfg: &State<ReloadableLitConfig>,
    tss_state: &State<Arc<TssState>>,
    restore_state: &State<Arc<RestoreState>>,
    data: Json<lit_sdk::admin::GetKeyBackupParameters>,
) -> Result<Vec<u8>, status::Custom<Value>> {
    let auth = JsonAuthSigExtended {
        auth_sig: data.auth.auth_sig.clone(),
    };
    admin::endpoints::admin_get_key_backup(cfg, tss_state, restore_state, auth, Some(data.epoch))
        .await
}
