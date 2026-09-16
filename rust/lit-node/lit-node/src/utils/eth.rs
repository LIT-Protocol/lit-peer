#![allow(dead_code)]

use crate::error::{Result, conversion_err};
use ethers::prelude::H160;
use ethers::types::Address;
use lit_rust_crypto::k256::ecdsa::{SigningKey, VerifyingKey};
use sha3::{Keccak256, digest::Digest};

pub trait EthereumAddress {
    fn to_eth_address_str(&self) -> Result<String> {
        let address = fmt_address(&self.to_eth_address()?.0)?;
        let mut buffer = String::new();
        buffer.push('0');
        buffer.push('x');
        buffer.push_str(&String::from_utf8(address.to_vec()).map_err(|e| {
            conversion_err(e, Some("Could not convert address to string".to_string()))
        })?);
        Ok(buffer)
    }

    fn to_eth_address(&self) -> Result<Address>;
}

impl EthereumAddress for VerifyingKey {
    fn to_eth_address(&self) -> Result<Address> {
        let pub_key_pt = self.to_encoded_point(false);
        let digest = keccak256(&pub_key_pt.as_bytes()[1..]);
        let last_20 = <[u8; 20]>::try_from(&digest[12..]).map_err(|e| {
            conversion_err(e, Some("Could not convert address to string".to_string()))
        })?;
        Ok(H160::from_slice(&last_20))
    }
}

impl EthereumAddress for SigningKey {
    fn to_eth_address(&self) -> Result<Address> {
        let public_key = self.verifying_key();
        public_key.to_eth_address()
    }
}

fn fmt_address(bytes: &[u8; 20]) -> Result<[u8; 40]> {
    let mut buffer = [0u8; 40];
    hex::encode_to_slice(bytes, &mut buffer)
        .map_err(|e| conversion_err(e, Some("Could not convert address to string".to_string())))?;

    let checksum = keccak256(&buffer);

    for i in 0..buffer.len() {
        let byte = checksum[i / 2];
        let nibble = 0xf & if i & 1 == 0 { byte >> 4 } else { byte };
        if nibble >= 8 {
            buffer[i] = buffer[i].to_ascii_uppercase();
        }
    }
    Ok(buffer)
}

fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::default();
    hasher.update(bytes);
    hasher.finalize().into()
}
