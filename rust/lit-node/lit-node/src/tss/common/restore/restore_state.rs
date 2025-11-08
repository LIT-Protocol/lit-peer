use blsful::inner_types::{G1Projective, InnerBls12381G1};
use bulletproofs::BulletproofCurveArithmetic as BCA;
use elliptic_curve::Field;
use elliptic_curve::bigint::{NonZero, U256};
use ethers::types::H160;
use sdd::{AtomicShared, Shared};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;
use tracing::{instrument, warn};

use crate::common::key_helper::KeyCache;
use crate::config::chain::CachedRootKey;
use crate::error::{Result, conversion_err, parser_err, unexpected_err};
use crate::tss::common::key_persistence::KeyPersistence;
use crate::tss::common::restore::eks_and_ds::{
    CurveRecoveryData, EksAndDs, RecPartyMemberIdType, RootKeyRecoveryLog,
};
use crate::tss::common::restore::point_reader::PointReader;
use crate::tss::common::tss_state::TssState;
use crate::utils::contract::get_backup_recovery_contract_with_signer;
use crate::version::{DataVersionReader, DataVersionWriter};
use lit_blockchain::contracts::backup_recovery::{BackupRecoveryErrors, RecoveredPeerId};
use lit_core::config::LitConfig;
use lit_node_core::CurveType;
use lit_node_core::PeerId;
use lit_node_core::{Blinders, CompressedBytes, CompressedHex};
use lit_recovery::models::UploadedShareData;
use verifiable_share_encryption::{DecryptionShare, VerifiableEncryptionDecryptor};

// DATIL_BACKUP: Remove this type once old Datil backup is obsolete.
type DatilDecryptionShare<C> = verifiable_share_encryption::v1::DecryptionShare<Vec<u8>, C>;

/// Keeps the state of the restore mode. Only used in case of
/// disaster recovery.
// When inner state is set, it means that the node is in RESTORE mode.
// Once the restoration is completed, the inner state is set to None.
pub struct RestoreState {
    blinders: AtomicShared<Blinders>,
    actively_restoring: AtomicBool,
    state: RwLock<Option<InnerState>>,
    restoring_root_keys: AtomicShared<Vec<CachedRootKey>>,
}

/// Inner state kept by RestoreState.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub(crate) struct InnerState {
    pub recovery_party_members: Vec<H160>,
    pub bls_recovery_data: Option<CurveRecoveryData<InnerBls12381G1>>,
    pub k256_recovery_data: Option<CurveRecoveryData<k256::Secp256k1>>,
    pub p256_recovery_data: Option<CurveRecoveryData<p256::NistP256>>,
    pub p384_recovery_data: Option<CurveRecoveryData<p384::NistP384>>,
    pub ed25519_recovery_data: Option<CurveRecoveryData<bulletproofs::Ed25519>>,
    pub ristretto25519_recovery_data: Option<CurveRecoveryData<bulletproofs::Ristretto25519>>,
    pub ed448_recovery_data: Option<CurveRecoveryData<ed448_goldilocks::Ed448>>,
    pub jubjub_recovery_data: Option<CurveRecoveryData<bulletproofs::JubJub>>,
    pub decaf377_recovery_data: Option<CurveRecoveryData<bulletproofs::Decaf377>>,
    pub bls12381g1_recovery_data: Option<CurveRecoveryData<InnerBls12381G1>>,
    pub threshold: usize,
    pub restored_key_cache: KeyCache,
}

impl RestoreState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            blinders: AtomicShared::from(Shared::new(Self::generate_blinders())),
            actively_restoring: AtomicBool::new(false),
            state: RwLock::new(None),
            restoring_root_keys: AtomicShared::from(Shared::new(Vec::new())),
        }
    }

    pub async fn prepare_for_recovery(&self, tss_state: Arc<TssState>) -> Result<()> {
        self.set_blinders(Blinders::default());
        self.set_actively_restoring(true);
        self.init_inner_state().await;
        tss_state
            .chain_data_config_manager
            .set_root_keys_from_chain()
            .await?;
        let root_keys = tss_state.chain_data_config_manager.root_keys();
        debug!("Restoring root keys: {:?}", &root_keys);
        DataVersionWriter::store(&self.restoring_root_keys, root_keys);
        Ok(())
    }

    pub async fn init_inner_state(&self) {
        self.state.write().await.take();
    }

    #[instrument(level = "debug", skip_all)]
    pub fn generate_blinders() -> Blinders {
        let mut rng = rand_core::OsRng;
        let bls_blinder = <InnerBls12381G1 as BCA>::Scalar::random(&mut rng);
        let k256_blinder = k256::Scalar::random(&mut rng);
        let p256_blinder = p256::Scalar::random(&mut rng);
        let p384_blinder = p384::Scalar::random(&mut rng);
        let ed25519_blinder = vsss_rs::curve25519::WrappedScalar::random(&mut rng);
        let ristretto25519_blinder = vsss_rs::curve25519::WrappedScalar::random(&mut rng);
        let ed448_blinder = ed448_goldilocks::Scalar::random(&mut rng);
        let jubjub_blinder = jubjub::Scalar::random(&mut rng);
        let decaf377_blinder = decaf377::Fr::random(&mut rng);
        let bls12381g1_blinder = <InnerBls12381G1 as BCA>::Scalar::random(&mut rng);
        Blinders {
            bls_blinder: Some(bls_blinder),
            k256_blinder: Some(k256_blinder),
            p256_blinder: Some(p256_blinder),
            p384_blinder: Some(p384_blinder),
            ed25519_blinder: Some(ed25519_blinder),
            ristretto25519_blinder: Some(ristretto25519_blinder),
            ed448_blinder: Some(ed448_blinder),
            jubjub_blinder: Some(jubjub_blinder),
            decaf377_blinder: Some(decaf377_blinder),
            bls12381g1_blinder: Some(bls12381g1_blinder),
        }
    }

    #[instrument(level = "debug", skip(self, inner_state))]
    pub(crate) async fn load_backup(&self, inner_state: InnerState) -> Result<()> {
        self.assert_actively_restoring()?;
        *self.state.write().await = Some(inner_state);
        Ok(())
    }

    pub async fn add_decryption_shares(
        &self,
        rpm_id: &RecPartyMemberIdType,
        share_data: &[UploadedShareData],
    ) -> Result<()> {
        self.assert_actively_restoring()?;
        let Some(inner) = &mut *self.state.write().await else {
            return Err(Self::ciphertexts_not_set());
        };
        for share in share_data.iter() {
            let curve = match share.curve.parse() {
                Ok(curve) => curve,
                Err(e) => {
                    let err_msg = format!("Not a valid curve: {}", share.curve);
                    return Err(parser_err(Error::new(ErrorKind::Other, err_msg), None));
                }
            };

            // DATIL_BACKUP: Remove this branch once old Datil backup is obsolete.
            if share.curve == lit_recovery::consts::BLS12381G1 {
                let datil_decryption_share = serde_json::from_str::<
                    DatilDecryptionShare<InnerBls12381G1>,
                >(&share.decryption_share);

                if let Ok(datil_decryption_share) = datil_decryption_share {
                    let normalized_pub_key = InnerBls12381G1::parse_old_backup_public_key(
                        &share.verification_key,
                    )
                    .map(|point| {
                        let helper = KeyPersistence::<G1Projective>::new(CurveType::BLS);
                        helper.pk_to_hex(&point)
                    });
                    let decryption_share = DecryptionShare::from(datil_decryption_share);
                    return Self::do_add_decryption_share(
                        &mut inner.bls_recovery_data,
                        rpm_id,
                        share,
                        decryption_share,
                        normalized_pub_key,
                    );
                }
            }

            // DATIL_BACKUP: Remove this branch once old Datil backup is obsolete.
            if share.curve == lit_recovery::consts::SECP256K1 {
                let datil_decryption_share = serde_json::from_str::<
                    DatilDecryptionShare<k256::Secp256k1>,
                >(&share.decryption_share);

                if let Ok(datil_decryption_share) = datil_decryption_share {
                    let normalized_pub_key = k256::Secp256k1::parse_old_backup_public_key(
                        &share.verification_key,
                    )
                    .map(|point| {
                        let helper = KeyPersistence::<k256::ProjectivePoint>::new(CurveType::K256);
                        helper.pk_to_hex(&point)
                    });
                    let decryption_share = DecryptionShare::from(datil_decryption_share);
                    return Self::do_add_decryption_share(
                        &mut inner.k256_recovery_data,
                        rpm_id,
                        share,
                        decryption_share,
                        normalized_pub_key,
                    );
                }
            }

            match curve {
                CurveType::BLS => {
                    Self::add_decryption_share(&mut inner.bls_recovery_data, rpm_id, share)?
                }
                CurveType::K256 => {
                    Self::add_decryption_share(&mut inner.k256_recovery_data, rpm_id, share)?
                }
                CurveType::P256 => {
                    Self::add_decryption_share(&mut inner.p256_recovery_data, rpm_id, share)?
                }
                CurveType::P384 => {
                    Self::add_decryption_share(&mut inner.p384_recovery_data, rpm_id, share)?
                }
                CurveType::Ed25519 => {
                    Self::add_decryption_share(&mut inner.ed25519_recovery_data, rpm_id, share)?
                }
                CurveType::Ristretto25519 => Self::add_decryption_share(
                    &mut inner.ristretto25519_recovery_data,
                    rpm_id,
                    share,
                )?,
                CurveType::Ed448 => {
                    Self::add_decryption_share(&mut inner.ed448_recovery_data, rpm_id, share)?
                }
                CurveType::RedJubjub => {
                    Self::add_decryption_share(&mut inner.jubjub_recovery_data, rpm_id, share)?
                }
                CurveType::RedDecaf377 => {
                    Self::add_decryption_share(&mut inner.decaf377_recovery_data, rpm_id, share)?
                }
                CurveType::BLS12381G1 => {
                    Self::add_decryption_share(&mut inner.bls12381g1_recovery_data, rpm_id, share)?
                }
            };
        }
        Ok(())
    }

    /// Returns the public keys of the private shares that are restored at this attempt
    #[instrument(level = "debug", skip(self))]
    pub async fn try_restore_key_shares(
        &self,
        peer_id: &PeerId,
        epoch: u64,
        staker_address: &str,
        realm_id: u64,
    ) -> RestoredKeyShares {
        debug!(
            "Trying to restore key shares: epoch: {}, staker_address: {}",
            epoch, staker_address
        );
        let mut restored_key_shares = RestoredKeyShares::default();

        let Some(state) = &*self.state.read().await else {
            return restored_key_shares;
        };

        if let Some(recovery_data) = &state.bls_recovery_data {
            restored_key_shares.bls_shares = recovery_data
                .try_restore(
                    state.threshold,
                    peer_id,
                    epoch,
                    realm_id,
                    staker_address,
                    &state.restored_key_cache,
                )
                .await;
        }

        if let Some(recovery_data) = &state.k256_recovery_data {
            restored_key_shares.k256_shares = recovery_data
                .try_restore(
                    state.threshold,
                    peer_id,
                    epoch,
                    realm_id,
                    staker_address,
                    &state.restored_key_cache,
                )
                .await;
        }

        if let Some(recovery_data) = &state.p256_recovery_data {
            restored_key_shares.p256_shares = recovery_data
                .try_restore(
                    state.threshold,
                    peer_id,
                    epoch,
                    realm_id,
                    staker_address,
                    &state.restored_key_cache,
                )
                .await;
        }

        if let Some(recovery_data) = &state.p384_recovery_data {
            restored_key_shares.p384_shares = recovery_data
                .try_restore(
                    state.threshold,
                    peer_id,
                    epoch,
                    realm_id,
                    staker_address,
                    &state.restored_key_cache,
                )
                .await;
        }

        if let Some(recovery_data) = &state.ed25519_recovery_data {
            restored_key_shares.ed25519_shares = recovery_data
                .try_restore(
                    state.threshold,
                    peer_id,
                    epoch,
                    realm_id,
                    staker_address,
                    &state.restored_key_cache,
                )
                .await;
        }

        if let Some(recovery_data) = &state.ristretto25519_recovery_data {
            restored_key_shares.ristretto25519_shares = recovery_data
                .try_restore(
                    state.threshold,
                    peer_id,
                    epoch,
                    realm_id,
                    staker_address,
                    &state.restored_key_cache,
                )
                .await;
        }

        if let Some(recovery_data) = &state.ed448_recovery_data {
            restored_key_shares.ed448_shares = recovery_data
                .try_restore(
                    state.threshold,
                    peer_id,
                    epoch,
                    realm_id,
                    staker_address,
                    &state.restored_key_cache,
                )
                .await;
        }

        if let Some(recovery_data) = &state.jubjub_recovery_data {
            restored_key_shares.jubjub_shares = recovery_data
                .try_restore(
                    state.threshold,
                    peer_id,
                    epoch,
                    realm_id,
                    staker_address,
                    &state.restored_key_cache,
                )
                .await;
        }

        if let Some(recovery_data) = &state.decaf377_recovery_data {
            restored_key_shares.decaf377_shares = recovery_data
                .try_restore(
                    state.threshold,
                    peer_id,
                    epoch,
                    realm_id,
                    staker_address,
                    &state.restored_key_cache,
                )
                .await;
        }

        if let Some(recovery_data) = &state.bls12381g1_recovery_data {
            restored_key_shares.bls12381g1_shares = recovery_data
                .try_restore(
                    state.threshold,
                    peer_id,
                    epoch,
                    realm_id,
                    staker_address,
                    &state.restored_key_cache,
                )
                .await;
        }

        restored_key_shares
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn mark_keys_restored(&self, restored_key_shares: &RestoredKeyShares) {
        let Some(state) = &mut *self.state.write().await else {
            return;
        };

        if let Some(data) = &mut state.bls_recovery_data {
            EksAndDs::mark_keys_restored(&mut data.eks_and_ds, &restored_key_shares.bls_shares);
        }
        if let Some(data) = &mut state.k256_recovery_data {
            EksAndDs::mark_keys_restored(&mut data.eks_and_ds, &restored_key_shares.k256_shares);
        }
        if let Some(data) = &mut state.p256_recovery_data {
            EksAndDs::mark_keys_restored(&mut data.eks_and_ds, &restored_key_shares.p256_shares);
        }
        if let Some(data) = &mut state.p384_recovery_data {
            EksAndDs::mark_keys_restored(&mut data.eks_and_ds, &restored_key_shares.p384_shares);
        }
        if let Some(data) = &mut state.ed25519_recovery_data {
            EksAndDs::mark_keys_restored(&mut data.eks_and_ds, &restored_key_shares.ed25519_shares);
        }
        if let Some(data) = &mut state.ristretto25519_recovery_data {
            EksAndDs::mark_keys_restored(
                &mut data.eks_and_ds,
                &restored_key_shares.ristretto25519_shares,
            );
        }
        if let Some(data) = &mut state.ed448_recovery_data {
            EksAndDs::mark_keys_restored(&mut data.eks_and_ds, &restored_key_shares.ed448_shares);
        }
        if let Some(data) = &mut state.jubjub_recovery_data {
            EksAndDs::mark_keys_restored(&mut data.eks_and_ds, &restored_key_shares.jubjub_shares);
        }
        if let Some(data) = &mut state.decaf377_recovery_data {
            EksAndDs::mark_keys_restored(
                &mut data.eks_and_ds,
                &restored_key_shares.decaf377_shares,
            );
        }
        if let Some(data) = &mut state.bls12381g1_recovery_data {
            EksAndDs::mark_keys_restored(
                &mut data.eks_and_ds,
                &restored_key_shares.bls12381g1_shares,
            );
        }
    }

    pub fn get_blinders(&self) -> DataVersionReader<Blinders> {
        DataVersionReader::new_unchecked(&self.blinders)
    }

    pub fn get_blinders_mut(&self) -> DataVersionWriter<Blinders> {
        DataVersionWriter::new_unchecked(&self.blinders)
    }

    pub fn set_blinders(&self, blinders: Blinders) {
        DataVersionWriter::store(&self.blinders, blinders);
    }

    pub async fn are_all_keys_restored(&self) -> bool {
        let Some(state) = &*self.state.read().await else {
            return false;
        };

        let restoring_root_keys = DataVersionReader::new_unchecked(&self.restoring_root_keys);

        let mut restored = true;
        for root_key in restoring_root_keys.iter() {
            let r = match root_key.curve_type {
                CurveType::BLS => CurveRecoveryData::are_all_keys_restored(
                    &state.bls_recovery_data,
                    &root_key.public_key,
                ),
                CurveType::K256 => CurveRecoveryData::are_all_keys_restored(
                    &state.k256_recovery_data,
                    &root_key.public_key,
                ),
                CurveType::P256 => CurveRecoveryData::are_all_keys_restored(
                    &state.p256_recovery_data,
                    &root_key.public_key,
                ),
                CurveType::P384 => CurveRecoveryData::are_all_keys_restored(
                    &state.p384_recovery_data,
                    &root_key.public_key,
                ),
                CurveType::Ed25519 => CurveRecoveryData::are_all_keys_restored(
                    &state.ed25519_recovery_data,
                    &root_key.public_key,
                ),
                CurveType::Ed448 => CurveRecoveryData::are_all_keys_restored(
                    &state.ed448_recovery_data,
                    &root_key.public_key,
                ),
                CurveType::Ristretto25519 => CurveRecoveryData::are_all_keys_restored(
                    &state.ristretto25519_recovery_data,
                    &root_key.public_key,
                ),
                CurveType::RedJubjub => CurveRecoveryData::are_all_keys_restored(
                    &state.jubjub_recovery_data,
                    &root_key.public_key,
                ),
                CurveType::RedDecaf377 => CurveRecoveryData::are_all_keys_restored(
                    &state.decaf377_recovery_data,
                    &root_key.public_key,
                ),
                CurveType::BLS12381G1 => CurveRecoveryData::are_all_keys_restored(
                    &state.bls12381g1_recovery_data,
                    &root_key.public_key,
                ),
            };
            debug!(
                "Root key is restored: {} {} {}",
                root_key.curve_type, root_key.public_key, r
            );
            restored &= r;
        }
        restored
    }

    pub async fn get_recovery_party_members(&self) -> Result<Vec<H160>> {
        let Some(state) = &*self.state.read().await else {
            return Err(Self::ciphertexts_not_set());
        };
        Ok(state.recovery_party_members.clone())
    }

    /// Clear the inner_state and regenerate the blinders.
    pub async fn clear(&self) {
        self.set_actively_restoring(false);
        self.set_blinders(Blinders::default());
        *self.state.write().await = None;
    }

    pub fn assert_actively_restoring(&self) -> Result<()> {
        match self.actively_restoring.load(Ordering::Acquire) {
            true => Ok(()),
            false => Err(unexpected_err(
                Error::new(ErrorKind::Other, "Not in RESTORE state"),
                None,
            )),
        }
    }

    pub fn set_actively_restoring(&self, active: bool) {
        self.actively_restoring.store(active, Ordering::Release);
    }

    async fn get_recovered_peer_id(&self) -> Result<PeerId> {
        self.assert_actively_restoring()?;
        let Some(inner) = &mut *self.state.write().await else {
            return Err(Self::ciphertexts_not_set());
        };

        #[rustfmt::skip]
        let peer_id = inner
            .bls_recovery_data.as_ref().and_then(|d| d.original_peer_id())
            .or(inner.k256_recovery_data.as_ref().and_then(|d| d.original_peer_id()))
            .or(inner.p256_recovery_data.as_ref().and_then(|d| d.original_peer_id()))
            .or(inner.p384_recovery_data.as_ref().and_then(|d| d.original_peer_id()))
            .or(inner.ed25519_recovery_data.as_ref().and_then(|d| d.original_peer_id()))
            .or(inner.ristretto25519_recovery_data.as_ref().and_then(|d| d.original_peer_id()))
            .or(inner.ed448_recovery_data.as_ref().and_then(|d| d.original_peer_id()))
            .or(inner.jubjub_recovery_data.as_ref().and_then(|d| d.original_peer_id()))
            .or(inner.decaf377_recovery_data.as_ref().and_then(|d| d.original_peer_id()))
            .or(inner.bls12381g1_recovery_data.as_ref().and_then(|d| d.original_peer_id()));

        match peer_id {
            Some(peer_id) => Ok(PeerId(NonZero::<U256>::from_uint(peer_id))),
            None => Err(Self::ciphertexts_not_set()),
        }
    }

    pub async fn pull_recovered_key_cache(&self) -> Result<KeyCache> {
        self.assert_actively_restoring()?;

        let Some(inner) = &mut *self.state.write().await else {
            return Err(Self::ciphertexts_not_set());
        };

        Ok(inner.restored_key_cache.clone())
    }

    pub async fn report_recovered_peer_id(
        &self,
        cfg: &LitConfig,
        current_peer_id: PeerId,
    ) -> Result<()> {
        let recovered_peer_id = self.get_recovered_peer_id().await?;

        let recovery_contract = match get_backup_recovery_contract_with_signer(cfg).await {
            Ok(recovery_contract) => recovery_contract,
            Err(e) => {
                warn!("RestoreState: failed to get the contract: {}", e);
                return Err(unexpected_err(e, None));
            }
        };

        match recovery_contract
            .set_recovered_peer_id(
                ethers::types::U256::from(recovered_peer_id),
                ethers::types::U256::from(current_peer_id),
            )
            .send()
            .await
        {
            Ok(tx) => {
                info!("RestoreState: reported peer id: {}", recovered_peer_id);
                match tx.confirmations(1).await {
                    Ok(s) => match s {
                        None => {
                            error!("RestoreState: reported peer id - shouldn't have happened");
                        }
                        Some(t) => {
                            info!("RestoreState reported peer id confirmed: {:?}", t);
                        }
                    },
                    Err(e) => {
                        error!("recovered peer id {} not set {}", recovered_peer_id, e);
                    }
                }
            }
            Err(e) => {
                warn!("RestoreState: failed to report the peer id: {}", e);
                if let Some(b) = e.decode_contract_revert::<BackupRecoveryErrors>() {
                    warn!("RestoreState: revert data: {:?}", b);
                }
                return Err(unexpected_err(e, None));
            }
        };
        Ok(())
    }

    pub async fn pull_recovered_peer_ids(&self, cfg: &LitConfig) -> Result<Vec<RecoveredPeerId>> {
        let recovery_contract = match get_backup_recovery_contract_with_signer(cfg).await {
            Ok(recovery_contract) => recovery_contract,
            Err(e) => {
                warn!("RestoreState: failed to get the contract: {}", e);
                return Err(unexpected_err(e, None));
            }
        };

        match recovery_contract.get_recovered_peer_ids().call().await {
            Ok(ids) => {
                info!("RestoreState: pulled recovered peer ids: {:?}", ids);
                Ok(ids)
            }
            Err(e) => {
                warn!("RestoreState: failed to read the recovered peer id: {}", e);
                if let Some(b) = e.decode_contract_revert::<BackupRecoveryErrors>() {
                    warn!("RestoreState: revert data: {:?}", b);
                }
                Err(unexpected_err(e, None))
            }
        }
    }

    // Logs the state of the restore process. Should be used in caution
    // since it logs a rather lengthy piece of information. The info is
    // formatted as json, so that it could be pretty-printed when needed.
    pub async fn log(&self) {
        let log_info = RestoreStateLog::from_restore_state(self).await;
        match serde_json::to_string(&log_info) {
            Ok(json) => info!("{}", json),
            Err(e) => info!("{:?}", log_info), // should never happen in practice.
        }
    }

    #[cfg(test)]
    pub async fn get_number_of_bls_ciphertexts(&self) -> usize {
        let Some(state) = &*self.state.read().await else {
            return 0;
        };
        state
            .bls_recovery_data
            .as_ref()
            .map_or(0, |d| d.eks_and_ds.len())
    }

    #[cfg(test)]
    pub async fn get_number_of_k256_ciphertexts(&self) -> usize {
        let Some(state) = &*self.state.read().await else {
            return 0;
        };
        state
            .k256_recovery_data
            .as_ref()
            .map_or(0, |d| d.eks_and_ds.len())
    }

    pub async fn get_restored_threshold(&self) -> usize {
        let Some(state) = &*self.state.read().await else {
            return 0;
        };
        state.threshold
    }

    #[cfg(test)]
    pub(crate) async fn fetch_bls_backup_by_pubkey(
        &self,
        pubkey: &str,
    ) -> Option<EksAndDs<InnerBls12381G1>> {
        let Some(state) = &*self.state.read().await else {
            return None;
        };
        self.fetch_backup(pubkey, &state.bls_recovery_data).cloned()
    }

    #[cfg(test)]
    pub(crate) async fn fetch_k256_backup_by_pubkey(
        &self,
        pubkey: &str,
    ) -> Option<EksAndDs<k256::Secp256k1>> {
        let Some(state) = &*self.state.read().await else {
            return None;
        };
        self.fetch_backup(pubkey, &state.k256_recovery_data)
            .cloned()
    }

    #[cfg(test)]
    fn fetch_backup<'a, C>(
        &'a self,
        pubkey: &str,
        curve_recovery_data: &'a Option<CurveRecoveryData<C>>,
    ) -> Option<&'a EksAndDs<C>>
    where
        C: VerifiableEncryptionDecryptor,
        C::Point: CompressedBytes,
        C::Scalar: CompressedBytes + From<PeerId>,
    {
        curve_recovery_data.as_ref().and_then(|d| {
            d.eks_and_ds
                .iter()
                .find(|x| x.encrypted_key_share.public_key == pubkey)
        })
    }

    #[cfg(test)]
    pub(crate) async fn bls_eksandds(&self) -> Option<EksAndDs<InnerBls12381G1>> {
        let Some(state) = &*self.state.read().await else {
            return None;
        };
        state
            .bls_recovery_data
            .as_ref()
            .map(|d| d.eks_and_ds.first())
            .flatten()
            .cloned()
    }

    #[cfg(test)]
    pub(crate) async fn k256_eksandds(&self) -> Option<EksAndDs<k256::Secp256k1>> {
        let Some(state) = &*self.state.read().await else {
            return None;
        };
        state
            .k256_recovery_data
            .as_ref()
            .map(|d| d.eks_and_ds.first())
            .flatten()
            .cloned()
    }

    fn ciphertexts_not_set() -> crate::error::Error {
        unexpected_err(
            Error::new(ErrorKind::Other, "Ciphertexts are not yet set"),
            None,
        )
    }

    #[instrument(level = "debug", skip_all)]
    fn add_decryption_share<C>(
        recovery_data: &mut Option<CurveRecoveryData<C>>,
        rpm_id: &RecPartyMemberIdType,
        share_data: &UploadedShareData,
    ) -> Result<()>
    where
        C: VerifiableEncryptionDecryptor,
        C::Point: CompressedBytes,
        C::Scalar: CompressedBytes + From<PeerId>,
    {
        let decryption_share = match serde_json::from_str(&share_data.decryption_share) {
            Ok(share) => share,
            Err(e) => return Err(conversion_err(e, None)),
        };

        Self::do_add_decryption_share(recovery_data, rpm_id, share_data, decryption_share, None)
    }

    #[instrument(level = "debug", skip_all)]
    fn do_add_decryption_share<C>(
        recovery_data: &mut Option<CurveRecoveryData<C>>,
        rpm_id: &RecPartyMemberIdType,
        share_data: &UploadedShareData,
        decryption_share: DecryptionShare<C>,
        // DATIL_BACKUP: Remove this argument once old Datil backup is obsolete.
        normalized_pub_key: Option<String>,
    ) -> Result<()>
    where
        C: VerifiableEncryptionDecryptor,
        C::Point: CompressedBytes,
        C::Scalar: CompressedBytes + From<PeerId>,
    {
        let recovery_data = match recovery_data {
            Some(rd) => rd,
            None => {
                let err_msg = format!("Curve is not being restored: {}", share_data.curve);
                return Err(parser_err(Error::new(ErrorKind::Other, err_msg), None));
            }
        };

        let pub_key = normalized_pub_key.unwrap_or(share_data.verification_key.clone());
        if share_data.encryption_key != recovery_data.encryption_key.to_compressed_hex() {
            let err_msg = format!(
                "Decryption share for root key {} is generated with wrong recovery key. \
                Expected: {}, found: {}",
                pub_key,
                recovery_data.encryption_key.to_compressed_hex(),
                share_data.encryption_key,
            );
            return Err(unexpected_err(Error::new(ErrorKind::Other, err_msg), None));
        }

        for eks_and_ds in recovery_data.eks_and_ds.iter_mut() {
            if pub_key == eks_and_ds.encrypted_key_share.public_key {
                eks_and_ds
                    .decryption_shares
                    .insert(rpm_id.clone(), decryption_share);
                info!(
                    "Inserted {} decryption share for pubkey {} from member {}",
                    share_data.curve, pub_key, rpm_id
                );
                return Ok(());
            }
        }

        let err_msg = format!(
            "An encrypted key share with pub_key {} does not exist.",
            share_data.verification_key
        );
        Err(unexpected_err(Error::new(ErrorKind::Other, err_msg), None))
    }
}

#[derive(Debug, Default)]
pub struct RestoredKeyShares {
    pub bls_shares: Vec<String>,
    pub k256_shares: Vec<String>,
    pub p256_shares: Vec<String>,
    pub p384_shares: Vec<String>,
    pub ed25519_shares: Vec<String>,
    pub ristretto25519_shares: Vec<String>,
    pub ed448_shares: Vec<String>,
    pub jubjub_shares: Vec<String>,
    pub decaf377_shares: Vec<String>,
    pub bls12381g1_shares: Vec<String>,
}

/// Used to log the state of the disaster recovery.
#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreStateLog {
    actively_restoring: bool,
    backups_loaded: bool,
    recovery_party_members: Vec<H160>,
    bls_enc_key: Option<String>,
    k256_enc_key: Option<String>,
    p256_enc_key: Option<String>,
    p384_enc_key: Option<String>,
    ed25519_enc_key: Option<String>,
    ristretto25519_enc_key: Option<String>,
    ed448_enc_key: Option<String>,
    jubjub_enc_key: Option<String>,
    decaf377_enc_key: Option<String>,
    bls12381g1_enc_key: Option<String>,
    bls_shares: Vec<RootKeyRecoveryLog>,
    k256_shares: Vec<RootKeyRecoveryLog>,
    p256_shares: Vec<RootKeyRecoveryLog>,
    p384_shares: Vec<RootKeyRecoveryLog>,
    ed25519_shares: Vec<RootKeyRecoveryLog>,
    ristretto25519_shares: Vec<RootKeyRecoveryLog>,
    ed448_shares: Vec<RootKeyRecoveryLog>,
    jubjub_shares: Vec<RootKeyRecoveryLog>,
    decaf377_shares: Vec<RootKeyRecoveryLog>,
    bls12381g1_shares: Vec<RootKeyRecoveryLog>,
    threshold: usize,
}

impl RestoreStateLog {
    pub async fn from_restore_state(restore_state: &RestoreState) -> Self {
        let reader = restore_state.state.read().await;
        match &*reader {
            Some(state) => Self {
                actively_restoring: restore_state.actively_restoring.load(Ordering::Acquire),
                backups_loaded: true,
                recovery_party_members: state.recovery_party_members.clone(),
                bls_enc_key: CurveRecoveryData::encryption_key(&state.bls_recovery_data),
                k256_enc_key: CurveRecoveryData::encryption_key(&state.k256_recovery_data),
                p256_enc_key: CurveRecoveryData::encryption_key(&state.p256_recovery_data),
                p384_enc_key: CurveRecoveryData::encryption_key(&state.p384_recovery_data),
                ed25519_enc_key: CurveRecoveryData::encryption_key(&state.ed25519_recovery_data),
                ristretto25519_enc_key: CurveRecoveryData::encryption_key(
                    &state.ristretto25519_recovery_data,
                ),
                ed448_enc_key: CurveRecoveryData::encryption_key(&state.ed448_recovery_data),
                jubjub_enc_key: CurveRecoveryData::encryption_key(&state.jubjub_recovery_data),
                decaf377_enc_key: CurveRecoveryData::encryption_key(&state.decaf377_recovery_data),
                bls12381g1_enc_key: CurveRecoveryData::encryption_key(
                    &state.bls12381g1_recovery_data,
                ),
                bls_shares: CurveRecoveryData::log_shares(&state.bls_recovery_data),
                k256_shares: CurveRecoveryData::log_shares(&state.k256_recovery_data),
                p256_shares: CurveRecoveryData::log_shares(&state.p256_recovery_data),
                p384_shares: CurveRecoveryData::log_shares(&state.p384_recovery_data),
                ed25519_shares: CurveRecoveryData::log_shares(&state.ed25519_recovery_data),
                ristretto25519_shares: CurveRecoveryData::log_shares(
                    &state.ristretto25519_recovery_data,
                ),
                ed448_shares: CurveRecoveryData::log_shares(&state.ed448_recovery_data),
                jubjub_shares: CurveRecoveryData::log_shares(&state.jubjub_recovery_data),
                decaf377_shares: CurveRecoveryData::log_shares(&state.decaf377_recovery_data),
                bls12381g1_shares: CurveRecoveryData::log_shares(&state.bls12381g1_recovery_data),
                threshold: state.threshold,
            },
            None => Self {
                actively_restoring: restore_state.actively_restoring.load(Ordering::Acquire),
                backups_loaded: false,
                recovery_party_members: Default::default(),
                bls_enc_key: Default::default(),
                k256_enc_key: Default::default(),
                p256_enc_key: Default::default(),
                p384_enc_key: Default::default(),
                ed25519_enc_key: Default::default(),
                ristretto25519_enc_key: Default::default(),
                ed448_enc_key: Default::default(),
                jubjub_enc_key: Default::default(),
                decaf377_enc_key: Default::default(),
                bls12381g1_enc_key: Default::default(),
                bls_shares: Default::default(),
                k256_shares: Default::default(),
                p256_shares: Default::default(),
                p384_shares: Default::default(),
                ed25519_shares: Default::default(),
                ristretto25519_shares: Default::default(),
                ed448_shares: Default::default(),
                jubjub_shares: Default::default(),
                decaf377_shares: Default::default(),
                bls12381g1_shares: Default::default(),
                threshold: 0,
            },
        }
    }
}

#[derive(Debug)]
pub enum NodeRecoveryStatus {
    Null,
    StartedInRestoreState,
    BackupsAreLoaded,
    AllKeysAreRestored,
    AbandonedRecoveryDueToNetworkState,
}

#[instrument(level = "debug", skip(cfg))]
pub async fn report_progress(cfg: &LitConfig, status: NodeRecoveryStatus) {
    let recovery_contract = match get_backup_recovery_contract_with_signer(cfg).await {
        Ok(recovery_contract) => recovery_contract,
        Err(e) => {
            warn!("RestoreState: failed to report progress: {}", e);
            return;
        }
    };

    match recovery_contract
        .set_node_recovery_status(status as u8)
        .send()
        .await
    {
        Ok(tx) => {
            info!("RestoreState: reported progress to recovery contract");
            match tx.confirmations(1).await {
                Ok(s) => match s {
                    None => {
                        warn!(
                            "RestoreState: shouldn't happen to confirm recovery contract for reporting progress"
                        );
                    }
                    Some(t) => {
                        info!(
                            "RestoreState: confirmed recovery contract for reporting progress: {:?}",
                            t
                        );
                    }
                },
                Err(e) => {
                    warn!(
                        "RestoreState: failed to confirm recovery contract for reporting progress: {}",
                        e
                    );
                }
            }
        }
        Err(e) => {
            warn!("RestoreState: failed to report progress: {}", e);
            if let Some(b) = e.decode_contract_revert::<BackupRecoveryErrors>() {
                warn!("RestoreState: revert data: {:?}", b);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::key_helper::KeyCache;
    use crate::tss::common::key_persistence::KeyPersistence;
    use crate::tss::common::key_share::KeyShare;
    use crate::tss::common::restore::eks_and_ds::DecryptionShare;
    use crate::tss::common::restore::point_reader::PointReader;
    use crate::tss::common::storage::{
        StorageType, read_key_share_from_disk, read_recovery_data_from_disk,
    };
    use async_std::path::PathBuf;
    use blsful::inner_types::G1Projective;
    use elliptic_curve::Group;
    use elliptic_curve::group::GroupEncoding;
    use k256::Secp256k1;
    use lit_node_core::CompressedBytes;
    use lit_node_core::CompressedHex;
    use lit_node_core::CurveType;
    use lit_recovery::models::{EncryptedKeyShare, OldEncryptedKeyShare, UploadedShareData};
    use verifiable_share_encryption::VerifiableEncryption;
    use vsss_rs::{DefaultShare, IdentifierPrimeField};

    #[tokio::test]
    async fn test_generate_serialize_and_deserialize_blinders() {
        let blinders = RestoreState::generate_blinders();
        assert_gen_ser_de_works::<<InnerBls12381G1 as BCA>::Point>(
            CurveType::BLS,
            blinders.bls_blinder,
        );
        assert_gen_ser_de_works::<k256::ProjectivePoint>(CurveType::K256, blinders.k256_blinder);
        assert_gen_ser_de_works::<p256::ProjectivePoint>(CurveType::P256, blinders.p256_blinder);
        assert_gen_ser_de_works::<p384::ProjectivePoint>(CurveType::P384, blinders.p384_blinder);
        assert_gen_ser_de_works::<vsss_rs::curve25519::WrappedEdwards>(
            CurveType::Ed25519,
            blinders.ed25519_blinder,
        );
        assert_gen_ser_de_works::<vsss_rs::curve25519::WrappedRistretto>(
            CurveType::Ristretto25519,
            blinders.ristretto25519_blinder,
        );
        assert_gen_ser_de_works::<ed448_goldilocks::EdwardsPoint>(
            CurveType::Ed448,
            blinders.ed448_blinder,
        );
        assert_gen_ser_de_works::<jubjub::SubgroupPoint>(
            CurveType::RedJubjub,
            blinders.jubjub_blinder,
        );
        assert_gen_ser_de_works::<decaf377::Element>(
            CurveType::RedDecaf377,
            blinders.decaf377_blinder,
        );
        assert_gen_ser_de_works::<<InnerBls12381G1 as BCA>::Point>(
            CurveType::BLS12381G1,
            blinders.bls12381g1_blinder,
        );
    }

    fn assert_gen_ser_de_works<G>(curve_type: CurveType, blinder: Option<G::Scalar>)
    where
        G: Group + GroupEncoding + Default + CompressedBytes,
        G::Scalar: CompressedBytes,
    {
        let helper = KeyPersistence::<G>::new(curve_type);
        let blinder = blinder.unwrap();
        let hex = helper.secret_to_hex(&blinder);
        let read_blinder = helper.secret_from_hex(&hex).unwrap();
        assert_eq!(blinder, read_blinder);
    }

    // Set blinders
    const BLS_BLINDER: &str = "23bd541973cfdab719c5079fc9d0433c194884fafee256ebab7298c813e23e6e";
    const K256_BLINDER: &str = "837405A0C9C4056299779C7A09BB5C8A3C5EF4C4F4650FE96A32B21C7CF3599D";

    // Encryption Keys
    const BLS_ENC_KEY: &str = "b0aa1aeaf1f4fa72e59905a4d0723ce4b6f53a277f75b38c9ae87a31fa7d40825c22b83dd18e821a316303e69681ee66";
    const K256_ENC_KEY: &str = "02a220b4caab1baa5d0b24612743803f1b40980ad56b7904ed83da3e012eb366a2";
    const BLS_DECRYPTION_KEY_SHARE_0: &str =
        "203c57923c4bbea92de6d996c565a340d82a898cfaf46263e20ef5d3c8791dc8";
    const BLS_DECRYPTION_KEY_SHARE_1: &str =
        "63fc660886a20faec04d0e8241ed8bf9556dee50498d8a7a98043cd5d43020fc";
    // const BLS_DECRYPTION_KEY_SHARE_2: &str =
    //    "33cecd2ba75ae36c1f796b65b4d39cac7ef3af10982856924df983d8dfe7242f";
    const K256_DECRYPTION_KEY_SHARE_0: &str =
        "30c0e6d9b645222fe6aacd653dc30a6aa3e0677973f55f47cbd2a1adbe649232";
    const K256_DECRYPTION_KEY_SHARE_1: &str =
        "9e1747c410032b8182a6336a55cc55280d62e1a1a01c464f4201107698223565";
    // const K256_DECRYPTION_KEY_SHARE_2: &str =
    //    "0b6da8ae69c134d31ea1996f6dd59fe6bc367ee31cfa8d1af85d20b2a1a99757";

    // Root keys
    const BLS_ROOT_KEY: &str = "83e63aebc6550937d5d4a5ef38b12c8f3a31f19a02202019e7bed30a1d0bfe49989287ffc2022f18d9c687ba9fe1ce29";
    const BLS_ROOT_PRIV_SHARE: &str =
        "13d296b0cd969aa67fe6aa50399919136d4dd0bbb16c95641a012dcaf0f92229";
    const K256_ROOT_KEY: &str =
        "029a18b213e730443c6d40c19b0a342c9e3605b7553aed17dbadae08c9754baf3f";
    const K256_ROOT_PRIV_SHARE: &str =
        "1ec291bf8fb5e54fb8a6b07442b00b38100be1a7ca6ab0641c182f6fd498da5f";

    #[tokio::test]
    async fn test_decrypt_key_shares_with_decryption_shares() {
        let bls_helper = KeyPersistence::<G1Projective>::new(CurveType::BLS);
        let k256_helper = KeyPersistence::<k256::ProjectivePoint>::new(CurveType::K256);

        // Create an instance of RestoreState
        let mut blinders = Blinders::default();
        blinders.bls_blinder = Some(bls_helper.secret_from_hex(BLS_BLINDER).unwrap());
        blinders.k256_blinder = Some(k256_helper.secret_from_hex(K256_BLINDER).unwrap());
        let restore_state = RestoreState {
            blinders: AtomicShared::new(blinders),
            actively_restoring: AtomicBool::new(true),
            state: RwLock::new(None),
            restoring_root_keys: AtomicShared::new(vec![
                CachedRootKey {
                    curve_type: CurveType::BLS,
                    public_key: BLS_ROOT_KEY.to_string(),
                },
                CachedRootKey {
                    curve_type: CurveType::K256,
                    public_key: K256_ROOT_KEY.to_string(),
                },
            ]),
        };

        // Generate recovery party members
        let recovery_party_members = vec![H160::random(), H160::random(), H160::random()];
        let recovery_member_0 = hex::encode(recovery_party_members[0].as_bytes());
        let recovery_member_1 = hex::encode(recovery_party_members[1].as_bytes());
        let recovery_member_2 = hex::encode(recovery_party_members[2].as_bytes());
        let realm_id = 1;
        let key_cache = KeyCache::default();
        // Load encrypted shares into the RestoreState instance
        let bls_enc_key = bls_helper.pk_from_hex(BLS_ENC_KEY).unwrap();
        let k256_enc_key = k256_helper.pk_from_hex(K256_ENC_KEY).unwrap();
        let path =
            PathBuf::from("./tests/test_data/test_decrypt_key_shares_with_decryption_shares/");
        let old_encrypted_bls_shares: Vec<OldEncryptedKeyShare<InnerBls12381G1>> =
            read_recovery_data_from_disk(
                &path,
                "*",
                StorageType::KeyShare(CurveType::BLS),
                &key_cache,
            )
            .await
            .unwrap();

        let encrypted_bls_shares: Vec<_> = old_encrypted_bls_shares
            .iter()
            .cloned()
            .map(|s| {
                let mut eks = EncryptedKeyShare::from(s);
                let pub_key =
                    InnerBls12381G1::parse_old_backup_public_key(&eks.public_key).unwrap();
                eks.public_key = pub_key.to_compressed_hex();
                EksAndDs::new(eks, None, CurveType::BLS).unwrap()
            })
            .collect();

        let encrypted_bls_share = encrypted_bls_shares[0].encrypted_key_share.clone();
        let old_encrypted_k256_shares: Vec<OldEncryptedKeyShare<Secp256k1>> =
            read_recovery_data_from_disk(
                &path,
                "*",
                StorageType::KeyShare(CurveType::K256),
                &key_cache,
            )
            .await
            .unwrap();

        let encrypted_k256_shares: Vec<_> = old_encrypted_k256_shares
            .iter()
            .cloned()
            .map(|s| {
                let mut eks = EncryptedKeyShare::from(s);
                let pub_key = Secp256k1::parse_old_backup_public_key(&eks.public_key).unwrap();
                eks.public_key = pub_key.to_compressed_hex();
                EksAndDs::new(eks, None, CurveType::K256).unwrap()
            })
            .collect();

        let encrypted_k256_share = encrypted_k256_shares[0].encrypted_key_share.clone();
        let mut inner = InnerState::default();
        inner.recovery_party_members = recovery_party_members;
        inner.threshold = 2;
        inner.bls_recovery_data = Some(CurveRecoveryData {
            encryption_key: bls_enc_key,
            blinder: restore_state.get_blinders().bls_blinder.unwrap().clone(),
            eks_and_ds: encrypted_bls_shares,
        });
        inner.k256_recovery_data = Some(CurveRecoveryData {
            encryption_key: k256_enc_key,
            blinder: restore_state.get_blinders().k256_blinder.unwrap().clone(),
            eks_and_ds: encrypted_k256_shares,
        });
        restore_state.load_backup(inner).await.unwrap();

        // Assert that the ciphertexts are loaded.
        assert_eq!(restore_state.get_number_of_bls_ciphertexts().await, 1);
        assert_eq!(restore_state.get_number_of_k256_ciphertexts().await, 1);

        // Generate and submit BLS decryption shares
        let bls_decryption_share_0 = generate_decryption_share(
            &encrypted_bls_share,
            String::from(BLS_DECRYPTION_KEY_SHARE_0),
            1,
            CurveType::BLS,
            String::from(BLS_ENC_KEY),
        );
        let bls_decryption_share_1 = generate_decryption_share(
            &encrypted_bls_share,
            String::from(BLS_DECRYPTION_KEY_SHARE_1),
            2,
            CurveType::BLS,
            String::from(BLS_ENC_KEY),
        );

        // Generate and submit k256 decryption shares
        let k256_decryption_share_0 = generate_decryption_share(
            &encrypted_k256_share,
            String::from(K256_DECRYPTION_KEY_SHARE_0),
            1,
            CurveType::K256,
            String::from(K256_ENC_KEY),
        );
        let k256_decryption_share_1 = generate_decryption_share(
            &encrypted_k256_share,
            String::from(K256_DECRYPTION_KEY_SHARE_1),
            2,
            CurveType::K256,
            String::from(K256_ENC_KEY),
        );

        restore_state
            .add_decryption_shares(
                &recovery_member_0,
                &vec![bls_decryption_share_0, k256_decryption_share_0],
            )
            .await
            .unwrap();

        restore_state
            .add_decryption_shares(
                &recovery_member_1,
                &vec![bls_decryption_share_1, k256_decryption_share_1],
            )
            .await
            .unwrap();

        // Make sure to restore all the root keys
        let staker_address = "SomeStakerAddress";
        let peer_id = PeerId::try_from(555usize).unwrap();
        let epoch = 0;
        let restored_key_shares = restore_state
            .try_restore_key_shares(&peer_id, epoch, staker_address, realm_id)
            .await;
        assert_eq!(restored_key_shares.bls_shares.len(), 1);
        assert_eq!(
            restored_key_shares.bls_shares[0],
            encrypted_bls_share.public_key
        );
        assert_eq!(restored_key_shares.k256_shares.len(), 1);
        assert_eq!(
            restored_key_shares.k256_shares[0],
            encrypted_k256_share.public_key
        );

        restore_state.mark_keys_restored(&restored_key_shares).await;
        assert!(restore_state.are_all_keys_restored().await);

        let recovered_key_cache = restore_state.pull_recovered_key_cache().await.unwrap();
        // Make sure that the recovered keys are written to the disk where we would expect them.
        let bls_keyshare = read_key_share_from_disk::<KeyShare>(
            CurveType::BLS,
            BLS_ROOT_KEY,
            staker_address,
            &peer_id,
            epoch,
            realm_id,
            &recovered_key_cache,
        )
        .await
        .unwrap();
        let k256_keyshare = read_key_share_from_disk::<KeyShare>(
            CurveType::K256,
            K256_ROOT_KEY,
            staker_address,
            &peer_id,
            epoch,
            realm_id,
            &recovered_key_cache,
        )
        .await
        .unwrap();

        // Make sure that the decrypted private key shares are what we expect them to be.
        assert_eq!(k256_keyshare.hex_private_share, K256_ROOT_PRIV_SHARE);
        assert_eq!(bls_keyshare.hex_private_share, BLS_ROOT_PRIV_SHARE);
    }

    // Helper function partly taken from the recovery tool.
    fn generate_decryption_share<C>(
        encrypted_key_share: &EncryptedKeyShare<C>,
        decryption_key_share: String,
        participant_id: u16,
        curve_type: CurveType,
        encryption_key: String,
    ) -> UploadedShareData
    where
        C: VerifiableEncryption + VerifiableEncryptionDecryptor,
    {
        use elliptic_curve::PrimeField;
        let mut decryption_key_bytes = hex::decode(decryption_key_share).unwrap();
        if curve_type == CurveType::BLS {
            decryption_key_bytes.reverse(); // Converting from Big Endian to Little Endian which is required by DecryptionShare
        }
        let mut repr = <C::Scalar as PrimeField>::Repr::default();
        repr.as_mut().copy_from_slice(&decryption_key_bytes);
        let value = C::Scalar::from_repr(repr).unwrap();
        let share = DefaultShare {
            identifier: IdentifierPrimeField(C::Scalar::from(participant_id as u64)),
            value: IdentifierPrimeField(value),
        };

        let decryption_share = DecryptionShare::<C>::new(&share, &encrypted_key_share.ciphertext);
        UploadedShareData {
            participant_id: 0, // Temporarily for backward compatibility with Datil
            session_id: Default::default(),
            encryption_key,
            verification_key: encrypted_key_share.public_key.clone(),
            decryption_share: serde_json::to_string(&decryption_share).unwrap(),
            subnet_id: Default::default(),
            curve: curve_type.as_str().to_string(),
        }
    }
}
