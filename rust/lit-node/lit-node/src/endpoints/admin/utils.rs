use crate::common::key_helper::KeyCache;
use crate::common::storage::{read_from_disk, write_to_disk};
use crate::error::{parser_err, unexpected_err_code};
use crate::tss::common::backup::RecoveryParty;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::restore::{
    InnerState, RestoreState,
    eks_and_ds::{CurveRecoveryData, EksAndDs},
    point_reader::PointReader,
};
use crate::tss::common::storage::{
    StorableFile, StorageType, copy_key_share_commitments_to_another_path,
    read_key_share_from_disk, read_recovery_data_from_disk,
};
use async_std::fs;
use async_std::path::{Path, PathBuf};
use blsful::inner_types::{G1Projective, GroupEncoding, InnerBls12381G1};
use bulletproofs::BulletproofCurveArithmetic as BCA;
use chrono::{DateTime, Utc};
use elliptic_curve::Group;
use k256::Secp256k1;
use lit_core::config::LitConfig;
use lit_core::error::Unexpected;
use lit_node_common::config::{LitNodeConfig, encrypted_key_path};
use lit_node_core::CurveType;
use lit_node_core::JsonAuthSig;
use lit_recovery::models::{EncryptedKeyShare, OldEncryptedKeyShare};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::process::Command;
use tokio_stream::StreamExt;
use tracing::trace;

use crate::endpoints::auth_sig::{LITNODE_ADMIN_RES, check_auth_sig};
use crate::error::{EC, Result, io_err, io_err_code, unexpected_err};
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tss::common::backup::BackupGenerator;
use crate::tss::common::key_share_commitment::KeyShareCommitments;
use crate::utils::traits::SignatureCurve;
use lit_node_core::PeerId;
use lit_node_core::{Blinders, CompressedBytes, CompressedHex};
use verifiable_share_encryption::{VerifiableEncryption, VerifiableEncryptionDecryptor};

pub(crate) fn check_admin_auth_sig(config: &LitConfig, auth_sig: &JsonAuthSig) -> Result<()> {
    let admin_address = config.admin_address()?;
    check_auth_sig(config, auth_sig, LITNODE_ADMIN_RES, &[admin_address])
}

// File names in tar'ed backup directory
const RECOVERY_PARTY_WALLET_ADDRESSES_FN: &str = "recovery_party_wallet_addresses";
const SESSION_ID_FN: &str = "session_id";
const ENCRYPTION_KEY_FN: &str = "_encryption_key";
const BLINDER_COMMITMENT_FN: &str = "_blinder_commitment";
const RECOVERY_PARTY_THRESHOLD_FN: &str = "threshold";
const VERSION_NO_FN: &str = "version_no";
const PEERS_FN: &str = "peers";
const VERSION_NO: u8 = 1;

fn enc_key_fn(curve_type: CurveType) -> String {
    format!("{}{}", curve_type.backup_prefix(), ENCRYPTION_KEY_FN)
}

fn blinder_comm_fn(curve_type: CurveType) -> String {
    format!("{}{}", curve_type.backup_prefix(), BLINDER_COMMITMENT_FN)
}

pub(crate) async fn encrypt_and_tar_backup_keys(
    cfg: Arc<LitConfig>,
    peer_id: PeerId,
    key_set_root_keys: &HashMap<CurveType, Vec<String>>,
    blinders: &Blinders,
    recovery_party: &RecoveryParty,
    peers: &SimplePeerCollection,
    epoch: u64,
) -> Result<Vec<u8>> {
    info!("Encrypting and tar'ing backup keys");
    let now: DateTime<Utc> = Utc::now();

    let staker_address = crate::endpoints::recovery::get_staker_address(&cfg)?;

    // Create the temporary dir in which we will save the resulting artifacts.
    let mut path = encrypted_key_path(&staker_address);
    let _ = std::fs::remove_dir_all(path.clone());
    path.push(format!("backup-{}/", now));
    fs::create_dir_all(&path)
        .await
        .map_err(|e| io_err(e, None))?;
    info!("Created backup directory {:?}", path);

    // Add a version no.
    write_to_disk(path.clone(), VERSION_NO_FN, &VERSION_NO).await?;

    // Get recovery party information and save them in the folder.
    write_to_disk(path.clone(), SESSION_ID_FN, &recovery_party.session_id).await?;
    write_to_disk(
        path.clone(),
        RECOVERY_PARTY_THRESHOLD_FN,
        &recovery_party.threshold,
    )
    .await?;
    write_to_disk(
        path.clone(),
        RECOVERY_PARTY_WALLET_ADDRESSES_FN,
        &recovery_party.party_members,
    )
    .await?;
    write_to_disk(path.clone(), PEERS_FN, peers).await?;
    info!(
        "Recovery party wallet addresses: {:?}",
        recovery_party.party_members
    );

    let key_cache = KeyCache::default();
    let mut tasks = tokio::task::JoinSet::new();
    let write_curve_recovery_data_args = Arc::new(WriteCurveRecoveryDataArgs {
        cfg: cfg.clone(),
        peer_id,
        root_keys: key_set_root_keys.clone(),
        epoch,
        staker_address: staker_address.clone(),
        peers: peers.clone(),
        path: path.clone(),
    });

    let args = write_curve_recovery_data_args.clone();
    let bls_encryption_key = recovery_party.bls_encryption_key;
    let bls_blinder = blinders
        .bls_blinder
        .ok_or(blinder_not_set_err(CurveType::BLS))?;
    tasks.spawn(async move {
        write_curve_recovery_data::<InnerBls12381G1>(
            args,
            CurveType::BLS,
            &bls_encryption_key,
            &bls_blinder,
            &(G1Projective::GENERATOR * bls_blinder),
        )
        .await
    });

    let args = write_curve_recovery_data_args.clone();
    let k256_encryption_key = recovery_party.k256_encryption_key;
    let k256_blinder = blinders
        .k256_blinder
        .ok_or(blinder_not_set_err(CurveType::K256))?;
    let k256_commitment = tasks.spawn(async move {
        write_curve_recovery_data::<Secp256k1>(
            args,
            CurveType::K256,
            &k256_encryption_key,
            &k256_blinder,
            &(k256::ProjectivePoint::GENERATOR * k256_blinder),
        )
        .await
    });

    let args = write_curve_recovery_data_args.clone();
    let p256_encryption_key = recovery_party.p256_encryption_key;
    let p256_blinder = blinders
        .p256_blinder
        .ok_or(blinder_not_set_err(CurveType::P256))?;
    tasks.spawn(async move {
        write_curve_recovery_data::<p256::NistP256>(
            args,
            CurveType::P256,
            &p256_encryption_key,
            &p256_blinder,
            &(p256::ProjectivePoint::GENERATOR * p256_blinder),
        )
        .await
    });

    let args = write_curve_recovery_data_args.clone();
    let p384_encryption_key = recovery_party.p384_encryption_key;
    let p384_blinder = blinders
        .p384_blinder
        .ok_or(blinder_not_set_err(CurveType::P384))?;
    tasks.spawn(async move {
        write_curve_recovery_data::<p384::NistP384>(
            args,
            CurveType::P384,
            &p384_encryption_key,
            &p384_blinder,
            &(p384::ProjectivePoint::GENERATOR * p384_blinder),
        )
        .await
    });

    let args = write_curve_recovery_data_args.clone();
    let ed25519_encryption_key = recovery_party.ed25519_encryption_key;
    let ed25519_blinder = blinders
        .ed25519_blinder
        .ok_or(blinder_not_set_err(CurveType::Ed25519))?;
    tasks.spawn(async move {
        write_curve_recovery_data::<bulletproofs::Ed25519>(
            args,
            CurveType::Ed25519,
            &ed25519_encryption_key,
            &ed25519_blinder,
            &(vsss_rs::curve25519::WrappedEdwards::generator() * ed25519_blinder),
        )
        .await
    });

    let args = write_curve_recovery_data_args.clone();
    let ristretto25519_encryption_key = recovery_party.ristretto25519_encryption_key;
    let ristretto25519_blinder = blinders
        .ristretto25519_blinder
        .ok_or(blinder_not_set_err(CurveType::Ristretto25519))?;
    tasks.spawn(async move {
        write_curve_recovery_data::<bulletproofs::Ristretto25519>(
            args,
            CurveType::Ristretto25519,
            &ristretto25519_encryption_key,
            &ristretto25519_blinder,
            &(vsss_rs::curve25519::WrappedRistretto::generator() * ristretto25519_blinder),
        )
        .await
    });

    let args = write_curve_recovery_data_args.clone();
    let ed448_encryption_key = recovery_party.ed448_encryption_key;
    let ed448_blinder = blinders
        .ed448_blinder
        .ok_or(blinder_not_set_err(CurveType::Ed448))?;
    tasks.spawn(async move {
        write_curve_recovery_data::<ed448_goldilocks::Ed448>(
            args,
            CurveType::Ed448,
            &ed448_encryption_key,
            &ed448_blinder,
            &(ed448_goldilocks::EdwardsPoint::GENERATOR * ed448_blinder),
        )
        .await
    });

    let args = write_curve_recovery_data_args.clone();
    let jubjub_encryption_key = recovery_party.jubjub_encryption_key;
    let jubjub_blinder = blinders
        .jubjub_blinder
        .ok_or(blinder_not_set_err(CurveType::RedJubjub))?;
    tasks.spawn(async move {
        write_curve_recovery_data::<bulletproofs::JubJub>(
            args,
            CurveType::RedJubjub,
            &jubjub_encryption_key,
            &jubjub_blinder,
            &(jubjub::SubgroupPoint::generator() * jubjub_blinder),
        )
        .await
    });

    let args = write_curve_recovery_data_args.clone();
    let decaf377_encryption_key = recovery_party.decaf377_encryption_key;
    let decaf377_blinder = blinders
        .decaf377_blinder
        .ok_or(blinder_not_set_err(CurveType::RedDecaf377))?;
    tasks.spawn(async move {
        write_curve_recovery_data::<bulletproofs::Decaf377>(
            args,
            CurveType::RedDecaf377,
            &decaf377_encryption_key,
            &decaf377_blinder,
            &(decaf377::Element::GENERATOR * decaf377_blinder),
        )
        .await
    });

    let args = write_curve_recovery_data_args.clone();
    let bls12381g1_encryption_key = recovery_party.bls12381g1_encryption_key;
    let bls12381g1_blinder = blinders
        .bls12381g1_blinder
        .ok_or(blinder_not_set_err(CurveType::BLS12381G1))?;
    tasks.spawn(async move {
        write_curve_recovery_data::<InnerBls12381G1>(
            args,
            CurveType::BLS12381G1,
            &bls12381g1_encryption_key,
            &bls12381g1_blinder,
            &(G1Projective::GENERATOR * bls12381g1_blinder),
        )
        .await
    });

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                error!("Failed to generate backup data: {}", e);
                return Err(io_err(e, None));
            }
            Err(e) => return Err(io_err(e, None)),
        }
    }
    trace!("All keys encrypted and saved");

    // zip up the newly created backup directory
    // equivalent to tar -czf - <path> ...
    let mut buffer = Vec::with_capacity(8192);
    lit_core::utils::tar::write_tar_gz(&path, &mut buffer)
        .map_err(|e| io_err(e, Some(format!("Unable to tar gzip {}", path.display()))))?;
    trace!("Tar'ed backup to {:?}", path);
    fs::remove_dir_all(path)
        .await
        .map_err(|e| io_err(e, Some("failed to remove the backup directory".to_string())))?;

    Ok(buffer)
}

struct WriteCurveRecoveryDataArgs {
    cfg: Arc<LitConfig>,
    root_keys: HashMap<CurveType, Vec<String>>,
    peer_id: PeerId,
    epoch: u64,
    staker_address: String,
    peers: SimplePeerCollection,
    path: PathBuf,
}

async fn write_curve_recovery_data<C>(
    args: Arc<WriteCurveRecoveryDataArgs>,
    curve_type: CurveType,
    encryption_key: &<C as BCA>::Point,
    blinder: &C::Scalar,
    blinder_commitment: &<C as BCA>::Point,
) -> Result<()>
where
    C: VerifiableEncryption + SignatureCurve<Point = <C as BCA>::Point> + Default + PointReader,
    <C as BCA>::Point: CompressedBytes + Group + GroupEncoding + Default,
    C::Scalar: CompressedBytes,
{
    let Some(root_keys) = args.root_keys.get(&curve_type) else {
        return Ok(());
    };

    // no data to back up
    if root_keys.is_empty() {
        return Ok(());
    }
    // Write the encryption key
    let enc_key_fn = enc_key_fn(curve_type);
    C::write_point(args.path.clone(), &enc_key_fn, encryption_key).await?;

    // Write the blinder commitment
    let blinder_comm_fn = blinder_comm_fn(curve_type);
    C::write_point(args.path.clone(), &blinder_comm_fn, blinder_commitment).await?;

    let empty_cache = KeyCache::default();
    let realm_id = args.peers.realm_id()?.as_u64();

    for root_key in root_keys {
        let storable_file = StorableFile {
            storage_type: StorageType::KeyShare(curve_type),
            pubkey: root_key.clone(),
            peer_id: args.peer_id,
            epoch: args.epoch,
            realm_id,
        };
        let share = read_key_share_from_disk::<KeyShare>(
            curve_type,
            root_key,
            &args.staker_address,
            &args.peer_id,
            args.epoch,
            realm_id,
            &empty_cache,
        )
        .await?;
        // Encrypt and save key shares
        let backup =
            BackupGenerator::<C>::generate_backup(*encryption_key, &share, blinder, &args.cfg)
                .await?;
        write_to_disk(args.path.clone(), &storable_file.file_name(), &backup).await?;

        copy_key_share_commitments_to_another_path(
            curve_type,
            root_key,
            &args.staker_address,
            &args.peer_id,
            args.epoch,
            realm_id,
            args.path.clone(),
        )
        .await?;
    }

    info!("Finished generating {} key backup.", curve_type);
    Ok(())
}

pub(crate) async fn untar_keys_stream<R: AsyncRead + Unpin>(
    cfg: &LitConfig,
    restore_state: &Arc<RestoreState>,
    stream: R,
) -> Result<()> {
    restore_state.assert_actively_restoring()?;

    let staker_address = &crate::endpoints::recovery::get_staker_address(cfg)?;

    // Create the temporary dir in which we will save the artefacts.
    let now: DateTime<Utc> = Utc::now();
    let mut path = encrypted_key_path(staker_address);
    path.push(format!("restore-{}/", now));

    // Untar the data
    untar_stream_to_path(path.as_path(), stream).await?;
    trace!("Untar'd backup to {}", path.display());

    let blinders = restore_state.get_blinders();
    let key_cache = KeyCache::default();

    let mut files = fs::read_dir(path.as_path())
        .await
        .map_err(|e| io_err(e, None))?;
    let mut output = Vec::new();
    while let Some(Ok(entry)) = files.next().await {
        output.push(entry.file_name().to_str().expect("file_name").to_string());
    }
    trace!("Reading files in backup {}: {:?}", path.display(), output);

    let recovery_party_members =
        read_from_disk(path.clone(), RECOVERY_PARTY_WALLET_ADDRESSES_FN).await?;
    trace!(
        "Recovery party wallet addresses: {:?}",
        recovery_party_members
    );
    let threshold = read_from_disk(path.clone(), RECOVERY_PARTY_THRESHOLD_FN).await?;
    trace!("Threshold: {:?}", threshold);

    let session_id: String = read_from_disk(path.clone(), SESSION_ID_FN).await?;
    trace!(
        "Session id: backup {}, key_set {}",
        session_id,
        restore_state.get_expected_recovery_session_id()
    );

    let peers: Result<SimplePeerCollection> = read_from_disk(path.clone(), PEERS_FN).await;
    if let Ok(peers) = peers {
        // Might be missing for legacy reasons
        trace!("Peers: {:?}", peers);
    }

    let bls_recovery_data = read_curve_recovery_data::<InnerBls12381G1>(
        blinders.bls_blinder,
        G1Projective::GENERATOR,
        CurveType::BLS,
        &path.clone(),
        &key_cache,
    )
    .await?;

    let k256_recovery_data = read_curve_recovery_data::<k256::Secp256k1>(
        blinders.k256_blinder,
        k256::ProjectivePoint::GENERATOR,
        CurveType::K256,
        &path.clone(),
        &key_cache,
    )
    .await?;

    let p256_recovery_data = read_curve_recovery_data::<p256::NistP256>(
        blinders.p256_blinder,
        p256::ProjectivePoint::GENERATOR,
        CurveType::P256,
        &path.clone(),
        &key_cache,
    )
    .await?;

    let p384_recovery_data = read_curve_recovery_data::<p384::NistP384>(
        blinders.p384_blinder,
        p384::ProjectivePoint::GENERATOR,
        CurveType::P384,
        &path.clone(),
        &key_cache,
    )
    .await?;

    let ed25519_recovery_data = read_curve_recovery_data::<bulletproofs::Ed25519>(
        blinders.ed25519_blinder,
        vsss_rs::curve25519::WrappedEdwards::generator(),
        CurveType::Ed25519,
        &path.clone(),
        &key_cache,
    )
    .await?;

    let ristretto25519_recovery_data = read_curve_recovery_data::<bulletproofs::Ristretto25519>(
        blinders.ristretto25519_blinder,
        vsss_rs::curve25519::WrappedRistretto::generator(),
        CurveType::Ristretto25519,
        &path.clone(),
        &key_cache,
    )
    .await?;

    let ed448_recovery_data = read_curve_recovery_data::<ed448_goldilocks::Ed448>(
        blinders.ed448_blinder,
        ed448_goldilocks::EdwardsPoint::GENERATOR,
        CurveType::Ed448,
        &path.clone(),
        &key_cache,
    )
    .await?;

    let jubjub_recovery_data = read_curve_recovery_data::<bulletproofs::JubJub>(
        blinders.jubjub_blinder,
        jubjub::SubgroupPoint::generator(),
        CurveType::RedJubjub,
        &path.clone(),
        &key_cache,
    )
    .await?;

    let decaf377_recovery_data = read_curve_recovery_data::<bulletproofs::Decaf377>(
        blinders.decaf377_blinder,
        decaf377::Element::GENERATOR,
        CurveType::RedDecaf377,
        &path.clone(),
        &key_cache,
    )
    .await?;

    let bls12381g1_recovery_data = read_curve_recovery_data::<InnerBls12381G1>(
        blinders.bls12381g1_blinder,
        G1Projective::GENERATOR,
        CurveType::BLS12381G1,
        &path.clone(),
        &key_cache,
    )
    .await?;

    let inner_state = InnerState {
        recovery_party_members,
        bls_recovery_data,
        k256_recovery_data,
        p256_recovery_data,
        p384_recovery_data,
        ed25519_recovery_data,
        ristretto25519_recovery_data,
        ed448_recovery_data,
        jubjub_recovery_data,
        decaf377_recovery_data,
        bls12381g1_recovery_data,
        threshold,
        restored_key_cache: KeyCache::default(),
    };

    restore_state.load_backup(inner_state).await?;

    let _ = std::fs::remove_dir_all(path);

    Ok(())
}

async fn read_curve_recovery_data<C>(
    blinder: Option<C::Scalar>,
    generator: <C as BCA>::Point,
    curve_type: CurveType,
    path: &PathBuf,
    key_cache: &KeyCache,
) -> Result<Option<CurveRecoveryData<C>>>
where
    C: VerifiableEncryptionDecryptor + SignatureCurve<Point = <C as BCA>::Point> + PointReader,
    <C as BCA>::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    // Check if the encryption key is written and return if not.
    let enc_key_fn = enc_key_fn(curve_type);
    let encryption_key: <C as BCA>::Point = match C::read_point(path.clone(), &enc_key_fn).await {
        Ok(enc_key) => enc_key,
        Err(e) => {
            warn!(
                "{} encryption key not found. Skipping {} recovery",
                curve_type, curve_type
            );
            return Ok(None);
        }
    };
    trace!("{} encryption key retrieved", curve_type);

    // Fail if the blinder is not set.
    let blinder = blinder.ok_or(blinder_not_set_err(curve_type))?;

    // Check if the blinder commitment matches the blinder.
    let blinder_comm_fn = blinder_comm_fn(curve_type);
    let blinder_commitment: <C as BCA>::Point =
        C::read_point(path.clone(), &blinder_comm_fn).await?;
    if blinder_commitment != generator * blinder {
        return Err(unexpected_err(
            format!(
                "{} blinder commitment does not match. Expected: {}, received: {}",
                curve_type,
                blinder_commitment.to_compressed_hex(),
                (generator * blinder).to_compressed_hex()
            ),
            None,
        ));
    }

    // Read the provided encrypted key shares.
    let mut encrypted_key_shares = read_key_shares::<C>(curve_type, path, key_cache).await?;

    // DATIL_BACKUP: Remove this loop once old Datil backup is obsolete.
    for share in encrypted_key_shares.iter_mut() {
        if let Some(pk) = C::parse_old_backup_public_key(&share.public_key) {
            info!("Old {} backup share is found", curve_type);
            share.public_key = pk.to_compressed_hex();
        }
    }

    // Read the key share commitments corresponding to given encrypted key shares.
    let eks_and_ds =
        read_key_share_commitments::<C>(encrypted_key_shares, curve_type, path, key_cache).await?;

    Ok(Some(CurveRecoveryData {
        encryption_key,
        blinder,
        eks_and_ds,
    }))
}

fn blinder_not_set_err(curve_type: CurveType) -> crate::error::Error {
    unexpected_err(format!("{} blinder is not set", curve_type), None)
}

async fn read_key_shares<C>(
    curve_type: CurveType,
    path: &PathBuf,
    key_cache: &KeyCache,
) -> Result<Vec<EncryptedKeyShare<C>>>
where
    C: VerifiableEncryptionDecryptor,
    C::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    let encrypted_shares: Vec<EncryptedKeyShare<C>> =
        match read_recovery_data_from_disk(path, "*", StorageType::KeyShare(curve_type), key_cache)
            .await
        {
            Ok(shares) => shares,
            // DATIL_BACKUP: Remove this branch once old Datil backup is obsolete.
            Err(_) if (curve_type == CurveType::BLS || curve_type == CurveType::K256) => {
                read_encrypted_key_shares_from_datil(curve_type, path).await?
            }
            Err(e) => return Err(e),
        };
    trace!(
        "{} Encrypted {} Shares are retrieved",
        encrypted_shares.len(),
        curve_type
    );
    Ok(encrypted_shares)
}

// DATIL_BACKUP: Remove this function once old Datil backup is obsolete.
async fn read_encrypted_key_shares_from_datil<C>(
    curve_type: CurveType,
    path: &PathBuf,
) -> Result<Vec<EncryptedKeyShare<C>>>
where
    C: VerifiableEncryptionDecryptor,
    C::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    let file_names = crate::tss::common::storage::fetch_recovery_file_names_in_path(
        StorageType::KeyShare(curve_type),
        "*",
        path.clone(),
    )
    .await?;

    let mut shares = Vec::with_capacity(file_names.len());
    for file_name in file_names.into_iter() {
        let mut path = path.clone();
        path.push(file_name.clone());
        let mut file = tokio::fs::File::open(path.clone()).await.map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeSystemFault,
                Some(format!("Could not open file: {:?}", path)),
            )
        })?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await.map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeSystemFault,
                Some(format!("Could not read file: {:?}", path)),
            )
        })?;

        trace!(
            "Deserializing old encrypted key share {}, curve {}",
            file_name, curve_type
        );

        let share: OldEncryptedKeyShare<C> =
            ciborium::from_reader(&mut std::io::Cursor::new(&buffer)).map_err(|e| {
                unexpected_err_code(
                    e,
                    EC::NodeSystemFault,
                    Some(format!("Could not parse cbor file: {:?}", path)),
                )
            })?;

        shares.push(EncryptedKeyShare::from(share));
    }
    Ok(shares)
}

async fn read_key_share_commitments<C>(
    encrypted_shares: Vec<EncryptedKeyShare<C>>,
    curve_type: CurveType,
    path: &PathBuf,
    key_cache: &KeyCache,
) -> Result<Vec<EksAndDs<C>>>
where
    C: VerifiableEncryptionDecryptor + SignatureCurve<Point = <C as BCA>::Point>,
    <C as BCA>::Point: CompressedBytes,
    C::Scalar: CompressedBytes + From<PeerId>,
{
    let mut shares = Vec::with_capacity(encrypted_shares.len());
    for encrypted_key_share in encrypted_shares.into_iter() {
        let key_share_commitments: Option<KeyShareCommitments<<C as BCA>::Point>> =
            read_recovery_data_from_disk::<KeyShareCommitments<<C as BCA>::Point>>(
                &path.clone(),
                &encrypted_key_share.public_key,
                StorageType::KeyShareCommitment(curve_type),
                key_cache,
            )
            .await?
            .pop();

        match key_share_commitments {
            Some(_) => trace!(
                "{} Key Share Commitments are retrieved for {}",
                curve_type, &encrypted_key_share.public_key
            ),
            None => trace!(
                "No {} Key Share Commitments are found for {}",
                curve_type, &encrypted_key_share.public_key
            ),
        };

        shares.push(EksAndDs::new(
            encrypted_key_share,
            key_share_commitments,
            curve_type,
        )?);
    }
    Ok(shares)
}

async fn untar_stream_to_path<R: AsyncRead + Unpin>(path: &Path, mut stream: R) -> Result<()> {
    fs::create_dir_all(&path)
        .await
        .map_err(|e| io_err(e, None))?;

    // Equivalent to tar -xvf --strip-components=1
    let mut buffer = Vec::with_capacity(8192);
    stream
        .read_to_end(&mut buffer)
        .await
        .map_err(|e| io_err(e, Some("Unable to read tar gz stream".to_string())))?;
    let mut cursor = std::io::Cursor::new(buffer);
    lit_core::utils::tar::read_tar_gz_strip_components(&mut cursor, path, 1)?;

    Ok(())
}

pub(crate) async fn purge_precomputes(cfg: &LitConfig) -> Result<()> {
    // Pattern to match: "./node_precomputes/**/*-{node_name}-*.cbor"
    delete_file(format!(
        "./node_precomputes/**/*-{}-*.cbor",
        cfg.external_addr()
            .expect_or_err("expected external_addr to be set")?
            .replace(':', "-")
    ))
    .await
    .map_err(|e| {
        io_err_code(
            e,
            EC::NodeSystemFault,
            Some("Unable to remove precomputes".into()),
        )
    })?;

    Ok(())
}

pub(crate) async fn delete_file<P: AsRef<str>>(file_path_or_pattern: P) -> Result<()> {
    Command::new("rm")
        .arg("-f")
        .arg(file_path_or_pattern.as_ref())
        .output()
        .await
        .map_err(|e| io_err(e, None))?;

    Ok(())
}

// DATIL_BACKUP: Remove this loop once old Datil backup is obsolete.
pub fn parse_datil_blinder_file(contents: &str) -> Result<Blinders> {
    use crate::error::parser_err;
    use rocket::serde::Deserialize;

    #[derive(Deserialize)]
    struct DatilBlindersFile {
        bls_blinder: String,
        k256_blinder: String,
    }

    let datil_blinders = match serde_json::from_str::<DatilBlindersFile>(contents) {
        Ok(blinders) => blinders,
        Err(e) => return Err(parser_err(e, Some("Error parsing blinders".into()))),
    };

    Ok(Blinders {
        bls_blinder: Some(parse_bls_blinder(&datil_blinders.bls_blinder)?),
        k256_blinder: Some(parse_k256_blinder(&datil_blinders.k256_blinder)?),
        ..Default::default()
    })
}

// DATIL_BACKUP: Remove this function once old Datil backup is obsolete.
fn parse_bls_blinder(blinder_str: &str) -> Result<<InnerBls12381G1 as BCA>::Scalar> {
    let blinder = <InnerBls12381G1 as BCA>::Scalar::from_be_hex(blinder_str);
    match blinder.into_option() {
        Some(blinder) => Ok(blinder),
        None => Err(parser_err(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Could not convert to bls key blinder:{}", blinder_str),
            ),
            None,
        )),
    }
}

// DATIL_BACKUP: Remove this function once old Datil backup is obsolete.
fn parse_k256_blinder(blinder_str: &str) -> Result<<Secp256k1 as BCA>::Scalar> {
    // This is the error closure so we don't repeat it in the code.
    let error = |blinder_str| {
        parser_err(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Could not convert to ecdsa key blinder:{}", blinder_str),
            ),
            None,
        )
    };

    let bytes = hex::decode(blinder_str).map_err(|e| error(blinder_str))?;
    let scalar_primitive = elliptic_curve::scalar::ScalarPrimitive::from_slice(&bytes)
        .map_err(|e| error(blinder_str))?;
    Ok(k256::Scalar::from(&scalar_primitive))
}

#[cfg(test)]
mod test {
    use crate::common::key_helper::KeyCache;
    use crate::endpoints::admin::utils::{encrypt_and_tar_backup_keys, untar_keys_stream};
    use crate::peers::peer_state::models::{SimplePeer, SimplePeerCollection};
    use crate::tests::key_shares::{
        TEST_BLS_KEY_SHARE, TEST_BLS_KEY_SHARE_COMMITMENT, TEST_ECDSA_KEY_SHARE,
        TEST_ECDSA_KEY_SHARE_COMMITMENT,
    };
    use crate::tests::key_shares::{TEST_BLS_PUB_KEY, TEST_ECDSA_PUB_KEY};
    use crate::tss::common::backup::RecoveryParty;
    use crate::tss::common::key_persistence::KeyPersistence;
    use crate::tss::common::key_share::KeyShare;
    use crate::tss::common::key_share_commitment::KeyShareCommitments;
    use crate::tss::common::restore::RestoreState;
    use crate::tss::common::restore::eks_and_ds::verify_decrypted_key_share;
    use crate::tss::common::storage::{
        read_key_share_from_disk, write_key_share_commitments_to_disk, write_key_share_to_disk,
    };
    use blsful::{
        Bls12381G1Impl, SecretKeyShare,
        inner_types::{G1Projective, InnerBls12381G1},
    };
    use bulletproofs::BulletproofCurveArithmetic as BCA;
    use elliptic_curve::{Field, Group, PrimeField};
    use k256::{ProjectivePoint, PublicKey, Secp256k1};
    use lit_node_core::CurveType;
    use lit_node_core::PeerId;
    use lit_recovery::models::{EncryptedKeyShare, UploadedShareData};
    use semver::Version;
    use std::sync::Arc;
    use tokio::fs;
    use verifiable_share_encryption::DecryptionShare;
    use vsss_rs::{DefaultShare, IdentifierPrimeField, ValuePrimeField};

    #[tokio::test]
    async fn run_backup_tests() {
        test_encrypt_tar_and_untar_backup_keys().await;
        test_untar_old_backup().await;
    }

    type K256Share =
        DefaultShare<IdentifierPrimeField<k256::Scalar>, ValuePrimeField<k256::Scalar>>;

    #[cfg(any(feature = "testing", test))]
    pub fn get_test_recovery_party() -> RecoveryParty {
        // Generate mock keys
        let mut rng = rand_core::OsRng;
        let bls_encryption_key = <InnerBls12381G1 as BCA>::Point::generator()
            * <InnerBls12381G1 as BCA>::Scalar::random(&mut rng);
        let k256_encryption_key = k256::ProjectivePoint::GENERATOR * k256::Scalar::random(&mut rng);
        let p256_encryption_key = p256::ProjectivePoint::GENERATOR * p256::Scalar::random(&mut rng);
        let p384_encryption_key = p384::ProjectivePoint::GENERATOR * p384::Scalar::random(&mut rng);
        let ed25519_encryption_key = vsss_rs::curve25519::WrappedEdwards::generator()
            * vsss_rs::curve25519::WrappedScalar::random(&mut rng);
        let ristretto25519_encryption_key = vsss_rs::curve25519::WrappedRistretto::generator()
            * vsss_rs::curve25519::WrappedScalar::random(&mut rng);
        let ed448_encryption_key =
            ed448_goldilocks::EdwardsPoint::GENERATOR * ed448_goldilocks::Scalar::random(&mut rng);
        let jubjub_encryption_key =
            jubjub::SubgroupPoint::generator() * jubjub::Fr::random(&mut rng);
        let decaf377_encryption_key = decaf377::Element::GENERATOR * decaf377::Fr::random(&mut rng);
        let bls12381g1_encryption_key =
            G1Projective::GENERATOR * <InnerBls12381G1 as BCA>::Scalar::random(&mut rng);

        // Mock recovery party members
        let mut party_members = vec![];
        for _ in 1..3 {
            party_members.push(ethers::types::H160::random());
        }

        RecoveryParty {
            party_members,
            session_id: "mock recovery party session id".to_string(),
            bls_encryption_key,
            k256_encryption_key,
            p256_encryption_key,
            p384_encryption_key,
            ed25519_encryption_key,
            ristretto25519_encryption_key,
            ed448_encryption_key,
            jubjub_encryption_key,
            decaf377_encryption_key,
            bls12381g1_encryption_key,
            threshold: 2,
        }
    }

    // Helper function
    pub fn get_test_recovery_party_with_encryption_keys() -> RecoveryParty {
        let mut recovery_party = get_test_recovery_party();
        recovery_party.bls_encryption_key = G1Projective::from_compressed(
            hex::decode(TEST_BLS_PUB_KEY)
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap(),
        )
        .unwrap();
        recovery_party.k256_encryption_key = ProjectivePoint::from(
            PublicKey::from_sec1_bytes(&hex::decode(TEST_ECDSA_PUB_KEY).unwrap()).unwrap(),
        );
        recovery_party
    }

    async fn test_encrypt_tar_and_untar_backup_keys() {
        let cfg = Arc::new(crate::tests::common::get_backup_config());
        let staker_address = &crate::endpoints::recovery::get_staker_address(&cfg)
            .expect("Failed to get staker address");
        let bls_key_helper = KeyPersistence::<G1Projective>::new(CurveType::BLS);
        let k256_key_helper = KeyPersistence::<ProjectivePoint>::new(CurveType::K256);
        let recovery_party = get_test_recovery_party_with_encryption_keys();
        let key_set_root_keys = maplit::hashmap! {
            CurveType::BLS => vec!["83fc126ef56547bb28734a4a5393a873b8c22a9ba2d507036285a506567b2b3a376fc524cd589bb018613e24c51ebbae".to_string()],
            CurveType::K256 => vec!["0268c27a16f03d19949f0a64d58c71ea32049b754888211cc25827f5449d26bf74".to_string()],
        };

        // Make sure that there is at least one ECDSA and one BLS key share.
        let bls_key: KeyShare = serde_json::from_str(TEST_BLS_KEY_SHARE).unwrap();
        let k256_key: KeyShare = serde_json::from_str(TEST_ECDSA_KEY_SHARE).unwrap();
        let bls_key_share_commitments: KeyShareCommitments<<InnerBls12381G1 as BCA>::Point> =
            serde_json::from_str(TEST_BLS_KEY_SHARE_COMMITMENT).unwrap();
        let k256_key_share_commitments: KeyShareCommitments<k256::ProjectivePoint> =
            serde_json::from_str(TEST_ECDSA_KEY_SHARE_COMMITMENT).unwrap();

        // Make sure the key shares and key share commitments match
        verify_decrypted_key_share::<InnerBls12381G1>(
            bls_key_helper
                .secret_from_hex(&bls_key.hex_private_share)
                .unwrap(),
            &bls_key_share_commitments,
            bls_key.peer_id,
        )
        .unwrap();

        verify_decrypted_key_share::<k256::Secp256k1>(
            k256_key_helper
                .secret_from_hex(&k256_key.hex_private_share)
                .unwrap(),
            &k256_key_share_commitments,
            k256_key.peer_id,
        )
        .unwrap();

        let key_cache = KeyCache::default();

        write_key_share_to_disk(
            CurveType::BLS,
            &bls_key.hex_public_key,
            &staker_address,
            &bls_key.peer_id,
            333,
            1,
            &key_cache,
            &bls_key,
        )
        .await
        .unwrap();
        write_key_share_to_disk(
            CurveType::K256,
            &k256_key.hex_public_key,
            &staker_address,
            &k256_key.peer_id,
            333,
            1,
            &key_cache,
            &k256_key,
        )
        .await
        .unwrap();

        write_key_share_commitments_to_disk(
            CurveType::BLS,
            &bls_key.hex_public_key,
            &staker_address,
            &bls_key.peer_id,
            333,
            1,
            &key_cache,
            &bls_key_share_commitments,
        )
        .await
        .unwrap();
        write_key_share_commitments_to_disk(
            CurveType::K256,
            &k256_key.hex_public_key,
            &staker_address,
            &k256_key.peer_id,
            333,
            1,
            &key_cache,
            &k256_key_share_commitments,
        )
        .await
        .unwrap();

        // Call the function to be tested
        let blinders = RestoreState::generate_blinders();
        let peers = SimplePeerCollection(vec![SimplePeer {
            socket_address: "127.0.0.1".to_string(),
            peer_id: bls_key.peer_id,
            staker_address: ethers::types::H160::from_slice(&hex::decode(&staker_address).unwrap()),
            key_hash: 0,
            kicked: false,
            version: Version::new(1, 0, 0),
            realm_id: ethers::prelude::U256::from(1),
        }]);

        let child = encrypt_and_tar_backup_keys(
            cfg.clone(),
            bls_key.peer_id,
            &key_set_root_keys,
            &blinders,
            &recovery_party,
            &peers,
            333,
        )
        .await
        .unwrap();

        let restore_state = Arc::new(RestoreState::new());
        restore_state.set_blinders(blinders);
        restore_state.set_actively_restoring(true);
        untar_keys_stream(&cfg, &restore_state, child.as_slice())
            .await
            .unwrap();

        // Make sure the keys and the key share commitments are loaded.
        // If the key share commitments are not loaded, the verification
        // will be silently skipped (to accommodate older backups).
        let bls_eksandds = restore_state
            .fetch_bls_backup_by_pubkey(&bls_key.hex_public_key)
            .await
            .expect("Encrypted BLS key share is not found");
        let k256_eksandds = restore_state
            .fetch_k256_backup_by_pubkey(&k256_key.hex_public_key)
            .await
            .expect("Encrypted k256 key share is not found");
        assert!(bls_eksandds.key_share_commitments.is_some());
        assert!(k256_eksandds.key_share_commitments.is_some());
        let encrypted_bls_key = &bls_eksandds.encrypted_key_share;
        let encrypted_k256_key = &k256_eksandds.encrypted_key_share;

        let (bls_dec_share_1, bls_dec_share_2) = get_bls_decryption_shares(encrypted_bls_key);
        let (k256_dec_share_1, k256_dec_share_2) = get_k256_decryption_shares(encrypted_k256_key);

        restore_state
            .add_decryption_shares(&"1".to_string(), &vec![bls_dec_share_1, k256_dec_share_1])
            .await
            .unwrap();
        restore_state
            .add_decryption_shares(&"2".to_string(), &vec![bls_dec_share_2, k256_dec_share_2])
            .await
            .unwrap();

        let peer_id = PeerId::try_from(555 as usize).unwrap();
        let epoch = 333;
        let realm_id = 1;
        let restored_key_shares = restore_state
            .try_restore_key_shares(&peer_id, epoch, staker_address, realm_id)
            .await;
        assert_eq!(restored_key_shares.bls_shares.len(), 1);
        assert_eq!(restored_key_shares.k256_shares.len(), 1);

        let restored_key_cache = restore_state.pull_recovered_key_cache().await.unwrap();

        // Check that we can read the restored key share and that the private share matches
        let read_bls_key = read_key_share_from_disk::<KeyShare>(
            CurveType::BLS,
            &bls_key.hex_public_key,
            staker_address,
            &peer_id,
            epoch,
            realm_id,
            &restored_key_cache,
        )
        .await
        .unwrap();

        let read_k256_key = read_key_share_from_disk::<KeyShare>(
            CurveType::K256,
            &k256_key.hex_public_key,
            staker_address,
            &peer_id,
            epoch,
            realm_id,
            &restored_key_cache,
        )
        .await
        .unwrap();

        assert_eq!(bls_key.hex_private_share, read_bls_key.hex_private_share);
        assert_eq!(k256_key.hex_private_share, read_k256_key.hex_private_share);

        restore_state.mark_keys_restored(&restored_key_shares).await;
        assert!(restore_state.are_all_keys_restored().await);
    }

    // Helper function
    fn get_bls_decryption_shares(
        vb: &EncryptedKeyShare<InnerBls12381G1>,
    ) -> (UploadedShareData, UploadedShareData) {
        use crate::tests::key_shares::{
            TEST_BLS_PRI_KEY_SHARE_1, TEST_BLS_PRI_KEY_SHARE_2, TEST_BLS_PUB_KEY,
            hex_to_bls_dec_key_share,
        };
        let dec_key_share_1 = hex_to_bls_dec_key_share(TEST_BLS_PRI_KEY_SHARE_1, 1);
        let dec_key_share_2 = hex_to_bls_dec_key_share(TEST_BLS_PRI_KEY_SHARE_2, 2);

        let dec_key_share_1 =
            SecretKeyShare::<Bls12381G1Impl>::from_v1_bytes(&dec_key_share_1).unwrap();
        let dec_key_share_2 =
            SecretKeyShare::<Bls12381G1Impl>::from_v1_bytes(&dec_key_share_2).unwrap();

        let decryption_share_1 =
            DecryptionShare::<InnerBls12381G1>::new(&dec_key_share_1.0, &vb.ciphertext);
        let decryption_share_2 =
            DecryptionShare::<InnerBls12381G1>::new(&dec_key_share_2.0, &vb.ciphertext);

        let share_data_1 = UploadedShareData {
            participant_id: 0, // Temporarily for backward compatibility with Datil
            session_id: Default::default(),
            encryption_key: String::from(TEST_BLS_PUB_KEY),
            verification_key: vb.public_key.clone(),
            decryption_share: serde_json::to_string(&decryption_share_1).unwrap(),
            subnet_id: Default::default(),
            curve: CurveType::BLS.as_str().to_string(),
        };

        let mut share_data_2 = share_data_1.clone();
        share_data_2.decryption_share = serde_json::to_string(&decryption_share_2).unwrap();

        (share_data_1, share_data_2)
    }

    // Helper function
    fn get_k256_decryption_shares(
        vb: &EncryptedKeyShare<Secp256k1>,
    ) -> (UploadedShareData, UploadedShareData) {
        use crate::tests::key_shares::{
            TEST_ECDSA_PRI_KEY_SHARE_1, TEST_ECDSA_PRI_KEY_SHARE_2, TEST_ECDSA_PUB_KEY,
            hex_to_k256_dec_key_share,
        };
        let dec_key_share_1 = hex_to_k256_dec_key_share(TEST_ECDSA_PRI_KEY_SHARE_1, 1);
        let dec_key_share_2 = hex_to_k256_dec_key_share(TEST_ECDSA_PRI_KEY_SHARE_2, 2);

        let key_share_1 =
            k256::Scalar::from_repr(k256::FieldBytes::clone_from_slice(&dec_key_share_1[1..]))
                .expect("Failed to create k256 scalar from bytes");
        let dec_key_share_1 = K256Share {
            identifier: IdentifierPrimeField(k256::Scalar::from(dec_key_share_1[0] as u64)),
            value: IdentifierPrimeField(key_share_1),
        };

        let key_share_2 =
            k256::Scalar::from_repr(k256::FieldBytes::clone_from_slice(&dec_key_share_2[1..]))
                .expect("Failed to create k256 scalar from bytes");
        let dec_key_share_2 = K256Share {
            identifier: IdentifierPrimeField(k256::Scalar::from(dec_key_share_2[0] as u64)),
            value: IdentifierPrimeField(key_share_2),
        };

        let decryption_share_1 =
            DecryptionShare::<Secp256k1>::new(&dec_key_share_1, &vb.ciphertext);
        let decryption_share_2 =
            DecryptionShare::<Secp256k1>::new(&dec_key_share_2, &vb.ciphertext);

        let share_data_1 = UploadedShareData {
            participant_id: 0, // Temporarily for backward compatibility with Datil
            session_id: Default::default(),
            encryption_key: String::from(TEST_ECDSA_PUB_KEY),
            verification_key: vb.public_key.clone(),
            decryption_share: serde_json::to_string(&decryption_share_1).unwrap(),
            subnet_id: Default::default(),
            curve: CurveType::K256.as_str().to_string(),
        };

        let mut share_data_2 = share_data_1.clone();
        share_data_2.decryption_share = serde_json::to_string(&decryption_share_2).unwrap();

        (share_data_1, share_data_2)
    }

    // Helper function
    async fn read_old_backup_tar_file() -> fs::File {
        fs::File::open("tests/test_data/test_untar_old_backup/backup.tar")
            .await
            .unwrap()
    }

    async fn test_untar_old_backup() {
        use crate::tests::key_shares::{
            TEST_BLS_BLINDER, TEST_ECDSA_BLINDER, TEST_OLD_BLS_KEY_SHARE, TEST_OLD_K256_KEY_SHARE,
        };
        let bls_helper = KeyPersistence::<G1Projective>::new(CurveType::BLS);
        let bls_blinder = bls_helper.secret_from_hex(TEST_BLS_BLINDER).unwrap();

        let k256_helper = KeyPersistence::<k256::ProjectivePoint>::new(CurveType::K256);
        let k256_blinder = k256_helper.secret_from_hex(TEST_ECDSA_BLINDER).unwrap();

        let cfg = crate::tests::common::get_backup_config();
        let staker_address = &crate::endpoints::recovery::get_staker_address(&cfg)
            .expect("Failed to get staker address");
        let child = read_old_backup_tar_file().await;

        let realm_id = 1;
        // Untar and load the old backup
        let recovery_party = get_test_recovery_party_with_encryption_keys();
        let restore_state = Arc::new(RestoreState::new());
        restore_state.set_actively_restoring(true);
        let mut blinders = restore_state.get_blinders_mut();
        blinders.bls_blinder = Some(bls_blinder);
        blinders.k256_blinder = Some(k256_blinder);
        blinders.commit();
        untar_keys_stream(&cfg, &restore_state, child)
            .await
            .unwrap();

        // Make sure the keys are loaded.
        let bls_eksandds = restore_state
            .bls_eksandds()
            .await
            .expect("Encrypted BLS key share is not found");
        let k256_eksandds = restore_state
            .k256_eksandds()
            .await
            .expect("Encrypted k256 key share is not found");
        let encrypted_bls_key = &bls_eksandds.encrypted_key_share;
        let encrypted_k256_key = &k256_eksandds.encrypted_key_share;

        // Check that the private shares are correctly decrypted.
        let bls_key: KeyShare = serde_json::from_str(TEST_OLD_BLS_KEY_SHARE).unwrap();
        let k256_key: KeyShare = serde_json::from_str(TEST_OLD_K256_KEY_SHARE).unwrap();

        let bls_key_helper = KeyPersistence::<G1Projective>::new(CurveType::BLS);
        let k256_key_helper = KeyPersistence::<ProjectivePoint>::new(CurveType::K256);

        let (bls_dec_share_1, bls_dec_share_2) = get_bls_decryption_shares(encrypted_bls_key);
        let (k256_dec_share_1, k256_dec_share_2) = get_k256_decryption_shares(encrypted_k256_key);
        restore_state
            .add_decryption_shares(&"1".to_string(), &vec![bls_dec_share_1, k256_dec_share_1])
            .await
            .unwrap();
        restore_state
            .add_decryption_shares(&"2".to_string(), &vec![bls_dec_share_2, k256_dec_share_2])
            .await
            .unwrap();

        let peer_id = PeerId::try_from(555 as usize).unwrap();
        let epoch = 333;
        let restored_key_shares = restore_state
            .try_restore_key_shares(&peer_id, epoch, staker_address, realm_id)
            .await;
        assert_eq!(restored_key_shares.bls_shares.len(), 1);
        assert_eq!(restored_key_shares.k256_shares.len(), 1);

        // Check that we can read the restored key share and that the private share matches
        let key_cache = restore_state.pull_recovered_key_cache().await.unwrap();
        let read_bls_key = read_key_share_from_disk::<KeyShare>(
            CurveType::BLS,
            &bls_key.hex_public_key,
            staker_address,
            &peer_id,
            epoch,
            realm_id,
            &key_cache,
        )
        .await
        .unwrap();

        let read_k256_key = read_key_share_from_disk::<KeyShare>(
            CurveType::K256,
            &k256_key.hex_public_key.to_lowercase(),
            staker_address,
            &peer_id,
            epoch,
            realm_id,
            &key_cache,
        )
        .await
        .unwrap();

        assert_eq!(bls_key.hex_private_share, read_bls_key.hex_private_share);
        assert_eq!(
            k256_key.hex_private_share.to_lowercase(),
            read_k256_key.hex_private_share
        );

        restore_state.mark_keys_restored(&restored_key_shares).await;
        assert!(restore_state.are_all_keys_restored().await);
    }
}
