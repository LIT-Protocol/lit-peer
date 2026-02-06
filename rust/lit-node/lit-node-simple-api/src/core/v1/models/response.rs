use lit_node_core::response::JsonPKPSigningResponse;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetApiKeyResponse {
    pub api_key: String,
    pub wallet_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HandshakeResponse {
    pub responses: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)] 
pub struct MintPkpResponse {
    pub pkp_public_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignWithPkpResponse {
    pub shares: Vec<JsonPKPSigningResponse>,
    pub curve_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LitActionResponses {
    pub responses: Vec<LitActionResponse>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LitActionResponse {
    pub signatures: Vec<SignWithPkpResponse>,
    pub response: String,
    pub logs: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptResponse {
    pub ciphertext: String,
    pub data_to_encrypt_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptResponse {
    pub decrypted_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombineSignatureSharesResponse {
    pub signature: String,
    pub signed_data: String,
    pub verifying_key: String,
    pub r: String,
    pub s: String,
    pub recovery_id: u8,
}