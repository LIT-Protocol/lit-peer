use ethers::types::U256;
use std::str::FromStr;

use crate::error::{EC, Error, Result, parser_err_code};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PayedEndpoint {
    EncryptionSign,
    LitAction,
    PkpSign,
    SignSessionKey,
}

impl FromStr for PayedEndpoint {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "encryption_sign" => Ok(PayedEndpoint::EncryptionSign),
            "lit_action" => Ok(PayedEndpoint::LitAction),
            "pkp_sign" => Ok(PayedEndpoint::PkpSign),
            "sign_session_key" => Ok(PayedEndpoint::SignSessionKey),
            _ => Err(parser_err_code(
                "",
                EC::NodeSerializationError,
                Some(format!("`{}` is not a valid PayedEndpoint", s)),
            )),
        }
    }
}

impl PayedEndpoint {
    pub fn as_str(&self) -> &str {
        match self {
            PayedEndpoint::EncryptionSign => "encryption_sign",
            PayedEndpoint::LitAction => "lit_action",
            PayedEndpoint::PkpSign => "pkp_sign",
            PayedEndpoint::SignSessionKey => "sign_session_key",
        }
    }

    pub fn get_all_product_ids() -> Vec<U256> {
        vec![
            U256::from(u8::from(&PayedEndpoint::EncryptionSign)),
            U256::from(u8::from(&PayedEndpoint::LitAction)),
            U256::from(u8::from(&PayedEndpoint::PkpSign)),
            U256::from(u8::from(&PayedEndpoint::SignSessionKey)),
        ]
    }
}

impl From<&PayedEndpoint> for u8 {
    fn from(endpoint: &PayedEndpoint) -> u8 {
        match endpoint {
            PayedEndpoint::EncryptionSign => 0,
            PayedEndpoint::LitAction => 1,
            PayedEndpoint::PkpSign => 2,
            PayedEndpoint::SignSessionKey => 3,
        }
    }
}
