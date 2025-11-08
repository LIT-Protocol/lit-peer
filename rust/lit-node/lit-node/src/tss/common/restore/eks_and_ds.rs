use crate::common::key_helper::KeyCache;
use crate::error::{Result, unexpected_err};
use crate::tss::common::backup::get_peer_id;
use crate::tss::common::key_persistence::KeyPersistence;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::key_share_commitment::KeyShareCommitments;
use crate::tss::common::storage::write_key_share_to_cache_only;
use crate::utils::traits::SignatureCurve;
use bulletproofs::BulletproofCurveArithmetic as BCA;
use elliptic_curve::bigint::{NonZero, U256};
use lit_node_core::CurveType;
use lit_node_core::PeerId;
use lit_node_core::{CompressedBytes, CompressedHex};
use lit_recovery::models::EncryptedKeyShare;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use verifiable_share_encryption::VerifiableEncryptionDecryptor;
use vsss_rs::{
    DefaultShare, FeldmanVerifierSet, IdentifierPrimeField, Share, ValueGroup,
    VecFeldmanVerifierSet,
};

/// Identifier for a Recovery Party member.
pub type RecPartyMemberIdType = String;
/// Decryption shares
pub type DecryptionShare<C> = verifiable_share_encryption::DecryptionShare<C>;

// Data kept for each supported CurveType
#[derive(Clone)]
pub(crate) struct CurveRecoveryData<C>
where
    C: VerifiableEncryptionDecryptor,
    C::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    pub encryption_key: C::Point,
    pub blinder: C::Scalar,
    pub eks_and_ds: Vec<EksAndDs<C>>,
}

impl<C> Debug for CurveRecoveryData<C>
where
    C: VerifiableEncryptionDecryptor,
    C::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CurveRecoveryData")
            .field("encryption_key", &self.encryption_key)
            .field("blinder.len()", &self.blinder.to_compressed().len())
            .field("eks_and_ds", &self.eks_and_ds)
            .finish()
    }
}

impl<C> CurveRecoveryData<C>
where
    C: VerifiableEncryptionDecryptor + SignatureCurve<Point = <C as BCA>::Point>,
    <C as BCA>::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    pub async fn try_restore(
        &self,
        threshold: usize,
        current_peer_id: &PeerId,
        epoch: u64,
        realm_id: u64,
        staker_address: &str,
        restore_key_cache: &KeyCache,
    ) -> Vec<String> {
        let mut restored_keys = Vec::new();
        for eks_and_ds in self.eks_and_ds.iter() {
            let restore_result = eks_and_ds
                .try_restore(
                    threshold,
                    &self.blinder,
                    current_peer_id,
                    epoch,
                    realm_id,
                    staker_address,
                    restore_key_cache,
                )
                .await;
            if let Some(public_key) = restore_result {
                restored_keys.push(public_key);
            };
        }
        restored_keys
    }

    // Checks if all `EksAndDs` instances are already marked as `restored`.
    pub fn are_all_keys_restored(data: &Option<CurveRecoveryData<C>>, root_key: &str) -> bool {
        let key_helper = KeyPersistence::<<C as BCA>::Point>::new(C::CURVE_TYPE);
        let public_key = key_helper
            .pk_from_hex(root_key)
            .expect("failed to get public key");

        // If there are no key shares found for this key and there should be,
        // then it's not restored, and there should only be one entry for this
        match data {
            Some(data) => {
                // Check that all keys are restored
                data.eks_and_ds
                    .iter()
                    .any(|eks_and_ds| eks_and_ds.restored && eks_and_ds.public_key == public_key)
            }
            None => true,
        }
    }

    pub fn encryption_key(data: &Option<CurveRecoveryData<C>>) -> Option<String> {
        data.as_ref().map(|d| d.encryption_key.to_compressed_hex())
    }

    pub fn log_shares(data: &Option<CurveRecoveryData<C>>) -> Vec<RootKeyRecoveryLog> {
        data.as_ref()
            .map(|d| d.eks_and_ds.iter().map(|s| s.into()).collect())
            .unwrap_or_default()
    }

    pub fn original_peer_id(&self) -> Option<U256> {
        self.eks_and_ds
            .first()
            .map(|x| x.encrypted_key_share.peer_id)
    }
}

/// Encrypted Key Share And Decryption Shares;
#[derive(Clone)]
pub(crate) struct EksAndDs<C>
where
    C: VerifiableEncryptionDecryptor,
    C::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    pub public_key: C::Point,
    pub encrypted_key_share: EncryptedKeyShare<C>,
    pub key_share_commitments: Option<KeyShareCommitments<C::Point>>,
    /// Each recovery member can submit only one decryption share.
    /// If they submit more, the newer overwrites the older.
    pub decryption_shares: BTreeMap<RecPartyMemberIdType, DecryptionShare<C>>,
    pub restored: bool,
    pub curve_type: CurveType,
}

impl<C> Debug for EksAndDs<C>
where
    C: VerifiableEncryptionDecryptor,
    C::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EksAndDs")
            .field("public_key", &self.public_key)
            .field("encrypted_key_share", &self.encrypted_key_share)
            .field("key_share_commitments", &self.key_share_commitments)
            .field("decryption_shares", &"")
            .field("restored", &self.restored)
            .field("curve_type", &self.curve_type)
            .finish()
    }
}

impl<C> EksAndDs<C>
where
    C: VerifiableEncryptionDecryptor + SignatureCurve<Point = <C as BCA>::Point>,
    <C as BCA>::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    pub fn new(
        encrypted_key_share: EncryptedKeyShare<C>,
        key_share_commitments: Option<KeyShareCommitments<<C as BCA>::Point>>,
        curve_type: CurveType,
    ) -> Result<Self> {
        let helper = KeyPersistence::<<C as BCA>::Point>::new(curve_type);
        let public_key = helper.pk_from_hex(&encrypted_key_share.public_key)?;
        info!(
            "Encrypted {} share {} is loaded {} key share commitments",
            &curve_type,
            &encrypted_key_share.public_key,
            match key_share_commitments {
                Some(_) => "with",
                None => "without",
            }
        );
        Ok(EksAndDs {
            public_key,
            encrypted_key_share,
            key_share_commitments,
            decryption_shares: BTreeMap::new(),
            restored: false,
            curve_type,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn try_restore(
        &self,
        threshold: usize,
        blinder: &C::Scalar,
        current_peer_id: &PeerId,
        epoch: u64,
        realm_id: u64,
        staker_address: &str,
        restore_key_cache: &KeyCache,
    ) -> Option<String> {
        // If this key is already restored, return.
        if self.restored {
            return None;
        }
        // If this key does not have enough decryption shares, don't attempt.
        if self.decryption_shares.len() < threshold {
            return None;
        }

        // Decrypt the private share
        let private_share = match C::decrypt_with_shares_and_unblind(
            blinder,
            &self.get_decryption_shares(),
            &self.encrypted_key_share.ciphertext,
            Some(C::signing_generator()),
        ) {
            Ok(share) => share,
            Err(e) => {
                error!(
                    "Failed to decrypt share {:?} due to error: {}",
                    self.encrypted_key_share.public_key, e
                );
                return None;
            }
        };

        let key_helper = KeyPersistence::<<C as BCA>::Point>::new(self.curve_type);
        match &self.key_share_commitments {
            Some(commitments) => {
                if let Err(e) = verify_decrypted_key_share::<C>(
                    private_share,
                    commitments,
                    get_peer_id(&self.encrypted_key_share),
                ) {
                    error!(
                        "Decrypted {} key share with public key {} is not correct: {}",
                        self.curve_type, &self.encrypted_key_share.public_key, e,
                    );
                    return None;
                }
            }

            None => {
                warn!(
                    "No key share commitments are found; skipping verification for restored key share {}",
                    &self.encrypted_key_share.public_key
                );
            }
        };

        // Generate the key share wrapper to be kept locally
        let key_helper = KeyPersistence::<<C as BCA>::Point>::new(self.curve_type);
        let key_share = KeyShare {
            hex_private_share: key_helper.secret_to_hex(&private_share),
            hex_public_key: key_helper.pk_to_hex(&self.public_key),
            curve_type: self.curve_type,
            // Save the original peer id, as it is needed later during the reshare.
            peer_id: get_peer_id(&self.encrypted_key_share),
            threshold: self.encrypted_key_share.threshold,
            total_shares: self.encrypted_key_share.total_shares,
            txn_prefix: self.encrypted_key_share.txn_prefix.clone(),
            realm_id,
            peers: self
                .encrypted_key_share
                .peers
                .iter()
                .map(|x| PeerId(NonZero::<U256>::from_uint(*x)))
                .collect(),
        };
        write_key_share_to_cache_only(
            self.curve_type,
            &self.encrypted_key_share.public_key,
            // Make sure to compute the file name with the peer id of
            // the current peer, so that it can later be found by this node.
            current_peer_id,
            staker_address,
            epoch,
            realm_id,
            restore_key_cache,
            &key_share,
        )
        .await
        .ok()?;

        info!(
            "Restored {} key: {}",
            self.curve_type, self.encrypted_key_share.public_key
        );
        Some(self.encrypted_key_share.public_key.clone())
    }

    // Given `pub_keys` is a subset of the pub_keys of `EksAndDs`s, in the same order,
    // this function marks all specified `EksAndDs` instances as `restored`.
    pub fn mark_keys_restored(eks_and_ds_vec: &mut [EksAndDs<C>], pub_keys: &[String]) {
        for pub_key in pub_keys.iter() {
            for eks_and_ds in eks_and_ds_vec.iter_mut() {
                if &eks_and_ds.encrypted_key_share.public_key == pub_key {
                    eks_and_ds.restored = true;
                    break;
                }
            }
        }
    }

    fn get_decryption_shares(&self) -> Vec<DecryptionShare<C>> {
        self.decryption_shares
            .values()
            .map(|s| (*s).clone())
            .collect()
    }
}

pub fn verify_decrypted_key_share<C>(
    decrypted_share: C::Scalar,
    key_share_commitments: &KeyShareCommitments<<C as SignatureCurve>::Point>,
    peer_id: PeerId,
) -> Result<()>
where
    C: VerifiableEncryptionDecryptor + SignatureCurve<Point = <C as BCA>::Point>,
    <C as SignatureCurve>::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    let identifier = C::Scalar::from(peer_id);
    let secret_share = DefaultShare::with_identifier_and_value(
        IdentifierPrimeField(identifier),
        IdentifierPrimeField(decrypted_share),
    );
    let verifiers = VecFeldmanVerifierSet::from(
        std::iter::once(ValueGroup(C::signing_generator()))
            .chain(
                key_share_commitments
                    .commitments
                    .iter()
                    .map(|c| ValueGroup(*c)),
            )
            .collect::<Vec<_>>(),
    );
    verifiers
        .verify_share(&secret_share)
        .map_err(|e| unexpected_err(e.to_string(), None))
}

/// Encrypted Key Shares And Recovery Members ...
// who have sent decryption shares for this key
#[derive(Debug, Serialize, Deserialize)]
pub struct RootKeyRecoveryLog {
    public_key: String,
    restored: bool,
    members_who_sent_dec_shares: Vec<String>,
}

impl<C> From<&EksAndDs<C>> for RootKeyRecoveryLog
where
    C: VerifiableEncryptionDecryptor,
    C::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    fn from(eks_and_ds: &EksAndDs<C>) -> Self {
        Self {
            public_key: eks_and_ds.encrypted_key_share.public_key.clone(),
            restored: eks_and_ds.restored,
            members_who_sent_dec_shares: eks_and_ds.decryption_shares.keys().cloned().collect(),
        }
    }
}
