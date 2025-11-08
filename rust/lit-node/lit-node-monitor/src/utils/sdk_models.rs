use serde::{Deserialize, Serialize};
use serde_bytes_base64::Bytes;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseWrapper {
    pub ok: bool,
    pub error: Option<String>,
    pub error_object: Option<String>,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSDKHandshakeResponse {
    pub server_public_key: String,
    pub subnet_public_key: String,
    pub network_public_key: String,
    pub network_public_key_set: String,
    pub client_sdk_version: String,
    pub hd_root_pubkeys: Vec<String>,
    pub attestation: Option<Attestation>,
    pub latest_blockhash: String,
    #[serde(default)]
    pub node_version: String,
    pub node_identity_key: String,
    pub epoch: u64,
    #[serde(default)]
    pub git_commit_hash: String,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(unused)]
pub enum AttestationType {
    AmdSevSnp,
    AdminSigned,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(unused)]
pub struct Attestation {
    #[serde(alias = "type", rename(serialize = "type"))]
    typ: AttestationType,
    noonce: Bytes,
    data: BTreeMap<String, Bytes>,
    signatures: Vec<Bytes>,
    report: Option<Bytes>,
    //  #[serde(skip)]
    // #[cfg(feature = "generate-via-service")]
    // session_id: Option<String>,
}
