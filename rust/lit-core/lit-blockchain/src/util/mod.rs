use alloy::hex::FromHex;
use ethers::abi::ErrorExt;
use ethers::prelude::*;
use lit_core::utils::binary::hex_to_bytes;

use crate::{
    contracts::forwarder::FORWARDER_ABI,
    error::{Result, conversion_err},
};

pub mod ether;
pub mod release;

pub fn string_to_eth_address<S>(address: S) -> Result<Address>
where
    S: AsRef<str>,
{
    let bytes = hex_to_bytes(address.as_ref())?;
    if bytes.len() != Address::len_bytes() {
        return Err(conversion_err(
            format!(
                "eth address has incorrect length, expected {}, got {}",
                Address::len_bytes(),
                bytes.len()
            ),
            None,
        ));
    }
    Ok(Address::from_slice(bytes.as_slice()))
}

pub fn string_to_u256<S>(input_str: S) -> Result<U256>
where
    S: AsRef<str>,
{
    let input_str = input_str.as_ref();
    if let Some(stripped) = input_str.strip_prefix("0x") {
        U256::from_str_radix(stripped, 16).map_err(|e| conversion_err(e, None))
    } else {
        Ok(U256::from_dec_str(input_str).map_err(|e| conversion_err(e, None))?)
    }
}

pub fn decode_revert<M>(
    e: &ethers::prelude::ContractError<M>, abi: &ethers::abi::Contract,
) -> String
where
    M: ethers::providers::Middleware,
{
    let ethers_bytes = {
        // Handle middleware errors, especially from the EIP2771GasRelayerMiddleware
        match e.as_middleware_error() {
            Some(middleware_error) => {
                // At this point, middleware_error MAY be ContractError(Revert(Bytes)) as a result of the EIP2771GasRelayerMiddleware.
                // One viable way to extract the bytes and cast back to a ContractError(Revert(Bytes)) is to stringify the error.
                let stringified_error = middleware_error.to_string();
                // Strip off "Contract call reverted with data: "
                let maybe_revert_bytes_str =
                    stringified_error.strip_prefix("Contract call reverted with data: ");
                if let Some(revert_bytes_str) = maybe_revert_bytes_str {
                    // Check if the length of the string is even and >0.
                    if revert_bytes_str.len() % 2 == 0 && revert_bytes_str.len() > 0 {
                        // Convert to bytes
                        let revert_bytes = match Bytes::from_hex(revert_bytes_str) {
                            Ok(bytes) => bytes,
                            Err(conversion_err) => {
                                return format!(
                                    "Contract Error is not a revert error: {:?}",
                                    conversion_err
                                );
                            }
                        };

                        revert_bytes
                    } else {
                        return format!("Contract Error is not a revert error: {:?}", e);
                    }
                } else {
                    return format!("Contract Error is not a revert error: {:?}", e);
                }
            }
            None => match e.as_revert() {
                Some(bytes) => bytes.to_owned(),
                None => return format!("Contract Error is not a revert error: {:?}", e),
            },
        }
    };

    let bytes = ethers_bytes.as_ref();
    if bytes.len() < 4 {
        return "Not enough bytes for a contract signature in this revert".to_string();
    }

    let mut errors = abi.errors().collect::<Vec<_>>();

    // Also add errors from Forwarder contract
    let forwarder_errors = FORWARDER_ABI.errors().collect::<Vec<_>>();
    errors.extend(forwarder_errors);

    let mut signature = [0; 4];
    signature.copy_from_slice(&bytes[..4]);
    let data_bytes = &bytes[4..];

    let error = match errors.into_iter().find(|x| x.selector() == signature) {
        Some(error) => error,
        None => {
            let revert_reason_text_sig: [u8; 4] = [0x08, 0xc3, 0x79, 0xa0]; // the standard code for the revert reason just being a text string
            if signature == revert_reason_text_sig {
                let revert_reason_text = match String::from_utf8(data_bytes.to_vec()) {
                    Ok(text) => text,
                    Err(_) => {
                        return "Contract Error is a revert error with invalid UTF-8".to_string();
                    }
                };
                return format!(
                    "Contract Error is a revert error with reason: {}",
                    revert_reason_text
                );
            }
            return "Contract Error could not be decoded.".to_string();
        }
    };

    let input_data = match error.decode(data_bytes) {
        Ok(data) => data,
        Err(_) => return "Contract Error could not be decoded.".to_string(),
    };

    format!(
        "Contract error details; Error name: {:?} / Error data: {:?}",
        error.abi_signature(),
        input_data
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_eth_address() {
        assert_eq!(
            string_to_eth_address("0123456789abcdef0123456789abcdef01234567").unwrap().to_string(),
            "0x0123…4567"
        );
        assert_eq!(
            string_to_eth_address("0000").unwrap_err().to_string(),
            "conversion error: eth address has incorrect length, expected 20, got 2"
        );
        assert_eq!(
            string_to_eth_address("not-hex").unwrap_err().to_string(),
            "conversion error: failed to decode hex from str: Invalid character 'n' at position 1"
        );
    }
}
