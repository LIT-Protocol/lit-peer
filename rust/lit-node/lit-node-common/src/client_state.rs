use crate::error::unexpected_err;
use lit_sdk::EncryptedPayload;
use rand_core::{OsRng, RngCore};
use sdd::{AtomicOwned, Guard, Owned, Tag};
use serde::Serialize;
use serde::de::DeserializeOwned;
use soteria_rs::{DEFAULT_BUF_SIZE, Protected};
use std::fmt::{Debug, Formatter};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

/// Manages the ephemeral key state of the nodes and the clients
pub struct ClientState {
    identity_keys: AtomicOwned<IdentityKeys>,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            identity_keys: AtomicOwned::new(IdentityKeys::default()),
        }
    }
}

impl Debug for ClientState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let guard = Guard::new();
        let identity_keys = self
            .identity_keys
            .load(Ordering::Acquire, &guard)
            .as_ref()
            .ok_or(std::fmt::Error)?;

        f.debug_struct("ClientState")
            .field("identity_keys", identity_keys)
            .finish()
    }
}

impl ClientState {
    pub fn json_encrypt<I>(
        &self,
        identity_to_use: IdentityKey,
        their_public_key: &[u8; 32],
        msg: I,
    ) -> lit_core::error::Result<EncryptedPayload<I>>
    where
        I: Serialize + DeserializeOwned + Sync,
    {
        let guard = Guard::new();
        let identity_keys = self.identity_keys(&guard);
        match identity_to_use {
            IdentityKey::Current => {
                let payload = identity_keys.current.json_encrypt(their_public_key, &msg)?;
                Ok(payload)
            }
            IdentityKey::Previous => {
                if let Some(previous_identity_keys) = &identity_keys.previous {
                    let payload = previous_identity_keys.json_encrypt(their_public_key, &msg)?;
                    Ok(payload)
                } else {
                    Err(unexpected_err("Invalid `identity_key_to_use`, tried to use previous, but previous identity key is not set".to_string(), None))
                }
            }
        }
    }

    pub fn json_decrypt<I>(
        &self,
        payload: &EncryptedPayload<I>,
    ) -> lit_core::error::Result<(I, IdentityKey, [u8; 32])>
    where
        I: Serialize + DeserializeOwned + Sync,
    {
        let guard = Guard::new();
        let identity_keys = self.identity_keys(&guard);
        match identity_keys.current.json_decrypt(payload) {
            Ok((msg, their_public_key)) => Ok((msg, IdentityKey::Current, their_public_key)),
            Err(_) => {
                // Try to decrypt with the previous identity keys
                if let Some(previous_identity_keys) = &identity_keys.previous {
                    previous_identity_keys
                        .json_decrypt(payload)
                        .map(|(m, k)| (m, IdentityKey::Previous, k))
                } else {
                    Err(unexpected_err(
                        "No previous identity keys loaded".to_string(),
                        None,
                    ))
                }
            }
        }
    }

    pub fn rotate_identity_keys(&self) {
        let guard = Guard::new();
        let identity_keys = self.identity_keys(&guard);
        let new_identity_keys = Owned::new(identity_keys.rotate());
        self.identity_keys
            .swap((Some(new_identity_keys), Tag::None), Ordering::Release);
    }

    pub fn get_current_identity_public_key(&self) -> [u8; 32] {
        let guard = Guard::new();
        self.identity_keys(&guard).current.public_key
    }

    pub fn get_current_identity_public_key_hex(&self) -> String {
        hex::encode(self.get_current_identity_public_key())
    }

    pub fn get_previous_identity_public_key(&self) -> Option<[u8; 32]> {
        let guard = Guard::new();
        self.identity_keys(&guard)
            .previous
            .as_ref()
            .map(|k| k.public_key)
    }

    pub fn get_previous_identity_public_key_hex(&self) -> Option<String> {
        self.get_previous_identity_public_key().map(hex::encode)
    }

    fn identity_keys<'a>(&self, guard: &'a Guard) -> &'a IdentityKeys {
        self.identity_keys
            .load(Ordering::Acquire, guard)
            .as_ref()
            .expect("identity_keys to always be set")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IdentityKey {
    Current,
    Previous,
}

impl IdentityKey {
    pub fn is_current(&self) -> bool {
        matches!(self, IdentityKey::Current)
    }

    pub fn is_previous(&self) -> bool {
        matches!(self, IdentityKey::Previous)
    }
}

#[derive(Debug, Clone)]
pub struct IdentityKeys {
    current: KeyPair,
    previous: Option<KeyPair>,
}

impl IdentityKeys {
    pub fn rotate(&self) -> Self {
        Self {
            current: KeyPair::generate(),
            previous: Some(self.current.clone()),
        }
    }
}

impl Default for IdentityKeys {
    fn default() -> Self {
        Self {
            current: KeyPair::generate(),
            previous: None,
        }
    }
}

#[derive(Clone)]
pub struct KeyPair {
    pub public_key: [u8; 32],
    pub secret_key: Arc<Mutex<Protected<DEFAULT_BUF_SIZE>>>,
}

impl Debug for KeyPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyPair")
            .field("public_key", &hex::encode(self.public_key))
            .field("secret_key", &"********")
            .finish()
    }
}

impl KeyPair {
    pub fn generate() -> Self {
        let mut seed = [0u8; 32];
        OsRng.fill_bytes(&mut seed);

        let mut sk = [0u8; 32];
        let mut pk = [0u8; 32];
        sodalite::box_keypair_seed(&mut pk, &mut sk, &seed);

        Self {
            public_key: pk,
            secret_key: Arc::new(Mutex::new(Protected::new(sk))),
        }
    }

    pub fn json_encrypt<I: Serialize + DeserializeOwned + Sync>(
        &self,
        their_public_key: &[u8; 32],
        msg: &I,
    ) -> lit_core::error::Result<EncryptedPayload<I>> {
        // release the lock on the secret key as soon as possible by enclosing in {} here
        let mut protected = self.secret_key.lock().expect("to unlock secret key");
        let Some(unprotected) = protected.unprotect() else {
            panic!("Failed to unlock secret key");
        };

        let secret_key = unprotected.as_ref();
        let secret_key: &[u8; 32] = secret_key.try_into().expect("to convert secret key");
        let payload = EncryptedPayload::json_encrypt(secret_key, their_public_key, msg)
            .map_err(|e| unexpected_err(e, None))?;
        Ok(payload)
    }

    pub fn json_decrypt<I: Serialize + DeserializeOwned + Sync>(
        &self,
        ciphertext: &EncryptedPayload<I>,
    ) -> lit_core::error::Result<(I, [u8; 32])> {
        // release the lock on the secret key as soon as possible by enclosing in {} here
        let mut protected = self.secret_key.lock().expect("to unlock secret key");
        let Some(unprotected) = protected.unprotect() else {
            panic!("Failed to unlock secret key");
        };

        let secret_key = unprotected.as_ref();
        let secret_key: &[u8; 32] = secret_key.try_into().expect("to convert secret key");
        ciphertext
            .json_decrypt(secret_key)
            .map_err(|e| unexpected_err(e, None))
    }
}
