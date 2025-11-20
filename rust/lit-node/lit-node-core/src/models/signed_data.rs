use serde::{Deserialize, Serialize};

/// Data representing a signature share from a distributed signing operation.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedData {
    /// The type of signature (e.g., "ECDSA", "BLS").
    pub sig_type: String,
    /// The signature share value (hex-encoded).
    pub signature_share: String,
    /// The public key associated with this signature (hex-encoded).
    pub public_key: String,
    /// Human-readable name for this signature.
    pub sig_name: String,
}
