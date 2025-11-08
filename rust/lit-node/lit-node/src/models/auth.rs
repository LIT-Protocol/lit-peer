use super::JsonAuthSig;
use ethers::types::U256;
use lit_node_core::LitResourceAbilityRequest;
use serde::{Deserialize, Deserializer, Serialize, de::Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionKeySignedMessageV2 {
    pub session_key: String,
    pub resource_ability_requests: Vec<LitResourceAbilityRequest>,
    pub capabilities: Vec<JsonAuthSig>,
    pub issued_at: String,
    pub expiration: String,
    pub node_address: String,
    #[serde(deserialize_with = "from_str_to_u256")]
    pub max_price: U256,
}

fn from_str_to_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let result = match s.starts_with("0x") {
        true => U256::from_str_radix(&s[2..], 16).map_err(D::Error::custom),
        false => U256::from_str_radix(s, 10).map_err(D::Error::custom),
    };

    if result.is_err() {
        debug!(
            "Deserializing max_price '{}' to U256 failed: {:?}",
            s, result
        );
    }
    result
}
