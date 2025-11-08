use ethers::types::{Address, U256};

use crate::functions::action_client::ExecutionState;
use crate::functions::{JobId, JobStatus};
use iri_string::spec::UriSpec;
use iri_string::types::RiString;
use lit_blockchain::resolver::rpc::config::RpcConfig;
#[cfg(feature = "lit-actions")]
use lit_core::config::LitConfig;
use lit_node_core::{
    AccessControlConditionItem, AuthMethod, AuthSigItem, Blinders, CurveType,
    EVMContractConditionItem, JsonAuthSig, NodeSet, SolRpcConditionItem,
    UnifiedAccessControlConditionItem,
};
use lit_recovery::models::UploadedShareData;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
#[cfg(feature = "lit-actions")]
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use web3::types::{Bytes, CallRequest};
use webauthn_rs_core::proto::PublicKeyCredential;

pub mod auth;
pub mod siwe;
pub mod webauthn_signature_verification_material;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonAdminSetBlindersRequest {
    pub auth_sig: JsonAuthSig,
    pub blinders: Blinders,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonAdminSetRequest {
    pub new_config: HashMap<String, String>,
    pub rpc_config: RpcConfig,
    pub auth_sig: JsonAuthSig,
}

// DATIL_BACKUP: Remove this struct once old Datil backup is obsolete.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonRecoverySetDecShare {
    pub auth_sig: JsonAuthSig,
    pub share_data: UploadedShareData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonRecoverySetDecShares {
    pub auth_sig: JsonAuthSig,
    pub share_data: Vec<UploadedShareData>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadShareRequest {
    pub auth_sig: JsonAuthSig, // auth sig from the backup party member
}

pub struct AllowlistCache {
    pub entries: RwLock<HashMap<[u8; 32], AllowlistEntry>>,
}

pub struct AllowlistEntry {
    pub allowed: bool,
}

pub struct AuthContextCache {
    pub auth_contexts: moka::future::Cache<String, AuthMethodResponse>,
}
pub struct AuthContextCacheExpiry;
impl moka::Expiry<String, AuthMethodResponse> for AuthContextCacheExpiry {
    /// Returns the duration of the expiration of the value that was just
    /// created.
    fn expire_after_create(
        &self,
        _key: &String,
        value: &AuthMethodResponse,
        _current_time: std::time::Instant,
    ) -> Option<Duration> {
        let exp = value.expiration;
        trace!("{}", exp);
        Some(Duration::new(exp as u64, 0))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthContext {
    pub action_ipfs_id_stack: Vec<String>,
    pub auth_sig_address: Option<String>,
    pub auth_method_contexts: Vec<AuthMethodResponse>,
    pub resources: Vec<RiString<UriSpec>>,
    pub custom_auth_resource: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthMethodResponse {
    pub user_id: String,
    pub app_id: String,
    pub auth_method_type: u32,
    #[serde(skip_serializing)]
    // Since it will be different for every node & which will cause issues for electing the BT leader and signing
    pub last_retrieved_at: SystemTime,
    #[serde(skip_serializing)]
    // Since it will be different for every node & which will cause issues for electing the BT leader and signing
    pub expiration: i64,
    pub used_for_sign_session_key_request: bool,
}

#[cfg(feature = "lit-actions")]
#[derive(Debug, Clone, Default)]
pub struct DenoExecutionEnv {
    pub tss_state: Option<crate::tss::common::tss_state::TssState>,
    pub cfg: Arc<LitConfig>,
    pub ipfs_cache: Option<Cache<String, Arc<String>>>,
    pub http_client: Option<reqwest::Client>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyRequest {
    pub session_key: String,
    pub auth_methods: Vec<AuthMethod>,
    pub pkp_public_key: Option<String>,
    pub auth_sig: Option<AuthSigItem>,
    pub siwe_message: String,
    #[serde(default = "default_epoch")]
    pub epoch: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonSignSessionKeyRequestV1 {
    pub session_key: String,
    pub auth_methods: Vec<AuthMethod>,
    pub pkp_public_key: Option<String>,
    pub auth_sig: Option<AuthSigItem>, // For backwards compatibility
    pub siwe_message: String,
    pub curve_type: CurveType,
    pub code: Option<String>,
    pub lit_action_ipfs_id: Option<String>,
    pub js_params: Option<Value>,
    #[serde(default = "default_epoch")]
    pub epoch: u64,
    pub node_set: Vec<NodeSet>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonJobStatusRequest {
    pub job_id: JobId,
    pub auth_sig: AuthSigItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonJobStatusResponse {
    pub id: JobId,
    pub status: JobStatus,
    #[serde(flatten)]
    pub result: Option<JsonJobResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum JsonJobResult {
    Success {
        #[serde(flatten)]
        state: ExecutionState,
    },
    Error {
        error: String,
    },
}

fn default_epoch() -> u64 {
    0 // this will indicate to the nodes that a valid value isn't coming from the SDK.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestConditions {
    pub access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    pub evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    pub sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    pub unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnifiedConditionCheckResult {
    pub result: bool,
    pub successful_auth_sig: JsonAuthSig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoapEvent {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoapEntry {
    pub event: PoapEvent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtSignedChainDataPayload {
    pub iss: String,
    pub chain: String,
    pub iat: u64,
    pub exp: u64,
    pub call_requests: Vec<CallRequest>,
    pub call_responses: Vec<Bytes>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonSigningResourceId {
    pub base_url: String,
    pub path: String,
    pub org_id: String,
    pub role: String,
    pub extra_data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SigningAccessControlConditionRequest {
    pub access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    pub evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    pub sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    pub unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
    pub chain: Option<String>,
    pub auth_sig: AuthSigItem,
    pub iat: u64,
    pub exp: u64,
    #[serde(default = "default_epoch")]
    pub epoch: u64,
}

/* accessControlConditions looks like this:
accessControlConditions: [
{
contractAddress: tokenAddress,
chain: 'ethereum',
standardContractType: 'ERC1155',
method: 'balanceOf',
parameters: [
':userAddress',
tokenId
      ],
      returnValueTest: {
        comparator: '>',
        value: 0
      }
    }
  ]
*/

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtHeader {
    pub alg: String,
    pub typ: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    pub iss: String,
    pub sub: String,
    pub chain: String,
    pub iat: u64,
    pub exp: u64,
    pub base_url: String,
    pub path: String,
    pub org_id: String,
    pub role: String,
    pub extra_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayloadV2 {
    pub iss: String,
    pub sub: String,
    pub chain: Option<String>,
    pub iat: u64,
    pub exp: u64,
    pub access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    pub evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    pub sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    pub unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecoveryShare {
    pub recovery_share: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonRecoveryShareResponse {
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerValidator {
    pub ip: u32,
    pub ipv6: u128,
    pub port: u32,
    pub address: Address,
    pub reward: U256,
    pub coms_sender_pub_key: U256,
    pub coms_receiver_pub_key: U256,
    pub index: u16,
    pub staker_address: Address,
    pub socket_addr: String,
    pub key_hash: u64,
    pub wallet_public_key: Vec<u8>,
    pub is_kicked: bool,
    pub version: String,
    pub realm_id: U256,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebAuthnAuthenticationRequest {
    pub credential: PublicKeyCredential,
    pub session_pubkey: String,
    pub siwe_message: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EthBlock {
    pub blockhash: String,
    pub timestamp: String,
    pub block_number: usize,
}

#[test]
fn serialize() {
    use lit_node_core::response::JsonSignSessionKeyResponseV2;
    let input = r#"{"ok":true,"error":null,"errorObject":null,"data":{"result":"success","signatureShare":{"ProofOfPossession":{"identifier":"0d7c3d1837353b3d50763bcb295951c339051be08de715f3396b9a7820512326","value":"88fa7225bd8f29e1961fb9dded561c0924553cdb2fd0b685c5d87e2ca0e062e84df0088a4b082c92fa4e9a71102634450010cd2a9bc1168aae6e08f1d85abdad1e492572f481b5d9960c757682faef9a294ebb147971c0bb218c59aeb0ea7b81"}},"shareId":"5e275960e8f09ff1c9abe0eff5149aaa260e76f09cb9cbccdcfbadcc5fa4f241","curveType":"BLS12381G1","siweMessage":"localhost:3000 wants you to sign in with your Ethereum account:\n0x9ADc989FeDCEE4666816a2630BcF340c6bCD7210\n\nI am delegating to a session key I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'. I further authorize the stated URI to perform the following actions on my behalf: (1) '*': '*' for 'lit-litaction://*'. (2) 'Auth': 'Auth' for 'lit-resolvedauthcontext://*'.\n\nURI: lit:session:bc13ae76387c640ea0b02ccaab0a313274df62d6b8d41047723a82a8eeff9b2a\nVersion: 1\nChain ID: 1\nNonce: 0x6f0a76a256deb7f7716ffabf26e6612ae497dc30cb238edbd89df1e78dc429f0\nIssued At: 2025-05-07T16:12:23Z\nExpiration Time: 2025-05-14T16:12:41.068Z\nResources:\n- urn:recap:eyJhdHQiOnsibGl0LWxpdGFjdGlvbjovLyoiOnsiKi8qIjpbe31dfSwibGl0LXJlc29sdmVkYXV0aGNvbnRleHQ6Ly8qIjp7IkF1dGgvQXV0aCI6W3siYXV0aF9jb250ZXh0Ijp7ImFjdGlvbklwZnNJZHMiOltdLCJhdXRoTWV0aG9kQ29udGV4dHMiOlt7ImFwcElkIjoibGl0IiwiYXV0aE1ldGhvZFR5cGUiOjEsInVzZWRGb3JTaWduU2Vzc2lvbktleVJlcXVlc3QiOnRydWUsInVzZXJJZCI6IjB4QTREMTIxRTA4ZjEyNmQyQjdmM2VkMjE4MGZDQTk5MUU0NzkyY2UyNCJ9XSwiYXV0aFNpZ0FkZHJlc3MiOm51bGwsImN1c3RvbUF1dGhSZXNvdXJjZSI6IiIsInJlc291cmNlcyI6W119fV19fSwicHJmIjpbXX0","dataSigned":"c30637f8565daaf59e5c2761cb76d4db6c1aa6790755b5f6f6da0a03631bef01","blsRootPubkey":"a380be9eb533a382fe6c33edce6da3dd97aefca7ce45f910bdcac214002966281eb012aa542c1387337099f8fb92f7fa"}}"#;
    let deserd = serde_json::from_str::<
        lit_node_core::response::GenericResponse<JsonSignSessionKeyResponseV2>,
    >(input);
    assert!(deserd.is_ok());
}
