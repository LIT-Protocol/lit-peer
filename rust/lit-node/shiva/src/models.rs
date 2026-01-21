use rocket::serde::{Deserialize, Serialize};

use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TestNetCreateRequest {
    pub node_count: usize,
    pub polling_interval: String,
    pub epoch_length: i32,
    pub custom_build_path: Option<String>,
    pub lit_action_server_custom_build_path: Option<String>,
    pub existing_config_path: Option<String>,
    pub which: Option<String>,
    pub ecdsa_round_timeout: Option<String>,
    pub enable_payment: Option<String>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TestNetResponse<T> {
    pub testnet_id: String,
    pub command: TestNetClientCommand,
    pub was_canceled: bool,
    pub body: Option<T>,
    pub last_state_observed: Option<String>,
    pub messages: Option<Vec<String>>,
    pub errors: Option<Vec<String>>,
}

impl<T> Default for TestNetResponse<T> {
    fn default() -> Self {
        Self {
            testnet_id: "".to_string(),
            command: TestNetClientCommand::Unknown,
            was_canceled: false,
            body: None,
            last_state_observed: None,
            messages: None,
            errors: None,
        }
    }
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractAddresses {
    pub lit_token: String,
    pub backup_recovery: String,
    pub staking: String,
    pub pkpnft: String,
    pub pubkey_router: String,
    pub pkp_permissions: String,
    pub pkp_helper: String,
    pub contract_resolver: String,
    pub key_deriver: String,
    pub payment_delegation: String,
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestNetInfo {
    pub contract_addresses: ContractAddresses,
    pub validator_addresses: Vec<String>,
    pub contract_resolver_abi: String,
    pub rpc_url: String,
    pub epoch_length: i32,
    pub contract_abis: ContractAbis,
}

pub enum TestNetCommand {
    GetInfo(flume::Sender<Option<TestNetInfo>>),
    StopRandom(flume::Sender<bool>),
    StopRandomAndWait(flume::Sender<bool>),
    Shutdown(flume::Sender<bool>),
    TransitionEpochAndWait(flume::Sender<bool>),
}

#[derive(Debug, Clone)]
pub struct TestNetCreateParams {
    pub uuid: String,
    pub node_count: usize,
    pub polling_interval: String,
    pub epoch_length: i32,
    pub existing_config_path: Option<String>,
    pub which: Option<String>,
    pub ecdsa_round_timeout: Option<String>,
    pub enable_payment: Option<String>,
    pub custom_build_path: Option<String>,
    pub lit_action_server_custom_build_path: Option<String>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, PartialEq)]
pub enum TestNetState {
    Busy,
    Active,
    Mutating,
    Shutdown,
    Term,
    UNKNOWN,
}

#[derive(Debug, Clone)]
pub struct TestnetHandler {
    pub state: TestNetState,
    pub channel: flume::Sender<TestNetCommand>,
    pub term_channel: flume::Sender<bool>,
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractAbis {
    pub lit_token: String,
    pub erc20: String,
    pub backup_recovery: String,
    pub staking: String,
    pub pkpnft: String,
    pub pubkey_router: String,
    pub pkp_permissions: String,
    pub pkp_helper: String,
    pub contract_resolver: String,
    pub payment_delegation: String,
}

#[derive(TS)]
#[ts(export)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TestNetClientCommand {
    CreateTestnet,
    Shutdown,
    Poke,
    GetInfo,
    GetTestnets,
    StopRandom,
    StopRandomAndWait,
    TransitionEpochAndWait,
    Unknown,
}
