use lit_rust_crypto::k256::ecdsa::{RecoveryId, Signature, SigningKey, VerifyingKey};
use sha3::{Keccak256, digest::Digest};

pub trait EthereumAddress {
    fn to_eth_address(&self) -> String;
}

// pub trait EthereumSignature {
//     fn sign_eth(&self, pre_hash: &[u8]) -> (Signature, RecoveryId);
// }

pub trait SiweSignature {
    fn sign_siwe(&self, pre_hash: &[u8]) -> (Signature, RecoveryId);
}

impl EthereumAddress for VerifyingKey {
    fn to_eth_address(&self) -> String {
        let pub_key_pt = self.to_encoded_point(false);
        let digest = keccak256(&pub_key_pt.as_bytes()[1..]);
        let last_20 = <[u8; 20]>::try_from(&digest[12..]).unwrap();
        let address = fmt_address(&last_20);
        let mut buffer = String::new();
        buffer.push('0');
        buffer.push('x');
        buffer.push_str(core::str::from_utf8(&address).expect("invalid utf8"));
        buffer
    }
}

impl EthereumAddress for SigningKey {
    fn to_eth_address(&self) -> String {
        let public_key = self.verifying_key();
        public_key.to_eth_address()
    }
}

// impl EthereumSignature for SigningKey {
//     fn sign_eth(&self, message: &[u8]) -> (Signature, RecoveryId) {
//         let digest = keccak256(message);
//         self.sign_prehash_recoverable(&digest).unwrap()
//     }
// }

impl SiweSignature for SigningKey {
    fn sign_siwe(&self, pre_hash: &[u8]) -> (Signature, RecoveryId) {
        const PREFIX: &str = "\x19Ethereum Signed Message:\n";
        let mut hasher = Keccak256::default();
        let len = pre_hash.len();
        let len_str = len.to_string();
        hasher.update(PREFIX.as_bytes());
        hasher.update(len_str.as_bytes());
        hasher.update(pre_hash);
        let digest = hasher.finalize();
        self.sign_prehash_recoverable(&digest).unwrap()
    }
}

fn fmt_address(bytes: &[u8; 20]) -> [u8; 40] {
    let mut buffer = [0u8; 40];
    hex::encode_to_slice(bytes, &mut buffer).unwrap();

    let checksum = keccak256(&buffer);

    for i in 0..buffer.len() {
        let byte = checksum[i / 2];
        let nibble = 0xf & if i & 1 == 0 { byte >> 4 } else { byte };
        if nibble >= 8 {
            buffer[i] = buffer[i].to_ascii_uppercase();
        }
    }
    buffer
}

fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::default();
    hasher.update(bytes);
    hasher.finalize().into()
}
