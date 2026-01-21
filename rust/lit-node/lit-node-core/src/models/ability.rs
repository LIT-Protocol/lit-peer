use serde::{Deserialize, Serialize};
use std::fmt;

/// Abilities that can be granted via authentication signatures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LitAbility {
    /// Ability to decrypt data protected by access control conditions.
    AccessControlConditionDecryption,
    /// Ability to sign data protected by access control conditions.
    AccessControlConditionSigning,
    /// Ability to use PKP (Programmable Key Pair) for signing.
    PKPSigning,
    /// Ability to execute Lit Actions (serverless functions).
    LitActionExecution,
    /// Ability to delegate payment for operations.
    PaymentDelegationAuth,
}

impl fmt::Display for LitAbility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LitAbility::AccessControlConditionDecryption => {
                write!(f, "access-control-condition-decryption")
            }
            LitAbility::AccessControlConditionSigning => {
                write!(f, "access-control-condition-signing")
            }
            LitAbility::PKPSigning => write!(f, "pkp-signing"),
            LitAbility::LitActionExecution => write!(f, "lit-action-execution"),
            LitAbility::PaymentDelegationAuth => write!(f, "lit-payment-delegation"),
        }
    }
}
