use crate::common::key_helper::KeyCache;
use crate::peers::peer_state::models::SimplePeer;
use crate::tss::common::key_share::KeyShare;
use crate::tss::common::storage::read_key_share_from_disk;
use crate::{
    error::{EC, unexpected_err, unexpected_err_code},
    peers::peer_state::models::SimplePeerCollection,
    tss::common::{
        hd_keys::get_derived_keyshare, key_share_commitment::KeyShareCommitments,
        storage::read_key_share_commitments_from_disk,
    },
};
use futures::future::join_all;
use lit_core::error::Result;
use lit_core::utils::binary::bytes_to_hex;
use lit_node_core::{
    CompressedBytes, CurveType, PeerId,
    hd_keys_curves_wasm::{HDDerivable, HDDeriver},
};
use lit_rust_crypto::{
    blsful::{
        Bls12381G2Impl, Pairing, PublicKey, SecretKey, SecretKeyShare, Signature, SignatureSchemes,
        SignatureShare,
        inner_types::{G1Projective, Scalar},
    },
    ed448_goldilocks,
    group::Group,
    k256, p256, p384, pallas,
    vsss_rs::{IdentifierPrimeField, Share},
};
use lit_vrf::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    collections::BTreeMap,
    fmt::{self, Debug, Display, Formatter},
};
use tracing::instrument;

const VRF_KEY_SHARE_VALIDATION_PREFIX: &str = "vrf-key-share-validation-";
/// Proofs for key share validation
#[derive(Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct KeyShareProofs {
    /// The epoch that was used to generate the proofs
    pub epoch: u64,
    /// The proofs for each curve type
    pub proofs: BTreeMap<CurveType, Vec<u8>>,
}

impl Display for KeyShareProofs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "KeyShareProofs {{ epoch: {}, proofs: {:?} }}",
            self.epoch,
            self.proofs
                .iter()
                .map(|(k, v)| (*k, bytes_to_hex(v)))
                .collect::<Vec<(CurveType, String)>>()
        )
    }
}

impl Debug for KeyShareProofs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyShareProofs")
            .field("epoch", &self.epoch)
            .field(
                "proofs",
                &self
                    .proofs
                    .iter()
                    .map(|(k, v)| (*k, bytes_to_hex(v)))
                    .collect::<Vec<(CurveType, String)>>(),
            )
            .finish()
    }
}

#[instrument(level = "debug", name = "compute_key_share_proofs", skip_all)]
pub async fn compute_key_share_proofs(
    noonce: &str,
    root_keys_map: &HashMap<CurveType, Vec<String>>,
    self_addr: &str,
    peers: &SimplePeerCollection,
    realm_id: u64,
    epoch: u64,
) -> Result<KeyShareProofs> {
    trace!("Computing key share proofs for epoch {}", epoch);
    let start = std::time::Instant::now();

    // Not part of the set yet
    if peers.is_empty() {
        trace!("No peers in the set, no KeyShareProofs to generate");
        return Ok(KeyShareProofs::default());
    }

    let self_peer = match peers.peer_at_address(self_addr) {
        Err(_) => {
            trace!("Peer {} not found in the set", self_addr);
            return Ok(KeyShareProofs::default()); // Not part of the set yet
        }
        Ok(peer) => peer,
    };
    let staker_address = &bytes_to_hex(self_peer.staker_address.as_bytes());
    let mut key_share_proofs = BTreeMap::new();

    let mut tasks = Vec::with_capacity(root_keys_map.len());
    for (&curve_type, inner_root_keys) in root_keys_map {
        let peers_clone = peers.clone();
        let self_peer_clone = self_peer.clone();
        let noonce_clone = noonce.to_string();
        let staker_address_clone = staker_address.clone();
        let inner_root_keys_clone = inner_root_keys.clone();

        // Spawn a new task for each curve_type
        let task = tokio::task::spawn(async move {
            let proof = compute_key_share_proof(
                &inner_root_keys_clone,
                &peers_clone,
                &self_peer_clone,
                &noonce_clone,
                curve_type,
                &staker_address_clone,
                epoch,
                realm_id,
            )
            .await;

            (curve_type, proof)
        });
        tasks.push(task);
    }

    let results = join_all(tasks).await;
    for result in results {
        let (curve_type, proof) = match result {
            Ok((curve_type, Ok(proof))) => (curve_type, proof),
            Ok((_, Err(e))) => {
                error!("Error computing key share proof: {:?}", e);
                return Err(unexpected_err("Error computing key share proof", None));
            }
            Err(e) => {
                error!("Error computing key share proof: {:?}", e);
                return Err(unexpected_err("Error computing key share proof", None));
            }
        };
        key_share_proofs.insert(curve_type, proof);
    }

    trace!(
        "Computed key share proofs in {}ms",
        start.elapsed().as_millis()
    );
    Ok(KeyShareProofs {
        epoch,
        proofs: key_share_proofs,
    })
}

#[allow(clippy::too_many_arguments)]
#[instrument(level = "debug", skip_all)]
pub async fn compute_key_share_proof(
    root_keys: &[String],
    peers: &SimplePeerCollection,
    self_peer: &SimplePeer,
    noonce: &str,
    curve_type: CurveType,
    staker_address: &str,
    epoch: u64,
    realm_id: u64,
) -> Result<Vec<u8>> {
    if curve_type == CurveType::BLS {
        if root_keys.is_empty() {
            return Err(unexpected_err(
                "No primary BLS key found!".to_string(),
                None,
            ));
        }
        // Don't use tss_state key cache, force read from disk
        let key_cache = KeyCache::default();

        let bls_key_share = read_key_share_from_disk::<KeyShare>(
            CurveType::BLS,
            &root_keys[0],
            staker_address,
            &self_peer.peer_id,
            epoch,
            realm_id,
            &key_cache,
        )
        .await?;

        let identifier =
            <<Bls12381G2Impl as Pairing>::PublicKey as Group>::Scalar::from(bls_key_share.peer_id);
        let value = bls_key_share.secret::<<Bls12381G2Impl as Pairing>::PublicKey>()?;

        let secret_key_share: SecretKeyShare<Bls12381G2Impl> = SecretKeyShare(
            <Bls12381G2Impl as Pairing>::SecretKeyShare::with_identifier_and_value(
                IdentifierPrimeField(identifier),
                IdentifierPrimeField(value),
            ),
        );

        let sks = secret_key_share
            .sign(SignatureSchemes::ProofOfPossession, noonce.as_bytes())
            .map_err(|e| unexpected_err(format!("Failed to sign message: {:?}", e), None))?;

        return postcard::to_stdvec(&sks)
            .map_err(|e| unexpected_err(e, Some("cannot serialize BLS proof".to_string())));
    }

    let args = ComputeKeyShareProofArgs {
        root_keys,
        noonce,
        curve_type,
        dst: curve_type.vrf_ctx(),
        peer_id: &self_peer.peer_id,
        staker_address,
        epoch,
        realm_id,
    };
    match curve_type {
        CurveType::K256 => compute_key_share_proof_internal::<k256::Secp256k1>(&args, None).await,
        CurveType::P256 => compute_key_share_proof_internal::<p256::NistP256>(&args, None).await,
        CurveType::P384 => compute_key_share_proof_internal::<p384::NistP384>(&args, None).await,
        CurveType::Ed25519 => {
            compute_key_share_proof_internal::<bulletproofs::Ed25519>(&args, None).await
        }
        CurveType::Ristretto25519 => {
            compute_key_share_proof_internal::<bulletproofs::Ristretto25519>(&args, None).await
        }
        CurveType::Ed448 => {
            compute_key_share_proof_internal::<ed448_goldilocks::Ed448>(&args, None).await
        }
        CurveType::RedJubjub => {
            compute_key_share_proof_internal::<bulletproofs::JubJub>(
                &args,
                Some(lit_rust_crypto::red_jubjub_signing_generator()),
            )
            .await
        }
        CurveType::RedDecaf377 => {
            compute_key_share_proof_internal::<bulletproofs::Decaf377>(&args, None).await
        }
        CurveType::RedPallas => {
            compute_key_share_proof_internal::<pallas::Pallas>(
                &args,
                Some(lit_rust_crypto::red_pallas_signing_generator()),
            )
            .await
        }
        CurveType::BLS12381G1 => {
            if root_keys.is_empty() {
                return Err(unexpected_err(
                    "No primary BLS key found!".to_string(),
                    None,
                ));
            }
            let vrf_deriver_id =
                format!("{}{}", VRF_KEY_SHARE_VALIDATION_PREFIX, curve_type.as_str());

            let deriver =
                <Scalar as HDDeriver>::create(vrf_deriver_id.as_bytes(), curve_type.vrf_ctx());
            let key_cache = KeyCache::default();
            let (sk, _) = get_derived_keyshare::<G1Projective>(
                deriver,
                root_keys,
                curve_type,
                staker_address,
                &self_peer.peer_id,
                epoch,
                realm_id,
                &key_cache,
            )
            .await?;
            let signature: Signature<Bls12381G2Impl> = SecretKey(sk)
                .sign(SignatureSchemes::ProofOfPossession, noonce.as_bytes())
                .map_err(|_| unexpected_err("cannot generate BLS proof".to_string(), None))?;

            postcard::to_stdvec(&signature)
                .map_err(|e| unexpected_err(e, Some("cannot serialize BLS proof".to_string())))
        }
        _ => Err(unexpected_err("Unsupported curve type", None)),
    }
}

#[instrument(level = "debug", skip_all)]
async fn compute_key_share_proof_internal<H>(
    args: &ComputeKeyShareProofArgs<'_>,
    generator: Option<H::Group>,
) -> Result<Vec<u8>>
where
    H: VrfProver + VrfVerifier,
    H::Group: HDDerivable + CompressedBytes,
    <H::Group as Group>::Scalar: HDDeriver + CompressedBytes,
{
    if args.root_keys.is_empty() {
        return Err(unexpected_err(
            format!("No root keys found for {}!", args.curve_type),
            None,
        ));
    }
    let vrf_deriver_id = format!(
        "{}{}",
        VRF_KEY_SHARE_VALIDATION_PREFIX,
        args.curve_type.as_str()
    );

    let alpha =
        <<H::Group as Group>::Scalar as HDDeriver>::create(args.noonce.as_bytes(), args.dst);
    let deriver =
        <<H::Group as Group>::Scalar as HDDeriver>::create(vrf_deriver_id.as_bytes(), args.dst);
    let key_cache = KeyCache::default();
    let (sk, _) = get_derived_keyshare::<H::Group>(
        deriver,
        args.root_keys,
        args.curve_type,
        args.staker_address,
        args.peer_id,
        args.epoch,
        args.realm_id,
        &key_cache,
    )
    .await?;
    let proof = H::vrf_prove(&sk, &alpha, generator)
        .map_err(|e| unexpected_err(e, Some("cannot generate VRF proof".into())))?;
    postcard::to_stdvec(&proof)
        .map_err(|e| unexpected_err(e, Some("cannot serialize VRF proof".into())))
}

struct ComputeKeyShareProofArgs<'a> {
    root_keys: &'a [String],
    noonce: &'a str,
    curve_type: CurveType,
    dst: &'a [u8],
    peer_id: &'a PeerId,
    staker_address: &'a str,
    epoch: u64,
    realm_id: u64,
}

#[instrument(level = "debug", name = "verify_key_share_proofs", skip_all)]
#[allow(clippy::too_many_arguments)]
pub async fn verify_key_share_proofs(
    root_keys: &HashMap<CurveType, Vec<String>>,
    noonce: &str,
    my_addr: &str,
    their_addr: &str,
    staker_address: &str,
    key_share_proofs: &KeyShareProofs,
    peers: &SimplePeerCollection,
    epoch: u64,
    realm_id: u64,
) -> Result<BTreeMap<CurveType, Result<()>>> {
    let start = std::time::Instant::now();

    // If we are in epoch 1 or 0, we don't have any peers yet or key shares
    if epoch <= 1 || key_share_proofs.epoch <= 1 {
        trace!(
            "Epoch is 0 or 1, no key share proofs to verify: {} - {}",
            epoch, key_share_proofs.epoch
        );
        return Ok(BTreeMap::new());
    }

    if epoch != key_share_proofs.epoch {
        trace!(
            "Expected epoch {} but got {}",
            epoch, key_share_proofs.epoch
        );
        return Err(unexpected_err_code(
            format!(
                "Epoch mismatch in key share proofs, ours: {}, theirs: {}",
                epoch, key_share_proofs.epoch
            ),
            EC::IncorrectInfoForKeyShareValidation,
            None,
        ));
    }

    if !peers.contains_address(their_addr) {
        return Err(unexpected_err(
            format!("Peer {} not found in the set", their_addr),
            None,
        ));
    }
    if key_share_proofs.proofs.is_empty() {
        return Err(unexpected_err_code(
            format!("Peer {} has no key share proofs", their_addr),
            EC::IncorrectInfoForKeyShareValidation,
            None,
        ));
    }

    let self_peer = peers.peer_at_address(my_addr)?;
    let their_peer = peers.peer_at_address(their_addr)?;
    let peer_id = their_peer.peer_id;

    let mut verification_checks = BTreeMap::new();
    let mut args = VerifyKeyShareProofArgs {
        root_keys: &[],
        noonce,
        curve_type: CurveType::BLS,
        dst: &[],
        staker_address,
        epoch,
        realm_id,
        my_peer_id: &self_peer.peer_id,
        their_peer_id: &peer_id,
        proof: &[],
    };
    for (&curve_type, proof) in &key_share_proofs.proofs {
        args.root_keys = &root_keys[&curve_type];
        args.proof = &proof[..];
        args.dst = curve_type.vrf_ctx();
        args.curve_type = curve_type;
        match curve_type {
            CurveType::BLS => {
                if args.root_keys.is_empty() {
                    return Err(unexpected_err("No root keys found!".to_string(), None));
                }
                let key_cache = KeyCache::default();
                let commitments =
                    read_key_share_commitments_from_disk::<KeyShareCommitments<G1Projective>>(
                        curve_type,
                        &args.root_keys[0],
                        staker_address,
                        &self_peer.peer_id,
                        epoch, // this will possibly not be the same epoch as the node doing the request, and the results will be mismatched proofs.
                        realm_id,
                        &key_cache,
                    )
                    .await?;
                let sig_share = postcard::from_bytes::<SignatureShare<Bls12381G2Impl>>(args.proof)
                    .map_err(|e| {
                        unexpected_err(e, Some("cannot deserialize BLS proof".to_string()))
                    })?;

                let signature_point = sig_share.as_raw_value().0.value.0;
                let signature = match sig_share {
                    SignatureShare::Basic(sig) => {
                        Signature::<Bls12381G2Impl>::Basic(signature_point)
                    }
                    SignatureShare::MessageAugmentation(sig) => {
                        Signature::<Bls12381G2Impl>::MessageAugmentation(signature_point)
                    }
                    SignatureShare::ProofOfPossession(sig) => {
                        Signature::<Bls12381G2Impl>::ProofOfPossession(signature_point)
                    }
                };
                let key_share_commitment =
                    commitments.compute_key_share_commitment(&Scalar::from(peer_id));
                let pub_key = PublicKey::<Bls12381G2Impl>(key_share_commitment);
                verification_checks.insert(
                    curve_type,
                    signature.verify(&pub_key, noonce.as_bytes()).map_err(|e| {
                        unexpected_err(e, Some("BLS proof verification failed".to_string()))
                    }),
                );
            }
            CurveType::K256 => {
                verification_checks.insert(
                    curve_type,
                    verify_key_share_proofs_internal::<k256::Secp256k1>(&args, None).await,
                );
            }
            CurveType::P256 => {
                verification_checks.insert(
                    curve_type,
                    verify_key_share_proofs_internal::<p256::NistP256>(&args, None).await,
                );
            }
            CurveType::P384 => {
                verification_checks.insert(
                    curve_type,
                    verify_key_share_proofs_internal::<p384::NistP384>(&args, None).await,
                );
            }
            CurveType::Ed25519 => {
                verification_checks.insert(
                    curve_type,
                    verify_key_share_proofs_internal::<bulletproofs::Ed25519>(&args, None).await,
                );
            }
            CurveType::Ristretto25519 => {
                verification_checks.insert(
                    curve_type,
                    verify_key_share_proofs_internal::<bulletproofs::Ristretto25519>(&args, None)
                        .await,
                );
            }
            CurveType::Ed448 => {
                verification_checks.insert(
                    curve_type,
                    verify_key_share_proofs_internal::<ed448_goldilocks::Ed448>(&args, None).await,
                );
            }
            CurveType::RedJubjub => {
                verification_checks.insert(
                    curve_type,
                    verify_key_share_proofs_internal::<bulletproofs::JubJub>(
                        &args,
                        Some(lit_rust_crypto::red_jubjub_signing_generator()),
                    )
                    .await,
                );
            }
            CurveType::RedDecaf377 => {
                verification_checks.insert(
                    curve_type,
                    verify_key_share_proofs_internal::<bulletproofs::Decaf377>(&args, None).await,
                );
            }
            CurveType::RedPallas => {
                verification_checks.insert(
                    curve_type,
                    verify_key_share_proofs_internal::<pallas::Pallas>(
                        &args,
                        Some(lit_rust_crypto::red_pallas_signing_generator()),
                    )
                    .await,
                );
            }
            CurveType::BLS12381G1 => {
                if args.root_keys.is_empty() {
                    return Err(unexpected_err("No root keys found!".to_string(), None));
                }
                let peer_id_scalar = Scalar::from(peer_id);
                let mut key_share_commitments = Vec::with_capacity(root_keys.len());
                let key_cache = KeyCache::default();
                for (i, root_key) in args.root_keys.iter().enumerate() {
                    let commitments =
                        read_key_share_commitments_from_disk::<KeyShareCommitments<G1Projective>>(
                            curve_type,
                            root_key,
                            staker_address,
                            &self_peer.peer_id,
                            epoch, // this will possibly not be the same epoch as the node doing the request, and the results will be mismatched proofs.
                            realm_id,
                            &key_cache,
                        )
                        .await?;
                    let key_share_commitment =
                        commitments.compute_key_share_commitment(&peer_id_scalar);
                    key_share_commitments.push(key_share_commitment);
                }
                let signature = postcard::from_bytes::<Signature<Bls12381G2Impl>>(args.proof)
                    .map_err(|e| {
                        unexpected_err(e, Some("cannot deserialize BLS proof".to_string()))
                    })?;

                let vrf_deriver_id =
                    format!("{}{}", VRF_KEY_SHARE_VALIDATION_PREFIX, curve_type.as_str());
                let deriver =
                    <Scalar as HDDeriver>::create(vrf_deriver_id.as_bytes(), curve_type.vrf_ctx());

                let key_share_commitment =
                    <Scalar as HDDeriver>::hd_derive_public_key(&deriver, &key_share_commitments);
                let pub_key = PublicKey::<Bls12381G2Impl>(key_share_commitment);
                verification_checks.insert(
                    curve_type,
                    signature.verify(&pub_key, noonce.as_bytes()).map_err(|e| {
                        unexpected_err(e, Some("BLS proof verification failed".to_string()))
                    }),
                );
            }
        }
    }
    trace!(
        "Verified key share proofs in {}ms",
        start.elapsed().as_millis()
    );
    Ok(verification_checks)
}

#[instrument(level = "debug", skip_all)]
#[allow(clippy::needless_lifetimes)]
async fn verify_key_share_proofs_internal<'a, H>(
    args: &VerifyKeyShareProofArgs<'a>,
    generator: Option<H::Group>,
) -> Result<()>
where
    H: VrfVerifier,
    H::Group: HDDerivable,
    <H::Group as Group>::Scalar: HDDeriver + From<PeerId>,
{
    let peer_id = <H::Group as Group>::Scalar::from(*args.their_peer_id);
    if args.root_keys.is_empty() {
        return Err(unexpected_err("No root keys found!".to_string(), None));
    }

    let mut key_share_commitments = Vec::with_capacity(args.root_keys.len());
    let key_cache = KeyCache::default();
    for (i, root_key) in args.root_keys.iter().enumerate() {
        let commitments = read_key_share_commitments_from_disk::<KeyShareCommitments<H::Group>>(
            args.curve_type,
            root_key,
            args.staker_address,
            args.my_peer_id,
            args.epoch,
            args.realm_id,
            &key_cache,
        )
        .await?;
        let key_share_commitment = commitments.compute_key_share_commitment(&peer_id);
        key_share_commitments.push(key_share_commitment);
    }

    let vrf_deriver_id = format!(
        "{}{}",
        VRF_KEY_SHARE_VALIDATION_PREFIX,
        args.curve_type.as_str()
    );

    let alpha =
        <<H::Group as Group>::Scalar as HDDeriver>::create(args.noonce.as_bytes(), args.dst);
    let deriver =
        <<H::Group as Group>::Scalar as HDDeriver>::create(vrf_deriver_id.as_bytes(), args.dst);
    let proof = postcard::from_bytes::<Proof<H::Group>>(args.proof)
        .map_err(|e| unexpected_err(e, Some("cannot deserialize VRF proof".to_string())))?;

    let key_share_verifier = deriver.hd_derive_public_key(&key_share_commitments);
    H::vrf_verify(key_share_verifier, alpha, &proof, generator)
        .map_err(|e| unexpected_err(e, Some("VRF proof verification failed".to_string())))
}

struct VerifyKeyShareProofArgs<'a> {
    root_keys: &'a [String],
    curve_type: CurveType,
    dst: &'a [u8],
    noonce: &'a str,
    staker_address: &'a str,
    epoch: u64,
    realm_id: u64,
    my_peer_id: &'a PeerId,
    their_peer_id: &'a PeerId,
    proof: &'a [u8],
}

#[cfg(test)]
mod tests {
    use super::*;
    use lit_rust_crypto::{
        ff::Field,
        vsss_rs::{DefaultShare, IdentifierPrimeField, shamir},
    };
    use rand::{RngCore, SeedableRng};

    #[test]
    fn dkg_and_test_vrf() {
        type K256Share =
            DefaultShare<IdentifierPrimeField<k256::Scalar>, IdentifierPrimeField<k256::Scalar>>;

        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);
        let secret_keys = (0..10)
            .map(|_| k256::Scalar::random(&mut rng))
            .collect::<Vec<k256::Scalar>>();

        let mut commitments = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        let mut shares = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        for &secret_key in &secret_keys {
            let inner_shares = shamir::split_secret::<K256Share>(
                6,
                10,
                &IdentifierPrimeField(secret_key),
                &mut rng,
            )
            .unwrap();
            for (j, &share) in inner_shares.iter().enumerate() {
                let commitment = k256::ProjectivePoint::GENERATOR * share.value.0;
                shares[j].push(share.value.0);
                commitments[j].push(commitment);
            }
        }

        let mut noonce = [0u8; 32];
        rng.fill_bytes(&mut noonce);
        let vrf_deriver_id = format!(
            "{}{}",
            VRF_KEY_SHARE_VALIDATION_PREFIX,
            CurveType::K256.as_str()
        );
        let alpha = k256::Scalar::create(&noonce, CurveType::K256.vrf_ctx());
        let deriver = k256::Scalar::create(vrf_deriver_id.as_bytes(), CurveType::K256.vrf_ctx());

        for (inner_shares, inner_commitments) in shares.iter().zip(commitments.iter()) {
            let key_share_prover = deriver.hd_derive_secret_key(inner_shares);
            let key_share_verifier = deriver.hd_derive_public_key(inner_commitments);
            let proof = k256::Secp256k1::vrf_prove(&key_share_prover, &alpha, None).unwrap();
            assert!(k256::Secp256k1::vrf_verify(key_share_verifier, alpha, &proof, None).is_ok());
        }
    }
}
