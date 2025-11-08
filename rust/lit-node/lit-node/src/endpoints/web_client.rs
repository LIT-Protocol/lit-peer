use crate::auth::auth_material::{
    AuthSigItemExtendedRef, ValidatedAddress, siwe_hash_to_bls_session_hash,
};
use crate::client_session::ClientSession;
use crate::error::{
    EC, connect_err_code, conversion_err, memory_limit_err_code, timeout_err_code, unexpected_err,
    unexpected_err_code, validation_err_code,
};
use crate::error::{parser_err, parser_err_code};
use crate::functions::{ActionStore, JobId, action_client};
use crate::models::auth::SessionKeySignedMessageV2;
use crate::models::{self, RequestConditions};
use crate::payment::delegated_usage::DelegatedUsageDB;
use crate::payment::dynamic::DynamicPayment;
use crate::payment::selection::get_payment_method;
use crate::payment::{payed_endpoint::PayedEndpoint, payment_tracker::PaymentTracker};
use crate::peers::grpc_client_pool::GrpcClientPool;
use crate::pkp;
use crate::pkp::auth::serialize_auth_context_for_checking_against_contract_data;
use crate::siwe_db::utils::make_timestamp_siwe_compatible;
use crate::siwe_db::{db, rpc::EthBlockhashCache};
use crate::tss::common::tss_state::TssState;
use crate::utils::attestation::create_attestation;
use crate::utils::encoding;
use crate::utils::rocket::guards::RequestHeaders;
use crate::utils::web::{
    check_condition_count, get_auth_context, get_bls_root_pubkey, get_ipfs_file,
    hash_access_control_conditions,
};
use crate::utils::web::{get_auth_context_from_session_sigs, get_signed_message};
use crate::version::DataVersionReader;
use crate::{access_control, error};
#[allow(unused_imports)]
use ethers::types::{Address, Bytes, I256};
use ipfs_hasher::IpfsHasher;
use lit_api_core::context::Tracer;
use lit_api_core::context::{SdkVersion, TracingRequired};
use lit_api_core::error::ApiError;
use lit_blockchain::resolver::rpc::{ENDPOINT_MANAGER, RpcHealthcheckPoller};
use lit_core::config::{LitConfig, ReloadableLitConfig};
use lit_node_common::{client_state::ClientState, config::LitNodeConfig};
use lit_node_core::CurveType;
use lit_node_core::SigningScheme;
use lit_node_core::request::{EncryptionSignRequest, JsonExecutionRequest};
use lit_node_core::response::{EncryptionSignResponse, GenericResponse};
use lit_node_core::{
    AccessControlConditionItem, AccessControlConditionResource, AuthSigItem,
    EVMContractConditionItem, EndpointVersion, LitActionResource, LitResource, LitResourceAbility,
    SolRpcConditionItem, UnifiedAccessControlConditionItem,
    constants::{CHAIN_ETHEREUM, LIT_RESOURCE_KEY_RAC, LIT_RESOURCE_PREFIX_RAC},
    request,
    request::JsonSDKHandshakeRequest,
    response::JsonSDKHandshakeResponse,
};
use moka::future::Cache;
use rocket::State;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value, serde_json::json};
use siwe_recap::Capability;
use std::collections::BTreeMap;
use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing::instrument;
use url::Url;

// Not dead code, rather a lint bug
// see https://github.com/rust-lang/rust/issues/92554
#[allow(dead_code)]
const MAX_JWT_EXPIRATION: u64 = 12 * 60 * 60; // 12 hours as seconds

#[cfg_attr(test, visibility::make(pub))]
const NODE_IDENTITY_KEY: &str = "node_identity_key";

#[instrument(level = "debug", name = "POST /web/encryption/sign", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn encryption_sign(
    session: &Arc<TssState>,
    remote_addr: SocketAddr,
    delegation_usage_db: Option<&State<Arc<DelegatedUsageDB>>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &ReloadableLitConfig,
    client_state: &Arc<ClientState>,
    encryption_sign_request: EncryptionSignRequest,
    client_session: Arc<ClientSession>,
    payment_tracker: &State<Arc<PaymentTracker>>,
    endpoint_version: EndpointVersion,
    request_id: String,
    http_client: &State<reqwest::Client>,
) -> status::Custom<Value> {
    let request_start = std::time::Instant::now();

    trace!(
        "encryption_sign, decrypted request: {:?}",
        encryption_sign_request
    );

    let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

    let cfg = cfg.load_full();

    let before = std::time::Instant::now();
    let cc_check = check_condition_count(
        &encryption_sign_request.access_control_conditions,
        &encryption_sign_request.evm_contract_conditions,
        &encryption_sign_request.sol_rpc_conditions,
        &encryption_sign_request.unified_access_control_conditions,
    );
    if let Err(e) = cc_check {
        return client_session
            .json_encrypt_err_custom_response("failed condition count check", e.handle());
    }
    timing.insert("check condition count".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    // Hash the access control condition
    let hash_res = hash_access_control_conditions(RequestConditions {
        access_control_conditions: encryption_sign_request.access_control_conditions.clone(),
        evm_contract_conditions: encryption_sign_request.evm_contract_conditions.clone(),
        sol_rpc_conditions: encryption_sign_request.sol_rpc_conditions.clone(),
        unified_access_control_conditions: encryption_sign_request
            .unified_access_control_conditions
            .clone(),
    });
    let hashed_access_control_conditions = match hash_res {
        Ok(hashed_access_control_conditions) => hashed_access_control_conditions,
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("failed control conditions", e.handle());
        }
    };
    timing.insert(
        "hash access control conditions".to_string(),
        before.elapsed(),
    );

    let lit_acc_resource = AccessControlConditionResource::new(format!(
        "{}/{}",
        hashed_access_control_conditions, encryption_sign_request.data_to_encrypt_hash
    ));
    trace!("lit_acc_resource: {:?}", lit_acc_resource);

    let before = std::time::Instant::now();
    // Validate auth sig item
    let bls_root_pubkey = match get_bls_root_pubkey(session).await {
        Ok(bls_root_pubkey) => bls_root_pubkey,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response("no bls root key", e.handle());
        }
    };
    trace!("bls_root_pubkey: {:?}", bls_root_pubkey);
    let validated_address = {
        match AuthSigItemExtendedRef(&encryption_sign_request.auth_sig)
            .validate_and_get_user_address(
                &lit_acc_resource.decrypt_ability(),
                &encryption_sign_request.chain.clone(),
                &cfg,
                &bls_root_pubkey,
                &endpoint_version,
            )
            .await
        {
            Err(e) => {
                return client_session.json_encrypt_err_custom_response(
                    "couldn't validate user address",
                    e.handle(),
                );
            }
            Ok(resp) => resp,
        }
    };
    timing.insert("validate auth sig".to_string(), before.elapsed());
    trace!("Validated user address: {:?}", validated_address);

    let epoch = match encryption_sign_request.epoch {
        0 => None,
        _ => Some(encryption_sign_request.epoch),
    };

    // Handle payment depending on the version
    if cfg.enable_payment().unwrap_or(true) {
        let delegation_usage_db = match delegation_usage_db {
            Some(db) => db,
            None => {
                let msg = format!(
                    "Delegation db is not provided to {}, version {}",
                    "encryption_sign", endpoint_version as u8
                );
                return client_session.json_encrypt_err_and_code(
                    &msg,
                    "delegation_usage_db_not_provided",
                    Status::PaymentRequired,
                );
            }
        };

        let before = std::time::Instant::now();
        let single_auth_sig = match &encryption_sign_request.auth_sig {
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
                    let err_msg = &format!(
                        "Parsing SessionKeySignedMessageV2 failed. \
                        The sessionSig is incorrectly formatted. : SessionKeySignedMessageV2: {}",
                        single_auth_sig.signed_message
                    );
                    error!("{}", err_msg);
                    return client_session
                        .json_encrypt_err_response(err_msg, Status::PaymentRequired);
                }
            };

        let user_address = match validated_address.evm_address() {
            Ok(address) => address,
            Err(e) => {
                return client_session
                    .json_encrypt_err_custom_response("invalid evm address", e.handle());
            }
        };
        let threshold = session.get_threshold().await;

        let payment_method = get_payment_method(
            &user_address,
            PayedEndpoint::EncryptionSign,
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
                    .json_encrypt_err_custom_response("unable to get payment method", e.handle());
            }
        };
        timing.insert("verify the payment".to_string(), before.elapsed());

        let before = std::time::Instant::now();
        payment_tracker.batches().add(pending_payment).await;
        timing.insert("register pending payment".to_string(), before.elapsed());
    };

    let before = std::time::Instant::now();
    // Check whether user satisfies access control conditions
    let check_result = check_multiple_access_control_conditions(
        &encryption_sign_request.auth_sig,
        &encryption_sign_request.access_control_conditions,
        &encryption_sign_request.evm_contract_conditions,
        &encryption_sign_request.sol_rpc_conditions,
        &encryption_sign_request.unified_access_control_conditions,
        cfg,
        &lit_acc_resource.decrypt_ability(),
        &encryption_sign_request.chain,
        request_id.to_owned(),
        &bls_root_pubkey,
        &endpoint_version,
        None,
        ipfs_cache,
        http_client,
    )
    .await;
    timing.insert(
        "check access control conditions".to_string(),
        before.elapsed(),
    );
    let result = match check_result {
        Ok(result) => result,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response(
                "failed to check access control conditions",
                e.handle(),
            );
        }
    };
    if !result.result {
        return client_session.json_encrypt_err_custom_response("failed access control conditions", validation_err_code("The access control condition check returned that you are not permitted to access this content.  Are you sure you meet the conditions?  Check the auth_sig and the other conditions", EC::NodeAccessControlConditionsReturnedNotAuthorized, None)
            .handle());
    }

    // Get the identity parameter to be signed.
    let identity_parameter = lit_acc_resource.get_resource_key().into_bytes();
    trace!("identity_parameter: {:?}", identity_parameter);

    let before = std::time::Instant::now();
    // Load the BLS secret key share as a blsful key for signing.
    let cipher_state = match session.get_cipher_state(SigningScheme::Bls12381) {
        Ok(cipher_state) => cipher_state,
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("can't load BLS key", e.handle());
        }
    };
    timing.insert("get cipher state".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    // Sign the identity parameter using the blsful secret key share.
    let (signature_share, share_peer_id) = match cipher_state.sign(&identity_parameter, epoch).await
    {
        Ok(signature_share) => signature_share,
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("unable to BLS sign", e.handle());
        }
    };
    timing.insert("sign identity parameter".to_string(), before.elapsed());

    timing.insert("total".to_string(), request_start.elapsed());
    debug!("POST /web/encryption/sign timing: {:?}", timing);

    client_session.json_encrypt_response_status(EncryptionSignResponse {
        result: "success".to_string(),
        signature_share,
        share_id: share_peer_id.to_string(),
    })
}

/*
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"clientPublicKey": "test"}' \
  http://localhost:7470/web/handshake
*/

#[instrument(level = "debug", name = "POST /web/handshake", skip_all, fields(correlation_id = tracing_required.correlation_id()))]
#[allow(clippy::too_many_arguments)]
pub async fn handshake(
    session: &State<Arc<TssState>>,
    remote_addr: SocketAddr,
    json_handshake_request: Json<JsonSDKHandshakeRequest>,
    tracing_required: TracingRequired,
    version: SdkVersion,
    cfg: &State<ReloadableLitConfig>,
    eth_blockhash_cache: &State<Arc<EthBlockhashCache>>,
    client_state: &Arc<ClientState>,
) -> status::Custom<Value> {
    let request_start = std::time::Instant::now();
    let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

    trace!(
        "
        handshake, request: {:?}, client_state: {:?}",
        json_handshake_request, client_state,
    );

    // Validate that the challenge exists in the request.
    let challenge = match &json_handshake_request.challenge {
        Some(challenge) => challenge,
        None => {
            return status::Custom(
                Status::BadRequest,
                json!(GenericResponse::err_and_data_json(
                    "".to_string(),
                    JsonSDKHandshakeResponse {
                        server_public_key: "ERR".to_string(),
                        subnet_public_key: "ERR".to_string(),
                        network_public_key: "ERR".to_string(),
                        network_public_key_set: "ERR".to_string(),
                        client_sdk_version: version.to_string(),
                        hd_root_pubkeys: vec![],
                        attestation: None,
                        latest_blockhash: "".to_string(),
                        node_version: crate::version::get_version().to_string(),
                        node_identity_key: "".to_string(),
                        epoch: 0,
                        git_commit_hash: crate::git_info::GIT_COMMIT_HASH.to_string(),
                    }
                )),
            );
        }
    };

    let cfg = cfg.load_full();

    let before = std::time::Instant::now();
    let ecdsa_root_keys = match session.get_dkg_state(CurveType::K256) {
        Ok(dkg_state) => dkg_state.root_keys().await,
        Err(_) => {
            debug!("Failed to acquire lock on hd_root_keys for ECDSA.");
            vec![]
        }
    };
    timing.insert("get ecdsa root keys".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    let bls_root_keys = match session.get_dkg_state(CurveType::BLS) {
        Ok(dkg_state) => dkg_state.root_keys().await,
        Err(_) => {
            debug!("Failed to acquire lock on hd_root_keys for BLS.");
            vec![]
        }
    };
    timing.insert("get bls root keys".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    // run the attestation
    let attestation = create_attestation(
        cfg,
        challenge.as_str(),
        Some(&[(
            NODE_IDENTITY_KEY.to_string(),
            client_state.get_current_identity_public_key().to_vec(),
        )]),
    )
    .await
    .map_err(|e| {
        #[cfg(not(feature = "testing"))]
        debug!("Error creating attestation: {:?}", e);
        unexpected_err(e, Some("error producing attestation".into()))
    })
    .ok();
    let attestation = match serde_json::to_value(&attestation) {
        Ok(attestation) => Some(attestation),
        Err(e) => {
            error!("unable to convert the attestation to a json object");
            return status::Custom(
                Status::BadRequest,
                json!(GenericResponse::err_and_data_json(
                    "".to_string(),
                    JsonSDKHandshakeResponse {
                        server_public_key: "ERR".to_string(),
                        subnet_public_key: "ERR".to_string(),
                        network_public_key: "ERR".to_string(),
                        network_public_key_set: "ERR".to_string(),
                        client_sdk_version: version.to_string(),
                        hd_root_pubkeys: vec![],
                        attestation: None,
                        latest_blockhash: "".to_string(),
                        node_version: crate::version::get_version().to_string(),
                        node_identity_key: "".to_string(),
                        epoch: 0,
                        git_commit_hash: crate::git_info::GIT_COMMIT_HASH.to_string(),
                    }
                )),
            );
        }
    };

    timing.insert("create attestation".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    let latest_blockhash = eth_blockhash_cache.blockhash.read().await.clone();
    timing.insert("get latest blockhash".to_string(), before.elapsed());

    timing.insert("total".to_string(), request_start.elapsed());

    trace!("POST /web/handshake timing: {:?}", timing);

    // the public key set is currently the bls root key... of which there is only one.
    if !bls_root_keys.is_empty() {
        let network_public_key = &bls_root_keys[0];

        let realm_id = session.peer_state.realm_id();
        let epoch = session.peer_state.epoch();

        return status::Custom(
            Status::Ok,
            json!(GenericResponse::ok(JsonSDKHandshakeResponse {
                server_public_key: "".to_string(),
                subnet_public_key: network_public_key.clone(),
                network_public_key: network_public_key.clone(),
                network_public_key_set: network_public_key.clone(),
                client_sdk_version: version.to_string(),
                hd_root_pubkeys: ecdsa_root_keys,
                attestation,
                latest_blockhash,
                node_version: crate::version::get_version().to_string(),
                node_identity_key: client_state.get_current_identity_public_key_hex(),
                epoch,
                git_commit_hash: crate::git_info::GIT_COMMIT_HASH.to_string(),
            })),
        );
    }

    status::Custom(
        Status::Ok,
        json!(GenericResponse::err_and_data_json(
            "".to_string(),
            JsonSDKHandshakeResponse {
                server_public_key: "ERR".to_string(),
                subnet_public_key: "ERR".to_string(),
                network_public_key: "ERR".to_string(),
                network_public_key_set: "ERR".to_string(),
                client_sdk_version: version.to_string(),
                hd_root_pubkeys: ecdsa_root_keys,
                attestation,
                latest_blockhash,
                node_version: crate::version::get_version().to_string(),
                node_identity_key: client_state.get_current_identity_public_key_hex(),
                epoch: 0,
                git_commit_hash: crate::git_info::GIT_COMMIT_HASH.to_string(),
            }
        )),
    )
}

#[cfg(feature = "lit-actions")]
#[instrument(level = "debug", skip_all, ret)]
pub(crate) async fn get_job_status(
    job_status_request: models::JsonJobStatusRequest,
    client_session: Arc<ClientSession>,
    action_store: &State<ActionStore>,
    tss_state: &State<Arc<TssState>>,
    cfg: &State<ReloadableLitConfig>,
    client_state: &Arc<ClientState>,
) -> status::Custom<Value> {
    let bls_root_pubkey = match get_bls_root_pubkey(tss_state).await {
        Ok(key) => key,
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("no bls root key exists", e.handle());
        }
    };
    let cfg = cfg.load_full();
    let lit_action_resource = LitActionResource::new("".to_string());

    let validated_address = match AuthSigItemExtendedRef(&job_status_request.auth_sig)
        .validate_and_get_user_address(
            &lit_action_resource.execution_ability(),
            &Some(CHAIN_ETHEREUM.to_string()),
            &cfg,
            &bls_root_pubkey,
            &EndpointVersion::V2,
        )
        .await
    {
        Ok(resp) => resp.address_str().to_lowercase(),
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("couldn't validate user address", e.handle());
        }
    };

    let mut store = action_store.inner().clone();
    let job_id = &job_status_request.job_id;
    match store.get_job(job_id).await {
        Ok(Some(job)) => {
            // Got the job, now check if the user is authorized to access it
            if let Some(authorized_address) = job.authorized_address() {
                if authorized_address != validated_address {
                    return client_session.json_encrypt_err_custom_response("unauthorized access to job id",
                        validation_err_code(
                            format!("The job is not authorized for this user. The address you presented is {validated_address} and the one used to enqueue the job was {authorized_address}"),
                            EC::NodeInvalidPKPAddress,
                            None,
                        ).handle());
                }
            } else {
                return client_session.json_encrypt_err_custom_response(
                    "unauthorized access to job id",
                    unexpected_err(
                        format!("Job with ID {job_id} does not have a valid authorized address"),
                        None,
                    )
                    .handle(),
                );
            }

            let response = models::JsonJobStatusResponse {
                id: job.id,
                status: job.status,
                result: job.result.map(|r| match r {
                    Ok(state) => models::JsonJobResult::Success { state },
                    Err(error) => models::JsonJobResult::Error { error },
                }),
                completed_at: job.completed_at,
            };

            client_session.json_encrypt_response_status(response)
        }
        Ok(None) => client_session.json_encrypt_err_custom_response(
            "job not found",
            validation_err_code(
                format!("Job with ID {job_id} not found"),
                EC::NodeJobNotFound,
                None,
            )
            .handle(),
        ),
        Err(e) => client_session.json_encrypt_err_custom_response(
            "error getting job status",
            unexpected_err(e, None).handle(),
        ),
    }
}

#[cfg(feature = "lit-actions")]
#[instrument(level = "debug", name = "POST /web/execute", skip_all, ret)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn execute_function(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    delegation_usage_db: Option<&State<Arc<DelegatedUsageDB>>>,
    cfg: &State<ReloadableLitConfig>,
    grpc_client_pool: &State<GrpcClientPool<tonic::transport::Channel>>,
    allowlist_cache: &State<Arc<models::AllowlistCache>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    client_state: &Arc<ClientState>,
    json_execution_request: JsonExecutionRequest,
    client_session: Arc<ClientSession>,
    request_headers: RequestHeaders<'_>,
    payment_tracker: &State<Arc<PaymentTracker>>,
    endpoint_version: EndpointVersion,
    request_id: String,
    action_store: &State<ActionStore>,
    http_client: &State<reqwest::Client>,
) -> status::Custom<Value> {
    use crate::utils::web::check_allowlist;
    use ethers::utils::keccak256;
    use lit_node_common::config::LitNodeConfig;
    use lit_node_core::{LitActionResource, constants::CHAIN_ETHEREUM, response};

    let request_start = std::time::Instant::now();

    trace!(
        "execute, request: {:}",
        format!("{:?}", json_execution_request)
    );

    let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

    let cfg = cfg.load_full();

    // get the derived IPFS ID so that we can auth the user against it
    let before = std::time::Instant::now();
    // determine if the user passed code or an ipfs hash
    let derived_ipfs_id;
    let code_to_run: Arc<String>;
    if let Some(code) = &json_execution_request.code {
        let decoded_bytes = match data_encoding::BASE64.decode(code.as_bytes()) {
            Ok(decoded_bytes) => decoded_bytes,
            Err(err) => {
                let handle = conversion_err(
                    err,
                    Some("Your Lit Action code could not be decoded from base64".into()),
                )
                .add_msg_to_details()
                .handle();
                return client_session
                    .json_encrypt_err_custom_response("invalid lit action code", handle);
            }
        };

        match String::from_utf8(decoded_bytes) {
            Ok(string_result) => code_to_run = Arc::new(string_result),
            Err(err) => {
                return client_session.json_encrypt_err_custom_response(
                    "can't decode lit action bytes to utf8",
                    conversion_err(err, Some("Error converting bytes to string".into()))
                        .add_msg_to_details()
                        .handle(),
                );
            }
        };

        // hash the code to get the ipfs id
        let ipfs_hasher = IpfsHasher::default();
        let cid = ipfs_hasher.compute(code_to_run.as_bytes());
        derived_ipfs_id = cid;
    } else if let Some(ipfs_id) = &json_execution_request.ipfs_id {
        // pull down the code from ipfs
        match get_ipfs_file(
            &ipfs_id.to_string(),
            &cfg,
            moka::future::Cache::clone(ipfs_cache),
            reqwest::Client::clone(http_client),
        )
        .await
        {
            Ok(ipfs_result) => code_to_run = ipfs_result,
            Err(err) => {
                return client_session.json_encrypt_err_custom_response(
                    "error retrieving ipfs code file",
                    err.handle(),
                );
            }
        };
        derived_ipfs_id = ipfs_id.clone();
    } else {
        return client_session.json_encrypt_err_custom_response(
            "no code or ipfs hash provided",
            validation_err_code("No code or ipfs hash provided", EC::NodeInvalidIPFSID, None)
                .add_source_to_details()
                .handle(),
        );
    }
    timing.insert("derived IPFS CID".to_string(), before.elapsed());

    trace!("derived_ipfs_id: {}", derived_ipfs_id);

    let capability_protocol_prefix = &"litAction".to_string();
    let lit_action_resource = LitActionResource::new(derived_ipfs_id.clone());

    let before = std::time::Instant::now();
    // Validate auth sig item
    let bls_root_pubkey = match get_bls_root_pubkey(tss_state).await {
        Ok(bls_root_pubkey) => bls_root_pubkey,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response("no bls root key", e.handle());
        }
    };

    let validated_address = {
        match AuthSigItemExtendedRef(&json_execution_request.auth_sig)
            .validate_and_get_user_address(
                &lit_action_resource.execution_ability(),
                &Some(CHAIN_ETHEREUM.to_string()),
                &cfg,
                &bls_root_pubkey,
                &endpoint_version,
            )
            .await
        {
            Err(e) => {
                return client_session
                    .json_encrypt_err_custom_response("can't validate user address", e.handle());
            }
            Ok(resp) => resp,
        }
    };
    timing.insert("auth sig validation".to_string(), before.elapsed());

    let epoch = match json_execution_request.epoch {
        0 => None,
        i => Some(i),
    };

    // Handle payment depending on the version
    let dynamic_payment = get_lit_action_dynamic_payment(
        &cfg,
        Some(json_execution_request.clone().auth_sig),
        delegation_usage_db,
        endpoint_version,
        &client_session,
        payment_tracker,
        &bls_root_pubkey,
        tss_state,
        &validated_address,
    )
    .await;

    let dynamic_payment = match dynamic_payment {
        Ok(dynamic_payment) => dynamic_payment,
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("error getting dynamic payment", e.handle());
        }
    };

    let before = std::time::Instant::now();
    // check if the IPFS id is in the allowlist
    if matches!(cfg.enable_actions_allowlist(), Ok(true)) {
        let allowlist_entry_id = keccak256(format!("LIT_ACTION_{}", derived_ipfs_id).as_bytes());
        let action_is_allowed =
            match check_allowlist(allowlist_cache, &allowlist_entry_id, &cfg).await {
                Ok(action_is_allowed) => action_is_allowed,
                Err(e) => {
                    return client_session
                        .json_encrypt_err_custom_response("lit action not allowed", e.handle());
                }
            };
        if !action_is_allowed {
            return client_session.json_encrypt_err_custom_response(
                "lit action not allowed",
                validation_err_code("Action not allowed", EC::NodeActionNotAllowed, None).handle(),
            );
        }
    }
    timing.insert("checked allowlist".to_string(), before.elapsed());

    // create the lit resource
    let lit_action_resource = LitActionResource::new(derived_ipfs_id.clone());

    // check for single or multiple auth sigs and do the session key
    // capability check.  set the wallet that provided the capabilities as the
    // main auth sig wallet.
    let auth_sig = match &json_execution_request.auth_sig {
        AuthSigItem::Single(single_auth_sig) => single_auth_sig,
        AuthSigItem::Multiple(_) => {
            return client_session.json_encrypt_err_and_code(
                "Multiple auth sigs not supported by Lit Actions",
                "unsupported_auth_sig",
                Status::BadRequest,
            );
        }
    };

    let before = std::time::Instant::now();
    trace!("getting auth context");

    let auth_context = match endpoint_version {
        EndpointVersion::Initial => {
            let auth_context = get_auth_context(
                Some(auth_sig.clone()),
                json_execution_request.auth_methods.clone(),
                json_execution_request.ipfs_id.clone(),
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
                    return client_session.json_encrypt_err_custom_response(
                        "error getting auth context",
                        e.handle(),
                    );
                }
            }
        }
        EndpointVersion::V1 | EndpointVersion::V2 => {
            let signed_message = match get_signed_message(&auth_sig.signed_message) {
                Ok(signed_message) => signed_message,
                Err(err_msg) => {
                    return client_session.json_encrypt_err_and_code(
                        &err_msg,
                        "unsupported_auth_sig",
                        Status::BadRequest,
                    );
                }
            };

            timing.insert("parsed session sig".to_string(), before.elapsed());

            let resolved_auth_context =
                match get_auth_context_from_session_sigs(signed_message).await {
                    Ok(resolved_auth_context) => resolved_auth_context,
                    Err(e) => {
                        error!("Error parsing AuthContext from sessionSig");
                        return client_session.json_encrypt_err_custom_response(
                            "error getting auth context",
                            e.handle(),
                        );
                    }
                };

            match resolved_auth_context {
                Some(resolved_auth_context) => resolved_auth_context,
                None => {
                    // Also create new auth_context for EOA authSig/sessionSigs
                    let new_auth_context = get_auth_context(
                        Some(auth_sig.clone()),
                        None,
                        Some(derived_ipfs_id.clone()),
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
                                "error getting auth context",
                                e.handle(),
                            );
                        }
                    }
                }
            }
        }
    };

    timing.insert("auth context".to_string(), before.elapsed());
    trace!("Got auth context");

    // TODO compare max_price to current cost of the operation.

    let deno_execution_env = models::DenoExecutionEnv {
        tss_state: Some(tss_state.as_ref().clone()),
        cfg,
        ipfs_cache: Some(moka::future::Cache::clone(ipfs_cache)),
        http_client: Some(reqwest::Client::clone(http_client)),
    };

    let http_headers = {
        let mut res: BTreeMap<String, String> = BTreeMap::new();
        for h in request_headers.headers.iter() {
            let (name, value) = (h.name.to_string(), h.value.to_string());
            // If necessary, combine multiple headers with the same name into a single header
            res.entry(name)
                .and_modify(|e| e.push_str(&format!(", {value}")))
                .or_insert(value);
        }
        res
    };

    trace!("spawning js execution task");

    let actions_config = tss_state.chain_data_config_manager.get_actions_config();

    let before = std::time::Instant::now();
    let mut client = match action_client::ClientBuilder::default()
        .js_env(deno_execution_env)
        .auth_context(auth_context)
        .auth_sig(Some(auth_sig.clone()))
        .request_id(request_id.clone())
        .http_headers(http_headers)
        .timeout_ms(actions_config.timeout_ms)
        .memory_limit_mb(actions_config.memory_limit_mb as u32)
        .max_code_length(actions_config.max_code_length as usize)
        .max_fetch_count(actions_config.max_fetch_count as u32)
        .max_sign_count(actions_config.max_sign_count as u32)
        .max_contract_call_count(actions_config.max_contract_call_count as u32)
        .max_broadcast_and_collect_count(actions_config.max_broadcast_and_collect_count as u32)
        .max_call_depth(actions_config.max_call_depth as u32)
        .max_retries(actions_config.max_retries as u32)
        .epoch(epoch)
        .endpoint_version(endpoint_version)
        .node_set(json_execution_request.node_set.clone())
        .dynamic_payment(dynamic_payment)
        .client_grpc_channels((*grpc_client_pool).clone())
        .build()
        .map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeJsExecutionError,
                Some("Error building action client".into()),
            )
        }) {
        Ok(client) => client,
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("error building action client", e.handle());
        }
    };

    let execution_options = action_client::ExecutionOptions {
        code: code_to_run,
        globals: json_execution_request.js_params.clone(),
        action_ipfs_id: Some(derived_ipfs_id),
    };

    if json_execution_request.is_async() && actions_config.async_actions_enabled {
        return match client
            .execute_js_async(execution_options, action_store)
            .await
        {
            Ok(job_id) => {
                info!("Submitted async action job with ID {job_id}");
                client_session.json_encrypt_response_status::<JobId>(job_id)
            }
            Err(err) => {
                error!("Error processing async action job: {err:?}");
                return client_session.json_encrypt_err_custom_response(
                    "error processing async action job",
                    unexpected_err_code(
                        err,
                        EC::NodeJsExecutionError,
                        Some("Error processing action job".into()),
                    )
                    .handle(),
                );
            }
        };
    }

    let execution_result = client.execute_js(execution_options).await;
    timing.insert("js execution".to_string(), before.elapsed());

    // apply to the pending payment
    if client.dynamic_payment.payment_enabled {
        timing.insert("verify the payment".to_string(), before.elapsed());
        let before = std::time::Instant::now();
        let pending_payment = client.dynamic_payment.to_pending_payment();
        payment_tracker.batches().add(pending_payment).await;
        timing.insert("register pending payment".to_string(), before.elapsed());
    }

    let execution_state = match execution_result {
        Ok(state) => state,
        Err(err) => {
            error!("error in Js comms result: {err:?}");
            let logs = client.logs();
            match err.kind() {
                lit_api_core::error::Kind::Timeout => {
                    let handle =
                        timeout_err_code(err, EC::NodeJsTimeoutError, None).handle_with_logs(logs);
                    return client_session.json_encrypt_err_custom_response("timeout", handle);
                }
                lit_api_core::error::Kind::MemoryLimit => {
                    let handle = memory_limit_err_code(err, EC::NodeJsMemoryLimitError, None)
                        .handle_with_logs(logs);
                    return client_session.json_encrypt_err_custom_response("memory limit", handle);
                }
                lit_api_core::error::Kind::Connect => {
                    let handle = connect_err_code(err, EC::NodeJsConnectionError, None)
                        .handle_with_logs(logs);
                    return client_session.json_encrypt_err_custom_response("connect", handle);
                }
                _ => {}
            }
            if let Some(source_err) = err.source() {
                return client_session.json_encrypt_err_and_code(
                    &source_err.to_string(),
                    logs,
                    Status::BadRequest,
                );
            }
            let handle = unexpected_err_code(
                err,
                EC::NodeJsExecutionError,
                Some("Error executing JS".into()),
            )
            .handle_with_logs(logs);
            return client_session.json_encrypt_err_custom_response("error executing js", handle);
        }
    };

    trace!("js execution task completed");

    timing.insert("total".to_string(), request_start.elapsed());
    debug!("POST /web/execute timing: {:?}", timing);

    client_session.json_encrypt_response_status(response::JsonExecutionResponse {
        success: true,
        signed_data: execution_state.signed_data,
        decrypted_data: json!({}),
        claim_data: execution_state.claim_data,
        response: execution_state.response,
        logs: execution_state.logs,
        payment_detail: Some(client.dynamic_payment.items),
    })
}

#[allow(clippy::too_many_arguments)]
#[doc = "Get the dynamic payment for a lit action.    This is used to determine if the lit action should be paid for.    If the lit action is not paid for, the lit action will fail.    If the lit action is paid for, the lit action will be executed. "]
pub async fn get_lit_action_dynamic_payment(
    cfg: &LitConfig,
    auth_sig: Option<AuthSigItem>,
    delegation_usage_db: Option<&State<Arc<DelegatedUsageDB>>>,
    endpoint_version: EndpointVersion,
    client_session: &ClientSession,
    payment_tracker: &Arc<PaymentTracker>,
    bls_root_pubkey: &str,
    tss_state: &Arc<TssState>,
    validated_address: &ValidatedAddress,
) -> error::Result<DynamicPayment> {
    match cfg.enable_payment().unwrap_or(true) {
        true => {
            let delegation_usage_db = match delegation_usage_db {
                Some(db) => db,
                None => {
                    let err_msg = format!(
                        "Delegation db is not provided to {}, version {}",
                        "execute_function", endpoint_version as u8
                    );
                    return Err(unexpected_err_code(
                        "Delegation db is not provided.",
                        EC::NodeJsExecutionError,
                        Some(err_msg),
                    ));
                }
            };

            let before = std::time::Instant::now();

            let signed_message = match auth_sig {
                Some(auth_sig) => {
                    let single_auth_sig = match auth_sig {
                        AuthSigItem::Single(single_auth_sig) => single_auth_sig,
                        AuthSigItem::Multiple(_) => {
                            return Err(unexpected_err_code(
                                "MultiAuthSig not supported for payment",
                                EC::NodeJsExecutionError,
                                None,
                            ));
                        }
                    };

                    let signed_message: SessionKeySignedMessageV2 =
                        match serde_json::from_str(&single_auth_sig.signed_message) {
                            Ok(signed_message) => signed_message,
                            Err(e) => {
                                let err_msg = "Parsing SessionKeySignedMessageV2 failed. \
                            The sessionSig is incorrectly formatted";
                                error!("{}", err_msg);
                                return Err(unexpected_err_code(
                                    e,
                                    EC::NodeJsExecutionError,
                                    Some(err_msg.into()),
                                ));
                            }
                        };

                    Some(signed_message)
                }
                None => None,
            };

            let user_address = match validated_address.evm_address() {
                Ok(address) => address,
                Err(e) => {
                    return Err(unexpected_err_code(
                        e,
                        EC::NodeJsExecutionError,
                        Some("no evm address".into()),
                    ));
                }
            };

            let threshold = tss_state.get_threshold().await;

            let max_price = match signed_message.clone() {
                Some(signed_message) => signed_message.max_price,
                None => ethers::types::U256::from(0),
            };

            let payment_method = get_payment_method(
                &user_address,
                PayedEndpoint::LitAction,
                threshold,
                max_price,
                signed_message,
                payment_tracker,
                delegation_usage_db,
                bls_root_pubkey,
                cfg,
            )
            .await;

            let pending_payment = match payment_method {
                Ok(payment) => payment,
                Err(e) => {
                    return Err(unexpected_err_code(
                        e,
                        EC::NodeJsExecutionError,
                        Some("error getting payment method".into()),
                    ));
                }
            };

            let price_multiplier = get_price_multiplier(tss_state, pending_payment.price).await?;

            DynamicPayment::load_from(
                pending_payment.payer,
                &tss_state.chain_data_config_manager,
                price_multiplier,
                pending_payment.spending_limit,
                true, // from the initial config eval above!
            )
        }
        false => Ok(DynamicPayment::default()),
    }
}

async fn get_price_multiplier(
    tss_state: &Arc<TssState>,
    pending_price: I256,
) -> error::Result<u64> {
    if pending_price.as_u64() == 0 {
        warn!("Division by zero: pending_price is zero.");
        return Ok(0);
    }

    let endpoint_type = PayedEndpoint::LitAction as usize;

    let (base_network_price, base_network_prices_len) = DataVersionReader::read_field_unchecked(
        &tss_state.chain_data_config_manager.base_network_prices,
        |base_network_prices| {
            let price = base_network_prices.get(endpoint_type).cloned();
            (price, base_network_prices.len())
        },
    );

    let base_network_price = match base_network_price {
        Some(base_network_price) => base_network_price,
        None => {
            return Err(unexpected_err_code(
                format!(
                    "Endpoint type {} not found in call to base_network_prices (len={})",
                    endpoint_type, base_network_prices_len
                ),
                EC::NodeJsExecutionError,
                Some("Invalid endpoint type when calculating price_multiplier".into()),
            ));
        }
    };

    if base_network_price.as_u64() == 0 {
        return Err(unexpected_err_code(
            "Division by zero: base_network_price is zero.",
            EC::NodeJsExecutionError,
            Some("Division by zero when calculating price_multiplier".into()),
        ));
    }

    let price_multiplier = pending_price.as_u64() / base_network_price.as_u64();

    Ok(price_multiplier)
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn sign_session_key(
    remote_addr: SocketAddr,
    tss_state: &State<Arc<TssState>>,
    auth_context_cache: &State<Arc<models::AuthContextCache>>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    cfg: &State<ReloadableLitConfig>,
    client_state: &Arc<ClientState>,
    json_sign_session_key_request: request::JsonSignSessionKeyRequestV2,
    client_session: Arc<ClientSession>,
    request_headers: RequestHeaders<'_>,
    endpoint_version: EndpointVersion,
    delegation_usage_db: Option<&State<Arc<DelegatedUsageDB>>>,
    payment_tracker: &State<Arc<PaymentTracker>>,
    request_id: String,
    http_client: &State<reqwest::Client>,
) -> status::Custom<Value> {
    use crate::{
        error::validation_err,
        services::contract::get_pkp_pubkey,
        utils::web::pubkey_bytes_to_eth_address_bytes,
        utils::{contract::get_pkp_permissions_contract, siwe::validate_siwe},
    };
    use lit_node_core::response;

    let request_start = std::time::Instant::now();
    trace!(
        "sign_session_key, request: {}",
        format!("{:?}", json_sign_session_key_request)
    );

    if json_sign_session_key_request.curve_type != CurveType::BLS {
        let handle = validation_err_code(
            format!(
                "Invalid curve_type: {} - only BLS curve type is allowed for this endpoint",
                &json_sign_session_key_request.curve_type
            ),
            EC::NodeInvalidCurveType,
            None,
        )
        .handle();
        return client_session.json_encrypt_err_custom_response("invalid curve type", handle);
    }

    if json_sign_session_key_request.auth_sig.is_some() {
        let handle = validation_err_code(
            "You can't provide an AuthSig on this endpoint.  Instead, send it as an AuthMethod in the authMethods array.",
            EC::NodeInvalidAuthSigForSessionKey,
            None,
        ).handle();
        return client_session.json_encrypt_err_custom_response("auth sig not supported", handle);
    }

    let cfg = cfg.load_full();

    let mut timing: BTreeMap<String, Duration> = BTreeMap::new();

    // Parse the SIWE message.
    let before = std::time::Instant::now();
    let parsed_siwe = match siwe::Message::from_str(&json_sign_session_key_request.siwe_message) {
        Ok(parsed_siwe) => parsed_siwe,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response(
                "malformed siwe",
                parser_err_code(
                    e,
                    EC::NodeSIWEMessageError,
                    Some("Error parsing SIWE message".into()),
                )
                .add_msg_to_details()
                .handle(),
            );
        }
    };
    timing.insert("parsed siwe message".to_string(), before.elapsed());

    if let Some(statement) = &parsed_siwe.statement {
        if statement.contains(LIT_RESOURCE_PREFIX_RAC) {
            return client_session.json_encrypt_err_custom_response(
                "missing resource prefix",
                validation_err_code(
                    "Can't define Auth Context resources in capability",
                    EC::NodeInvalidAuthContextResource,
                    None,
                )
                .add_msg_to_details()
                .handle(),
            );
        }
    }

    let origin_domain = match get_domain_from_request_origin(
        request_headers
            .headers
            .get_one("Origin")
            .unwrap_or("http://localhost"),
    ) {
        Ok(origin_domain) => origin_domain,
        Err(e) => {
            error!(
                "Error getting origin domain - swallowing and using default of localhost: {:?}",
                e
            );
            "http://localhost".into()
        }
    };
    trace!("Origin: {:?}", origin_domain);

    let before = std::time::Instant::now();
    // convert the auth methods into an auth context by resolving the oauth ids
    // from the oauth endpoints
    let bls_root_pubkey = match get_bls_root_pubkey(tss_state).await {
        Ok(bls_root_pubkey) => bls_root_pubkey,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response("no bls root key", e.handle());
        }
    };
    let mut auth_context = match get_auth_context(
        None,
        Some(json_sign_session_key_request.auth_methods.clone()),
        None,
        Some(auth_context_cache),
        true, // mark the auth method as used to prevent replay attacks
        cfg.clone(),
        Some(origin_domain),
        &bls_root_pubkey,
        &endpoint_version,
        reqwest::Client::clone(http_client),
    )
    .await
    {
        Ok(auth_context) => auth_context,
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("can't get auth context", e.handle());
        }
    };
    timing.insert("auth context".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    // Retrieve the PKP pubkey to sign. If the PKP pubkey is provided in the request, use that.
    // Otherwise, retrieve it from the smart contract using the auth method ID.
    let pkp_public_key = {
        if let Some(pkp_public_key) = &json_sign_session_key_request.pkp_public_key {
            let pubkey = encoding::hex_to_bytes(pkp_public_key).map_err(|e| {
                conversion_err(
                    e,
                    Some("Unable to convert PKP public key from hex to bytes".into()),
                )
            });
            let pubkey = match pubkey {
                Ok(pubkey) => pubkey,
                Err(e) => {
                    return client_session
                        .json_encrypt_err_custom_response("invalid pkp public key", e.handle());
                }
            };

            // Convert the PKP public key to an ETH address.
            let pkp_eth_address = match pubkey_bytes_to_eth_address_bytes(pubkey.to_vec()) {
                Ok(pkp_eth_address) => pkp_eth_address,
                Err(e) => {
                    return client_session.json_encrypt_err_custom_response(
                        "unable to convert pkp public key to an ETH address",
                        parser_err(e, Some("Error hex decoding pkpPublicKey".into()))
                            .add_msg_to_details()
                            .handle(),
                    );
                }
            };

            // Validate the SIWE message contains the correct PKP public key.
            if parsed_siwe.address != pkp_eth_address {
                return client_session.json_encrypt_err_custom_response(
                    "siwe message does not contain the correct pkp public key",
                    parser_err_code(
                        format!(
                            "Address in SIWE message {} does not match PKP ETH address {}",
                            encoding::bytes_to_hex(parsed_siwe.address),
                            encoding::bytes_to_hex(pkp_eth_address)
                        ),
                        EC::NodeSIWEMessageError,
                        None,
                    )
                    .add_source_to_details()
                    .handle(),
                );
            }

            Bytes::from(pubkey)
        } else {
            // Derive auth method ID. For now, just use the auth method from the auth context.
            if auth_context.auth_method_contexts.is_empty() {
                return client_session.json_encrypt_err_custom_response(
                    "no auth method provided",
                    validation_err("No auth methods provided", None)
                        .add_source_to_details()
                        .handle(),
                );
            }
            let auth_method = auth_context.auth_method_contexts[0].clone();
            let auth_method_id =
                match serialize_auth_context_for_checking_against_contract_data(&auth_method) {
                    Ok(auth_method_id) => auth_method_id,
                    Err(e) => {
                        return client_session.json_encrypt_err_custom_response(
                            "can't check auth context against contract data",
                            e.handle(),
                        );
                    }
                };

            // Get PKP permissions contract.
            let pkp_permissions_contract = match get_pkp_permissions_contract(cfg.clone()).await {
                Ok(pkp_permissions_contract) => pkp_permissions_contract,
                Err(e) => {
                    error!("Failed to get PKP permissions contract");
                    return client_session.json_encrypt_err_custom_response(
                        "can't get pkp permissions information from contract",
                        e.handle(),
                    );
                }
            };

            // Get PKP public key.
            let pubkey = get_pkp_pubkey(
                &pkp_permissions_contract,
                auth_method.auth_method_type,
                Bytes::from(auth_method_id),
            )
            .await;
            match pubkey {
                Ok(pubkey) => pubkey,
                Err(e) => {
                    error!("Failed to get PKP pubkey");
                    return client_session.json_encrypt_err_custom_response(
                        "can't get the pkp public key",
                        e.handle(),
                    );
                }
            }
        }
    };
    timing.insert("retrieved pkp pubkey".to_string(), before.elapsed());

    // Convert the PKP public key to an ETH address.
    let pkp_eth_address = match pubkey_bytes_to_eth_address_bytes(pkp_public_key.to_vec()) {
        Ok(pkp_eth_address) => pkp_eth_address,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response(
                "can't convert pkp public key to an ETH address",
                parser_err(e, Some("Error hex decoding pkpPublicKey".into()))
                    .add_msg_to_details()
                    .handle(),
            );
        }
    };

    // Handle payment
    if cfg.enable_payment().unwrap_or(true) {
        let before = std::time::Instant::now();
        let delegation_usage_db = match delegation_usage_db {
            Some(db) => db,
            None => {
                let msg = format!(
                    "Delegation db is not provided to {}, version {}",
                    "encryption_sign", endpoint_version as u8
                );
                return client_session.json_encrypt_err_and_code(
                    &msg,
                    "delegation_usage_db_not_provided",
                    Status::PaymentRequired,
                );
            }
        };

        let threshold = tss_state.get_threshold().await;

        let payment_method = get_payment_method(
            &Address::from(pkp_eth_address),
            PayedEndpoint::SignSessionKey,
            threshold,
            json_sign_session_key_request.max_price,
            None,
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
                    .json_encrypt_err_custom_response("unable to get payment method", e.handle());
            }
        };
        timing.insert("verify the payment".to_string(), before.elapsed());

        let before = std::time::Instant::now();
        payment_tracker.batches().add(pending_payment).await;
        timing.insert("register pending payment".to_string(), before.elapsed());
    };

    timing.insert("computed payment".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    // Validate the SIWE message.
    if let Ok(true) = cfg.enable_siwe_validation() {
        // check the uri field
        if parsed_siwe.uri != json_sign_session_key_request.session_key {
            return client_session.json_encrypt_err_custom_response(
                "invalid siwe uri message",
                parser_err_code(
                    format!(
                        "URI in SIWE message {} does not match URI in request {}",
                        parsed_siwe.uri, json_sign_session_key_request.session_key
                    ),
                    EC::NodeSIWEMessageError,
                    None,
                )
                .add_source_to_details()
                .handle(),
            );
        }

        // Validate SIWE.
        if let Err(e) = validate_siwe(&parsed_siwe) {
            return client_session
                .json_encrypt_err_custom_response("invalid siwe message", e.handle());
        }
    }
    timing.insert("validated siwe".to_string(), before.elapsed());

    let epoch = match json_sign_session_key_request.epoch {
        0 => None,
        i => Some(i),
    };

    let mut derived_ipfs_id = None;

    // Validate the Lit Action result if present
    match (
        &json_sign_session_key_request.code,
        &json_sign_session_key_request.lit_action_ipfs_id,
    ) {
        (Some(_), Some(_)) => {
            return client_session.json_encrypt_err_custom_response(
                "lit action code and ipfs id both present",
                unexpected_err("Can't provide both code & lit_action_ipfs_id", None)
                    .add_source_to_details()
                    .handle(),
            );
        }
        (None, None) => trace!("Not running Lit Action for signing session sigs"),
        (Some(_), None) | (None, Some(_)) => {
            trace!("Running Lit Action for signing session sigs");

            let code_to_run;

            if let Some(code) = &json_sign_session_key_request.code {
                let decoded_bytes = match data_encoding::BASE64.decode(code.as_bytes()) {
                    Ok(decoded_bytes) => decoded_bytes,
                    Err(err) => {
                        return client_session.json_encrypt_err_custom_response(
                            "can't decode lit action code from base64",
                            conversion_err(
                                err,
                                Some(
                                    "Your Lit Action code could not be decoded from base64".into(),
                                ),
                            )
                            .add_msg_to_details()
                            .handle(),
                        );
                    }
                };

                match String::from_utf8(decoded_bytes) {
                    Ok(string_result) => code_to_run = Arc::new(string_result),
                    Err(err) => {
                        return client_session.json_encrypt_err_custom_response("can't convert decoded lit action code into utf8", conversion_err(
                            err,
                            Some("Your Lit Action code could not be converted from base64 decoded bytes into a string.  Please check your base64 encoding code.".into()),
                        )
                        .add_msg_to_details()
                        .handle());
                    }
                };

                // hash the code to get the ipfs id
                let ipfs_hasher = IpfsHasher::default();
                let cid = ipfs_hasher.compute(code_to_run.as_bytes());
                derived_ipfs_id = Some(cid);
            } else {
                #[allow(clippy::unwrap_used)]
                let ipfs_id = json_sign_session_key_request
                    .lit_action_ipfs_id
                    .as_ref()
                    .unwrap(); // We check that either the code is provided or the ipfs_cid
                // pull down the code from ipfs
                match get_ipfs_file(
                    &ipfs_id.to_string(),
                    &cfg,
                    moka::future::Cache::clone(ipfs_cache),
                    reqwest::Client::clone(http_client),
                )
                .await
                {
                    Ok(ipfs_result) => code_to_run = ipfs_result,
                    Err(err) => {
                        return client_session.json_encrypt_err_custom_response(
                            "unable to pull down ipfs file",
                            err.handle(),
                        );
                    }
                };
                derived_ipfs_id = Some(ipfs_id.clone());
            }

            let deno_execution_env = models::DenoExecutionEnv {
                tss_state: Some(tss_state.as_ref().clone()),
                cfg: cfg.clone(),
                ipfs_cache: Some(moka::future::Cache::clone(ipfs_cache)),
                http_client: Some(reqwest::Client::clone(http_client)),
            };

            trace!("spawning js execution task");

            let http_headers = {
                let mut res: BTreeMap<String, String> = BTreeMap::new();
                for h in request_headers.headers.iter() {
                    let (name, value) = (h.name.to_string(), h.value.to_string());
                    // If necessary, combine multiple headers with the same name into a single header
                    res.entry(name)
                        .and_modify(|e| e.push_str(&format!(", {value}")))
                        .or_insert(value);
                }
                res
            };

            let validated_address: ValidatedAddress =
                match &json_sign_session_key_request.pkp_public_key {
                    Some(pkp_public_key) => ValidatedAddress::from_signed_session_request(
                        json_sign_session_key_request.clone(),
                    ),
                    None => {
                        return client_session.json_encrypt_err_custom_response(
                            "pkp public key is not provided",
                            unexpected_err("pkp public key is not provided", None).handle(),
                        );
                    }
                };

            let auth_sig = json_sign_session_key_request.auth_sig;

            // Handle payment depending on the version
            let dynamic_payment = get_lit_action_dynamic_payment(
                &cfg,
                auth_sig,
                delegation_usage_db,
                endpoint_version,
                &client_session,
                payment_tracker,
                &bls_root_pubkey,
                tss_state,
                &validated_address,
            )
            .await;

            let mut client = match action_client::ClientBuilder::default()
                .js_env(deno_execution_env)
                .auth_context(auth_context.clone())
                .request_id(request_id)
                .http_headers(http_headers)
                .epoch(epoch)
                .endpoint_version(endpoint_version)
                .build()
                .map_err(|e| {
                    unexpected_err_code(
                        e,
                        EC::NodeJsExecutionError,
                        Some("Error building action client".into()),
                    )
                }) {
                Ok(client) => client,
                Err(e) => {
                    return client_session.json_encrypt_err_custom_response(
                        "error building action client",
                        e.handle(),
                    );
                }
            };
            let execution_result = client
                .execute_js(action_client::ExecutionOptions {
                    code: code_to_run,
                    globals: json_sign_session_key_request.js_params.clone(),
                    action_ipfs_id: derived_ipfs_id.clone(),
                })
                .await;
            let execution_state = match execution_result {
                Ok(state) => state,
                Err(err) => {
                    let logs = client.logs();
                    match err.kind() {
                        lit_api_core::error::Kind::Timeout => {
                            return client_session.json_encrypt_err_custom_response(
                                "timeout",
                                timeout_err_code(err, EC::NodeJsTimeoutError, None)
                                    .handle_with_logs(logs),
                            );
                        }
                        lit_api_core::error::Kind::MemoryLimit => {
                            return client_session.json_encrypt_err_custom_response(
                                "memory limit",
                                memory_limit_err_code(err, EC::NodeJsMemoryLimitError, None)
                                    .handle_with_logs(logs),
                            );
                        }
                        _ => {}
                    }

                    if let Some(source_err) = err.source() {
                        return client_session.json_encrypt_err_and_code(
                            &source_err.to_string(),
                            logs,
                            Status::BadRequest,
                        );
                    }

                    return client_session.json_encrypt_err_custom_response(
                        "error executing lit action javascript",
                        unexpected_err_code(
                            err,
                            EC::NodeJsExecutionError,
                            Some("Error executing JS".into()),
                        )
                        .handle_with_logs(logs),
                    );
                }
            };

            trace!("js execution task completed");

            if execution_state.response.contains("true") {
                info!("Successful Lit Actions validation");

                match &derived_ipfs_id {
                    Some(derived_ipfs_id) => {
                        // Add to auth_context as we ran & verfied Lit Action auth
                        auth_context
                            .action_ipfs_id_stack
                            .push(derived_ipfs_id.clone());

                        auth_context.custom_auth_resource = execution_state.response;
                    }
                    None => {
                        warn!(
                            "Undefined derived_ipfs_id despite the user providing code/lit_actions_ipfs_id"
                        );
                    }
                };
            } else {
                return client_session.json_encrypt_err_custom_response(
                    "authentication failed for session_sig signing via lit action",
                    validation_err_code(
                        "Authentication failed for session_sig signing via Lit Action",
                        EC::NodeLitActionsSessionSigAuthenticationFailed,
                        None,
                    )
                    .add_source_to_details()
                    .handle(),
                );
            }
        }
    }

    let port = match cfg.as_ref().external_port() {
        Ok(port) => port,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response(
                "can't read port from config",
                unexpected_err(e, Some("Error getting port from config".into()))
                    .add_msg_to_details()
                    .handle(),
            );
        }
    };

    let rpc_provider = match ENDPOINT_MANAGER.get_provider(CHAIN_ETHEREUM) {
        Ok(rpc_provider) => rpc_provider,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response(
                "can't read rpc provider from DB",
                unexpected_err(e, Some("Error getting RPC Provider for DB".into()))
                    .add_msg_to_details()
                    .handle(),
            );
        }
    };

    let before = std::time::Instant::now();
    let block =
        match db::retrieve_and_store_blockhash(parsed_siwe.nonce.clone(), port, rpc_provider).await
        {
            Ok(retrieve_blockhash_res) => retrieve_blockhash_res,
            Err(e) => {
                return client_session.json_encrypt_err_custom_response(
                    "can't fetch blockhash",
                    validation_err(e, Some("Error fetching block from hash".into()))
                        .add_msg_to_details()
                        .handle(),
                );
            }
        };
    let siwe_timestamp = match make_timestamp_siwe_compatible(&block.timestamp) {
        Ok(res) => res,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response(
                "can't make siwe compatible timestamp",
                e.handle(),
            );
        }
    };
    timing.insert("retrieved blockhash".to_string(), before.elapsed());

    let hex_pubkey = encoding::bytes_to_hex(&pkp_public_key);

    if json_sign_session_key_request.curve_type != CurveType::BLS {
        return client_session.json_encrypt_err_custom_response(
            &format!(
                "invalid curve type: expected {}, received {}",
                CurveType::BLS,
                json_sign_session_key_request.curve_type
            ),
            validation_err_code(
                format!(
                    "Invalid curve_type: {}",
                    json_sign_session_key_request.curve_type
                ),
                EC::NodeInvalidCurveType,
                None,
            )
            .handle(),
        );
    }

    let before = std::time::Instant::now();
    // sign the session key via BLS using the lit network key
    // Add auth_context to resources
    let mut notabene = BTreeMap::new();
    notabene.insert(
        LIT_RESOURCE_KEY_RAC.to_string(),
        match serde_json::to_value(&auth_context).map_err(|e| {
            unexpected_err(
                e,
                Some("Error while inserting auth_context into siwe resource".into()),
            )
        }) {
            Ok(res) => res,
            Err(e) => {
                return client_session.json_encrypt_err_custom_response(
                    "can't add auth context into siwe resource",
                    e.handle(),
                );
            }
        },
    );
    let mut capabilities = Capability::<Value>::default();
    let resource = "Auth/Auth".to_string();
    let resource_prefix = format!("{}://*", LIT_RESOURCE_PREFIX_RAC); // TODO: Scope with uri
    let capabilities = match capabilities
        .with_actions_convert(resource_prefix, [(resource, [notabene])])
        .map_err(|e| {
            unexpected_err(
                e,
                Some("Error while converting capabilities resource into actions".into()),
            )
        }) {
        Ok(res) => res,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response(
                "can't convert capabilities into actions",
                e.handle(),
            );
        }
    };

    let user_capabilities =
        match Capability::<Value>::extract_and_verify(&parsed_siwe).map_err(|err| {
            parser_err_code(
                err,
                EC::NodeSIWECapabilityInvalid,
                Some("Unable to extract and verify user's capability object".into()),
            )
        }) {
            Ok(res) => match res {
                Some(res) => res,
                None => {
                    return client_session.json_encrypt_err_custom_response(
                        "can't convert SIWE into capabilities",
                        conversion_err("Unable to convert SIWE into Capability object", None)
                            .add_msg_to_details()
                            .handle(),
                    );
                }
            },
            Err(e) => {
                return client_session
                    .json_encrypt_err_custom_response("can't extract capabilities", e.handle());
            }
        };

    for ability in user_capabilities.abilities().iter() {
        if ability.0.scheme_str() == LIT_RESOURCE_PREFIX_RAC {
            return client_session.json_encrypt_err_custom_response(
                "invalid auth context in resources",
                validation_err_code(
                    "Can't define Auth Context resources in capability",
                    EC::NodeInvalidAuthContextResource,
                    None,
                )
                .add_msg_to_details()
                .handle(),
            );
        }
    }

    let merged_capabilities: Capability<Value> = capabilities.clone().merge(user_capabilities);

    // Construct a new SIWE message with the PKP address populated.
    let siwe_to_sign = match merged_capabilities
        .build_message(siwe::Message {
            domain: parsed_siwe.domain,
            address: pkp_eth_address,
            statement: parsed_siwe.statement,
            uri: parsed_siwe.uri,
            version: parsed_siwe.version,
            chain_id: parsed_siwe.chain_id,
            nonce: block.blockhash,
            issued_at: siwe_timestamp,
            expiration_time: parsed_siwe.expiration_time,
            not_before: parsed_siwe.not_before,
            request_id: parsed_siwe.request_id,
            resources: vec![],
        })
        .map_err(|e| {
            unexpected_err(
                e,
                Some("Error while building Siwe message from capabilities".into()),
            )
        }) {
        Ok(res) => res,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response(
                "can't build a siwe message from capabilities",
                e.handle(),
            );
        }
    };

    // Construct the payload to sign.
    let to_sign = match siwe_to_sign.eip191_hash() {
        Ok(to_sign) => to_sign,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response(
                "siwe can't be eip191 hashed",
                parser_err_code(
                    e,
                    EC::NodeSIWEMessageError,
                    Some("Error hashing SIWE message".into()),
                )
                .add_msg_to_details()
                .handle(),
            );
        }
    };
    timing.insert("computed SIWE message hash".to_string(), before.elapsed());

    debug!(
        "Signing session key using BLS {} with PKP {}",
        json_sign_session_key_request.session_key, pkp_public_key
    );

    let before = std::time::Instant::now();
    let bls_root_pubkey = match get_bls_root_pubkey(tss_state).await {
        Ok(bls_root_pubkey) => bls_root_pubkey,
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("No bls root key exists", e.handle());
        }
    };
    timing.insert("get bls root pubkey".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    // check permissions
    let is_authed = match pkp::auth::check_pkp_auth(
        derived_ipfs_id,
        None,
        hex_pubkey.clone(),
        auth_context.clone(),
        &cfg,
        &[2],
        &bls_root_pubkey,
    )
    .await
    {
        Ok(is_authed) => is_authed,
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("pkp is not authorized", e.handle());
        }
    };

    if !is_authed {
        return client_session.json_encrypt_err_custom_response(
            "pkp is not authorized to sign",
            validation_err_code(
                format!(
                    "You are not authorized to sign using this PKP: {}",
                    hex_pubkey
                ),
                EC::NodePKPNotAuthorized,
                None,
            )
            .handle(),
        );
    }
    timing.insert("check pkp auth".to_string(), before.elapsed());

    let before = std::time::Instant::now();
    let cipher_state = match tss_state.get_cipher_state(SigningScheme::Bls12381) {
        Ok(cipher_state) => cipher_state,
        Err(e) => {
            return client_session
                .json_encrypt_err_custom_response("unable to get BLS cipher state", e.handle());
        }
    };
    timing.insert("get cipher state".to_string(), before.elapsed());

    let to_sign = siwe_hash_to_bls_session_hash(to_sign.into());

    debug!(
        "session sign, bls_root_pubkey: {} to_sign: {:?}",
        bls_root_pubkey, to_sign
    );
    let before = std::time::Instant::now();
    let (signature_share, share_peer_id) = match cipher_state.sign(&to_sign, epoch).await {
        Ok(signature_share) => signature_share,
        Err(e) => {
            return client_session.json_encrypt_err_custom_response(
                "unable to create signature share",
                e.add_detail("Error signing with BLS key").handle(),
            );
        }
    };
    timing.insert("signing".to_string(), before.elapsed());
    timing.insert("total".to_string(), request_start.elapsed());
    debug!("POST /web/sign_session_key timing: {:?}", timing);

    client_session.json_encrypt_response_status(response::JsonSignSessionKeyResponseV2 {
        result: "success".to_string(),
        signature_share,
        share_id: share_peer_id.to_string(),
        curve_type: json_sign_session_key_request.curve_type.to_string(),
        siwe_message: siwe_to_sign.to_string(),
        data_signed: encoding::bytes_to_hex(to_sign),
        bls_root_pubkey,
    })
}

// Not dead code, rather a lint bug
// see https://github.com/rust-lang/rust/issues/92554
#[allow(dead_code)]
fn get_domain_from_request_origin(origin: &str) -> error::Result<String> {
    let origin = Url::parse(origin).map_err(|e| {
        conversion_err(e, Some(format!("Unable to parse origin URL of {}", origin)))
    })?;
    let domain = origin.domain().ok_or_else(|| {
        conversion_err(
            format!("Unable to parse domain from origin URL {}", origin),
            None,
        )
    })?;
    Ok(domain.to_string())
}

// Not dead code, rather a lint bug
// see https://github.com/rust-lang/rust/issues/92554
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
#[instrument(level = "debug", skip_all)]
pub async fn check_multiple_access_control_conditions(
    auth_sig_item: &AuthSigItem,
    access_control_conditions: &Option<Vec<AccessControlConditionItem>>,
    evm_contract_conditions: &Option<Vec<EVMContractConditionItem>>,
    sol_rpc_conditions: &Option<Vec<SolRpcConditionItem>>,
    unified_access_control_conditions: &Option<Vec<UnifiedAccessControlConditionItem>>,
    cfg: Arc<LitConfig>,
    requested_lit_resource_ability: &LitResourceAbility,
    chain: &Option<String>,
    request_id: String,
    bls_root_pubkey: &String,
    endpoint_version: &EndpointVersion,
    current_action_ipfs_id: Option<&String>,
    ipfs_cache: &State<Cache<String, Arc<String>>>,
    http_client: &State<reqwest::Client>,
) -> error::Result<models::UnifiedConditionCheckResult> {
    if let Some(access_control_conditions) = &access_control_conditions {
        let auth_sig = access_control::get_ethereum_auth_sig(auth_sig_item)?;

        let result = access_control::check_access_control_conditions(
            access_control_conditions,
            auth_sig,
            requested_lit_resource_ability,
            chain,
            cfg,
            &request_id,
            bls_root_pubkey,
            endpoint_version,
            current_action_ipfs_id,
            moka::future::Cache::clone(ipfs_cache),
            reqwest::Client::clone(http_client),
        )
        .await?;
        return Ok(models::UnifiedConditionCheckResult {
            result,
            successful_auth_sig: (*auth_sig).clone(),
        });
    } else if let Some(evm_contract_conditions) = &evm_contract_conditions {
        let auth_sig = access_control::get_ethereum_auth_sig(auth_sig_item)?;

        let result = access_control::evm_contract::check_access_control_conditions(
            evm_contract_conditions,
            auth_sig,
            requested_lit_resource_ability,
            chain,
            &cfg,
            bls_root_pubkey,
            endpoint_version,
            current_action_ipfs_id,
        )
        .await?;

        return Ok(models::UnifiedConditionCheckResult {
            result,
            successful_auth_sig: auth_sig.clone(),
        });
    } else if let Some(sol_rpc_conditions) = &sol_rpc_conditions {
        let auth_sig = access_control::get_solana_auth_sig(auth_sig_item)?;

        let result = access_control::sol_rpc::check_access_control_conditions(
            sol_rpc_conditions,
            auth_sig,
            bls_root_pubkey,
            current_action_ipfs_id,
        )
        .await?;
        return Ok(models::UnifiedConditionCheckResult {
            result,
            successful_auth_sig: auth_sig.clone(),
        });
    } else if let Some(unified_access_control_conditions) = &unified_access_control_conditions {
        return access_control::unified::check_access_control_conditions(
            unified_access_control_conditions,
            auth_sig_item,
            chain.clone(),
            requested_lit_resource_ability,
            cfg,
            &request_id,
            bls_root_pubkey,
            endpoint_version,
            current_action_ipfs_id,
            moka::future::Cache::clone(ipfs_cache),
            reqwest::Client::clone(http_client),
        )
        .await;
    }

    Err(validation_err_code("Missing access control conditions", EC::NodeMissingAccessControlConditions, None)
        .add_detail("You must pass either access_control_conditions or evm_contract_conditions or sol_rpc_conditions or unified_access_control_conditions"))
}
