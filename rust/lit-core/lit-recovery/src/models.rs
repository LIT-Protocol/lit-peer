use bulletproofs::BulletproofCurveArithmetic as BCA;
use lit_rust_crypto::elliptic_curve::bigint::U256;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use verifiable_share_encryption::{Ciphertext, Proof, v1};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadedShareData {
    // DATIL_BACKUP: Remove participant_id once old Datil backup is obsolete.
    #[serde(default)] // If the value is not present when deserializing, use default.
    pub participant_id: usize, // numeric identifier used when the decryption share was generated
    pub session_id: String,       // lower hex encoding of the session_id
    pub encryption_key: String,   // lower hex encoding of canonical point form i.e compressed point
    pub verification_key: String, // lower hex encoding of canonical point form i.e compressed point
    pub decryption_share: String, // lower hex encoding of decryption share bytes and their identifiers
    pub subnet_id: String,        // staking contract address
    pub curve: String,            // See constants for curve names
}

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadedShareData {
    pub session_id: String,
    pub encryption_key: String,
    pub decryption_key_share: String,
    pub subnet_id: String,
    pub curve: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EncryptedKeyShare<C: BCA> {
    // Identifies the subnet this backup belongs to.
    pub subnet_id: String,
    // Identifies the node this backup comes from.
    pub staker_address: String,
    // Identifies the DKG round
    pub txn_prefix: String,
    // Ciphertext and proof corresponding to the secret key share
    #[serde(bound(serialize = "Ciphertext<C>: Serialize"))]
    #[serde(bound(deserialize = "Ciphertext<C>: Deserialize<'de>"))]
    pub ciphertext: Ciphertext<C>,
    #[serde(bound(serialize = "Proof<C>: Serialize"))]
    #[serde(bound(deserialize = "Proof<C>: Deserialize<'de>"))]
    pub proof: Proof<C>,
    // Remaining metadata to recover the 'KeyShare<Secp256k1>' type.
    pub public_key: String,
    pub peer_id: U256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_index: Option<u16>,
    pub threshold: usize,
    pub total_shares: usize,
    #[serde(default)] // If the value is not present when deserializing, use default.
    pub peers: Vec<U256>,
    #[serde(default = "default_realm_id")]
    pub realm_id: u64,
}

impl<C: BCA> Debug for EncryptedKeyShare<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncryptedKeyShare")
            .field("subnet_id", &self.subnet_id)
            .field("staker_address", &self.staker_address)
            .field("txn_prefix", &self.txn_prefix)
            .field("ciphertext.len()", &self.ciphertext.c1.len())
            .field("proof", &self.proof)
            .field("public_key", &self.public_key)
            .field("peer_id", &self.peer_id)
            .field("share_index", &self.share_index)
            .field("threshold", &self.threshold)
            .field("total_shares", &self.total_shares)
            .finish()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OldEncryptedKeyShare<C: BCA> {
    // Identifies the subnet this backup belongs to.
    pub subnet_id: String,
    // Identifies the node this backup comes from.
    pub staker_address: String,
    // Identifies the DKG round
    pub txn_prefix: String,
    // Ciphertext and proof corresponding to the secret key share
    #[serde(bound(serialize = "v1::Ciphertext<C>: Serialize"))]
    #[serde(bound(deserialize = "v1::Ciphertext<C>: Deserialize<'de>"))]
    pub ciphertext: v1::Ciphertext<C>,
    #[serde(bound(serialize = "v1::Proof<C>: Serialize"))]
    #[serde(bound(deserialize = "v1::Proof<C>: Deserialize<'de>"))]
    pub proof: v1::Proof<C>,
    // Remaining metadata to recover the 'KeyShare<Secp256k1>' type.
    pub public_key: String,
    pub share_index: u16,
    pub threshold: u16,
    pub total_shares: u16,
    #[serde(default = "default_realm_id")]
    pub realm_id: u64,
}

fn default_realm_id() -> u64 {
    1
}

impl<C: BCA> From<OldEncryptedKeyShare<C>> for EncryptedKeyShare<C> {
    fn from(old_backup: OldEncryptedKeyShare<C>) -> Self {
        EncryptedKeyShare {
            subnet_id: old_backup.subnet_id,
            staker_address: old_backup.staker_address,
            txn_prefix: old_backup.txn_prefix,
            ciphertext: old_backup.ciphertext.into(),
            proof: old_backup.proof.into(),
            public_key: old_backup.public_key,
            peer_id: U256::MAX,
            share_index: Some(old_backup.share_index),
            threshold: old_backup.threshold as usize,
            total_shares: old_backup.total_shares as usize,
            peers: vec![],
            realm_id: old_backup.realm_id,
        }
    }
}
