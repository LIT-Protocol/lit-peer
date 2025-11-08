use crate::eth::*;
use bulletproofs::k256::ecdsa::SigningKey;
use serde::Serialize;

/// Borrowed from https://github.com/LIT-Protocol/lit-assets/blob/develop/rust/lit-node/src/auth/auth_material.rs#L161
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonAuthSig {
    pub sig: String,
    pub derived_via: String,
    pub signed_message: String,

    pub address: String,
    pub algo: Option<String>,
}

impl JsonAuthSig {
    pub fn new(signing_key: &SigningKey, signed_message: String) -> Self {
        let address = signing_key.to_eth_address();
        // let signed_message = format!(
        //     "lit-recovery wants you to sign in with your Ethereum account: {}
        //
        // URI: app://lit-recovery
        // Version: 1
        // Chain ID: {}
        // Nonce: 32891756
        // Issued At: {}
        // Resources:
        // - litNodeRecovery://*
        // ",
        //     address,
        //     CONTRACT_CHRONICLE_CHAIN_ID,
        //     Utc::now().to_string()
        // );

        let (signature, recovery_id) = signing_key.sign_siwe(signed_message.as_bytes());
        let mut buffer = [0u8; 65];
        buffer[..64].copy_from_slice(&signature.to_bytes());
        buffer[64] = recovery_id.to_byte();

        Self {
            sig: hex::encode(buffer),
            derived_via: "web3.eth.personal.sign".to_string(),
            signed_message,
            address,
            algo: None,
        }
    }
}

#[ignore]
#[test]
fn siwe_auth_test() {
    use cryptex::KeyRing;

    let mut keyring = cryptex::get_os_keyring(env!("CARGO_PKG_NAME")).unwrap();
    let secret = keyring.get_secret(crate::KEYRING_KEY_NAME).unwrap();
    let key = SigningKey::from_slice(secret.as_slice()).unwrap();
    let jwt = JsonAuthSig::new(&key, "".to_string());
    assert!(check_auth_sig(&jwt, "litNodeRecovery://*", &[]).is_ok());
}

#[cfg(test)]
pub(crate) fn check_auth_sig(
    auth_sig: &JsonAuthSig, _resource: &str, _valid_addresses: &[ethers::types::H160],
) -> Result<(), ()> {
    use std::str::FromStr;

    match ethers::types::Signature::from_str(&auth_sig.sig) {
        Ok(sig) => {
            let presented_address = ethers::types::Address::from_str(&auth_sig.address)
                .expect("Failed to parse auth sig address");

            assert!(sig.verify(auth_sig.signed_message.clone(), presented_address).is_ok());

            // match sig.recover(auth_sig.signed_message.clone()) {
            //     Ok(recovered_address) => {
            //         println!("presented_address: {:?}", presented_address);
            //         println!("recovered_address: {:?}", recovered_address);
            //
            //         if recovered_address != presented_address {
            //             return Err(validation_err_code(
            //                 "Recovered Address not equal to Presented Address",
            //                 EC::NodeAdminUnauthorized,
            //                 None,
            //             )
            //                 .add_source_to_details());
            //         }
            //
            //         if !valid_addresses.contains(&recovered_address) {
            //             return Err(validation_err_code(
            //                 "Recovered Address is not a Valid Address",
            //                 EC::NodeAdminUnauthorized,
            //                 None,
            //             )
            //                 .add_source_to_details());
            //         }
            //
            //         // check the signed message.  it must be a SIWE
            //         let message: Message = auth_sig.signed_message.parse().map_err(|e| {
            //             parser_err_code(
            //                 e,
            //                 EC::NodeAdminUnauthorized,
            //                 Some("Parse error on SIWE".into()),
            //             )
            //         })?;
            //
            //         // make sure litNodeAdmin resource is present
            //         let mut resource_present = false;
            //         for r in &message.resources {
            //             if r.as_str() == resource {
            //                 resource_present = true;
            //                 break;
            //             }
            //         }
            //
            //         if !resource_present {
            //             return Err(validation_err_code(
            //                 "Required resource of litNodeAdmin://* not found",
            //                 EC::NodeAdminUnauthorized,
            //                 None,
            //             )
            //                 .add_source_to_details());
            //         }
            //
            //         validate_siwe(&message)
            //             .map_err(|e| validation_err(e, Some("SIWE validation failed".into())))?;
            //
            //         let sig_as_array = encoding::hex_to_bytes(&auth_sig.sig)
            //             .map_err(|e| parser_err_code(e, EC::NodeSIWESigConversionError, None))?
            //             .try_into()
            //             .map_err(|_| {
            //                 conversion_err("Could not convert into fixed size slice", None)
            //             })?;
            //
            //         let verification_result =
            //             message.verify_eip191(&sig_as_array).map_err(|err| {
            //                 validation_err(err, Some("Error verifying SIWE signature".into()))
            //             })?;
            //
            //         println!(
            //             "SIWE verification result: {:?}",
            //             encoding::bytes_to_hex(&verification_result)
            //         );
            //
            //         // convert compressed pubkey to uncompressed
            //         let mut pubkey_fixed_length: [u8; 33] = [0; 33];
            //         pubkey_fixed_length.copy_from_slice(&verification_result);
            //         let pubkey =
            //             PublicKey::parse_compressed(&pubkey_fixed_length).map_err(|e| {
            //                 conversion_err_code(
            //                     e,
            //                     EC::NodeAdminUnauthorized,
            //                     Some("Error parsing pubkey".into()),
            //                 )
            //                     .add_msg_to_details()
            //             })?;
            //
            //         // convert verification_result public key to eth address
            //         let pubkey_hash = keccak256(&pubkey.serialize()[1..]);
            //         let mut pubkey_hash_fixed_length: [u8; 20] = [0; 20];
            //         pubkey_hash_fixed_length.copy_from_slice(&pubkey_hash[12..]);
            //         let siwe_recovered_address =
            //             ethers::types::Address::from_slice(&pubkey_hash_fixed_length);
            //         if siwe_recovered_address != presented_address {
            //             // Here we're not providing the full error as a detail to the user for security.
            //             return Err(validation_err_code(
            //                 "Authentication error: recovered address does not match presented address",
            //                 EC::NodeAdminUnauthorized,
            //                 None,
            //             ));
            //         }
            //
            //         Ok(())
            //     }
            //     Err(e) => {
            //         // Here we're not providing the full error as a detail to the user for security.
            //         Err(validation_err_code(
            //             e,
            //             EC::NodeAdminUnauthorized,
            //             Some("Invalid signature recovery".into()),
            //         ))
            //     }
            // }
            Ok(())
        }
        Err(_) => {
            // Here we're not providing the full error as a detail to the user for security.
            Err(())
        }
    }
}
