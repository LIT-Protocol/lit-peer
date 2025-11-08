use crate::config::LitNodeConfig;
use crate::error::{Result, unexpected_err};
use k256::ecdsa::{RecoveryId, Signature, SigningKey, VerifyingKey, signature::DigestVerifier};
use lit_attestation::kdf::Kdf;
use lit_core::config::LitConfig;
use rand_chacha::ChaCha20Rng;
use rand_core::{CryptoRngCore, SeedableRng};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha3::{Digest, Keccak256};
use std::fmt::{self, Debug, Display, Formatter};
use std::sync::Arc;

#[derive(Clone)]
pub struct EthWalletKeys {
    secret_key: SigningKey,
}

impl Debug for EthWalletKeys {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", VerifyingKey::from(&self.secret_key))
    }
}

impl Display for EthWalletKeys {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vk = VerifyingKey::from(&self.secret_key);
        let ep = vk.to_encoded_point(true);
        write!(f, "{}", hex::encode(ep.as_bytes()))
    }
}

impl Serialize for EthWalletKeys {
    fn serialize<S>(&self, s: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.secret_key.to_bytes();
        serdect::array::serialize_hex_lower_or_bin(&bytes, s)
    }
}

impl<'de> Deserialize<'de> for EthWalletKeys {
    fn deserialize<D>(d: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut bytes = [0u8; 32];
        let read = serdect::array::deserialize_hex_or_bin(&mut bytes, d)?;
        if read.len() != 32 {
            return Err(serde::de::Error::custom("Invalid length for secret key"));
        }
        let bytes: k256::FieldBytes = bytes.into();
        let secret_key = SigningKey::from_bytes(&bytes).map_err(serde::de::Error::custom)?;
        Ok(EthWalletKeys { secret_key })
    }
}

impl EthWalletKeys {
    pub async fn generate(cfg: &Arc<LitConfig>) -> Result<Self> {
        // Add some randomness to the context, eg. use time.
        let context = format!("attested_wallet_keys_{}", chrono::Utc::now().timestamp());
        let seed = Kdf::try_derive(cfg, &context, Some(cfg.external_addr()?)).await?;
        let mut rng = ChaCha20Rng::from_seed(seed);
        Ok(Self::random(&mut rng))
    }

    pub fn random<R: CryptoRngCore>(rng: &mut R) -> Self {
        Self {
            secret_key: SigningKey::random(rng),
        }
    }

    pub fn signing_key(&self) -> &SigningKey {
        &self.secret_key
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        VerifyingKey::from(&self.secret_key)
    }

    pub fn verifying_key_compressed_vec(&self) -> Vec<u8> {
        let vk = VerifyingKey::from(&self.secret_key);
        let ep = vk.to_encoded_point(true);
        ep.as_bytes().to_vec()
    }

    pub fn verifying_key_compressed_hex(&self) -> String {
        let vk = VerifyingKey::from(&self.secret_key);
        let ep = vk.to_encoded_point(true);
        hex::encode(ep.as_bytes())
    }

    pub fn verifying_key_uncompressed_vec(&self) -> Vec<u8> {
        let vk = VerifyingKey::from(&self.secret_key);
        let ep = vk.to_encoded_point(false);
        ep.as_bytes().to_vec()
    }

    pub fn verifying_key_uncompressed_hex(&self) -> String {
        let vk = VerifyingKey::from(&self.secret_key);
        let ep = vk.to_encoded_point(false);
        hex::encode(ep.as_bytes())
    }

    pub fn verifying_key_compressed_array(&self) -> [u8; 33] {
        let vk = VerifyingKey::from(&self.secret_key);
        let ep = vk.to_encoded_point(true);
        ep.as_bytes().try_into().expect("Invalid length")
    }

    pub fn verifying_key_uncompressed_array(&self) -> [u8; 65] {
        let vk = VerifyingKey::from(&self.secret_key);
        let ep = vk.to_encoded_point(false);
        ep.as_bytes().try_into().expect("Invalid length")
    }

    pub fn sign_eth<B: AsRef<[u8]>>(&self, msg: B) -> Result<(Signature, RecoveryId)> {
        let digest = Keccak256::new_with_prefix(msg);
        self.secret_key
            .sign_digest_recoverable(digest)
            .map_err(|e| unexpected_err(e, Some("Failed to sign message".to_string())))
    }

    pub fn verify_signature<B: AsRef<[u8]>>(&self, msg: B, signature: &Signature) -> Result<()> {
        let vk = VerifyingKey::from(&self.secret_key);
        vk.verify_digest(Keccak256::new_with_prefix(msg), signature)
            .map_err(|e| unexpected_err(e, Some("Failed to verify signature".to_string())))
    }

    pub fn verify_eth<B: AsRef<[u8]>>(
        &self,
        msg: B,
        signature: &Signature,
        recovery_id: RecoveryId,
    ) -> Result<()> {
        let expected_vk = VerifyingKey::from(&self.secret_key);
        let recovered_key = Self::recover_verifying_key(msg, signature, recovery_id)?;
        if expected_vk != recovered_key {
            return Err(unexpected_err(
                "Signature verification failed".to_string(),
                None,
            ));
        }
        Ok(())
    }

    pub fn recover_verifying_key<B: AsRef<[u8]>>(
        msg: B,
        signature: &Signature,
        recovery_id: RecoveryId,
    ) -> Result<VerifyingKey> {
        VerifyingKey::recover_from_digest(Keccak256::new_with_prefix(msg), signature, recovery_id)
            .map_err(|e| unexpected_err(e, Some("Failed to recover public key".to_string())))
    }

    pub fn recovery_id<B: AsRef<[u8]>>(&self, msg: B, signature: &Signature) -> Result<RecoveryId> {
        let vk = VerifyingKey::from(&self.secret_key);
        RecoveryId::trial_recovery_from_digest(&vk, Keccak256::new_with_prefix(msg), signature)
            .map_err(|e| unexpected_err(e, Some("Failed to get recovery id".to_string())))
    }
}
