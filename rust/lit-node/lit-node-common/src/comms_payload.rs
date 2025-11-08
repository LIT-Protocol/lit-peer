use crate::coms_keys::ComsKeys;
use crate::error::{Result, unexpected_err};
use aes_gcm::aead::{Aead, Payload};
use aes_gcm::{AeadCore, Aes128Gcm, Key, KeyInit, Nonce};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Sha512, digest::Digest};
use x25519_dalek::{PublicKey, StaticSecret};
use zeroize::Zeroize;

#[derive(Debug, Clone)]
pub struct ComsUnencryptedPayload(Vec<u8>);

impl AsRef<[u8]> for ComsUnencryptedPayload {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl ComsUnencryptedPayload {
    pub fn from_object<T>(object: T) -> Result<Self>
    where
        T: Serialize,
    {
        let payload = postcard::to_stdvec(&object)
            .map_err(|_| unexpected_err("Failed to serialize payload", None))?;
        Ok(Self(payload))
    }

    pub fn into_object<D: DeserializeOwned>(self) -> Result<D> {
        postcard::from_bytes(&self.0)
            .map_err(|_| unexpected_err("Failed to deserialize payload", None))
    }

    pub fn encrypt(
        &self,
        my_coms_keys: &ComsKeys,
        recipient_public_key: &PublicKey,
    ) -> ComsEncryptedPayload {
        let aad = construct_aad(
            ComsEncryptedPayload::VERSION,
            recipient_public_key,
            &my_coms_keys.sender_public_key(),
        );

        let key = symm_key(
            my_coms_keys.my_sender_private_key(),
            recipient_public_key,
            aad.as_slice(),
        );
        let nonce = Aes128Gcm::generate_nonce(&mut OsRng);
        let payload = Payload {
            msg: self.0.as_slice(),
            aad: aad.as_slice(),
        };
        let cipher = Aes128Gcm::new(&key);
        let mut output = Vec::with_capacity(self.0.len() + ComsEncryptedPayload::OVERHEAD);
        output.extend_from_slice(ComsEncryptedPayload::VERSION.to_le_bytes().as_slice());
        output.extend_from_slice(nonce.as_slice());
        output.extend_from_slice(
            &cipher
                .encrypt(&nonce, payload)
                .expect("Failed to encrypt payload"),
        );

        ComsEncryptedPayload(output)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComsEncryptedPayload(Vec<u8>);

impl ComsEncryptedPayload {
    /// Current version of the ComsPayload
    pub const VERSION: u16 = 1;
    /// 2 for the version, 12 for the nonce, 16 for the AEAD tag
    pub const OVERHEAD: usize = 30;

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    pub fn version(&self) -> u16 {
        u16::from_le_bytes([self.0[0], self.0[1]])
    }

    pub fn nonce(&self) -> &[u8] {
        &self.0[2..14]
    }

    pub fn ciphertext(&self) -> &[u8] {
        &self.0[14..]
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn decrypt(
        &self,
        my_coms_keys: &ComsKeys,
        sender_public_key: &PublicKey,
    ) -> Result<ComsUnencryptedPayload> {
        if self.0.len() < Self::OVERHEAD {
            return Err(unexpected_err("Payload too short", None));
        }
        if self.version() != Self::VERSION {
            return Err(unexpected_err("Incorrect Version", None));
        }
        let aad = construct_aad(
            Self::VERSION,
            &my_coms_keys.receiver_public_key(),
            sender_public_key,
        );
        let key = symm_key(
            my_coms_keys.my_receiver_private_key(),
            sender_public_key,
            aad.as_slice(),
        );
        let nonce = Nonce::from_slice(self.nonce());
        let payload = Payload {
            msg: self.ciphertext(),
            aad: aad.as_slice(),
        };
        let cipher = Aes128Gcm::new(&key);
        let plaintext = cipher
            .decrypt(nonce, payload)
            .expect("Failed to encrypt payload");
        Ok(ComsUnencryptedPayload(plaintext))
    }
}

fn symm_key(
    my_private_key: &StaticSecret,
    their_public_key: &PublicKey,
    aad: &[u8],
) -> Key<Aes128Gcm> {
    let mut shared_secret = my_private_key.diffie_hellman(their_public_key);
    let mut h = Sha512::default();
    h.update(aad);
    h.update(shared_secret.as_bytes());
    let mut result = h.finalize();
    shared_secret.zeroize();
    let key = Key::<Aes128Gcm>::clone_from_slice(&result[..16]);
    result.zeroize();
    key
}

fn construct_aad(
    version: u16,
    recipient_public_key: &PublicKey,
    sender_public_key: &PublicKey,
) -> Vec<u8> {
    version
        .to_le_bytes()
        .into_iter()
        .chain(recipient_public_key.to_bytes())
        .chain(sender_public_key.to_bytes())
        .collect::<Vec<_>>()
}
