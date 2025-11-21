//! This module implements the gRPC client for the Lit Actions server (lit_actions).
//! The client initiates JS code execution and handles op requests from the server.
//! It holds all configuration data (including secrets) and manages state; none of
//! which are shared with lit_actions, enabling a secure execution environment.

use std::borrow::BorrowMut;
use std::collections::{BTreeMap, HashMap};
use std::error::Error as _;
use std::path::PathBuf;
use std::sync::Arc;

use super::{ActionJob, ActionStore, JobId};
use crate::access_control::rpc_url;
use crate::error::{connect_err, conversion_err, memory_limit_err, timeout_err, unexpected_err};
use crate::models::{self, RequestConditions, UnifiedConditionCheckResult};
use crate::p2p_comms::CommsManager;
use crate::payment::dynamic::DynamicPayment;
use crate::peers::{grpc_client_pool::GrpcClientPool, peer_state::models::SimplePeerCollection};
use crate::pkp;
use crate::tasks::utils::generate_hash;
use crate::tss::common::hd_keys::get_derived_keyshare;
use crate::tss::common::tss_state::TssState;
use crate::utils::encoding;
use crate::utils::tracing::inject_tracing_metadata;
use crate::utils::web::{get_bls_root_pubkey, hash_access_control_conditions};
use anyhow::{Context as _, Result, bail};
use base64_light::base64_decode;
use blsful::inner_types::GroupEncoding;
use blsful::{Bls12381G2Impl, SignatureShare};
use derive_builder::Builder;
use ecdsa::SignatureSize;
use elliptic_curve::generic_array::ArrayLength;
use elliptic_curve::{CurveArithmetic, PrimeCurve};
use ethers::utils::keccak256;
use futures::{FutureExt as _, TryFutureExt};
use hd_keys_curves::{HDDerivable, HDDeriver};
use lit_actions_grpc::tokio_stream::StreamExt as _;
use lit_actions_grpc::tonic::{
    Code, Extensions, Request, Status, metadata::MetadataMap, transport::Error as TransportError,
};
use lit_actions_grpc::{proto::*, unix};
use lit_blockchain::resolver::rpc::{ENDPOINT_MANAGER, RpcHealthcheckPoller};
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;
use tracing::{debug, instrument};

use crate::tss::common::curve_state::CurveState;
use lit_node_common::config::LitNodeConfig as _;
use lit_node_core::{
    AccessControlConditionResource, AuthSigItem, BeHex, CompressedBytes, EndpointVersion,
    JsonAuthSig, LitActionPriceComponent, LitResource, NodeSet, PeerId, SignableOutput, SignedData,
    SigningScheme, UnifiedAccessControlConditionItem, response,
};
use lit_sdk::signature::{SignedDataOutput, combine_and_verify_signature_shares};

const DEFAULT_TIMEOUT_MS: u64 = 30_000; // 30s
const DEFAULT_ASYNC_TIMEOUT_MS: u64 = 300_000; // 5m
const DEFAULT_CLIENT_TIMEOUT_MS_BUFFER: u64 = 5_000;
const DEFAULT_MEMORY_LIMIT_MB: u32 = 256; // 256MB

const DEFAULT_MAX_CODE_LENGTH: usize = 16 * 1024 * 1024; // 16MB
const DEFAULT_MAX_CONSOLE_LOG_LENGTH: usize = 1024 * 100; // 100KB
const DEFAULT_MAX_CONTRACT_CALL_COUNT: u32 = 30;
const DEFAULT_MAX_FETCH_COUNT: u32 = 50;
const DEFAULT_MAX_RESPONSE_LENGTH: usize = 1024 * 100; // 100KB
const DEFAULT_MAX_SIGN_COUNT: u32 = 10; // 10 signature requests per action execution
const DEFAULT_MAX_BROADCAST_AND_COLLECT_COUNT: u32 = 30;
const DEFAULT_MAX_CALL_DEPTH: u32 = 5;
const DEFAULT_MAX_RETRIES: u32 = 3;

#[derive(Debug, Default, Clone, Builder, Serialize, Deserialize)]
pub struct Client {
    // Config
    #[builder(default, setter(into, strip_option))]
    socket_path: Option<PathBuf>,
    #[builder(default, setter(into))]
    #[serde(skip)]
    pub(crate) js_env: models::DenoExecutionEnv,
    #[builder(default, setter(into))]
    auth_context: models::AuthContext,
    #[builder(default, setter(into))]
    auth_sig: Option<JsonAuthSig>,
    #[builder(default, setter(into))]
    request_id: Option<String>,
    #[builder(default, setter(into))]
    http_headers: BTreeMap<String, String>,
    #[builder(default, setter(into))]
    epoch: Option<u64>,
    #[builder(default, setter(into))]
    endpoint_version: EndpointVersion,
    #[builder(default, setter(into))]
    node_set: Vec<NodeSet>,

    // Limits
    #[builder(default = "DEFAULT_TIMEOUT_MS")]
    timeout_ms: u64,
    #[builder(default = "DEFAULT_ASYNC_TIMEOUT_MS")]
    #[serde(skip)]
    async_timeout_ms: u64,
    #[builder(default = "DEFAULT_MEMORY_LIMIT_MB")]
    memory_limit_mb: u32,
    #[builder(default = "DEFAULT_MAX_CODE_LENGTH")]
    max_code_length: usize,
    #[builder(default = "DEFAULT_MAX_RESPONSE_LENGTH")]
    max_response_length: usize,
    #[builder(default = "DEFAULT_MAX_CONSOLE_LOG_LENGTH")]
    max_console_log_length: usize,
    #[builder(default = "DEFAULT_MAX_FETCH_COUNT")]
    max_fetch_count: u32,
    #[builder(default = "DEFAULT_MAX_SIGN_COUNT")]
    max_sign_count: u32,
    #[builder(default = "DEFAULT_MAX_CONTRACT_CALL_COUNT")]
    max_contract_call_count: u32,
    #[builder(default = "DEFAULT_MAX_BROADCAST_AND_COLLECT_COUNT")]
    max_broadcast_and_collect_count: u32,
    #[builder(default = "DEFAULT_MAX_CALL_DEPTH")]
    max_call_depth: u32,
    #[builder(default = "DEFAULT_MAX_RETRIES")]
    max_retries: u32,

    #[builder(default)]
    #[serde(skip)]
    pub(crate) client_grpc_channels: GrpcClientPool<tonic::transport::Channel>,

    #[builder(default)]
    pub dynamic_payment: DynamicPayment,
    // State
    #[builder(setter(skip))]
    #[serde(skip)]
    pub(crate) state: ExecutionState,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionState {
    pub response: String,
    pub logs: String,
    #[serde(skip)]
    pub fetch_count: u32,
    #[serde(skip)]
    pub sign_count: u32,
    pub signed_data: HashMap<String, SignedData>,
    #[serde(skip)]
    pub claim_count: u32,
    pub claim_data: HashMap<String, response::JsonPKPClaimKeyResponse>,
    #[serde(skip)]
    pub contract_call_count: u32,
    #[serde(skip)]
    pub broadcast_and_collect_count: u32,
    #[serde(skip)]
    pub ops_count: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExecutionOptions {
    pub code: Arc<String>,
    pub globals: Option<serde_json::Value>,
    pub action_ipfs_id: Option<String>,
}

impl From<&str> for ExecutionOptions {
    fn from(code: &str) -> Self {
        Self {
            code: Arc::new(code.to_string()),
            ..Default::default()
        }
    }
}

impl From<String> for ExecutionOptions {
    fn from(code: String) -> Self {
        Self {
            code: Arc::new(code),
            ..Default::default()
        }
    }
}

impl Client {
    pub fn new(socket_path: impl Into<PathBuf>) -> Self {
        ClientBuilder::default()
            .socket_path(socket_path)
            .build()
            .expect("cannot fail")
    }

    fn lit_config(&self) -> &LitConfig {
        self.js_env.cfg.as_ref()
    }

    fn socket_path(&self) -> PathBuf {
        self.socket_path.clone().unwrap_or_else(|| {
            self.lit_config()
                .actions_socket()
                .expect("invalid socket path in config")
        })
    }

    fn client_timeout(&self) -> Duration {
        Duration::from_millis(self.timeout_ms + DEFAULT_CLIENT_TIMEOUT_MS_BUFFER)
    }

    pub fn request_id(&self) -> String {
        self.request_id.clone().unwrap_or_default()
    }

    pub fn logs(&self) -> &str {
        &self.state.logs
    }

    pub fn authorized_address(&self) -> Option<String> {
        self.auth_context
            .auth_sig_address
            .as_ref()
            .map(|s| s.to_lowercase())
    }

    fn tss_state_and_txn_prefix(&self) -> Result<(TssState, String)> {
        let tss_state = self
            .js_env
            .tss_state
            .clone()
            .expect_or_err("No TSS state found")?;
        let txn_prefix = self.request_id();
        Ok((tss_state, txn_prefix))
    }

    fn ipfs_cache(&self) -> Result<Cache<String, Arc<String>>> {
        if let Some(ipfs_cache) = self.js_env.ipfs_cache.clone() {
            return Ok(ipfs_cache);
        }

        bail!("No IPFS cache found");
    }

    fn http_cache(&self) -> Result<reqwest::Client> {
        if let Some(http_cache) = self.js_env.http_client.clone() {
            return Ok(http_cache);
        }
        bail!("No HTTP cache found");
    }

    fn metadata(&self) -> Result<MetadataMap> {
        let mut md = MetadataMap::new();
        md.insert(
            "x-host",
            self.lit_config()
                .external_addr()
                .unwrap_or_default()
                .parse()?,
        );
        md.insert("x-request-id", self.request_id().parse()?);

        // Add trace context to the metadata for distributed tracing.
        inject_tracing_metadata(&mut md);

        Ok(md)
    }

    fn reset_state(&mut self) {
        std::mem::take(&mut self.state);
    }

    #[instrument(level = "debug", skip_all, ret)]
    pub async fn execute_js_async(
        &self,
        opts: impl Into<ExecutionOptions>,
        store: &ActionStore,
    ) -> Result<JobId> {
        store
            .clone()
            .submit_job(ActionJob::new(
                Client {
                    timeout_ms: self.async_timeout_ms,
                    ..self.clone()
                },
                opts,
            ))
            .await
    }

    #[instrument(level = "debug", skip_all, ret)]
    pub async fn execute_js(
        &mut self,
        opts: impl Into<ExecutionOptions>,
    ) -> Result<ExecutionState, crate::error::Error> {
        self.reset_state();
        self.dynamic_payment
            .add(LitActionPriceComponent::BaseAmount, 1)?;
        let opts = opts.into();
        let timeout = self.client_timeout();

        let auth_context = {
            let mut ctx = self.auth_context.clone();
            if let Some(id) = &opts.action_ipfs_id {
                ctx.action_ipfs_id_stack.push(id.clone());
            }
            ctx
        };

        // Hand-roll retry loop as crates like tokio-retry or again don't play well with &mut self
        let mut retry = 0;
        loop {
            let execution = Box::pin(self.execute_js_inner(
                opts.code.clone(),
                opts.globals.clone(),
                &auth_context,
                0,
            ));
            let execution_result =
                tokio::time::timeout(timeout, execution)
                    .await
                    .map_err(|_| {
                        timeout_err(
                            format!("lit_actions didn't respond within {timeout:?}"),
                            None,
                        )
                    })?;
            match execution_result {
                Ok(state) => return Ok(state),
                Err(e) => {
                    let last_error = if let Some(status) = e.downcast_ref::<Status>() {
                        let msg = status.message();
                        match status.code() {
                            Code::DeadlineExceeded => {
                                return Err(timeout_err(
                                    msg,
                                    Some("tonic deadline exceeded error".to_string()),
                                ));
                            }
                            Code::ResourceExhausted => {
                                return Err(memory_limit_err(
                                    msg,
                                    Some("tonic resource exhausted error".to_string()),
                                ));
                            }
                            Code::Unavailable => {
                                // This error occurs when NGINX can't connect to any healthy
                                // lit_actions instance and returns the gRPC status code 14
                                connect_err(msg, Some("tonic unavailable error".to_string()))
                            }
                            // NB: We could also retry on `Code::Internal if msg == "h2 protocol error: error reading a body from connection"`.
                            // However, that likely means lit_actions has crashed *while* executing JS, which we can't recover from.
                            _ => return Err(unexpected_err(msg, None)),
                        }
                    } else if let Some(te) = e.downcast_ref::<TransportError>() {
                        // This error occurs when the socket file is missing or lit_actions is down
                        // - connection error: No such file or directory (os error 2)
                        // - connection error: Connection refused (os error 61)
                        connect_err(
                            te.source().unwrap_or(te).to_string(),
                            Some("tonic error".to_string()),
                        )
                    } else if let Some(se) = e.downcast_ref::<flume::SendError<ExecuteJsRequest>>()
                    {
                        // This error occurs when NGINX can't connect to any healthy lit_actions instance
                        // - connection error: sending on a closed channel
                        connect_err(
                            se.source().unwrap_or(se).to_string(),
                            Some("flume error".to_string()),
                        )
                    } else {
                        return Err(unexpected_err(e, None));
                    };

                    // Never retry in-flight requests, which may have modified state
                    if retry >= self.max_retries || self.state.ops_count != 0 {
                        return Err(last_error);
                    }
                    let backoff = Duration::from_secs(2u64.pow(retry));
                    error!("Retrying execute_js in {backoff:?}, cause: {last_error:?}");
                    tokio::time::sleep(backoff).await;
                    retry += 1;
                }
            }
        }
    }

    #[instrument(level = "debug", skip(self), err)]
    async fn execute_js_inner(
        &mut self,
        code: Arc<String>,
        globals: Option<serde_json::Value>,
        auth_context: &models::AuthContext,
        call_depth: u32,
    ) -> Result<ExecutionState> {
        if code.len() > self.max_code_length {
            bail!(
                "Code payload is too large ({} bytes). Max length is {} bytes.",
                code.len(),
                self.max_code_length,
            );
        }

        let (outbound_tx, outbound_rx) = flume::bounded(0);

        let socket_path = self.socket_path();
        let channel = self
            .client_grpc_channels
            .create_or_get_connection(&socket_path.display().to_string(), || {
                unix::connect_to_socket(socket_path).map_err(|e| {
                    connect_err(
                        e,
                        Some("Error creating connection to lit-action server".to_string()),
                    )
                })
            })
            .await?;
        let mut stream = ActionClient::new(channel)
            .execute_js(Request::from_parts(
                self.metadata()?,
                Extensions::default(),
                outbound_rx.into_stream(),
            ))
            // Fix "implementation of `std::marker::Send` is not general enough"
            // Workaround for compiler bug https://github.com/rust-lang/rust/issues/96865
            // See https://github.com/rust-lang/rust/issues/100013#issuecomment-2052045872
            .boxed()
            .await?
            .into_inner();

        // Send initial execution request to server
        outbound_tx
            .send_async(
                ExecutionRequest {
                    code: code.to_string(),
                    js_params: globals.and_then(|v| serde_json::to_vec(&v).ok()),
                    auth_context: serde_json::to_vec(&auth_context).ok(),
                    http_headers: self.http_headers.clone(),
                    timeout: Some(self.timeout_ms),
                    memory_limit: Some(self.memory_limit_mb),
                }
                .into(),
            )
            .await
            .context("failed to send execution request")?;

        // Handle responses from server
        while let Some(resp) = stream.try_next().await? {
            match resp.union {
                // Return final result from server
                Some(UnionResponse::Result(res)) => {
                    if !res.success {
                        bail!(res.error);
                    }
                    // Return current state, which might be updated by subsequent code executions
                    return Ok(self.state.clone());
                }
                // Handle op requests
                Some(op) => {
                    let resp = self
                        .handle_op(op, auth_context, call_depth)
                        .await
                        .unwrap_or_else(|e| {
                            ErrorResponse {
                                error: e.to_string(),
                            }
                            .into()
                        });
                    outbound_tx
                        .send_async(resp)
                        .await
                        .context("failed to send op response")?;
                }
                // Ignore empty responses
                None => {}
            };
        }

        bail!("Server unexpectedly closed connection")
    }

    #[instrument(level = "debug", skip(self), err)]
    async fn handle_op(
        &mut self,
        op: UnionResponse,
        auth_context: &models::AuthContext,
        mut call_depth: u32,
    ) -> Result<ExecuteJsRequest> {
        trace!("handle_op: {:?}", op);
        self.state.ops_count += 1;

        let action_ipfs_id = auth_context.action_ipfs_id_stack.last().cloned();

        Ok(match op {
            UnionResponse::SetResponse(SetResponseRequest { response }) => {
                if response.len() > self.max_response_length {
                    bail!(
                        "Response is too long. Max length is {} bytes",
                        self.max_response_length
                    );
                }
                self.state.response = response;
                SetResponseResponse {}.into()
            }
            UnionResponse::Print(PrintRequest { message }) => {
                if self.state.logs.len() + message.len() > self.max_console_log_length {
                    bail!(
                        "Console.log is printing something that is too long. Max length for all logs in a single request is {} bytes",
                        self.max_console_log_length
                    );
                }
                self.state.logs.push_str(&message);
                PrintResponse {}.into()
            }
            UnionResponse::IncrementFetchCount(IncrementFetchCountRequest {}) => {
                self.state.fetch_count += 1;
                self.pay(LitActionPriceComponent::Fetches, 1).await?;
                if self.state.fetch_count > self.max_fetch_count {
                    bail!(
                        "You may not send more than {} HTTP requests per session and you have attempted to exceed that limit.",
                        self.max_fetch_count
                    );
                }
                IncrementFetchCountResponse {
                    fetch_count: self.state.fetch_count,
                }
                .into()
            }
            UnionResponse::PkpPermissionsGetPermitted(PkpPermissionsGetPermittedRequest {
                method,
                token_id,
            }) => {
                self.pay(LitActionPriceComponent::ContractCalls, 1).await?;
                let resources =
                    pkp::utils::pkp_permissions_get_permitted(method, self.lit_config(), token_id)
                        .await?;
                PkpPermissionsGetPermittedResponse {
                    resources: serde_json::to_vec(&resources)?,
                }
                .into()
            }
            UnionResponse::PkpPermissionsGetPermittedAuthMethodScopes(
                PkpPermissionsGetPermittedAuthMethodScopesRequest {
                    token_id,
                    method,
                    user_id,
                    max_scope_id,
                },
            ) => {
                self.pay(LitActionPriceComponent::ContractCalls, 1).await?;
                let scopes = pkp::utils::pkp_permissions_get_permitted_auth_method_scopes(
                    token_id,
                    self.lit_config(),
                    method,
                    user_id,
                    max_scope_id,
                )
                .await?;
                PkpPermissionsGetPermittedAuthMethodScopesResponse { scopes }.into()
            }
            UnionResponse::PkpPermissionsIsPermitted(PkpPermissionsIsPermittedRequest {
                method,
                token_id,
                params,
            }) => {
                self.pay(LitActionPriceComponent::ContractCalls, 1).await?;
                let is_permitted = pkp::utils::pkp_permissions_is_permitted(
                    token_id,
                    self.lit_config(),
                    method,
                    serde_json::from_slice(&params)?,
                )
                .await?;
                PkpPermissionsIsPermittedResponse { is_permitted }.into()
            }
            UnionResponse::PkpPermissionsIsPermittedAuthMethod(
                PkpPermissionsIsPermittedAuthMethodRequest {
                    token_id,
                    method,
                    user_id,
                },
            ) => {
                self.pay(LitActionPriceComponent::ContractCalls, 1).await?;
                use lit_blockchain::resolver::contract::ContractResolver;

                let cfg = self.lit_config();
                let resolver = ContractResolver::try_from(cfg)?;
                let contract = resolver.pkp_permissions_contract(cfg).await?;
                let is_permitted = pkp::utils::pkp_permissions_is_permitted_auth_method(
                    token_id, cfg, method, user_id,
                )
                .await?;
                PkpPermissionsIsPermittedAuthMethodResponse { is_permitted }.into()
            }
            UnionResponse::PubkeyToTokenId(PubkeyToTokenIdRequest { public_key }) => {
                let bytes = encoding::hex_to_bytes(public_key)?;
                let token_id = format!("0x{}", bytes_to_hex(keccak256(bytes).as_slice()));
                PubkeyToTokenIdResponse { token_id }.into()
            }
            UnionResponse::SignEcdsa(SignEcdsaRequest {
                to_sign,
                public_key,
                sig_name,
                eth_personal_sign,
            }) => {
                self.pay(LitActionPriceComponent::Signatures, 1).await?;

                let success = if eth_personal_sign {
                    // Prepend the Ethereum Signed Message according to EIP-191
                    let mut message =
                        format!("\x19Ethereum Signed Message:\n{}", to_sign.len()).into_bytes();
                    message.extend(&to_sign);

                    // Hash it using keccak256
                    let hashed_message = keccak256(message);

                    self.sign_helper(
                        hashed_message.into(),
                        public_key,
                        sig_name,
                        &[2], // AuthMethodScope::SignPersonalMessage
                        self.epoch,
                        action_ipfs_id,
                        SigningScheme::EcdsaK256Sha256,
                    )
                    .await
                } else {
                    self.sign_helper(
                        to_sign,
                        public_key,
                        sig_name,
                        &[1], // AuthMethodScope::SignAnything
                        self.epoch,
                        action_ipfs_id,
                        SigningScheme::EcdsaK256Sha256,
                    )
                    .await
                }?;
                SignEcdsaResponse { success }.into()
            }
            UnionResponse::Sign(SignRequest {
                to_sign,
                public_key,
                sig_name,
                signing_scheme,
            }) => {
                self.pay(LitActionPriceComponent::Signatures, 1).await?;

                let scheme = signing_scheme
                    .parse::<SigningScheme>()
                    .map_err(|e| conversion_err(e, None))?;
                let success = self
                    .sign_helper(
                        to_sign,
                        public_key,
                        sig_name,
                        &[1],
                        self.epoch,
                        action_ipfs_id,
                        scheme,
                    )
                    .await?;
                SignResponse { success }.into()
            }
            UnionResponse::AesDecrypt(AesDecryptRequest {
                symmetric_key,
                ciphertext,
            }) => {
                let plaintext = super::aes::aes_decrypt(symmetric_key, ciphertext).await?;
                AesDecryptResponse { plaintext }.into()
            }
            UnionResponse::GetLatestNonce(GetLatestNonceRequest { address, chain }) => {
                use ethers::prelude::*;
                use std::str::FromStr;

                self.pay(LitActionPriceComponent::ContractCalls, 1).await?;

                let provider = ENDPOINT_MANAGER.get_provider(&chain)?;
                let addr = ethers::types::Address::from_str(&address)
                    .map_err(|e| conversion_err(e, None))?;
                let latest_nonce = provider.get_transaction_count(addr, None).await?;
                trace!("op_get_latest_nonce; addr: {addr}, latest_nonce: {latest_nonce}");
                GetLatestNonceResponse {
                    nonce: format!("0x{latest_nonce:x}"),
                }
                .into()
            }
            UnionResponse::CheckConditions(CheckConditionsRequest {
                conditions,
                auth_sig,
                chain,
            }) => {
                self.pay(LitActionPriceComponent::ContractCalls, 1).await?;

                let json_auth_sig = self.parse_json_authsig_helper(auth_sig)?;
                let conditions: Vec<UnifiedAccessControlConditionItem> =
                    serde_json::from_slice(&conditions)?;

                let chain = match chain {
                    Some(chain) => chain,
                    None => {
                        bail!("Chain is required for access control checks");
                    }
                };
                let res = self
                    .check_access_control_conditions_helper(
                        &conditions,
                        json_auth_sig,
                        chain,
                        action_ipfs_id,
                    )
                    .await?;

                CheckConditionsResponse {
                    success: res.result,
                }
                .into()
            }
            UnionResponse::ClaimKeyIdentifier(ClaimKeyIdentifierRequest { key_id }) => {
                use ethers::prelude::*;
                use lit_node_core::response;

                // XXX: This value is never used. Should we enforce a limit?
                self.state.claim_count += 1;

                let ipfs_id = match action_ipfs_id {
                    Some(id) => id,
                    None => {
                        bail!("Could not find IPFS ID for this action, aborting claim operation")
                    }
                };

                let serialized = format!("{}_{}", ipfs_id.clone(), key_id.clone());
                let as_bytes = serialized.as_bytes().to_vec();
                let formatted_key_id = keccak256(as_bytes).to_vec();
                let wallet = lit_blockchain::contracts::load_wallet(self.lit_config(), None)
                    .map_err(|e| unexpected_err(e, None))?;
                let signature = wallet
                    .sign_message(&formatted_key_id)
                    .await
                    .map_err(|e| unexpected_err(e, Some("Could not sign message".into())))?;

                let signature = bytes_to_hex(signature.to_vec());
                let key_id_hex = bytes_to_hex(formatted_key_id.clone());

                self.state.claim_data.insert(
                    key_id,
                    response::JsonPKPClaimKeyResponse {
                        signature,
                        derived_key_id: key_id_hex,
                    },
                );

                ClaimKeyIdentifierResponse {
                    success: "success".to_string(),
                }
                .into()
            }
            UnionResponse::CallContract(CallContractRequest { chain, txn }) => {
                use ethers::prelude::*;
                use ethers::types::transaction::eip2718::TypedTransaction;
                use ethers::utils::rlp::Rlp;

                self.pay(LitActionPriceComponent::ContractCalls, 1).await?;

                self.state.contract_call_count += 1;
                if self.state.contract_call_count > self.max_contract_call_count {
                    bail!(
                        "You may invoke contract calls more than {} times per session and you have attempted to exceed that limit.",
                        self.max_contract_call_count
                    );
                }

                // FIXME: ideally we should try each index until we find one that works.  0 is hardcoded for now.
                let provider: Arc<Provider<Http>> =
                    ENDPOINT_MANAGER.get_provider(chain.as_str())?;
                let txn_bytes = encoding::hex_to_bytes(&txn)?;
                let rlp = Rlp::new(&txn_bytes);
                let mut decoded_txn = TransactionRequest::decode_unsigned_rlp(&rlp)?;

                // set gas limit if none is passed, otherwise the txn call will fail
                if decoded_txn.gas.is_none()
                    || decoded_txn.gas.unwrap_or(U256::zero()) == U256::zero()
                {
                    // set 10 million gas limit.  chain gas limit is 30m on ethereum, but it used to be 10m.
                    decoded_txn = decoded_txn.gas(ethers::types::U256::from(10000000));
                }

                let typed_txn: TypedTransaction = decoded_txn.into();
                let result = provider.call_raw(&typed_txn).await;
                let result = match result {
                    Ok(r) => r,
                    Err(e) => {
                        error!("Error calling contract: {:?}", e);
                        return Err(e.into());
                    }
                };

                CallContractResponse {
                    result: format!("0x{}", bytes_to_hex(result)),
                }
                .into()
            }
            UnionResponse::CallChild(CallChildRequest { ipfs_id, params }) => {
                self.pay(LitActionPriceComponent::CallDepth, 1).await?;

                call_depth += 1;
                if call_depth > self.max_call_depth {
                    bail!(
                        "The recursion limit of a child action is {} and you have attempted to exceed that limit.",
                        self.max_call_depth
                    );
                }

                // Pull down the lit action code from IPFS
                let code = crate::utils::web::get_ipfs_file(
                    &ipfs_id,
                    self.lit_config(),
                    self.ipfs_cache()?,
                    self.http_cache()?,
                )
                .await?;

                let globals = params
                    .map(|params| serde_json::from_slice::<serde_json::Value>(&params))
                    .transpose()?;

                let auth_context = {
                    let mut ctx = auth_context.clone();
                    ctx.action_ipfs_id_stack.push(ipfs_id.clone());
                    ctx
                };

                // NB: Using execute_js_inner instead of execute_js to avoid resetting state
                let res = Box::pin(self.execute_js_inner(code, globals, &auth_context, call_depth))
                    .await?;

                CallChildResponse {
                    response: res.response,
                }
                .into()
            }
            UnionResponse::BroadcastAndCollect(BroadcastAndCollectRequest { name, value }) => {
                self.pay(LitActionPriceComponent::Broadcasts, 1).await?;

                self.increment_broad_and_collect_counter()?;

                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let txn_prefix = format!("{}_{}", txn_prefix, name);

                let tss_state = Arc::new(tss_state);
                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0", &self.node_set).await?;
                let values = cm
                    .broadcast_and_collect::<String, String>(value.clone())
                    .await?;

                let mut values: Vec<String> = values.into_iter().map(|(k, v)| v).collect();
                values.push(value);

                BroadcastAndCollectResponse { name, values }.into()
            }
            UnionResponse::DecryptAndCombine(DecryptAndCombineRequest {
                access_control_conditions,
                ciphertext,
                data_to_encrypt_hash,
                auth_sig,
                chain,
            }) => {
                self.increment_broad_and_collect_counter()?;
                self.pay(LitActionPriceComponent::Broadcasts, 1).await?;
                self.pay(LitActionPriceComponent::Decrypts, 1).await?;

                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let json_auth_sig = self.parse_json_authsig_helper(auth_sig)?;
                let txn_prefix = format!("{}_{}", txn_prefix, generate_hash(ciphertext.clone()));
                let tss_state = Arc::new(tss_state);

                let conditions: Vec<UnifiedAccessControlConditionItem> =
                    serde_json::from_slice(&access_control_conditions)?;

                if !self
                    .check_access_control_conditions_helper(
                        &conditions,
                        json_auth_sig,
                        chain,
                        action_ipfs_id,
                    )
                    .await?
                    .result
                {
                    bail!(
                        "Access control conditions check failed.  Check that you are allowed to decrypt this item."
                    );
                }

                let identity_parameter = get_identity_param(&conditions, &data_to_encrypt_hash)?;

                // Load the BLS secret key share as a blsful key for signing.
                let cipher_state = match tss_state.get_cipher_state(SigningScheme::Bls12381) {
                    Ok(cipher_state) => cipher_state,
                    Err(e) => {
                        bail!("Couldn't get BLS ciper state: {:?}", e);
                    }
                };

                // Sign the identity parameter using the blsful secret key share.
                let (signature_share, share_id) = match cipher_state
                    .sign(&identity_parameter, None, self.epoch)
                    .await
                {
                    Ok(signature_share) => signature_share,
                    Err(e) => {
                        bail!("Couldn't sign the identity parameter: {:?}", e);
                    }
                };

                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0", &self.node_set).await?;
                let mut shares = cm
                    .broadcast_and_collect::<SignatureShare<Bls12381G2Impl>, SignatureShare<Bls12381G2Impl>>(
                        signature_share,
                    )
                    .await?;

                shares.push((PeerId::ONE, signature_share)); // lazy - it's not zero, but we don't seem to care!

                let network_pubkey = get_bls_root_pubkey(&tss_state, None)?;
                let network_pubkey = blsful::PublicKey::try_from(&hex::decode(&network_pubkey)?)?;

                let serialized_decryption_shares =
                    shares.iter().map(|(_, share)| *share).collect::<Vec<_>>();
                let ciphertext = serde_bare::from_slice(&base64_decode(&ciphertext))?;

                let decrypted = lit_sdk::encryption::verify_and_decrypt_with_signatures_shares(
                    &network_pubkey,
                    &identity_parameter,
                    &ciphertext,
                    &serialized_decryption_shares,
                );

                let decrypted = match decrypted {
                    Ok(decrypted) => decrypted,
                    Err(e) => {
                        bail!("Failed to decrypt and combine: {:?}", e);
                    }
                };

                let result = match std::str::from_utf8(&decrypted) {
                    Ok(result) => result.to_string(),
                    Err(e) => {
                        bail!("Failed to convert decrypted bytes to string.")
                    }
                };

                DecryptAndCombineResponse { result }.into()
            }
            UnionResponse::DecryptToSingleNode(DecryptToSingleNodeRequest {
                access_control_conditions,
                ciphertext,
                data_to_encrypt_hash,
                auth_sig,
                chain,
            }) => {
                trace!("Ciphertext: {:?}", &ciphertext);

                self.increment_broad_and_collect_counter()?;
                self.pay(LitActionPriceComponent::Broadcasts, 1).await?;
                self.pay(LitActionPriceComponent::Decrypts, 1).await?;

                let json_auth_sig = self.parse_json_authsig_helper(auth_sig)?;

                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let request_hash = generate_hash(txn_prefix.clone());
                let tss_state = Arc::new(tss_state);

                let peers = tss_state.peer_state.peers();
                let peers = peers.peers_for_nodeset(&self.node_set);

                let (leader_addr, is_leader) = self.leader_helper(request_hash).await?;

                let txn_prefix = format!("{}_{}", txn_prefix, generate_hash(ciphertext.clone()));

                let conditions: Vec<UnifiedAccessControlConditionItem> =
                    serde_json::from_slice(&access_control_conditions)?;

                if !self
                    .check_access_control_conditions_helper(
                        &conditions,
                        json_auth_sig,
                        chain,
                        action_ipfs_id,
                    )
                    .await?
                    .result
                {
                    bail!(
                        "Access control conditions check failed.  Check that you are allowed to decrypt this item."
                    );
                }

                let identity_parameter = get_identity_param(&conditions, &data_to_encrypt_hash)?;

                // Load the BLS secret key share as a blsful key for signing.
                let cipher_state = match tss_state.get_cipher_state(SigningScheme::Bls12381) {
                    Ok(cipher_state) => cipher_state,
                    Err(e) => {
                        bail!("Couldn't get BLS ciper state: {:?}", e);
                    }
                };

                // Sign the identity parameter using the blsful secret key share.
                let (signature_share, share_index) = match cipher_state
                    .sign(&identity_parameter, None, self.epoch)
                    .await
                {
                    Ok(signature_share) => signature_share,
                    Err(e) => {
                        bail!("Couldn't sign the identity parameter: {:?}", e);
                    }
                };

                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0", &self.node_set).await?;
                let leader_peer = peers.peer_at_address(&leader_addr)?;

                let result = match is_leader {
                    false => {
                        let r = cm
                            .send_direct::<SignatureShare<Bls12381G2Impl>>(
                                &leader_peer,
                                signature_share,
                            )
                            .await?;
                        "".to_string() // empty string?
                    }
                    true => {
                        let expected_peers = peers.all_peers_except(&leader_peer.socket_address); // I'm the leader!

                        // let threshold = (crate::utils::consensus::get_threshold_count(peers.0.len())
                        //     as u16)
                        //     - 1;
                        let threshold = peers.0.len();
                        let mut shares = cm
                            .collect_from_earliest::<SignatureShare<Bls12381G2Impl>>(
                                &expected_peers,
                                threshold,
                            )
                            .await?;

                        shares.push((PeerId::ONE, signature_share)); // lazy - it's not zero, but we don't seem to care!

                        let network_pubkey = get_bls_root_pubkey(&tss_state, None)?;
                        let network_pubkey =
                            blsful::PublicKey::try_from(&hex::decode(&network_pubkey)?)?;

                        let serialized_decryption_shares =
                            shares.iter().map(|(_, share)| *share).collect::<Vec<_>>();
                        let ciphertext = serde_bare::from_slice(&base64_decode(&ciphertext))?;

                        let decrypted =
                            lit_sdk::encryption::verify_and_decrypt_with_signatures_shares(
                                &network_pubkey,
                                &identity_parameter,
                                &ciphertext,
                                &serialized_decryption_shares,
                            );

                        let decrypted = match decrypted {
                            Ok(decrypted) => decrypted,
                            Err(e) => {
                                bail!("Failed to decrypt and combine: {:?}", e);
                            }
                        };

                        let result = match std::str::from_utf8(&decrypted) {
                            Ok(result) => result.to_string(),
                            Err(e) => {
                                bail!("Failed to convert decrypted bytes to string.")
                            }
                        };

                        result
                    }
                };

                DecryptToSingleNodeResponse { result }.into()
            }
            UnionResponse::SignAndCombineEcdsa(SignAndCombineEcdsaRequest {
                to_sign,
                public_key,
                sig_name,
            }) => {
                // we both the signatures and the broadcasts for this operation.
                self.pay(LitActionPriceComponent::Signatures, 1).await?;
                self.pay(LitActionPriceComponent::Broadcasts, 1).await?;

                self.increment_broad_and_collect_counter()?;
                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let txn_prefix = format!("{}_combine_{}", txn_prefix, sig_name);
                let tss_state = Arc::new(tss_state);

                let result = self
                    .sign_helper(
                        to_sign.clone(),
                        public_key.clone(),
                        sig_name.clone(),
                        &[1], // AuthMethodScope::SignAnything
                        self.epoch,
                        action_ipfs_id,
                        SigningScheme::EcdsaK256Sha256,
                    )
                    .await?;

                // remove the signed data from the state if we find it.  May want to check scaling of borrow_mut after POC.
                let signed_data = self
                    .state
                    .signed_data
                    .borrow_mut()
                    .remove(&sig_name)
                    .expect_or_err("No signed data found")?;

                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0", &self.node_set).await?;
                let mut shares = cm
                    .broadcast_and_collect::<SignedData, SignedData>(signed_data.clone())
                    .await?
                    .into_iter()
                    .filter_map(|(_, share)| {
                        if let Ok(signature_share) =
                            serde_json::from_str::<SignableOutput>(&share.signature_share)
                        {
                            Some(signature_share)
                        } else {
                            error!("Empty share: {:?}", share);
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                if let Ok(signature_share) =
                    serde_json::from_str::<SignableOutput>(&signed_data.signature_share)
                {
                    shares.push(signature_share);
                }

                debug!(
                    "<<<<<<<<<<< share_count: {}, shares: {:?}",
                    shares.len(),
                    shares
                );

                // If combine_and_verify_signature_shares returns Ok then it's a valid signature
                let signed_output = combine_and_verify_signature_shares(&shares)?;
                let sig: k256::ecdsa::Signature = serde_json::from_str(&signed_output.signature)
                    .expect_or_err("Failed to parse signature")?;

                // done inline, as we may remove this code.
                #[derive(serde::Serialize, serde::Deserialize)]
                struct Rsv {
                    r: String,
                    s: String,
                    v: u8,
                }

                let result = serde_json::to_string(&Rsv {
                    r: sig.r().to_be_hex(),
                    s: sig.s().to_be_hex(),
                    v: signed_output
                        .recovery_id
                        .expect_or_err("No recovery id found")?,
                })
                .expect("unable to serialize");

                SignAndCombineEcdsaResponse { result }.into()
            }
            UnionResponse::SignAndCombine(SignAndCombineRequest {
                to_sign,
                public_key,
                sig_name,
                signing_scheme,
            }) => {
                // we both the signatures and the broadcasts for this operation.
                self.pay(LitActionPriceComponent::Signatures, 1).await?;
                self.pay(LitActionPriceComponent::Broadcasts, 1).await?;

                let scheme = signing_scheme
                    .parse::<SigningScheme>()
                    .map_err(|e| conversion_err(e, None))?;

                if scheme == SigningScheme::Bls12381 {
                    return Err(anyhow::Error::msg(format!(
                        "{} is not supported in this context. Use {} instead",
                        scheme,
                        SigningScheme::Bls12381G1ProofOfPossession
                    )));
                }
                self.increment_broad_and_collect_counter()?;
                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let txn_prefix = format!("{}_combine_{}", txn_prefix, signing_scheme);
                let tss_state = Arc::new(tss_state);

                let result = self
                    .sign_helper(
                        to_sign.clone(),
                        public_key,
                        sig_name.clone(),
                        &[1], // AuthMethodScope::SignAnything
                        self.epoch,
                        action_ipfs_id,
                        scheme,
                    )
                    .await?;

                // remove the signed data from the state if we find it.  May want to check scaling of borrow_mut after POC.
                let signed_data = self
                    .state
                    .signed_data
                    .borrow_mut()
                    .remove(&sig_name)
                    .expect_or_err("No signed data found")?;

                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0", &self.node_set).await?;
                let mut shares = cm
                    .broadcast_and_collect::<SignedData, SignedData>(signed_data.clone())
                    .await?
                    .into_iter()
                    .filter_map(|(_, share)| {
                        if let Ok(signature_share) =
                            serde_json::from_str::<SignableOutput>(&share.signature_share)
                        {
                            Some(signature_share)
                        } else {
                            error!("Empty share: {:?}", share);
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                if let Ok(signature_share) =
                    serde_json::from_str::<SignableOutput>(&signed_data.signature_share)
                {
                    shares.push(signature_share);
                }

                debug!(
                    "<<<<<<<<<<< share_count: {}, shares: {:?}",
                    shares.len(),
                    shares
                );

                // If combine_and_verify_signature_shares returns Ok then it's a valid signature
                let signed_output = combine_and_verify_signature_shares(&shares)?;

                let result = match scheme {
                    SigningScheme::EcdsaK256Sha256 => {
                        self.ecdsa_result::<k256::Secp256k1>(&signed_output)?
                    }
                    SigningScheme::EcdsaP256Sha256 => {
                        self.ecdsa_result::<p256::NistP256>(&signed_output)?
                    }
                    SigningScheme::EcdsaP384Sha384 => {
                        self.ecdsa_result::<p384::NistP384>(&signed_output)?
                    }
                    SigningScheme::SchnorrK256Sha256
                    | SigningScheme::SchnorrP256Sha256
                    | SigningScheme::SchnorrP384Sha384
                    | SigningScheme::SchnorrK256Taproot
                    | SigningScheme::SchnorrEd25519Sha512
                    | SigningScheme::SchnorrRistretto25519Sha512
                    | SigningScheme::SchnorrEd448Shake256
                    | SigningScheme::SchnorrRedJubjubBlake2b512
                    | SigningScheme::SchnorrRedDecaf377Blake2b512
                    | SigningScheme::SchnorrkelSubstrate => {
                        let frost_signature: lit_frost::Signature =
                            serde_json::from_str(&signed_output.signature)
                                .expect("unable to decode hex");
                        // Should be an even number of bytes
                        debug_assert_eq!(frost_signature.value.len() & 1, 0);
                        let mid = frost_signature.value.len() / 2;

                        #[derive(Serialize)]
                        struct LitActionFrostSignature {
                            r: String,
                            s: String,
                        }

                        serde_json::to_string(&LitActionFrostSignature {
                            r: hex::encode(&frost_signature.value[..mid]),
                            s: hex::encode(&frost_signature.value[mid..]),
                        })
                        .expect("unable to serialize")
                    }
                    SigningScheme::Bls12381G1ProofOfPossession => {
                        let mut value = BTreeMap::new();
                        value.insert("value", signed_output.signature);
                        serde_json::to_string(&value).expect("unable to serialize")
                    }
                    SigningScheme::Bls12381 => {
                        // shouldn't happen but just in case
                        unreachable!()
                    }
                };

                SignAndCombineResponse { result }.into()
            }
            UnionResponse::GetRpcUrl(GetRpcUrlRequest { chain }) => {
                let result = rpc_url(chain)
                    .unwrap_or_else(|e| format!("Error getting RPC URL: {:?}", e).to_string());
                GetRpcUrlResponse { result }.into()
            }

            UnionResponse::P2pBroadcast(P2pBroadcastRequest { name, value }) => {
                trace!("Starting Broadcast with name: {}", &name);
                self.pay(LitActionPriceComponent::Broadcasts, 1).await?;

                self.increment_broad_and_collect_counter()?;
                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let txn_prefix = format!("{}_{}", txn_prefix, name);
                let tss_state = Arc::new(tss_state);

                trace!(
                    "Broadcasting to all peers: {} with header {}",
                    &value, &txn_prefix
                );

                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0", &self.node_set).await?;
                let result = cm.broadcast::<String>(value.clone()).await?;

                P2pBroadcastResponse { result }.into()
            }

            UnionResponse::P2pCollectFromLeader(P2pCollectFromLeaderRequest { name }) => {
                trace!("Starting P2PCollect with name: {}", &name);

                self.pay(LitActionPriceComponent::Broadcasts, 1).await?;

                self.increment_broad_and_collect_counter()?;

                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let tss_state = Arc::new(tss_state);

                // note that the default leader function doesn't take a function parameter, thus we need to generate a hash from the transaction id only
                let request_hash = generate_hash(txn_prefix.clone());
                let txn_prefix = format!("{}_{}", txn_prefix, name);
                let (leader_addr, is_leader) = self.leader_helper(request_hash).await?;

                let peers = tss_state.peer_state.peers();
                let peers = peers.peers_for_nodeset(&self.node_set);

                let leader = peers
                    .peer_at_address(&leader_addr)
                    .expect_or_err("Leader address not in peer list")?;
                let from_peers = SimplePeerCollection(vec![leader.clone()]);

                let cm = CommsManager::new(&tss_state, 0, &txn_prefix, "0", &self.node_set).await?;
                let values = cm.collect_from::<String>(&from_peers).await?;

                let value = match values.is_empty() {
                    true => "".to_string(),
                    false => values[0].1.clone().to_string(),
                };

                P2pCollectFromLeaderResponse { name, value }.into()
            }

            UnionResponse::IsLeader(IsLeaderRequest {}) => {
                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let request_hash = generate_hash(txn_prefix.clone());
                let (leader_addr, result) = self.leader_helper(request_hash).await?;
                IsLeaderResponse { result }.into()
            }

            UnionResponse::EncryptBls(EncryptBlsRequest {
                access_control_conditions,
                to_encrypt,
            }) => {
                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let tss_state = Arc::new(tss_state);
                let network_pubkey = get_bls_root_pubkey(&tss_state, None)?;
                let network_pubkey = blsful::PublicKey::try_from(&hex::decode(&network_pubkey)?)?;

                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(&to_encrypt);
                let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

                let conditions: Vec<UnifiedAccessControlConditionItem> =
                    serde_json::from_slice(&access_control_conditions)?;

                let message_bytes = &to_encrypt;
                let identity_param = get_identity_param(&conditions, &data_to_encrypt_hash)?;

                let ciphertext = match lit_sdk::encryption::encrypt_time_lock(
                    &network_pubkey,
                    message_bytes,
                    &identity_param,
                ) {
                    Ok(ciphertext) => {
                        data_encoding::BASE64.encode(&serde_bare::to_vec(&ciphertext)?)
                    }
                    Err(e) => {
                        bail!("Failed to encrypt: {:?}", e);
                    }
                };

                EncryptBlsResponse {
                    ciphertext,
                    data_to_encrypt_hash,
                }
                .into()
            }
            UnionResponse::UpdateResourceUsage(UpdateResourceUsageRequest { tick, used_kb }) => {
                // For now, we'll just return a success response
                let r = self
                    .dynamic_payment
                    .add(LitActionPriceComponent::MemoryUsage, used_kb as u64);

                let cancel_action = r.is_err();

                UpdateResourceUsageResponse { cancel_action }.into()
            }
            UnionResponse::Result(_) => unreachable!(), // handled in main loop
            UnionResponse::SignAsAction(SignAsActionRequest {
                to_sign,
                sig_name,
                signing_scheme,
            }) => {
                debug!("sign_as_action");
                self.pay(LitActionPriceComponent::Signatures, 1).await?;
                self.pay(LitActionPriceComponent::Broadcasts, 1).await?;

                let scheme = signing_scheme
                    .parse::<SigningScheme>()
                    .map_err(|e| conversion_err(e, None))?;

                if scheme == SigningScheme::Bls12381 {
                    return Err(anyhow::Error::msg(format!(
                        "{} is not supported in this context. Use {} instead",
                        scheme,
                        SigningScheme::Bls12381G1ProofOfPossession
                    )));
                }
                self.increment_broad_and_collect_counter()?;

                let action_ipfs_id = action_ipfs_id.as_ref().ok_or_else(|| {
                    anyhow::Error::msg("No current action ipfs id is specified".to_string())
                })?;
                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let txn_prefix = format!("{}_signasaction_{}", txn_prefix, scheme);
                let tss_state = Arc::new(tss_state);

                self.state.sign_count += 1;
                if self.state.sign_count > self.max_sign_count {
                    bail!(
                        "You may not sign more than {} times per session and you have attempted to exceed that limit.",
                        self.max_sign_count,
                    );
                }

                let result = self
                    .sign_with_action(
                        &to_sign,
                        sig_name,
                        tss_state.clone(),
                        &txn_prefix,
                        action_ipfs_id,
                        scheme,
                    )
                    .await?;
                SignAsActionResponse { result }.into()
            }
            UnionResponse::GetActionPublicKey(GetActionPublicKeyRequest {
                signing_scheme,
                action_ipfs_cid,
            }) => {
                debug!(
                    "get_action_public_key: {}, {}",
                    signing_scheme, action_ipfs_cid
                );
                let scheme = signing_scheme
                    .parse::<SigningScheme>()
                    .map_err(|e| conversion_err(e, None))?;

                if scheme == SigningScheme::Bls12381 {
                    return Err(anyhow::Error::msg(format!(
                        "{} is not supported in this context. Use {} instead",
                        scheme,
                        SigningScheme::Bls12381G1ProofOfPossession
                    )));
                }

                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let txn_prefix = format!("{}_signasaction_{}", txn_prefix, scheme);
                let tss_state = Arc::new(tss_state);
                let curve_type = scheme.curve_type();
                let curve_state = CurveState::new(tss_state.peer_state.clone(), curve_type, None);
                let root_keys = curve_state.root_keys()?;
                let pubkey = lit_sdk::signature::get_lit_action_public_key(
                    scheme,
                    &action_ipfs_cid,
                    &root_keys,
                )?;
                GetActionPublicKeyResponse { result: pubkey }.into()
            }
            UnionResponse::VerifyActionSignature(VerifyActionSignatureRequest {
                signing_scheme,
                action_ipfs_cid,
                to_sign,
                sign_output,
            }) => {
                debug!(
                    "verify_action_signature: {}, {}, {}, {}",
                    signing_scheme,
                    action_ipfs_cid,
                    hex::encode(to_sign),
                    sign_output
                );
                let scheme = signing_scheme
                    .parse::<SigningScheme>()
                    .map_err(|e| conversion_err(e, None))?;

                if scheme == SigningScheme::Bls12381 {
                    return Err(anyhow::Error::msg(format!(
                        "{} is not supported in this context. Use {} instead",
                        scheme,
                        SigningScheme::Bls12381G1ProofOfPossession
                    )));
                }
                let curve_type = scheme.curve_type();

                let (tss_state, txn_prefix) = self.tss_state_and_txn_prefix()?;
                let txn_prefix = format!("{}_signasaction_{}", txn_prefix, scheme);
                let tss_state = Arc::new(tss_state);
                let curve_state = CurveState::new(tss_state.peer_state.clone(), curve_type, None);
                let root_keys = curve_state.root_keys()?;
                let pubkey = lit_sdk::signature::get_lit_action_public_key(
                    scheme,
                    &action_ipfs_cid,
                    &root_keys,
                )?;
                let output = serde_json::from_str::<SignedDataOutput>(&sign_output)?;
                if output.verifying_key != pubkey {
                    tracing::error!(
                        "action signature public key doesn't match what is expected: expected {} - received {}",
                        pubkey,
                        output.verifying_key
                    );
                    VerifyActionSignatureResponse { result: false }.into()
                } else {
                    match lit_sdk::signature::verify_signature(scheme, &output) {
                        Err(e) => {
                            tracing::error!("action signature verification failed {:?}", e);
                            VerifyActionSignatureResponse { result: false }.into()
                        }
                        Ok(_) => VerifyActionSignatureResponse { result: true }.into(),
                    }
                }
            }
        })
    }

    async fn pay(&mut self, price_component: LitActionPriceComponent, price: u64) -> Result<()> {
        if let Err(e) = self.dynamic_payment.add(price_component, price) {
            bail!(e);
        }
        Ok(())
    }

    fn parse_json_authsig_helper(&self, auth_sig: Option<Vec<u8>>) -> Result<JsonAuthSig> {
        match auth_sig {
            Some(auth_sig) => match serde_json::from_slice(&auth_sig)? {
                AuthSigItem::Single(auth_sig) => Ok(auth_sig),
                _ => bail!("Only supports Json AuthSig"),
            },
            None => match &self.auth_sig {
                Some(auth_sig) => Ok(auth_sig.clone()),
                None => bail!(
                    "No auth_sig found.  You must either pass one to the function, or set one in your client when sending the request"
                ),
            },
        }
    }

    fn increment_broad_and_collect_counter(&mut self) -> Result<()> {
        self.state.broadcast_and_collect_count += 1;
        if self.state.broadcast_and_collect_count > self.max_broadcast_and_collect_count {
            bail!(
                "You may not use broadcast and collect functionality more than {} times per session and you have attempted to exceed that limit.",
                self.max_broadcast_and_collect_count,
            );
        };
        Ok(())
    }

    pub fn get_identity_param(
        &self,
        conditions: &[UnifiedAccessControlConditionItem],
        data_to_encrypt_hash: &str,
    ) -> Result<Vec<u8>> {
        let hash_res = hash_access_control_conditions(RequestConditions {
            access_control_conditions: None,
            evm_contract_conditions: None,
            sol_rpc_conditions: None,
            unified_access_control_conditions: Some(conditions.to_vec()),
        });
        let hashed_access_control_conditions = match hash_res {
            Ok(hashed_access_control_conditions) => hashed_access_control_conditions,
            Err(e) => {
                bail!("Couldn't hash access control conditions: {:?}", e);
            }
        };

        let identity_param = AccessControlConditionResource::new(format!(
            "{}/{}",
            hashed_access_control_conditions, data_to_encrypt_hash
        ))
        .get_resource_key()
        .into_bytes();
        // Get the identity parameter to be signed.
        Ok(identity_param)
    }

    #[allow(clippy::too_many_arguments)]
    async fn sign_helper(
        &mut self,
        to_sign: Vec<u8>,
        pubkey: String,
        sig_name: String,
        required_scopes: &[usize],
        epoch: Option<u64>,
        action_ipfs_id: Option<String>,
        signing_scheme: SigningScheme,
    ) -> Result<String> {
        self.state.sign_count += 1;
        if self.state.sign_count > self.max_sign_count {
            bail!(
                "You may not sign more than {} times per session and you have attempted to exceed that limit.",
                self.max_sign_count,
            );
        }

        debug!(
            "sign_helper() called with to_sign: {:?}, pubkey: {}, sig_name: {}",
            bytes_to_hex(to_sign.clone()),
            pubkey,
            sig_name
        );

        let bls_root_pubkey = self.get_bls_root_pubkey().await?;

        // accept pubkey with and without 0x prefix
        let pubkey = pubkey.replace("0x", "");

        if self.auth_sig.is_none() {
            return Err(anyhow::anyhow!(
                "You can not sign without providing an auth_sig. You must create a session with the PKP, and then pass session sigs in, which will be converted to an auth sig per node. Refer the the docs on creating and using session sigs."
            ));
        }

        let result = pkp::utils::sign(
            self.lit_config(),
            &to_sign,
            pubkey,
            self.request_id(),
            action_ipfs_id,
            self.auth_sig.clone(),
            self.auth_context.clone(),
            self.js_env.tss_state.clone(),
            required_scopes,
            epoch,
            &bls_root_pubkey,
            &self.node_set,
            signing_scheme,
        )
        .await
        .map_err(|e| anyhow::anyhow!(format!("Failed to sign: {:?}", e)))?;

        debug!("Lit Action signing with {} complete.", signing_scheme);

        let mut padded_pubkey = match &result {
            SignableOutput::EcdsaSignedMessageShare(e) => e.public_key.clone(),
            SignableOutput::FrostSignedMessageShare(f) => f.public_key.clone(),
            SignableOutput::BlsSignedMessageShare(b) => b.public_key.clone(),
        };

        // pad the pubkey with a zero at the front if it's odd because hex strings should be even and zero padded
        if padded_pubkey.len() % 2 == 1 {
            padded_pubkey = format!("0{}", &padded_pubkey);
        }

        // this state is persisted across calls by deno, and so we can use it to
        // return data to the client that called this Lit function via HTTP
        self.state.signed_data.insert(
            sig_name.to_string(),
            SignedData {
                sig_type: signing_scheme.to_string(),
                signature_share: serde_json::to_string(&result)
                    .map_err(|e| unexpected_err(e, None))?,
                public_key: padded_pubkey,
                sig_name,
            },
        );

        Ok("success".to_string())
    }

    fn ecdsa_result<C>(&self, signed_output: &SignedDataOutput) -> Result<String>
    where
        C: PrimeCurve + CurveArithmetic,
        C::Scalar: BeHex,
        SignatureSize<C>: ArrayLength<u8>,
    {
        #[derive(Serialize)]
        struct Rsv {
            r: String,
            s: String,
            v: u8,
        }
        let sig: ecdsa::Signature<C> = serde_json::from_str(&signed_output.signature)
            .expect_or_err("Failed to parse signature")?;
        let result = serde_json::to_string(&Rsv {
            r: sig.r().to_be_hex(),
            s: sig.s().to_be_hex(),
            v: signed_output
                .recovery_id
                .expect_or_err("No recovery id found")?,
        })
        .expect_or_err("unable to serialize")?;
        Ok(result)
    }

    async fn check_access_control_conditions_helper(
        &mut self,
        conditions: &Vec<UnifiedAccessControlConditionItem>,
        json_auth_sig: JsonAuthSig,
        chain: String,
        action_ipfs_id: Option<String>,
    ) -> Result<UnifiedConditionCheckResult> {
        trace!("access_control_conditions: {:?}", &conditions);
        self.pay(LitActionPriceComponent::ContractCalls, 1).await?;

        let bls_root_pubkey = self.get_bls_root_pubkey().await?;
        let lit_action_resource = lit_node_core::LitActionResource::new("".to_string());

        crate::access_control::unified::check_access_control_conditions(
            conditions,
            &AuthSigItem::Single(json_auth_sig),
            Some(chain),
            &lit_action_resource.execution_ability(),
            self.js_env.cfg.clone(),
            &self.request_id(),
            &bls_root_pubkey,
            &self.endpoint_version,
            action_ipfs_id.as_ref(),
            self.ipfs_cache()?,
            self.http_cache()?,
        )
        .await
        .map_err(|e| anyhow::anyhow!(format!("Error checking access control conditions: {:?}", e)))
    }

    async fn get_bls_root_pubkey(&self) -> Result<String> {
        let tss_state = match &self.js_env.tss_state {
            Some(tss_state) => Arc::new(tss_state.clone()),
            None => {
                return Err(anyhow::anyhow!("No TSS state found"));
            }
        };
        get_bls_root_pubkey(&tss_state, None)
            .map_err(|e| anyhow::anyhow!(format!("Error getting BLS root pubkey: {:?}", e)))
    }

    async fn leader_helper(&self, request_hash: u64) -> Result<(String, bool)> {
        let tss_state = match &self.js_env.tss_state {
            Some(tss_state) => tss_state,
            None => {
                return Err(anyhow::anyhow!("No TSS state found"));
            }
        };
        let peers = tss_state.peer_state.peers();
        let peers = peers.peers_for_nodeset(&self.node_set);
        let addr = &tss_state.addr;
        let leader = peers.leader_for_active_peers(request_hash)?;

        let is_leader = addr == &leader.socket_address;

        Ok((leader.socket_address.clone(), is_leader))
    }

    #[allow(clippy::too_many_arguments)]
    async fn sign_with_action(
        &mut self,
        to_sign: &[u8],
        sig_name: String,
        tss_state: Arc<TssState>,
        txn_prefix: &str,
        action_ipfs_id: &str,
        signing_scheme: SigningScheme,
    ) -> Result<String> {
        debug!(
            "sign_with_action() to_sign: {}, sig_name: {}, signing_scheme: {}",
            hex::encode(to_sign),
            sig_name,
            signing_scheme
        );

        let curve_type = signing_scheme.curve_type();
        let mut sign_state = tss_state.get_signing_state(signing_scheme)?;
        let curve_state = CurveState::new(tss_state.peer_state.clone(), curve_type, None);
        let key_id = keccak256(format!("lit_action_{}", action_ipfs_id));
        let epoch = tss_state.get_keyshare_epoch().await;
        let pubkey = self
            .get_action_pubkey(tss_state.clone(), action_ipfs_id, None, signing_scheme)
            .await?;
        let my_result = sign_state
            .sign_with_pubkey(
                to_sign,
                pubkey,
                Some(key_id.to_vec()),
                self.request_id().as_bytes().to_vec(),
                None,
                Some(epoch),
                &self.node_set,
            )
            .await?;

        let mut padded_pubkey = match &my_result {
            SignableOutput::EcdsaSignedMessageShare(e) => e.public_key.clone(),
            SignableOutput::FrostSignedMessageShare(f) => f.public_key.clone(),
            SignableOutput::BlsSignedMessageShare(b) => b.public_key.clone(),
        };

        // pad the pubkey with a zero at the front if it's odd because hex strings should be even and zero padded
        if padded_pubkey.len() % 2 == 1 {
            padded_pubkey = format!("0{}", &padded_pubkey);
        }
        let signed_data = SignedData {
            sig_type: signing_scheme.to_string(),
            signature_share: serde_json::to_string(&my_result)
                .map_err(|e| unexpected_err(e, None))?,
            public_key: padded_pubkey.clone(),
            sig_name: sig_name.clone(),
        };
        let cm = CommsManager::new(&tss_state, 0, txn_prefix, "0", &self.node_set).await?;
        let mut shares = cm
            .broadcast_and_collect::<SignedData, SignedData>(signed_data.clone())
            .await?
            .into_iter()
            .filter_map(|(_, share)| {
                if let Ok(signature_share) =
                    serde_json::from_str::<SignableOutput>(&share.signature_share)
                {
                    Some(signature_share)
                } else {
                    error!("Empty share: {:?}", share);
                    None
                }
            })
            .collect::<Vec<_>>();
        shares.push(my_result.clone());

        info!(
            "<<<<<<<<<<< share_count: {}, shares: {:?}",
            shares.len(),
            shares
        );
        // If combine_and_verify_signature_shares returns Ok then it's a valid signature
        let signed_output = combine_and_verify_signature_shares(&shares)?;

        self.state.signed_data.insert(sig_name.clone(), signed_data);
        let result = serde_json::to_string(&signed_output)?;
        debug!("sign_with_action result: {}", result);
        Ok(result)
    }

    async fn get_action_pubkey(
        &self,
        tss_state: Arc<TssState>,
        action_ipfs_id: &str,
        key_set_id: Option<&str>,
        signing_scheme: SigningScheme,
    ) -> Result<Vec<u8>> {
        let pubkey = match signing_scheme {
            SigningScheme::Bls12381G1ProofOfPossession => CompressedBytes::to_compressed(
                &derive_ipfs_keys::<blsful::inner_types::G1Projective>(
                    tss_state,
                    action_ipfs_id,
                    key_set_id,
                    signing_scheme,
                )
                .await?
                .1,
            ),
            SigningScheme::EcdsaK256Sha256
            | SigningScheme::SchnorrK256Sha256
            | SigningScheme::SchnorrK256Taproot => derive_ipfs_keys::<k256::ProjectivePoint>(
                tss_state,
                action_ipfs_id,
                key_set_id,
                signing_scheme,
            )
            .await?
            .1
            .to_compressed(),
            SigningScheme::EcdsaP256Sha256 | SigningScheme::SchnorrP256Sha256 => {
                derive_ipfs_keys::<p256::ProjectivePoint>(
                    tss_state,
                    action_ipfs_id,
                    key_set_id,
                    signing_scheme,
                )
                .await?
                .1
                .to_compressed()
            }
            SigningScheme::EcdsaP384Sha384 | SigningScheme::SchnorrP384Sha384 => {
                derive_ipfs_keys::<p384::ProjectivePoint>(
                    tss_state,
                    action_ipfs_id,
                    key_set_id,
                    signing_scheme,
                )
                .await?
                .1
                .to_compressed()
            }
            SigningScheme::SchnorrEd25519Sha512 => {
                derive_ipfs_keys::<vsss_rs::curve25519::WrappedEdwards>(
                    tss_state,
                    action_ipfs_id,
                    key_set_id,
                    signing_scheme,
                )
                .await?
                .1
                .to_compressed()
            }
            SigningScheme::SchnorrRistretto25519Sha512 | SigningScheme::SchnorrkelSubstrate => {
                derive_ipfs_keys::<vsss_rs::curve25519::WrappedRistretto>(
                    tss_state,
                    action_ipfs_id,
                    key_set_id,
                    signing_scheme,
                )
                .await?
                .1
                .to_compressed()
            }
            SigningScheme::SchnorrEd448Shake256 => {
                derive_ipfs_keys::<ed448_goldilocks::EdwardsPoint>(
                    tss_state,
                    action_ipfs_id,
                    key_set_id,
                    signing_scheme,
                )
                .await?
                .1
                .to_compressed()
            }
            SigningScheme::SchnorrRedDecaf377Blake2b512 => derive_ipfs_keys::<decaf377::Element>(
                tss_state,
                action_ipfs_id,
                key_set_id,
                signing_scheme,
            )
            .await?
            .1
            .to_compressed(),
            SigningScheme::SchnorrRedJubjubBlake2b512 => derive_ipfs_keys::<jubjub::SubgroupPoint>(
                tss_state,
                action_ipfs_id,
                key_set_id,
                signing_scheme,
            )
            .await?
            .1
            .to_compressed(),
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported derive action pubkey signing scheme: {}",
                    signing_scheme
                ));
            }
        };
        Ok(pubkey)
    }
}

pub fn get_identity_param(
    conditions: &[UnifiedAccessControlConditionItem],
    data_to_encrypt_hash: &str,
) -> Result<Vec<u8>> {
    let hash_res = hash_access_control_conditions(RequestConditions {
        access_control_conditions: None,
        evm_contract_conditions: None,
        sol_rpc_conditions: None,
        unified_access_control_conditions: Some(conditions.to_vec()),
    });
    let hashed_access_control_conditions = match hash_res {
        Ok(hashed_access_control_conditions) => hashed_access_control_conditions,
        Err(e) => {
            bail!("Couldn't hash access control conditions: {:?}", e);
        }
    };

    let identity_param = AccessControlConditionResource::new(format!(
        "{}/{}",
        hashed_access_control_conditions, data_to_encrypt_hash
    ))
    .get_resource_key()
    .into_bytes();
    // Get the identity parameter to be signed.
    Ok(identity_param)
}

async fn derive_ipfs_keys<G>(
    tss_state: Arc<TssState>,
    action_ipfs_id: &str,
    key_set_id: Option<&str>,
    signing_scheme: SigningScheme,
) -> Result<(G::Scalar, G)>
where
    G: HDDerivable + GroupEncoding + Default + CompressedBytes,
    G::Scalar: HDDeriver + CompressedBytes,
{
    let key_id = keccak256(format!("lit_action_{}", action_ipfs_id));
    let curve_type = signing_scheme.curve_type();
    let curve_state = CurveState::new(
        tss_state.peer_state.clone(),
        curve_type,
        key_set_id.map(String::from),
    );
    let root_keys = curve_state.root_keys()?;
    let staker_address = &tss_state.peer_state.hex_staker_address();
    let peers = tss_state.peer_state.peers();
    let self_peer = peers.peer_at_address(&tss_state.addr)?;
    let epoch = tss_state.get_keyshare_epoch().await;
    let realm_id = tss_state.peer_state.realm_id();
    let deriver = <G::Scalar as HDDeriver>::create(&key_id, signing_scheme.id_sign_ctx());
    let (sk, vk) = get_derived_keyshare::<G>(
        deriver,
        &root_keys,
        curve_type,
        staker_address,
        &self_peer.peer_id,
        epoch,
        realm_id,
        &tss_state.key_cache,
    )
    .await?;
    Ok((sk, vk))
}
