use super::deprecated_endpoint_error;
use crate::auth::auth_material::JsonAuthSigExtended;
use crate::client_session::ClientSessionHandler;
use crate::endpoints::{admin, pkp, recovery, web_client};
use crate::models;
use crate::payment::delegated_usage::DelegatedUsageDB;
use crate::siwe_db::rpc::EthBlockhashCache;
use crate::tss::common::restore::RestoreState;
use crate::tss::common::tss_state::TssState;
use crate::utils::rocket::guards::RequestHeaders;
use crate::utils::web::with_timeout;
use lit_api_core::context::{SdkVersion, Tracer, Tracing, TracingRequired};
use lit_api_core::error::ApiError;
use lit_core::config::ReloadableLitConfig;
use lit_node_common::client_state::ClientState;
use lit_node_core::{
    request::{self, EncryptionSignRequest, JsonPKPClaimKeyRequest, SDKHandshakeRequest},
    response::GenericResponse,
};
use lit_sdk::EncryptedPayload;
use moka::future::Cache;
use rocket::response::status;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::{Data, Route, State};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::instrument;

#[allow(dead_code)]
pub(crate) fn routes() -> Vec<Route> {
    routes![
        admin_set,
        admin_get,
        admin_get_key_backup,
        admin_set_key_backup,
        admin_get_blinders,
        admin_set_blinders,
        recovery_set_dec_share,
        recovery_set_dec_shares,
        recovery_get_dec_key_share,
        recovery_delete_dec_key_share,
        handshake,
        encryption_sign,
        sign_session_key,
        pkp_sign,
        pkp_claim,
        execute_function
    ]
}

#[post("/web/admin/set", format = "json", data = "<request>")]
#[instrument(level = "debug", name = "POST /web/admin/set", skip_all, ret)]
pub async fn admin_set(
    remote_addr: SocketAddr,
    cfg: &State<ReloadableLitConfig>,
    request: Json<models::JsonAdminSetRequest>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        admin::endpoints::admin_set(remote_addr, cfg, request).await
    })
    .await
}

#[post("/web/admin/get", format = "json", data = "<auth>")]
#[instrument(level = "debug", name = "POST /web/admin/get", skip_all, ret)]
pub async fn admin_get(
    cfg: &State<ReloadableLitConfig>,
    auth: Json<JsonAuthSigExtended>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        admin::endpoints::admin_get(cfg, auth.0).await
    })
    .await
}

#[get("/web/admin/get_key_backup?<auth>&<node_set_hash>")]
#[instrument(level = "debug", name = "GET /web/admin/get_key_backup", skip_all, ret)]
pub async fn admin_get_key_backup(
    cfg: &State<ReloadableLitConfig>,
    tss_state: &State<Arc<TssState>>,
    restore_state: &State<Arc<RestoreState>>,
    auth: JsonAuthSigExtended,
    node_set_hash: Option<String>,
) -> Result<Vec<u8>, status::Custom<Value>> {
    admin::endpoints::admin_get_key_backup(cfg, tss_state, restore_state, auth, None).await
}

#[post("/web/admin/set_key_backup", format = "binary", data = "<data>")]
#[instrument(
    level = "trace",
    name = "POST /web/admin/set_key_backup",
    skip_all,
    ret
)]
pub async fn admin_set_key_backup(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    admin_auth_sig: JsonAuthSigExtended,
    data: Data<'_>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        admin::endpoints::admin_set_key_backup(cfg, restore_state, admin_auth_sig, data).await
    })
    .await
}

#[post("/web/admin/get_blinders", format = "json", data = "<auth>")]
#[instrument(level = "debug", name = "POST /web/admin/get_blinders", skip_all, ret)]
pub async fn admin_get_blinders(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    auth: Json<JsonAuthSigExtended>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        admin::endpoints::admin_get_blinders(cfg, restore_state, auth.0).await
    })
    .await
}

#[post("/web/admin/set_blinders", format = "binary", data = "<data>")]
#[instrument(level = "debug", name = "POST /web/admin/set_blinders", skip_all, ret)]
pub async fn admin_set_blinders(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    admin_auth_sig: JsonAuthSigExtended,
    data: Data<'_>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        use rocket::data::ToByteUnit;

        // Read the data into a string.
        let contents = match data.open(1024_u32.bytes()).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => {
                return crate::error::parser_err("Payload too large.".to_string(), None).handle();
            }
            Err(e) => {
                return crate::error::parser_err(e, Some("Error reading blinders".into())).handle();
            }
        };

        #[derive(serde::Deserialize)]
        struct BlindersFile {
            blinders: lit_node_core::Blinders,
        }

        let blinders = match serde_json::from_str::<BlindersFile>(&contents) {
            Ok(blinders) => blinders.blinders,
            Err(e) => match admin::utils::parse_datil_blinder_file(&contents) {
                Ok(blinders) => blinders,
                Err(e) => {
                    return crate::error::parser_err(e, Some("Error parsing blinders".into()))
                        .handle();
                }
            },
        };

        let sig = admin_auth_sig.into();
        admin::endpoints::admin_set_blinders(cfg, restore_state, &sig, &blinders).await
    })
    .await
}

#[post("/web/recovery/set_dec_share", format = "json", data = "<request>")]
#[instrument(
    level = "trace",
    name = "POST /web/recovery/set_dec_share",
    skip_all,
    ret
)]
pub async fn recovery_set_dec_share(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    request: Json<models::JsonRecoverySetDecShare>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        recovery::endpoints::recovery_set_dec_share(cfg, restore_state, request).await
    })
    .await
}

#[post("/web/recovery/set_dec_shares", format = "json", data = "<request>")]
#[instrument(
    level = "trace",
    name = "POST /web/recovery/set_dec_shares",
    skip_all,
    ret
)]
pub async fn recovery_set_dec_shares(
    cfg: &State<ReloadableLitConfig>,
    restore_state: &State<Arc<RestoreState>>,
    request: Json<models::JsonRecoverySetDecShares>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        recovery::endpoints::recovery_set_dec_shares(cfg, restore_state, request).await
    })
    .await
}

// DATIL_BACKUP: Remove this function once old Datil backup is obsolete.
#[post("/web/recovery/get_dec_share", format = "json", data = "<request>")]
#[instrument(
    level = "trace",
    name = "POST /web/recovery/get_dec_share",
    skip_all,
    ret
)]
pub async fn recovery_get_dec_key_share(
    restore_state: &State<Arc<TssState>>,
    cfg: &State<ReloadableLitConfig>,
    request: Json<models::DownloadShareRequest>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        recovery::endpoints::recovery_get_dec_key_share(restore_state, cfg, request).await
    })
    .await
}

#[post("/web/recovery/delete_dec_share", format = "json", data = "<request>")]
#[instrument(
    level = "trace",
    name = "POST /web/recovery/delete_dec_share",
    skip_all,
    ret
)]
pub async fn recovery_delete_dec_key_share(
    restore_state: &State<Arc<TssState>>,
    cfg: &State<ReloadableLitConfig>,
    request: Json<models::DownloadShareRequest>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        recovery::endpoints::recovery_delete_dec_key_share(restore_state, cfg, request).await
    })
    .await
}

/*
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"clientPublicKey": "test"}' \
  http://localhost:7470/web/handshake
*/
#[post("/web/handshake", format = "json", data = "<json_handshake_request>")]
#[instrument(level = "debug", name = "POST /web/handshake", skip_all, fields(correlation_id = tracing_required.correlation_id()))]
#[allow(clippy::too_many_arguments)]
pub async fn handshake(
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    json_handshake_request: Json<SDKHandshakeRequest>,
    tracing_required: TracingRequired,
    version: SdkVersion,
    cfg: &State<ReloadableLitConfig>,
    eth_blockhash_cache: &State<Arc<EthBlockhashCache>>,
    client_state: &State<Arc<ClientState>>,
) -> status::Custom<Value> {
    with_timeout(&cfg.load_full(), None, None, async move {
        web_client::handshake_v0(
            session,
            remote_addr,
            json_handshake_request,
            tracing_required,
            version,
            cfg,
            eth_blockhash_cache,
            client_state,
        )
        .await
    })
    .await
}

#[allow(clippy::too_many_arguments)]
#[post(
    "/web/encryption/sign",
    format = "json",
    data = "<encryption_sign_request>"
)]
#[instrument(level = "debug", name = "POST /web/encryption/sign", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
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
    "/web/sign_session_key",
    format = "json",
    data = "<json_sign_session_key_request>"
)]
#[instrument(level = "debug", name = "POST /web/sign_session_key", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn sign_session_key(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    delegation_usage_db: &State<Arc<DelegatedUsageDB>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    json_sign_session_key_request: Json<models::JsonSignSessionKeyRequest>,
    tracing: Tracing,
    request_headers: RequestHeaders<'_>,
) -> status::Custom<Value> {
    deprecated_endpoint_error()
}

#[post("/web/pkp/sign", format = "json", data = "<json_pkp_signing_request>")]
#[instrument(level = "debug", name = "POST /web/pkp/sign", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
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

#[post("/web/pkp/claim", format = "json", data = "<json_pkp_claim_request>")]
#[instrument(level = "debug", name = "POST /web/pkp/claim", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn pkp_claim(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    client_state: &State<Arc<ClientState>>,
    json_pkp_claim_request: Json<EncryptedPayload<JsonPKPClaimKeyRequest>>,
    tracing: Tracing,
    http_client: &State<reqwest::Client>,
) -> status::Custom<Value> {
    let (pkp_claim_request, client_session) =
        match client_state.json_decrypt_to_session(&json_pkp_claim_request.0) {
            Ok(data) => data,
            Err(e) => {
                let handle = e.handle();
                let msg = GenericResponse::err_and_data_json("can't decrypt".to_string(), handle.1);
                return status::Custom(handle.0, json!(msg));
            }
        };
    let client_session = Arc::new(client_session);

    with_timeout(
        &cfg.load_full(),
        None,
        Some(client_session.clone()),
        async move {
            pkp::pkp_claim(
                remote_addr,
                tss_state,
                auth_context_cache,
                cfg,
                allowlist_cache,
                client_state,
                pkp_claim_request,
                client_session,
                http_client,
            )
            .await
        },
    )
    .await
}

/*
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"js_base64": "Y29uc29sZS5sb2coInJ1bm5pbmchIik7Cgpjb25zdCBnbyA9IGFzeW5jICgpID0+IHsKICBsZXQgZGF0YSA9IGF3YWl0IGZldGNoKAogICAgImh0dHBzOi8vaXBmcy5saXRnYXRld2F5LmNvbS9pcGZzL1FtTmlEckRuVGlTbzR5NzhxS3dhWmJvcThLZlQ5WTNDUnJuTTdwZlVtRzFFRnEiCiAgKS50aGVuKChyZXNwb25zZSkgPT4gcmVzcG9uc2UuanNvbigpKTsKICBjb25zb2xlLmxvZygiZGF0YTogIiArIEpTT04uc3RyaW5naWZ5KGRhdGEsIG51bGwsIDIpKTsKfTsKCmdvKCk7CmNvbnNvbGUubG9nKCJmZXRjaGluZyBraWNrZWQgb2ZmLCBidXQgd2FpdGluZyBmb3IgcHJvbWlzZXMiKTsK"}' \
  http://localhost:7470/web/execute
*/
#[cfg(feature = "lit-actions")]
#[post("/web/execute", format = "json", data = "<json_execution_request>")]
#[instrument(level = "debug", name = "POST /web/execute", skip_all, fields(correlation_id = tracing.correlation_id()), ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn execute_function(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
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
