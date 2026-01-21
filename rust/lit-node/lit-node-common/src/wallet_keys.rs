use crate::config::LitNodeConfig;
use ethers::utils::hex;
use hkdf::Hkdf;
use lit_attestation::kdf::Kdf;
use lit_core::config::LitConfig;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::fmt::{self, Display, Formatter};
use std::sync::Arc;

use crate::error::{Result, unexpected_err};

type LocalKdf = Hkdf<Sha256>;
const ATTESTED_WALLET_KEY_INFO: &[u8] = b"lit-node-attested-wallet-signing-key";

/// These are x25519 keys
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WalletKeys {
    secret_key: [u8; 32],
    public_key: [u8; 32],
}

impl Display for WalletKeys {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for b in &self.public_key {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl WalletKeys {
    /// Generate a new random keypair.
    pub async fn generate(cfg: &Arc<LitConfig>) -> Result<Self> {
        // Add some randomness to the context, eg. use time.
        let context = format!("attested_wallet_keys_{}", chrono::Utc::now().timestamp());
        let seed = Kdf::try_derive(cfg, &context, Some(cfg.external_addr()?)).await?;
        Ok(Self::from_seed(&seed))
    }

    pub fn generate_seed() -> Self {
        use rand_core::RngCore;
        let mut seed = [0u8; 32];
        rand_core::OsRng.fill_bytes(&mut seed);
        Self::from_seed(&seed)
    }

    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let mut sk = [0u8; 32];
        let mut pk = [0u8; 32];
        sodalite::box_keypair_seed(&mut pk, &mut sk, seed);
        Self {
            secret_key: sk,
            public_key: pk,
        }
    }

    pub fn from_secret_key(secret_key: &[u8; 32]) -> Self {
        let mut pk = [0u8; 32];
        sodalite::scalarmult_base(&mut pk, secret_key);
        Self {
            secret_key: *secret_key,
            public_key: pk,
        }
    }

    pub fn from_components(secret_key: &[u8; 32], public_key: &[u8; 32]) -> Result<Self> {
        let mut pk = [0u8; 32];
        sodalite::scalarmult_base(&mut pk, secret_key);
        if pk != *public_key {
            return Err(unexpected_err("public keys do not match", None));
        }
        Ok(Self {
            secret_key: *secret_key,
            public_key: *public_key,
        })
    }

    /// The public key
    pub fn get_public_key(&self) -> &[u8; 32] {
        &self.public_key
    }

    /// The public key as a hex string
    pub fn get_public_key_hex(&self) -> String {
        hex::encode(self.public_key)
    }

    /// The secret key
    pub fn get_secret_key(&self) -> &[u8; 32] {
        &self.secret_key
    }

    /// The secret key as a hex string
    pub fn get_secret_key_hex(&self) -> String {
        hex::encode(self.secret_key)
    }

    /// Convert the x25519 secret key to an ECDSA signing key
    ///
    /// This is for future proofing against callers that try
    /// to use the x25519 keys for signing on secp256k1
    /// which they should not do. x25519 keys cannot be used
    /// for generating and verifying signatures.
    pub fn get_signing_key(&self) -> k256::SecretKey {
        let scalar = self.get_signing_key_scalar();
        let nz_scalar = k256::NonZeroScalar::new(scalar).expect("scalar to not be zero");
        k256::SecretKey::from(nz_scalar)
    }

    /// The public key for verification when using the signing key
    pub fn get_verification_key(&self) -> k256::PublicKey {
        let signing_key = self.get_signing_key();
        signing_key.public_key()
    }

    fn get_signing_key_scalar(&self) -> k256::Scalar {
        let mut key = k256::WideBytes::default();
        let hkdf = LocalKdf::new(Some(&self.public_key), &self.secret_key);
        hkdf.expand(ATTESTED_WALLET_KEY_INFO, &mut key)
            .expect("failed to expand hkdf");
        <k256::Scalar as k256::elliptic_curve::ops::ReduceNonZero<
            k256::elliptic_curve::bigint::U512,
        >>::reduce_nonzero_bytes(&key)
    }
}
