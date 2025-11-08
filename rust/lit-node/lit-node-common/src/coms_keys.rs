use crate::error::Result;
use lit_core::error::Unexpected;
use lit_core::utils::binary::hex_to_bytes;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Debug, Formatter};
use x25519_dalek::{PublicKey, StaticSecret};

#[derive(Clone)]
pub struct KeyPair {
    secret_key: StaticSecret,
    public_key: PublicKey,
}

impl KeyPair {
    pub fn from_components(secret_key: StaticSecret, public_key: PublicKey) -> Self {
        Self {
            secret_key,
            public_key,
        }
    }

    pub fn from_secret_key(secret_key: StaticSecret) -> Self {
        let public_key = PublicKey::from(&secret_key);
        Self {
            secret_key,
            public_key,
        }
    }

    pub fn from_bytes(secret_key: &[u8; 32]) -> Self {
        let secret_key = StaticSecret::from(*secret_key);
        let public_key = PublicKey::from(&secret_key);
        Self {
            secret_key,
            public_key,
        }
    }

    pub fn generate() -> Self {
        let secret_key = StaticSecret::random_from_rng(rand::rngs::OsRng);
        let public_key = PublicKey::from(&secret_key);
        Self::from_components(secret_key, public_key)
    }

    pub fn private_key(&self) -> &StaticSecret {
        &self.secret_key
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}

impl Debug for KeyPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyPair")
            .field("secret_key", &"********")
            .field("public_key", &self.public_key)
            .finish()
    }
}

impl Serialize for KeyPair {
    fn serialize<S>(&self, s: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.secret_key.to_bytes();
        serdect::array::serialize_hex_lower_or_bin(&bytes, s)
    }
}

impl<'de> Deserialize<'de> for KeyPair {
    fn deserialize<D>(d: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut bytes = [0u8; 32];
        serdect::array::deserialize_hex_or_bin(&mut bytes, d)?;
        Ok(Self::from_bytes(&bytes))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Keys for inter-node communication.
pub struct ComsKeys {
    sender_pair: KeyPair,
    receiver_pair: KeyPair,
}

impl Default for ComsKeys {
    fn default() -> Self {
        Self::new()
    }
}

impl ComsKeys {
    pub fn new() -> Self {
        Self {
            sender_pair: KeyPair::generate(),
            receiver_pair: KeyPair::generate(),
        }
    }

    pub fn new_from_secret_keys(
        sender_private_key: StaticSecret,
        receiver_private_key: StaticSecret,
    ) -> Self {
        Self {
            sender_pair: KeyPair::from_secret_key(sender_private_key),
            receiver_pair: KeyPair::from_secret_key(receiver_private_key),
        }
    }

    pub fn new_from_keypairs(
        sender_private_key: StaticSecret,
        sender_public_key: PublicKey,
        receiver_private_key: StaticSecret,
        receiver_public_key: PublicKey,
    ) -> Self {
        Self {
            sender_pair: KeyPair::from_components(sender_private_key, sender_public_key),
            receiver_pair: KeyPair::from_components(receiver_private_key, receiver_public_key),
        }
    }

    pub fn from_node_config(sender_privkey: String, receiver_privkey: String) -> Result<Self> {
        let sender_privkey = hex_to_bytes(sender_privkey)
            .expect_or_err("Failed to hex encode LIT_NODE_COMS_KEYS_SENDER_PRIVKEY config var")?;
        let receiver_privkey = hex_to_bytes(receiver_privkey)
            .expect_or_err("Failed to hex encode LIT_NODE_COMS_KEYS_RECEIVER_PRIVKEY config var")?;

        let sender_privkey_array: [u8; 32] = sender_privkey
            .as_slice()
            .try_into()
            .expect_or_err("LIT_NODE_COMS_KEYS_SENDER_PRIVKEY slice with incorrect length")?;
        let sender_key_pair = KeyPair::from_bytes(&sender_privkey_array);

        let receiver_privkey_array: [u8; 32] = receiver_privkey
            .as_slice()
            .try_into()
            .expect_or_err("LIT_NODE_COMS_KEYS_RECEIVER_PRIVKEY slice with incorrect length")?;
        let receiver_key_pair = KeyPair::from_bytes(&receiver_privkey_array);

        Ok(Self {
            sender_pair: sender_key_pair,
            receiver_pair: receiver_key_pair,
        })
    }

    pub fn parse_secret_key(secret_key: &str) -> Result<StaticSecret> {
        let secret_key = hex_to_bytes(secret_key)
            .expect_or_err("Failed to hex encode LIT_NODE_COMS_KEYS_SENDER_PRIVKEY config var")?;

        let secret_key_array: [u8; 32] = secret_key
            .as_slice()
            .try_into()
            .expect_or_err("LIT_NODE_COMS_KEYS_SENDER_PRIVKEY slice with incorrect length")?;
        let secret_key = StaticSecret::from(secret_key_array);

        Ok(secret_key)
    }

    pub fn receiver_public_key(&self) -> PublicKey {
        self.receiver_pair.public_key
    }
    pub fn sender_public_key(&self) -> PublicKey {
        self.sender_pair.public_key
    }
    pub fn my_sender_private_key(&self) -> &StaticSecret {
        &self.sender_pair.secret_key
    }

    pub fn my_receiver_private_key(&self) -> &StaticSecret {
        &self.receiver_pair.secret_key
    }
}
