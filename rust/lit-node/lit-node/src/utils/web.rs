use crate::auth::auth_material::JsonAuthSigExtendedRef;
use crate::client_session::ClientSession;
use crate::error::parser_err_code;
use crate::error::{EC, Result, conversion_err, ipfs_err, unexpected_err, validation_err_code};
use crate::error::{unexpected_err_code, validation_err};
use crate::models;
use crate::models::AuthContext;
use crate::models::RequestConditions;
use crate::models::auth::SessionKeySignedMessageV2;
use crate::tss::common::tss_state::TssState;
use crate::utils::encoding;
use ethers::utils::keccak256;
use ipfs_hasher::IpfsHasher;
use iri_string::spec::UriSpec;
use iri_string::types::RiString;
use lit_blockchain::resolver::rpc::RPC_RESOLVER;
use lit_blockchain::resolver::rpc::config::RpcEntry;
use lit_blockchain::resolver::rpc::config::RpcKind;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use lit_node_common::config::{CFG_KEY_WEB_CLIENT_TIMEOUT_SEC_DEFAULT, LitNodeConfig};
use lit_node_core::CurveType;
use lit_node_core::{
    AccessControlConditionItem, AuthMethod, EVMContractConditionItem, EndpointVersion, JsonAuthSig,
    SolRpcCondition, SolRpcConditionItem, SolRpcConditionItemV0, SolRpcConditionV2Options,
    UnifiedAccessControlConditionItem,
    constants::{AUTH_SIG_DERIVED_VIA_BLS_NETWORK_SIG, LIT_RESOURCE_PREFIX_RAC},
};
use moka::future::Cache;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Value;
use rocket::serde::json::json;
#[allow(unused_imports)]
use rocket::time::OffsetDateTime;
use sha2::{Digest, Sha256};
use siwe::Message;
#[allow(unused_imports)]
use siwe::VerificationOpts;
use siwe_recap::Capability;
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::time::timeout;
use tracing::Instrument;
use tracing::debug_span;
use tracing::instrument;

const MAX_CACHE_DURATION_FOR_AUTH_CONTEXT: u64 = 60 * 60; // 1 hour
pub const MAX_CONDITION_COUNT: u64 = 30;
const AUTH_METHOD_CONTEXTS: &str = "authMethodContexts";
const AUTH_CONTEXT: &str = "auth_context";
const LAST_RETRIEVED_AT: &str = "lastRetrievedAt";
const EXPIRATION: &str = "expiration";

#[cfg(feature = "lit-actions")]
pub async fn check_allowlist(
    allowlist_cache: &models::AllowlistCache,
    allowlist_entry_id: &[u8; 32],
    cfg: &LitConfig,
) -> Result<bool> {
    use crate::error::blockchain_err;
    use crate::models::AllowlistEntry;
    use lit_blockchain::resolver::contract::ContractResolver;

    trace!(
        "Checking allowlist for entry id: {:?}",
        bytes_to_hex(allowlist_entry_id)
    );
    // first, check the cache.  pull a read lock.
    let allowlist = allowlist_cache.entries.read().await;
    let allowlist_entry = allowlist.get(allowlist_entry_id);
    if let Some(allowlist_entry) = allowlist_entry {
        return Ok(allowlist_entry.allowed);
    }
    drop(allowlist);

    // if not in the cache, check the chain and then cache the result
    let resolver = ContractResolver::try_from(cfg)
        .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;
    let allowlist_contract = resolver.allowlist_contract(cfg).await?;
    let is_allowed = allowlist_contract
        .is_allowed(*allowlist_entry_id)
        .call()
        .await
        .map_err(|e| blockchain_err(e, Some("Error checking allowlist".into())))?;

    // only cache the result if it's allowed.
    // this is because we want to be able to allow more things in the future
    if is_allowed {
        // pull a write lock to update the cache
        let mut allowlist = allowlist_cache.entries.write().await;
        allowlist.insert(
            *allowlist_entry_id,
            AllowlistEntry {
                allowed: is_allowed,
            },
        );
        drop(allowlist);
    }

    Ok(is_allowed)
}

pub fn get_signed_message(
    signed_message: &str,
) -> std::result::Result<SessionKeySignedMessageV2, String> {
    serde_json::from_str::<SessionKeySignedMessageV2>(signed_message).map_err(|e| {
        let err_msg = "Parsing SessionKeySignedMessageV2 failed. \
                        Either an AuthSig is passed or the sessionSig is incorrectly formatted";
        error!("{}", err_msg);
        String::from(err_msg)
    })
}

pub async fn get_auth_context_from_session_sigs(
    session_key_signed_message: SessionKeySignedMessageV2,
) -> Result<Option<AuthContext>> {
    let mut resolved_auth_context: Option<AuthContext> = None;

    // loop over capabilities and find any for lit-resolvedauthcontext://*
    for capability_auth_sig in session_key_signed_message.capabilities.iter() {
        let eoa_capacity_auth_sig =
            capability_auth_sig.derived_via != AUTH_SIG_DERIVED_VIA_BLS_NETWORK_SIG;
        if eoa_capacity_auth_sig {
            continue;
        }

        // At this point we've already verified the validity of the `capacity_auth_sig` against the BLS Root pubkey in the validate_bls_auth_sig_basic function
        let signed_message = capability_auth_sig
            .signed_message
            .parse::<Message>()
            .map_err(|e| {
                parser_err_code(
                    e,
                    EC::NodeWalletSignatureJSONError,
                    Some("Error parsing wallet sig signed message".into()),
                )
            })?;

        let capabilities_res = Capability::<Value>::extract_and_verify(&signed_message);
        let capability = match capabilities_res {
            Ok(capabilities) => match capabilities {
                Some(capability) => capability,
                None => {
                    error!("Capabilities are empty");
                    continue;
                }
            },
            Err(e) => {
                error!("Error extracting and verifying capabilities: {:?}", e);
                continue;
            }
        };

        for ability in capability.abilities().iter() {
            if ability.0.scheme_str() == LIT_RESOURCE_PREFIX_RAC {
                for inner_ability in ability.1 {
                    if inner_ability.0.clone().into_inner() == *"Auth/Auth".to_string() {
                        // loop over the restrictions
                        for mut map in inner_ability.1.clone().into_iter() {
                            if let Some(auth_context) = map.get_mut(AUTH_CONTEXT) {
                                if let Some(auth_method_contexts) =
                                    auth_context[AUTH_METHOD_CONTEXTS].as_array_mut()
                                {
                                    // loop over and set all the last retrieved at and expirations
                                    for auth_method in auth_method_contexts.iter_mut() {
                                        // Set it here as we skipped it in the sessionSig.  Set it to 0 because we don't know the real value.
                                        auth_method[LAST_RETRIEVED_AT] =
                                            json!(SystemTime::UNIX_EPOCH);
                                        // set the expiration time to the time from the SIWE, since that's the only relevant expiration time to them.
                                        match signed_message.expiration_time {
                                            Some(ref expiration_time) => {
                                                let expiration_time: &siwe::TimeStamp =
                                                    expiration_time;
                                                auth_method[EXPIRATION] =
                                                    json!(expiration_time.as_ref().unix_timestamp())
                                            }
                                            None => {
                                                return Err(validation_err_code(
                                                    "Could not extract expiration time from SIWE message",
                                                    EC::NodeSIWEMessageError,
                                                    None,
                                                ));
                                            }
                                        }
                                    }
                                }

                                resolved_auth_context = Some(
                                    match serde_json::from_value::<AuthContext>(
                                        auth_context.clone(),
                                    ) {
                                        Ok(auth_context) => auth_context,
                                        Err(e) => {
                                            return Err(validation_err_code(
                                                e,
                                                EC::NodeAuthContextFromSessionSigError,
                                                Some(
                                                    "Unable to resolve sessionSig authContext"
                                                        .into(),
                                                ),
                                            ));
                                        }
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(resolved_auth_context)
}

#[instrument(level = "debug", skip_all)]
#[allow(clippy::too_many_arguments)]
pub async fn get_auth_context(
    auth_sig: Option<JsonAuthSig>,
    auth_methods: Option<Vec<AuthMethod>>,
    action_ipfs_id: Option<String>,
    auth_context_cache: Option<&models::AuthContextCache>,
    mark_as_used_for_sign_session_key_request: bool,
    cfg: Arc<LitConfig>,
    webauthn_origin_domain: Option<String>,
    bls_root_pubkey: &str,
    endpoint_version: &EndpointVersion,
    http_client: reqwest::Client,
) -> Result<AuthContext> {
    use crate::{pkp::auth::verify_auth_method, utils::contract::get_pkp_permissions_contract};
    use lit_node_core::LitActionResource;

    let mut verified_wallet_address = None;
    let mut recap_message: Vec<RiString<UriSpec>> = vec![];
    if let Some(auth_sig) = auth_sig {
        // create the lit resource
        let lit_action_resource =
            LitActionResource::new(action_ipfs_id.clone().unwrap_or("".to_string()));

        // validate the authsig
        let validate_res = JsonAuthSigExtendedRef(&auth_sig)
            .validate_and_get_wallet_sig(
                &lit_action_resource.execution_ability(),
                &None,
                &cfg,
                bls_root_pubkey,
                endpoint_version,
            )
            .await
            .map_err(|e| {
                validation_err_code(e, EC::NodeInvalidAuthSig, Some("Invalid AuthSig".into()))
            });
        match validate_res {
            Ok(valid_auth_sig) => {
                // Get user address of auth sig
                verified_wallet_address = Some(
                    JsonAuthSigExtendedRef(&auth_sig)
                        .user_address(bls_root_pubkey)
                        .await
                        .map_err(|e| {
                            validation_err_code(
                                e,
                                EC::NodeInvalidAuthSig,
                                Some("Invalid AuthSig".into()),
                            )
                        })?,
                );
            }
            Err(e) => {
                return Err(e);
            }
        }
        match auth_sig.signed_message.as_str().parse::<Message>() {
            Ok(message) => {
                recap_message = message.resources;
            }
            Err(e) => {
                trace!(
                    "Could not parse siwe signed message as ReCap message, keeping empty collection"
                );
            }
        }
    }

    // if there's no cache, then we only care about the wallet address
    let auth_context_cache = match auth_context_cache {
        Some(auth_context_cache) => auth_context_cache,
        None => {
            let auth_context = AuthContext {
                action_ipfs_id_stack: vec![],
                auth_sig_address: verified_wallet_address,
                auth_method_contexts: vec![],
                resources: vec![],
                custom_auth_resource: "".to_string(),
            };

            return Ok(auth_context);
        }
    };

    let mut auth_method_contexts: Vec<models::AuthMethodResponse> = vec![];
    // load up all the auth methods
    if let Some(methods) = auth_methods {
        // Get PKP permissions contract.
        let pkp_permissions_contract = Arc::new(get_pkp_permissions_contract(cfg.clone()).await?);

        for method in methods {
            // check the cache first
            let cache_key = format!("{}-{}", method.auth_method_type, method.access_token);
            if let Some(auth_method_context) =
                auth_context_cache.auth_contexts.get(&cache_key).await
            {
                trace!(
                    "Found auth method context in cache: {:?}",
                    auth_method_context
                );
                // check if it's expired
                if auth_method_context.last_retrieved_at
                    + Duration::from_secs(MAX_CACHE_DURATION_FOR_AUTH_CONTEXT)
                    > SystemTime::now()
                    && !auth_method_context.used_for_sign_session_key_request
                {
                    // it's not expired or already marked as used, so we can use it
                    auth_method_contexts.push(auth_method_context.clone());
                    continue;
                }
            }

            let auth_method_response = verify_auth_method(
                &method,
                cfg.clone(),
                pkp_permissions_contract.clone(),
                webauthn_origin_domain.clone(),
                http_client.clone(),
            )
            .await;
            let mut auth_method_response = match auth_method_response {
                Ok(auth_method_response) => auth_method_response,
                Err(e) => {
                    return Err(unexpected_err(
                        e,
                        Some("Failed to verify auth method".into()),
                    ));
                }
            };
            if mark_as_used_for_sign_session_key_request {
                auth_method_response.used_for_sign_session_key_request = true;
            }
            auth_method_contexts.push(auth_method_response.clone());

            // insert this into the cache
            auth_context_cache
                .auth_contexts
                .insert(cache_key.clone(), auth_method_response.clone())
                .await;
        }
    }

    let mut action_ipfs_id_stack = vec![];
    if let Some(action_ipfs_id) = action_ipfs_id.clone() {
        action_ipfs_id_stack.push(action_ipfs_id);
    }
    let auth_context = AuthContext {
        action_ipfs_id_stack,
        auth_sig_address: verified_wallet_address,
        auth_method_contexts,
        resources: recap_message,
        custom_auth_resource: "".to_string(),
    };

    Ok(auth_context)
}

fn recursive_unified_access_control_condition_count(
    conditions: &Vec<UnifiedAccessControlConditionItem>,
) -> u64 {
    let mut count = 0;
    for condition in conditions {
        match condition {
            UnifiedAccessControlConditionItem::Condition(condition) => {
                count += 1;
            }
            UnifiedAccessControlConditionItem::Operator(conditions) => {
                continue;
            }
            UnifiedAccessControlConditionItem::Group(conditions) => {
                count += recursive_unified_access_control_condition_count(conditions);
            }
        }
    }
    count
}

#[instrument(level = "trace")]
fn recursive_sol_rpc_condition_count(conditions: &Vec<SolRpcConditionItem>) -> u64 {
    let mut count = 0;
    for condition in conditions {
        match condition {
            SolRpcConditionItem::Condition(condition) => {
                count += 1;
            }
            SolRpcConditionItem::Operator(conditions) => {
                continue;
            }
            SolRpcConditionItem::Group(conditions) => {
                count += recursive_sol_rpc_condition_count(conditions);
            }
        }
    }
    count
}

#[instrument(level = "trace")]
fn recursive_evm_contract_condition_count(conditions: &Vec<EVMContractConditionItem>) -> u64 {
    let mut count = 0;
    for condition in conditions {
        match condition {
            EVMContractConditionItem::Condition(condition) => {
                count += 1;
            }
            EVMContractConditionItem::Operator(conditions) => {
                continue;
            }
            EVMContractConditionItem::Group(conditions) => {
                count += recursive_evm_contract_condition_count(conditions);
            }
        }
    }
    count
}

#[instrument(level = "trace")]
fn recursive_access_control_condition_count(conditions: &Vec<AccessControlConditionItem>) -> u64 {
    let mut count = 0;
    for condition in conditions {
        match condition {
            AccessControlConditionItem::Condition(condition) => {
                count += 1;
            }
            AccessControlConditionItem::Operator(conditions) => {
                continue;
            }
            AccessControlConditionItem::Group(conditions) => {
                count += recursive_access_control_condition_count(conditions);
            }
        }
    }
    count
}

#[instrument(level = "trace")]
pub fn check_condition_count(
    access_control_conditions: &Option<Vec<AccessControlConditionItem>>,
    evm_contract_conditions: &Option<Vec<EVMContractConditionItem>>,
    sol_rpc_conditions: &Option<Vec<SolRpcConditionItem>>,
    unified_access_control_conditions: &Option<Vec<UnifiedAccessControlConditionItem>>,
) -> Result<()> {
    if let Some(conditions) = access_control_conditions {
        let count = recursive_access_control_condition_count(conditions);
        if count > MAX_CONDITION_COUNT {
            return Err(validation_err_code(
                format!(
                    "Too many conditions, max is {}, got {}",
                    MAX_CONDITION_COUNT, count
                ),
                EC::NodeTooManyConditions,
                None,
            ));
        }
    }

    if let Some(conditions) = evm_contract_conditions {
        let count = recursive_evm_contract_condition_count(conditions);
        if count > MAX_CONDITION_COUNT {
            return Err(validation_err_code(
                format!(
                    "Too many conditions, max is {}, got {}",
                    MAX_CONDITION_COUNT, count
                ),
                EC::NodeTooManyConditions,
                None,
            ));
        }
    }

    if let Some(conditions) = sol_rpc_conditions {
        let count = recursive_sol_rpc_condition_count(conditions);
        if count > MAX_CONDITION_COUNT {
            return Err(validation_err_code(
                format!(
                    "Too many conditions, max is {}, got {}",
                    MAX_CONDITION_COUNT, count
                ),
                EC::NodeTooManyConditions,
                None,
            ));
        }
    }

    if let Some(conditions) = unified_access_control_conditions {
        let count = recursive_unified_access_control_condition_count(conditions);
        if count > MAX_CONDITION_COUNT {
            return Err(validation_err_code(
                format!(
                    "Too many conditions, max is {}, got {}",
                    MAX_CONDITION_COUNT, count
                ),
                EC::NodeTooManyConditions,
                None,
            ));
        }
    }

    Ok(())
}

#[instrument(level = "trace")]
pub fn pubkey_to_eth_address_bytes(pubkey: &str) -> Result<[u8; 20]> {
    pubkey_bytes_to_eth_address_bytes(
        encoding::hex_to_bytes(pubkey).map_err(|e| conversion_err(e, None))?,
    )
}

#[instrument(level = "trace")]
pub fn pubkey_bytes_to_eth_address_bytes(pubkey: Vec<u8>) -> Result<[u8; 20]> {
    // remove the first byte from the pubkey, which is the 04 prefix.
    if pubkey.len() < 33 {
        return Err(validation_err(
            "Invalid pubkey length.  Expected generally 33 or 64 bytes.",
            None,
        ));
    }

    let bytes = pubkey[1..].to_vec();
    let hashed = keccak256(bytes);
    let mut address_bytes = [0u8; 20];
    address_bytes.copy_from_slice(&hashed[12..32]);
    Ok(address_bytes)
}

// pub fn pubkey_to_eth_address(pubkey: &str) -> String {
//     let bytes = pubkey_to_eth_address_bytes(pubkey);
//     let mut address = "0x".to_string();
//     address.push_str(&encoding::bytes_to_hex(&bytes));
//     return address;
// }

#[instrument(level = "debug", skip(ipfs_cache))]
pub async fn get_ipfs_file(
    ipfs_id: &String,
    cfg: &LitConfig,
    ipfs_cache: Cache<String, Arc<String>>,
    http_cache: reqwest::Client,
) -> Result<Arc<String>> {
    // check the cache first
    if let Some(cached_file) = ipfs_cache.get(ipfs_id).await {
        // cache hit
        return Ok(cached_file);
    }
    // cache miss
    let text_result = Arc::new(retrieve_from_ipfs(ipfs_id, cfg, http_cache).await?);
    ipfs_cache
        .insert(ipfs_id.clone(), text_result.clone())
        .await;
    Ok(text_result)
}

async fn retrieve_from_ipfs(
    ipfs_id: &String,
    cfg: &LitConfig,
    http_client: reqwest::Client,
) -> Result<String> {
    let default_entry = RpcEntry::new(RpcKind::IPFS, cfg.ipfs_gateway(), None, None);

    let rpc_resolver_struct = &RPC_RESOLVER;
    let rpc_resolver = rpc_resolver_struct.load_full();
    let mut ipfs_gateways = rpc_resolver
        .resolve("ipfs_gateways")
        .unwrap_or(&vec![])
        .clone();

    if ipfs_gateways.is_empty() {
        ipfs_gateways.push(default_entry);
    }

    let gateway = ipfs_gateways
        .first()
        .expect("ipfs_gateways should always have one entry");

    let start_time = SystemTime::now();
    // TODO: set a max filesize for retrieval
    // TODO: use apikey & headers
    let url = gateway.url().replace("{}", ipfs_id.as_str());
    let req = http_client.get(&url).send().instrument(debug_span!("fetch_ipfs_file")).await
        .map_err(|e| {
            if e.is_timeout() {
                ipfs_err(e, Some("Timeout error getting code from ipfs".into()))
                    .add_detail(format!("Timeout error getting code from ipfs. Try getting it yourself in a browser and see if it works: {url}"))
            } else {
                ipfs_err(e, Some("Error getting ipfs file".into()))
                    .add_detail(format!("Error getting ipfs file: {}", ipfs_id))
            }
        })?;

    if req.status() != 200 {
        return Err(ipfs_err(
            format!(
                "Error getting code from ipfs url. Status code: {}.  Url: {url}",
                req.status()
            ),
            None,
        )
        .add_detail(format!("Error getting ipfs file: {}", ipfs_id)));
    }
    let text_result = req.text().await.map_err(|e| {
        conversion_err(
            e,
            Some("Failed to get text from response during IPFS fetch".into()),
        )
    })?;

    if text_result.len() > 30000000 {
        return Err(ipfs_err(
            format!(
                "Error getting code from ipfs url. File too large: {}",
                text_result.len()
            ),
            None,
        ));
    }

    // verify the hash
    let ipfs_hasher = IpfsHasher::default();
    let cid = ipfs_hasher.compute(text_result.as_bytes());
    if cid != ipfs_id.clone() {
        return Err(ipfs_err(
            format!(
                "Error getting code from ipfs url.  Hash mismatch.  Expected: {}  Actual: {}",
                ipfs_id, cid
            ),
            None,
        ));
    }

    let end_time = SystemTime::now();
    let elapsed = end_time
        .duration_since(start_time)
        .map_err(|e| unexpected_err(e, Some("Unable to get duration".into())))?;
    debug!("Retrieved from IPFS in {}ms", elapsed.as_millis());

    Ok(text_result)
}

#[instrument(level = "trace")]
pub fn hash_access_control_conditions(req: RequestConditions) -> Result<String> {
    // hash the access control condition and thing to decrypt
    let mut hasher = Sha256::new();

    // we need to check if we got passed an access control condition or an evm contract condition
    if let Some(access_control_conditions) = &req.access_control_conditions {
        let stringified_access_control_conditions =
            serde_json::to_string(access_control_conditions)
                .expect_or_err("Could not stringify")?;
        trace!(
            "stringified_access_control_conditions: {:?}",
            stringified_access_control_conditions
        );
        hasher.update(stringified_access_control_conditions.as_bytes());
    } else if let Some(evm_contract_conditions) = &req.evm_contract_conditions {
        let stringified_access_control_conditions =
            serde_json::to_string(evm_contract_conditions).expect_or_err("Could not stringify")?;
        trace!(
            "stringified_access_control_conditions: {:?}",
            stringified_access_control_conditions
        );
        hasher.update(stringified_access_control_conditions.as_bytes());
    } else if let Some(sol_rpc_conditions) = &req.sol_rpc_conditions {
        // hash differently if this is v1 or v2 conditions
        let mut is_v2 = false;
        for condition_item in sol_rpc_conditions {
            if let SolRpcConditionItem::Condition(condition) = condition_item
                && condition.pda_params.is_some()
            {
                is_v2 = true;
                break;
            }
        }
        if is_v2 {
            // we can just hash directly
            let stringified_access_control_conditions =
                serde_json::to_string(&req.sol_rpc_conditions)
                    .expect_or_err("Could not stringify")?;
            debug!(
                "stringified_access_control_conditions: {:?}",
                stringified_access_control_conditions
            );
            hasher.update(stringified_access_control_conditions.as_bytes());
        } else {
            // need to massage into v1 condition array
            let v1_conditions = convert_sol_rpc_conditions_to_v1(sol_rpc_conditions);
            let stringified_access_control_conditions =
                serde_json::to_string(&v1_conditions).expect_or_err("Could not stringify")?;
            debug!(
                "stringified_access_control_conditions: {:?}",
                stringified_access_control_conditions
            );
            hasher.update(stringified_access_control_conditions.as_bytes());
        }
    } else if let Some(unified_access_control_conditions) = &req.unified_access_control_conditions {
        let stringified_access_control_conditions =
            serde_json::to_string(unified_access_control_conditions)
                .expect_or_err("Could not stringify")?;
        trace!(
            "stringified_access_control_conditions: {:?}",
            stringified_access_control_conditions
        );
        hasher.update(stringified_access_control_conditions.as_bytes());
    } else {
        return Err(validation_err_code(
            "Missing access control conditions",
            EC::NodeMissingAccessControlConditions,
            None,
        ));
    }

    let hashed_access_control_conditions = bytes_to_hex(hasher.finalize());
    debug!(
        "hashed access control conditions: {:?}",
        hashed_access_control_conditions
    );
    Ok(hashed_access_control_conditions)
}

#[instrument(level = "trace")]
pub fn convert_sol_rpc_conditions_to_v1(
    sol_rpc_conditions: &Vec<SolRpcConditionItem>,
) -> Vec<SolRpcConditionItemV0> {
    // need to massage into v1 condition array
    let mut v1_conditions: Vec<SolRpcConditionItemV0> = Vec::new();
    for condition_item in sol_rpc_conditions {
        match condition_item {
            SolRpcConditionItem::Condition(condition) => {
                v1_conditions.push(SolRpcConditionItemV0::Condition(
                    sol_rpc_condition_v2_to_v1(condition),
                ));
            }
            SolRpcConditionItem::Operator(operator) => {
                v1_conditions.push(SolRpcConditionItemV0::Operator(*operator));
            }
            SolRpcConditionItem::Group(group) => {
                v1_conditions.push(SolRpcConditionItemV0::Group(group.clone()));
            }
        }
    }
    v1_conditions
}

#[instrument(level = "trace")]
pub fn sol_rpc_condition_v2_to_v1(condition: &SolRpcConditionV2Options) -> SolRpcCondition {
    SolRpcCondition {
        method: condition.method.clone(),
        params: condition.params.clone(),
        chain: condition.chain.clone(),
        return_value_test: condition.return_value_test.clone(),
    }
}

#[instrument(level = "debug", skip_all)]
pub fn check_lit_session_auth_sig(auth_sig: &JsonAuthSig) -> Result<bool> {
    let pk_bytes = encoding::hex_to_bytes(&auth_sig.address)?;
    let public_key = ed25519_dalek::VerifyingKey::from_bytes(
        &<[u8; 32]>::try_from(&pk_bytes[..]).map_err(|e| conversion_err(e, None))?,
    )
    .map_err(|e| conversion_err(e, None))?;
    let message = auth_sig.signed_message.as_bytes();
    let signature =
        ed25519_dalek::Signature::from_str(&auth_sig.sig).map_err(|e| conversion_err(e, None))?;
    Ok(public_key.verify_strict(message, &signature).is_ok())
}

#[instrument(level = "trace")]
pub fn pubkey_to_token_id(pubkey: &str) -> Result<String> {
    let pubkey_bytes = encoding::hex_to_bytes(pubkey).map_err(|e| conversion_err(e, None))?;
    if pubkey_bytes.len() != 65 {
        return Err(validation_err(
            format!(
                "Invalid pubkey length.  Expected 65 bytes, got {}",
                pubkey_bytes.len()
            ),
            None,
        ));
    }

    let token_id = bytes_to_hex(keccak256(&pubkey_bytes));
    Ok(token_id)
}

pub async fn get_bls_root_pubkey(tss_state: &TssState) -> Result<String> {
    let curve_state = tss_state.get_dkg_state(CurveType::BLS)?;
    let bls_root_pubkeys = curve_state.root_keys().await;
    match bls_root_pubkeys.first() {
        Some(bls_root_key) => Ok(bls_root_key.clone()),
        None => Err(unexpected_err_code(
            "No BLS root key found",
            EC::NodeBLSRootKeyNotFound,
            None,
        )),
    }
}

// Generic helper function to enforce the timeout logic on any async route handler
pub async fn with_timeout<F>(
    cfg: &LitConfig,
    custom_timeout_ms: Option<u64>,
    client_session: Option<Arc<ClientSession>>,
    handler: F,
) -> Custom<Value>
where
    F: Future<Output = Custom<Value>> + Send,
{
    let timeout_duration = match custom_timeout_ms {
        Some(ms) => Duration::from_millis(ms),
        None => Duration::from_secs(
            cfg.web_client_timeout_s()
                .unwrap_or(CFG_KEY_WEB_CLIENT_TIMEOUT_SEC_DEFAULT) as u64,
        ),
    };

    match timeout(timeout_duration, handler).await {
        Ok(response) => response,
        Err(_) => match client_session {
            Some(session) => session.json_encrypt_err_and_code(
                "Request timed out",
                "request_timeout",
                Status::RequestTimeout,
            ),
            None => Custom(
                Status::RequestTimeout,
                json!({"message": "Request timed out", "errorCode": "request_timeout"}),
            ),
        },
    }
}

pub fn default_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .use_rustls_tls()
        .timeout(Duration::from_secs(30))
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(30)
        .build()
        .expect("Error building request client")
}
