use super::deprecated_endpoint_error;
use crate::models;
use crate::payment::delegated_usage::DelegatedUsageDB;
use crate::payment::payment_tracker::PaymentTracker;
use crate::tss::common::tss_state::TssState;
use crate::utils::rocket::guards::RequestHeaders;
use lit_api_core::context::{Tracer, Tracing};
use lit_core::config::ReloadableLitConfig;
use lit_node_common::client_state::ClientState;
use lit_node_core::request;
use lit_node_core::request::EncryptionSignRequest;
use lit_sdk::EncryptedPayload;
use moka::future::Cache;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use rocket::{Route, State};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[allow(dead_code)]
pub(crate) fn routes() -> Vec<Route> {
    routes![
        encryption_sign,
        sign_session_key,
        pkp_sign,
        execute_function
    ]
}

#[allow(clippy::too_many_arguments)]
#[post(
    "/web/encryption/sign/v1",
    format = "json",
    data = "<encryption_sign_request>"
)]
#[instrument(level = "debug", name = "POST /web/encryption/sign/v1", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
pub(crate) async fn encryption_sign(
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    delegation_usage_db: &State<Arc<DelegatedUsageDB>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    encryption_sign_request: Json<EncryptionSignRequest>,
    tracing: Tracing,
) -> status::Custom<Value> {
    deprecated_endpoint_error()
}

#[post(
    "/web/sign_session_key/v1",
    format = "json",
    data = "<json_sign_session_key_request>"
)]
#[instrument(level = "debug", name = "POST /web/sign_session_key/v1", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn sign_session_key(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    delegation_usage_db: Option<&State<Arc<DelegatedUsageDB>>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    client_state: &State<Arc<ClientState>>,
    payment_tracker: &State<Arc<PaymentTracker>>,
    json_sign_session_key_request: Json<EncryptedPayload<models::JsonSignSessionKeyRequestV1>>,
    tracing: Tracing,
    request_headers: RequestHeaders<'_>,
) -> status::Custom<Value> {
    deprecated_endpoint_error()
}

#[post(
    "/web/pkp/sign/v1",
    format = "json",
    data = "<json_pkp_signing_request>"
)]
#[instrument(level = "debug", name = "POST /web/pkp/sign/v1", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn pkp_sign(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    delegation_usage_db: &State<Arc<DelegatedUsageDB>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    json_pkp_signing_request: Json<request::JsonPKPSigningRequest>,
    tracing: Tracing,
) -> status::Custom<Value> {
    deprecated_endpoint_error()
}

#[cfg(feature = "lit-actions")]
#[post("/web/execute/v1", format = "json", data = "<json_execution_request>")]
#[instrument(level = "debug", name = "POST /web/execute/v1", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn execute_function(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<crate::tss::common::tss_state::TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    delegation_usage_db: &State<Arc<DelegatedUsageDB>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    json_execution_request: Json<request::JsonExecutionRequest>,
    tracing: Tracing,
    request_headers: RequestHeaders<'_>,
) -> status::Custom<Value> {
    deprecated_endpoint_error()
}
