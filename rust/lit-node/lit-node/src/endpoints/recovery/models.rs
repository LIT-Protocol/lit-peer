use crate::tss::common::key_share::KeyShare;

#[derive(Debug, Clone)]
pub struct RecoveryShares {
    pub bls_encryption_share: KeyShare,
    pub k256_signing_share: KeyShare,
    pub p256_signing_share: KeyShare,
    pub p384_signing_share: KeyShare,
    pub ed25519_signing_share: KeyShare,
    pub ristretto25519_signing_share: KeyShare,
    pub ed448_signing_share: KeyShare,
    pub jubjub_signing_share: KeyShare,
    pub decaf377_signing_share: KeyShare,
    pub bls12381g1_signing_share: KeyShare,
}
