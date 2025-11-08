use lit_node_core::{
    AccessControlConditionItem, EVMContractConditionItem, JsonAuthSig, SolRpcConditionItem,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ValidateConditionRequest {
    pub access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    pub evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    pub sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    pub chain: String,
    pub auth_sig: JsonAuthSig,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PKPKeyRequest {
    pub id: String,
    pub chain: String,
    pub key_type: String,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PKPKeyResponse {
    pub chain: String,
    pub public_key: String,
    pub signature: String,
    pub key_type: String,
    pub signature_r: ethers::types::U256,
    pub signature_s: ethers::types::U256,
    pub signature_v: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignWithPublicHashRequest {
    pub pubkey: Vec<u8>,
    pub auth_sig: JsonAuthSig,
    pub iat: u64,
    pub exp: u64,
}
