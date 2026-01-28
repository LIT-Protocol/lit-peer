//! Wrapper types for external dependencies that cannot derive ToSchema.
//!
//! These types provide OpenAPI schema representations for types from external crates
//! like `ethabi::Function` that don't implement utoipa's `ToSchema` trait.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Schema wrapper for `ethabi::Function`.
///
/// Represents the ABI definition of a smart contract function used in
/// EVM contract access control conditions.
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "balanceOf",
    "type": "function",
    "inputs": [{"name": "owner", "type": "address"}],
    "outputs": [{"name": "", "type": "uint256"}],
    "stateMutability": "view"
}))]
pub struct FunctionAbiSchema {
    /// The name of the function
    pub name: String,
    /// The type (always "function" for functions)
    #[serde(rename = "type")]
    pub type_: String,
    /// The input parameters
    pub inputs: Vec<AbiParam>,
    /// The output parameters
    pub outputs: Vec<AbiParam>,
    /// The state mutability (view, pure, nonpayable, payable)
    #[serde(rename = "stateMutability", skip_serializing_if = "Option::is_none")]
    pub state_mutability: Option<String>,
}

/// An ABI parameter definition.
#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AbiParam {
    /// The parameter name
    pub name: String,
    /// The parameter type (e.g., "address", "uint256", "bytes32")
    #[serde(rename = "type")]
    pub type_: String,
    /// For tuple types, the component parameters (recursive structure)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<Vec<Object>>)]
    pub components: Option<Vec<AbiParam>>,
    /// Whether this is an indexed parameter (for events)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
}
