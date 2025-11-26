use crate::auth::auth_material::AuthSigItemExtendedRef;
use crate::error::unexpected_err;
use crate::models::auth::SessionKeySignedMessageV2;
use crate::models::{AllowlistCache, AuthContextCache};
use crate::payment::delegated_usage::DelegatedUsageDB;
use crate::payment::selection::get_payment_method;
use crate::payment::{payed_endpoint::PayedEndpoint, payment_tracker::PaymentTracker};
use crate::pkp::auth::AuthMethodScope;
use crate::pkp::utils::{claim_key, sign};
use crate::tss::common::tss_state::TssState;
use crate::utils::web::get_auth_context;
use lit_node_common::config::LitNodeConfig;

use crate::client_session::ClientSession;
use crate::utils::web::pubkey_to_token_id;
use crate::utils::web::{
    get_auth_context_from_session_sigs, get_bls_root_pubkey, get_signed_message,
};
use lit_api_core::error::ApiError;
use lit_core::config::ReloadableLitConfig;
use lit_node_common::client_state::ClientState;
use lit_node_core::request::JsonPKPClaimKeyRequest;
use lit_node_core::request::JsonPKPSigningRequest;
use lit_node_core::response::GenericResponse;
use lit_node_core::response::JsonPKPSigningResponse;
use lit_node_core::{AuthSigItem, EndpointVersion, PKPNFTResource, constants::CHAIN_ETHEREUM};
use rocket::State;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Value, serde_json::json};
use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

#[allow(clippy::too_many_arguments)]
pub(crate) async fn pkp_sign(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<AuthContextCache>>,
    delegation_usage_db: Option<&State<Arc<DelegatedUsageDB>>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<AllowlistCache>>,
    client_state: &Arc<ClientState>,
    json_pkp_signing_request: JsonPKPSigningRequest,
    client_session: Arc<ClientSession>,
    payment_tracker: &State<Arc<PaymentTracker>>,
    endpoint_version: EndpointVersion,
    request_id: String,
    http_client: &State<reqwest::Client>,
) -> status::Custom<Value> {
    trace!("pkp sign, request: {:?}", json_pkp_signing_request);
    let cfg = cfg.load_full();

    let mut timing: BTreeMap<String, Duration> = BTreeMap::new();
    let request_start = std::time::Instant::now();
    let before = std::time::Instant::now();

    let token_id = match pubkey_to_token_id(&json_pkp_signing_request.pubkey) {
        Ok(token_id) => token_id,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response(
                "can't get token id from public key",
                e.handle(),
            );
        }
    };
    let resource = PKPNFTResource::new(token_id);
    let resource_ability = resource.signing_ability();

    // Validate auth sig item
    let bls_root_pubkey = match get_bls_root_pubkey(tss_state, None) {
        Ok(bls_root_pubkey) => bls_root_pubkey,
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("No bls root key exists", e.handle());
        }
    };

    let validated_address = {
        match AuthSigItemExtendedRef(&json_pkp_signing_request.auth_sig)
            .validate_and_get_user_address(
                &resource_ability,
                &Some(CHAIN_ETHEREUM.to_string()),
                &cfg,
                &bls_root_pubkey,
                &endpoint_version,
            )
            .await
        {
            Err(e) => {
                return client_session
                    .json_encrypt_err_custom_response("invalid user address", e.handle());
            }
            Ok(resp) => resp,
        }
    };
    timing.insert("auth sig validation".to_string(), before.elapsed());
    let before = std::time::Instant::now();

    // Handle payment depending on the version

    if cfg.enable_payment().unwrap_or(true) {
        let delegation_usage_db = match delegation_usage_db {
            Some(db) => db,
            None => {
                let msg = format!(
                    "Delegation db is not provided to {}, version {}",
                    "pkp_sign", endpoint_version as u8
                );
                return client_session.json_encrypt_err_and_code(
                    &msg,
                    "delegation_usage_db_not_provided",
                    Status::PaymentRequired,
                );
            }
        };

        let before = std::time::Instant::now();
        let single_auth_sig = match &json_pkp_signing_request.auth_sig {
            AuthSigItem::Single(single_auth_sig) => single_auth_sig,
            AuthSigItem::Multiple(_) => {
                let err_msg = "MultiAuthSig not supported for payment";
                error!("{}", err_msg);
                return client_session.json_encrypt_err_response(err_msg, Status::PaymentRequired);
            }
        };

        let signed_message: SessionKeySignedMessageV2 =
            match serde_json::from_str(&single_auth_sig.signed_message) {
                Ok(signed_message) => signed_message,
                Err(e) => {
                    let err_msg = "Parsing SessionKeySignedMessageV2 failed. \
                        The sessionSig is incorrectly formatted";
                    error!("{}", err_msg);
                    return client_session
                        .json_encrypt_err_response(err_msg, Status::PaymentRequired);
                }
            };

        let user_address = match validated_address.evm_address() {
            Ok(address) => address,
            Err(e) => {
                return client_session.json_encrypt_err_custom_response(
                    "can't convert address to an evm address",
                    e.handle(),
                );
            }
        };

        let peers = tss_state.peer_state.peers();

        let curve_type = json_pkp_signing_request.signing_scheme.curve_type();
        let threshold = match tss_state
            .get_threshold_using_current_epoch_realm_peers_for_curve(
                &peers,
                curve_type,
                Some(json_pkp_signing_request.epoch),
            )
            .await
        {
            Ok(t) => t,
            Err(e) => {
                return client_session
                    .json_encrypt_err_custom_response("unable to get threshold", e.handle());
            }
        };

        let payment_method = get_payment_method(
            &user_address,
            PayedEndpoint::PkpSign,
            threshold,
            signed_message.max_price,
            Some(signed_message),
            payment_tracker,
            delegation_usage_db,
            &bls_root_pubkey,
            &cfg,
        )
        .await;

        let pending_payment = match payment_method {
            Ok(payment) => payment,
            Err(e) => {
                return client_session
                    .json_encrypt_err_custom_response("can't get payment method", e.handle());
            }
        };
        timing.insert("verify the payment".to_string(), before.elapsed());

        let before = std::time::Instant::now();
        payment_tracker.batches().add(pending_payment).await;
        timing.insert("register pending payment".to_string(), before.elapsed());
    };

    let before = std::time::Instant::now();

    // check for single or multiple auth sigs and do the session key
    // capability check.  set the wallet that provided the capabilities as the
    // main auth sig wallet.
    let auth_sig = match &json_pkp_signing_request.auth_sig {
        AuthSigItem::Single(single_auth_sig) => single_auth_sig.clone(),
        AuthSigItem::Multiple(_) => {
            return client_session.json_encrypt_err_and_code(
                "Multiple auth sigs not supported by Lit Actions",
                "unsupported_auth_sig",
                Status::BadRequest,
            );
        }
    };

    let before = std::time::Instant::now();

    let auth_context = match endpoint_version {
        EndpointVersion::Initial => {
            let auth_context = get_auth_context(
                Some(auth_sig.clone()),
                json_pkp_signing_request.auth_methods.clone(),
                None,
                Some(auth_context_cache),
                false,
                cfg.clone(),
                None,
                &bls_root_pubkey,
                &endpoint_version,
                reqwest::Client::clone(http_client),
            )
            .await;

            match auth_context {
                Ok(auth_context) => auth_context,
                Err(e) => {
                    return client_session
                        .json_encrypt_err_custom_response("invalid auth context", e.handle());
                }
            }
        }
        EndpointVersion::V1 | EndpointVersion::V2 => {
            let msg = get_signed_message(&auth_sig.signed_message);
            let signed_message = match msg {
                Ok(signed_message) => signed_message,
                Err(err_msg) => {
                    return client_session.json_encrypt_err_and_code(
                        &err_msg,
                        "unsupported_auth_sig",
                        Status::BadRequest,
                    );
                }
            };

            let resolved_auth_context =
                match get_auth_context_from_session_sigs(signed_message).await {
                    Ok(resolved_auth_context) => resolved_auth_context,
                    Err(e) => {
                        error!("Error parsing AuthContext from sessionSig");
                        return client_session.json_encrypt_err_custom_response(
                            "can't parse auth context from session signature",
                            e.handle(),
                        );
                    }
                };

            trace!("resolved_auth_context- {:?}", resolved_auth_context);

            match resolved_auth_context {
                Some(resolved_auth_context) => resolved_auth_context,
                None => {
                    // Also create new auth_context for EOA authSig/sessionSigs
                    let new_auth_context = get_auth_context(
                        Some(auth_sig.clone()),
                        None,
                        None,
                        None,
                        false,
                        cfg.clone(),
                        None,
                        &bls_root_pubkey,
                        &endpoint_version,
                        reqwest::Client::clone(http_client),
                    )
                    .await;

                    match new_auth_context {
                        Ok(new_auth_context) => new_auth_context,
                        Err(e) => {
                            return client_session.json_encrypt_err_custom_response(
                                "can't create an auth context from the EOA auth-sig",
                                e.handle(),
                            );
                        }
                    }
                }
            }
        }
    };

    timing.insert("auth context".to_string(), before.elapsed());
    let before = std::time::Instant::now();
    trace!("Got auth context");

    let epoch = match json_pkp_signing_request.epoch {
        0 => None,
        i => Some(i),
    };
    trace!(
        "PKP Signing scheme: {}",
        json_pkp_signing_request.signing_scheme
    );

    let result = sign(
        cfg.as_ref(),
        &json_pkp_signing_request.to_sign,
        json_pkp_signing_request.pubkey.clone(),
        request_id.clone(),
        None, // Only the first one as we only allow running a single Lit Action now for session creation
        Some(auth_sig.clone()), // This works with EOA wallets as well as we're passing SessionSig/AuthSig here
        auth_context,
        Some(tss_state.as_ref().clone()),
        &[AuthMethodScope::SignAnything as usize],
        epoch,
        &bls_root_pubkey,
        &json_pkp_signing_request.node_set,
        json_pkp_signing_request.signing_scheme,
    )
    .await
    .map_err(|e| unexpected_err(e, Some("Error signing with the PKP".to_string())));
    timing.insert("sign".to_string(), before.elapsed());

    let result = match result {
        Ok(result) => client_session.json_encrypt_response_status(JsonPKPSigningResponse {
            success: true,
            signed_data: json_pkp_signing_request.to_sign.clone(),
            signature_share: result,
        }),
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("unable to get signature share", e.handle());
        }
    };

    timing.insert("total".to_string(), request_start.elapsed());

    debug!("POST /web/pkp/sign timing: {:?}", timing);

    result
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn pkp_claim(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<AuthContextCache>>,
    cfg: &State<ReloadableLitConfig>,
    allowlist_cache: &State<Arc<AllowlistCache>>,
    client_state: &Arc<ClientState>,
    pkp_claim_request: JsonPKPClaimKeyRequest,
    client_session: Arc<ClientSession>,
    http_client: &State<reqwest::Client>,
) -> status::Custom<Value> {
    let request_start = std::time::Instant::now();
    debug!(
        "pkp claim, request: {:}",
        format!("{:?}", pkp_claim_request)
    );

    let cfg = cfg.load_full();

    let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

    let before = std::time::Instant::now();

    let claim_res = claim_key(
        cfg.as_ref(),
        &pkp_claim_request,
        reqwest::Client::clone(http_client),
    )
    .await;
    timing.insert("claim key".to_string(), before.elapsed());

    timing.insert("total".to_string(), request_start.elapsed());

    debug!("POST /web/pkp/claim timing: {:?}", timing);

    match claim_res {
        Ok(resp) => status::Custom(
            Status::Accepted,
            json!(client_session.json_encrypt_response(GenericResponse::ok(resp))),
        ),
        Err(e) => client_session.json_encrypt_err_custom_response(
            "an error occurred while claiming key",
            unexpected_err(e, Some("An error occurred in claim process".into())).handle(),
        ),
    }
}
