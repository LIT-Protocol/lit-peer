use bulletproofs::{BulletproofCurveArithmetic as BCA, BulletproofCurveArithmetic};
use ethers::types::H160;
use std::marker::PhantomData;
use verifiable_share_encryption::VerifiableEncryption;

use crate::error::{Result, io_err, unexpected_err};
use crate::tss::common::key_persistence::KeyPersistence;
use crate::tss::common::key_share::KeyShare;
use crate::utils::contract::get_backup_recovery_contract;
use crate::utils::traits::SignatureCurve;
use lit_blockchain::contracts::backup_recovery::RecoveryKey;
use lit_core::config::LitConfig;
use lit_node_common::config::LitNodeConfig;
use lit_node_core::{CompressedBytes, CurveType, PeerId};
use lit_recovery::models::EncryptedKeyShare;
use lit_rust_crypto::{
    blsful::inner_types::{G1Projective, InnerBls12381G1},
    decaf377, ed448_goldilocks,
    elliptic_curve::bigint::{NonZero, U256},
    jubjub, k256, p256, p384, pallas, vsss_rs,
};

/// Internally kept version
#[derive(Default)]
pub struct RecoveryParty {
    pub party_members: Vec<H160>,
    pub session_id: String,
    pub bls_encryption_key: <InnerBls12381G1 as BCA>::Point,
    pub k256_encryption_key: k256::ProjectivePoint,
    pub p256_encryption_key: p256::ProjectivePoint,
    pub p384_encryption_key: p384::ProjectivePoint,
    pub ed25519_encryption_key: vsss_rs::curve25519::WrappedEdwards,
    pub ristretto25519_encryption_key: vsss_rs::curve25519::WrappedRistretto,
    pub ed448_encryption_key: ed448_goldilocks::EdwardsPoint,
    pub jubjub_encryption_key: jubjub::SubgroupPoint,
    pub decaf377_encryption_key: decaf377::Element,
    pub bls12381g1_encryption_key: <InnerBls12381G1 as BCA>::Point,
    pub pallas_encryption_key: pallas::Point,
    pub threshold: usize,
}

pub async fn get_recovery_party(cfg: &LitConfig) -> Result<RecoveryParty> {
    let recovery_contract = get_backup_recovery_contract(cfg).await?;
    let state = recovery_contract
        .get_backup_party_state()
        .await
        .map_err(|e| {
            unexpected_err(
                e,
                Some(
                    "Cannot retrieve the recovery party state from the smart contract".to_string(),
                ),
            )
        })?;

    info!("Retrieved recovery party state: {:?}", state);

    if state.party_members.is_empty()
        || state.session_id.is_empty()
        || state.registered_recovery_keys.is_empty()
    {
        return Err(unexpected_err("Recovery state is empty".to_string(), None));
    }

    let mut rp = RecoveryParty {
        party_members: state.party_members,
        session_id: state.session_id.to_string(),
        threshold: state.party_threshold.as_usize(),
        ..Default::default()
    };

    set_recovery_party_keys(&mut rp, &state.registered_recovery_keys)?;
    Ok(rp)
}

fn set_recovery_party_keys(
    recovery_party: &mut RecoveryParty,
    registered_recovery_keys: &[RecoveryKey],
) -> Result<()> {
    for recovery_key in registered_recovery_keys {
        match CurveType::try_from(recovery_key.key_type).map_err(|e| io_err(e, None))? {
            CurveType::BLS => {
                trace!("Reading bls encryption key");
                recovery_party.bls_encryption_key = read_bls_pub_key(&recovery_key.pubkey)?;
            }
            CurveType::K256 => {
                trace!("Reading k256 encryption key");
                recovery_party.k256_encryption_key = read_k256_pub_key(&recovery_key.pubkey)?;
            }
            CurveType::P256 => {
                trace!("Reading p256 encryption key");
                recovery_party.p256_encryption_key = read_p256_pub_key(&recovery_key.pubkey)?;
            }
            CurveType::P384 => {
                trace!("Reading p384 encryption key");
                recovery_party.p384_encryption_key = read_p384_pub_key(&recovery_key.pubkey)?;
            }
            CurveType::Ed25519 => {
                trace!("Reading ed25519 encryption key");
                recovery_party.ed25519_encryption_key = read_ed25519_pub_key(&recovery_key.pubkey)?;
            }
            CurveType::Ristretto25519 => {
                trace!("Reading ristretto25519 encryption key");
                recovery_party.ristretto25519_encryption_key =
                    read_ristretto25519_pub_key(&recovery_key.pubkey)?;
            }
            CurveType::Ed448 => {
                trace!("Reading ed448 encryption key");
                recovery_party.ed448_encryption_key = read_ed448_pub_key(&recovery_key.pubkey)?;
            }
            CurveType::RedJubjub => {
                trace!("Reading jubjub encryption key");
                recovery_party.jubjub_encryption_key = read_jubjub_pub_key(&recovery_key.pubkey)?;
            }
            CurveType::RedDecaf377 => {
                trace!("Reading decaf377 encryption key");
                recovery_party.decaf377_encryption_key =
                    read_decaf377_pub_key(&recovery_key.pubkey)?;
            }
            CurveType::BLS12381G1 => {
                trace!("Reading bls12381g1 encryption key");
                recovery_party.bls12381g1_encryption_key = read_bls_pub_key(&recovery_key.pubkey)?;
            }
            CurveType::RedPallas => {
                trace!("Reading pallas encryption key");
                recovery_party.pallas_encryption_key = read_pallas_pub_key(&recovery_key.pubkey)?;
            }
        }
    }
    Ok(())
}

pub fn read_bls_pub_key(bytes: &[u8]) -> Result<<InnerBls12381G1 as BCA>::Point> {
    let helper = KeyPersistence::<G1Projective>::new(CurveType::BLS);
    helper.pk_from_bytes(bytes)
}

fn read_k256_pub_key(bytes: &[u8]) -> Result<k256::ProjectivePoint> {
    let helper = KeyPersistence::<k256::ProjectivePoint>::new(CurveType::K256);
    helper.pk_from_bytes(bytes)
}

pub struct BackupGenerator<V>(pub PhantomData<V>);

impl<V> BackupGenerator<V>
where
    V: VerifiableEncryption
        + SignatureCurve<Point = <V as BulletproofCurveArithmetic>::Point>
        + Default,
    <V as BulletproofCurveArithmetic>::Point: CompressedBytes,
    V::Scalar: CompressedBytes,
{
    pub async fn generate_backup(
        encryption_key: <V as BulletproofCurveArithmetic>::Point,
        disk_share: &KeyShare,
        blinder: &V::Scalar,
        cfg: &LitConfig,
    ) -> Result<EncryptedKeyShare<V>> {
        let rng = rand_core::OsRng;

        let key_helper =
            KeyPersistence::<<V as BulletproofCurveArithmetic>::Point>::new(disk_share.curve_type);
        let key_share = &key_helper.secret_from_hex(&disk_share.hex_private_share)?;

        let (ciphertext, proof) = V::blind_encrypt_and_prove(
            encryption_key,
            key_share,
            blinder,
            &[],
            Some(V::signing_generator()),
            rng,
        );

        Ok(EncryptedKeyShare {
            subnet_id: cfg.subnet_id()?,
            staker_address: cfg.staker_address()?,
            peer_id: disk_share.peer_id.into(),
            share_index: None,
            ciphertext,
            proof,
            txn_prefix: disk_share.txn_prefix.clone(),
            public_key: disk_share.hex_public_key.clone(),
            threshold: disk_share.threshold,
            total_shares: disk_share.total_shares,
            peers: disk_share.peers.iter().map(U256::from).collect(),
            realm_id: disk_share.realm_id,
        })
    }
}

fn read_p256_pub_key(bytes: &[u8]) -> Result<p256::ProjectivePoint> {
    let helper = KeyPersistence::<p256::ProjectivePoint>::new(CurveType::P256);
    helper.pk_from_bytes(bytes)
}

fn read_p384_pub_key(bytes: &[u8]) -> Result<p384::ProjectivePoint> {
    let helper = KeyPersistence::<p384::ProjectivePoint>::new(CurveType::P384);
    helper.pk_from_bytes(bytes)
}

fn read_ed25519_pub_key(bytes: &[u8]) -> Result<vsss_rs::curve25519::WrappedEdwards> {
    let helper = KeyPersistence::<vsss_rs::curve25519::WrappedEdwards>::new(CurveType::Ed25519);
    helper.pk_from_bytes(bytes)
}

fn read_ristretto25519_pub_key(bytes: &[u8]) -> Result<vsss_rs::curve25519::WrappedRistretto> {
    let helper =
        KeyPersistence::<vsss_rs::curve25519::WrappedRistretto>::new(CurveType::Ristretto25519);
    helper.pk_from_bytes(bytes)
}

fn read_ed448_pub_key(bytes: &[u8]) -> Result<ed448_goldilocks::EdwardsPoint> {
    let helper = KeyPersistence::<ed448_goldilocks::EdwardsPoint>::new(CurveType::Ed448);
    helper.pk_from_bytes(bytes)
}

fn read_jubjub_pub_key(bytes: &[u8]) -> Result<jubjub::SubgroupPoint> {
    let helper = KeyPersistence::<jubjub::SubgroupPoint>::new(CurveType::RedJubjub);
    helper.pk_from_bytes(bytes)
}

fn read_decaf377_pub_key(bytes: &[u8]) -> Result<decaf377::Element> {
    let helper = KeyPersistence::<decaf377::Element>::new(CurveType::RedDecaf377);
    helper.pk_from_bytes(bytes)
}

fn read_pallas_pub_key(bytes: &[u8]) -> Result<pallas::Point> {
    let helper = KeyPersistence::<pallas::Point>::new(CurveType::RedPallas);
    helper.pk_from_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common::get_backup_config;
    use crate::tss::common::backup::EncryptedKeyShare;
    use crate::tss::common::key_persistence::KeyPersistence;
    use crate::tss::common::key_share::KeyShare;
    use bulletproofs::BulletproofCurveArithmetic as BCA;
    use lit_node_core::{CompressedHex, CurveType, PeerId};
    use lit_rust_crypto::{
        ff::{Field, PrimeFieldBits},
        vsss_rs::{DefaultShare, IdentifierPrimeField},
    };
    use test_case::test_case;
    use verifiable_share_encryption::{VerifiableEncryption, VerifiableEncryptionDecryptor};

    fn get_enc_dec_key_pair<C>() -> (<C as BCA>::Point, C::Scalar)
    where
        C: BCA + SignatureCurve<Point = <C as BCA>::Point>,
    {
        let mut rng = rand_core::OsRng;
        let decryption_key = C::Scalar::random(&mut rng);
        let encryption_key = C::signing_generator() * decryption_key;
        (encryption_key, decryption_key)
    }

    fn recover_keyshare_from_backup<C>(
        blinder: &C::Scalar,
        decryption_key: &C::Scalar,
        backup: &EncryptedKeyShare<C>,
        curve_type: CurveType,
    ) -> KeyShare
    where
        C: VerifiableEncryption
            + VerifiableEncryptionDecryptor
            + SignatureCurve<Point = <C as BCA>::Point>
            + Default,
        <C as BCA>::Point: CompressedBytes,
        C::Scalar: PrimeFieldBits + CompressedBytes,
    {
        let private_share = C::decrypt_and_unblind(
            blinder,
            decryption_key,
            &backup.ciphertext,
            Some(C::signing_generator()),
        )
        .unwrap();
        let public_key =
            <<C as BCA>::Point as CompressedHex>::from_uncompressed_hex(&backup.public_key)
                .unwrap();

        let key_helper = KeyPersistence::<<C as BCA>::Point>::new(curve_type);
        let private_share = key_helper.secret_to_hex(&private_share);
        let public_key = key_helper.pk_to_hex(&public_key.into());

        KeyShare {
            hex_private_share: private_share,
            hex_public_key: public_key,
            curve_type,
            peer_id: get_peer_id(backup),
            threshold: backup.threshold,
            total_shares: backup.total_shares,
            txn_prefix: backup.txn_prefix.clone(),
            realm_id: 1,
            peers: vec![],
        }
    }

    fn generate_keyshare<C>(curve_type: CurveType) -> KeyShare
    where
        C: BCA + SignatureCurve<Point = <C as BCA>::Point> + Default,
        <C as BCA>::Point: CompressedBytes,
        C::Scalar: CompressedBytes,
    {
        let secret_key = C::Scalar::random(&mut rand_core::OsRng);
        let public_key = C::signing_generator() * secret_key;
        let shares = vsss_rs::shamir::split_secret::<
            DefaultShare<IdentifierPrimeField<C::Scalar>, IdentifierPrimeField<C::Scalar>>,
        >(
            2,
            3,
            &IdentifierPrimeField(secret_key),
            &mut rand_core::OsRng,
        )
        .unwrap();

        let key_helper = KeyPersistence::<<C as BCA>::Point>::new(curve_type);
        let private_share = key_helper.secret_to_hex(&shares[0].value);
        let public_key = key_helper.pk_to_hex(&public_key.into());

        KeyShare {
            hex_private_share: private_share,
            hex_public_key: public_key,
            curve_type,
            peer_id: PeerId::ONE,
            threshold: 2,
            total_shares: 3,
            txn_prefix: "test".to_string(),
            realm_id: 1,
            peers: vec![],
        }
    }

    #[tokio::test]
    #[test_case(k256::Secp256k1, CurveType::K256; "Recovery using k256")]
    #[test_case(p256::NistP256, CurveType::P256; "Recovery using p256")]
    #[test_case(p384::NistP384, CurveType::P384; "Recovery using p384")]
    #[test_case(bulletproofs::Ed25519, CurveType::Ed25519; "Recovery using Ed25519")]
    #[test_case(bulletproofs::Ristretto25519, CurveType::Ristretto25519; "Recovery using Ristretto25519")]
    #[test_case(ed448_goldilocks::Ed448, CurveType::Ed448; "Recovery using Ed448")]
    #[test_case(bulletproofs::JubJub, CurveType::RedJubjub; "Recovery using RedJubjub")]
    #[test_case(bulletproofs::Decaf377, CurveType::RedDecaf377; "Recovery using RedDecaf377")]
    async fn test_backup_recovery_cycle<C>(_c: C, curve_type: CurveType)
    where
        C: VerifiableEncryption
            + VerifiableEncryptionDecryptor
            + SignatureCurve<Point = <C as BCA>::Point>
            + Default,
        C::Scalar: PrimeFieldBits + CompressedBytes,
        <C as BCA>::Point: CompressedBytes,
        verifiable_share_encryption::Ciphertext<C>: PartialEq + Eq,
    {
        let (encryption_key, decryption_key) = get_enc_dec_key_pair::<C>();
        let key_share = generate_keyshare::<C>(curve_type);
        let mut rng = rand_core::OsRng;
        let blinder = <C as BCA>::Scalar::random(&mut rng);
        let cfg = get_backup_config();

        // Generate verifiable backup. Generation process includes verifying the proof.
        let backup =
            BackupGenerator::<C>::generate_backup(encryption_key, &key_share, &blinder, &cfg)
                .await
                .unwrap();

        let key_helper = KeyPersistence::<<C as BCA>::Point>::new(curve_type);
        let private_share = key_helper
            .secret_from_hex(&key_share.hex_private_share)
            .unwrap();

        // This part is to assert that the produced pair is verifiable:
        let verification_key = C::signing_generator() * (private_share + blinder);
        C::verify(
            encryption_key,
            verification_key,
            &backup.ciphertext,
            &backup.proof,
            &[],
            Some(C::signing_generator()),
        )
        .unwrap();

        // Assert that serialization and deserialization results back in the same data.
        let backup_json = serde_json::to_string(&backup).unwrap();
        let deserialized_backup: EncryptedKeyShare<C> = serde_json::from_str(&backup_json).unwrap();
        assert_eq!(
            backup.ciphertext, deserialized_backup.ciphertext,
            "ciphertexts must match"
        );
        assert_eq!(
            backup.public_key, deserialized_backup.public_key,
            "public keys must match"
        );
        assert_eq!(
            get_peer_id(&backup),
            get_peer_id(&deserialized_backup),
            "peer_ids must match"
        );
        assert_eq!(
            backup.threshold, deserialized_backup.threshold,
            "thresholds must match"
        );
        assert_eq!(
            backup.total_shares, deserialized_backup.total_shares,
            "total shares must match"
        );

        // Check that the decryption generates the same key share.
        let recovered_key_share = recover_keyshare_from_backup::<C>(
            &blinder,
            &decryption_key,
            &deserialized_backup,
            curve_type,
        );
        assert_eq!(
            key_share.hex_private_share.to_lowercase(),
            recovered_key_share.hex_private_share.to_lowercase(),
            "private shares must match"
        );
    }

    use crate::common::key_helper::KeyCache;
    use crate::tss::common::storage::StorageType;
    use crate::tss::common::storage::read_recovery_data_from_disk;
    use async_std::path::PathBuf;
    use std::str::FromStr;
    use verifiable_share_encryption::DecryptionShare;

    #[tokio::test]
    #[test_case(InnerBls12381G1, CurveType::BLS; "BLS")]
    #[test_case(k256::Secp256k1, CurveType::K256; "k256")]
    #[test_case(p256::NistP256, CurveType::P256; "p256")]
    #[test_case(p384::NistP384, CurveType::P384; "p384")]
    #[test_case(bulletproofs::Ed25519, CurveType::Ed25519; "Ed25519")]
    #[test_case(bulletproofs::Ristretto25519, CurveType::Ristretto25519; "Ristretto25519")]
    #[test_case(ed448_goldilocks::Ed448, CurveType::Ed448; "Ed448")]
    #[test_case(bulletproofs::JubJub, CurveType::RedJubjub; "RedJubjub")]
    #[test_case(bulletproofs::Decaf377, CurveType::RedDecaf377; "RedDecaf377")]
    #[test_case(InnerBls12381G1, CurveType::BLS12381G1; "BLS12381G1")]
    async fn test_backup_recovery_with_real_shares<C>(_c: C, curve_type: CurveType)
    where
        C: VerifiableEncryption
            + VerifiableEncryptionDecryptor
            + SignatureCurve<Point = <C as BCA>::Point>
            + Default,
        <C as BCA>::Point: CompressedBytes,
        C::Scalar: CompressedBytes + From<PeerId>,
    {
        let path = "tests/test_data/3wayDKG/";
        let cfg = get_backup_config();
        let key_cache = KeyCache::default();
        let helper = KeyPersistence::<<C as BCA>::Point>::new(curve_type);

        // Generate a secret and a blinder
        let mut rng = rand_core::OsRng;
        let secret = C::Scalar::random(&mut rng);
        let blinder = C::Scalar::random(&mut rng);

        // Read in the key shares which are previously generated by the nodes.
        let mut key_shares: Vec<KeyShare> = read_recovery_data_from_disk(
            &PathBuf::from_str(path).unwrap(),
            "*",
            StorageType::KeyShare(curve_type),
            &key_cache,
        )
        .await
        .unwrap();

        // There are 3 keys with the same pub_key
        assert_eq!(key_shares.len(), 3);
        let dec_key_1 = key_shares.pop().unwrap();
        let dec_key_2 = key_shares.pop().unwrap();
        let dec_key_3 = key_shares.pop().unwrap();
        // All have the same pub key
        assert_eq!(dec_key_1.hex_public_key, dec_key_2.hex_public_key);
        assert_eq!(dec_key_1.hex_public_key, dec_key_3.hex_public_key);
        assert_eq!(dec_key_2.hex_public_key, dec_key_3.hex_public_key);
        // All have different private keys
        assert_ne!(dec_key_1.hex_private_share, dec_key_2.hex_private_share);
        assert_ne!(dec_key_1.hex_private_share, dec_key_3.hex_private_share);
        assert_ne!(dec_key_2.hex_private_share, dec_key_3.hex_private_share);

        let encryption_key = helper.pk_from_hex(&dec_key_1.hex_public_key).unwrap();

        // Encrypt and blind
        let (ciphertext, proof) = C::blind_encrypt_and_prove(
            encryption_key,
            &secret,
            &blinder,
            &[],
            Some(C::signing_generator()),
            rng,
        );

        // Generate decryption shares
        let decryption_shares = vec![dec_key_1, dec_key_2, dec_key_3]
            .into_iter()
            .map(|s| {
                DecryptionShare::new(
                    &s.default_share::<<C as BCA>::Point>().unwrap(),
                    &ciphertext,
                )
            })
            .collect::<Vec<_>>();

        let decrypted_secret = C::decrypt_with_shares_and_unblind(
            &blinder,
            &decryption_shares,
            &ciphertext,
            Some(C::signing_generator()),
        )
        .unwrap();
        assert_eq!(secret, decrypted_secret);
    }
}

pub fn get_peer_id<C: BCA>(share: &EncryptedKeyShare<C>) -> PeerId {
    if let Some(share_index) = &share.share_index {
        // Not sure if this is correct. Old share indices start with 0.
        // However, 0 is not a valid PeerId. Let's use share_index+1,
        // as this is what we use to have in the lit-recovery tool.
        return PeerId::from_u16(*share_index + 1);
    }
    PeerId(NonZero::<U256>::from_uint(share.peer_id))
}
