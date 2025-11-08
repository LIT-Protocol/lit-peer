use crate::component::{dkg::dkg, utils::virtual_node_collection::VirtualNodeCollection};
use elliptic_curve::generic_array::ArrayLength;
use elliptic_curve::group::{Curve, GroupEncoding};
use elliptic_curve::{CurveArithmetic, FieldBytesSize, NonZeroScalar, PrimeCurve};
use ethers::utils::keccak256;
use futures::future::join_all;
use hd_keys_curves::{HDDerivable, HDDeriver};
use k256::ecdsa::hazmat::DigestPrimitive;
use lit_fast_ecdsa::SignatureShare;
use lit_node::peers::peer_state::models::SimplePeerCollection;
use lit_node::tasks::presign_manager::models::{PreSignatureValue, Presign};
use lit_node::tss::common::dkg_type::DkgType;
use lit_node::tss::common::tss_state::TssState;
use lit_node::tss::ecdsa_damfast::DamFastState;
use lit_node::utils::traits::SignatureCurve;
use lit_node_core::CompressedBytes;
use lit_node_core::PeerId;
use lit_node_core::SigningScheme;
use serde::Serialize;
use std::ops::Add;
use std::sync::Arc;
use test_case::test_case;
use tokio::task::JoinHandle;
use tracing::info;

#[test_case(SigningScheme::EcdsaK256Sha256;  "Sign ECDSA w/K256")]
#[test_case(SigningScheme::EcdsaP256Sha256; "Sign ECDSA w/P256")]
#[test_case(SigningScheme::EcdsaP384Sha384; "Sign ECDSA w/P384")]
#[tokio::test]
#[doc = "Test that ecdsa doesn't work with only 2 nodes"]
async fn sign_lower_bad_threshold(signing_scheme: SigningScheme) {
    const NUM_NODES: usize = 2;

    crate::common::setup_logging();
    info!("Starting test: ecdsa_dkg");
    let epoch = 1;
    let mut vnc = VirtualNodeCollection::new(NUM_NODES).await;

    vnc.update_cdm_epoch(epoch + 1).await;
    vnc.update_cdm_realm_id(1).await;

    let peers = SimplePeerCollection::default();
    let root_pubkey1 = dkg(&vnc, signing_scheme.curve_type(), epoch, None, &peers).await;
    let root_pubkey2 = dkg(&vnc, signing_scheme.curve_type(), epoch, None, &peers).await;
    info!(
        "Generated ECDSA root_pubkey1/root_pubkey2: {:?} / {:?}",
        root_pubkey1, root_pubkey2
    );

    let txn = "LIT1";
    let request_id = "lit_9489d2c30aa7b".as_bytes(); // Random
    let txn_prefix = format!(
        "{}_full_{}",
        txn,
        String::from_utf8(request_id.into()).unwrap(),
    );

    // generate Presigns
    info!("Generating Presigns");
    let presign_shares = generate_presigns(&vnc, txn_prefix, signing_scheme, NUM_NODES).await;

    presign_shares.iter().for_each(|r| {
        assert!(r.is_err(), "Presign generation should've failed");
    });
}

#[test_case(SigningScheme::EcdsaK256Sha256, k256::Secp256k1; "Sign ECDSA w/K256")]
#[test_case(SigningScheme::EcdsaP256Sha256, p256::NistP256; "Sign ECDSA w/P256")]
#[test_case(SigningScheme::EcdsaP384Sha384, p384::NistP384; "Sign ECDSA w/P384")]
#[tokio::test]
#[doc = "Test that signs a message using a set of virtual nodes."]
async fn sign_with_pubkey<C>(signing_scheme: SigningScheme, _c: C)
where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive + SignatureCurve,
    C::ProjectivePoint: GroupEncoding + HDDerivable + CompressedBytes,
    C::AffinePoint: Serialize,
    C::Scalar: HDDeriver + From<PeerId> + Serialize + CompressedBytes,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    let node_count = 3;
    do_sign_with_pubkey(_c, signing_scheme, node_count, 0).await;
}

pub async fn do_sign_with_pubkey<C>(
    _c: C,
    signing_scheme: SigningScheme,
    num_nodes: usize,
    _node_change: i16,
) where
    C: PrimeCurve + CurveArithmetic + DigestPrimitive + SignatureCurve,
    C::ProjectivePoint: GroupEncoding + HDDerivable + CompressedBytes,
    C::AffinePoint: Serialize,
    C::Scalar: HDDeriver + From<PeerId> + Serialize + CompressedBytes,
    <FieldBytesSize<C> as Add>::Output: ArrayLength<u8>,
{
    crate::common::setup_logging();
    info!("Starting test: ecdsa_dkg");
    let epoch = 1;
    let mut vnc = VirtualNodeCollection::new(num_nodes).await;

    vnc.update_cdm_epoch(epoch + 1).await;
    vnc.update_cdm_realm_id(1).await;
    let peers = SimplePeerCollection::default();
    let root_pubkey1 = dkg(&vnc, signing_scheme.curve_type(), epoch, None, &peers).await;
    let root_pubkey2 = dkg(&vnc, signing_scheme.curve_type(), epoch, None, &peers).await;
    info!(
        "Generated ECDSA root_pubkey1/root_pubkey2: {:?} / {:?}",
        root_pubkey1, root_pubkey2
    );

    // Arguments for generate_hd_key_signature_share_from_key_id:
    let txn = "LIT1";
    let request_id = "lit_9489d2c30aa7b".as_bytes(); // Random
    let txn_prefix = &format!(
        "{}_full_{}",
        txn,
        String::from_utf8(request_id.into()).unwrap(),
    );

    // generate Presigns
    info!("Generating Presigns");
    let start = std::time::Instant::now();
    let presign_shares =
        generate_presigns(&vnc, txn_prefix.clone(), signing_scheme, num_nodes).await;
    info!(
        "Generated Presigns in {} seconds",
        start.elapsed().as_secs()
    );

    let presign_shares = presign_shares
        .into_iter()
        .map(|tp| tp.unwrap())
        .collect::<Vec<_>>();

    let mut message_bytes = keccak256("Hello world!".as_bytes()).to_vec();
    if signing_scheme.ecdsa_message_len() != message_bytes.len() {
        message_bytes.resize(signing_scheme.ecdsa_message_len(), 0);
    }
    let key_id = "id".as_bytes();

    let signing_peers = vnc.peers();
    let mut v = Vec::new();
    let hd_root_keys = vec![root_pubkey1.clone(), root_pubkey2.clone()];

    for presign_share in presign_shares {
        let peer = match signing_peers
            .0
            .iter()
            .filter(|p| p.key_hash == presign_share.staker_hash)
            .last()
        {
            Some(p) => p,
            None => continue,
        };

        info!(
            "Key peer index {:} / staker address {:}",
            peer.peer_id, peer.staker_address
        );

        let node = vnc.node_by_staker_address(peer.staker_address).unwrap();
        let mut damfast_state = node.damfast_state(signing_scheme).clone();
        let peers = signing_peers.clone();

        let hd_root_keys = hd_root_keys.clone();
        let loop_message_bytes = message_bytes.clone();
        let jh: JoinHandle<(
            SignatureShare<C>,
            C::ProjectivePoint,
            C::Scalar,
            NonZeroScalar<C>,
        )> = tokio::task::spawn(async move {
            let sig_share = damfast_state
                .generate_signature_share_from_key_id::<C>(
                    &loop_message_bytes,
                    Some(hd_root_keys),
                    presign_share,
                    request_id,
                    &peers,
                    key_id,
                )
                .await;
            sig_share.expect("DamFast error signing with presig")
        });
        v.push(jh);
    }

    // wait for all nodes to complete
    let results = join_all(v).await;

    let mut sig_shares = Vec::new();
    let mut scalar_shares: Vec<C::Scalar> = Vec::new();
    let mut public_key = C::AffinePoint::default();
    for result in &results {
        assert!(result.is_ok());
        let share = result.as_ref().unwrap();

        sig_shares.push(share.0.clone());
        scalar_shares.push(share.0.clone().s);
        public_key = share.1.to_affine();
    }

    // damfast
    let sig = SignatureShare::<C>::combine_into_signature(&sig_shares).unwrap();
    info!("Full signature: {:?}", sig);
    info!("Public key: {:?}", public_key);

    let k_sig = ecdsa::Signature::<C>::try_from(sig.clone()).unwrap();

    info!("k_sig: {:?}", k_sig);

    // everything was keccak256'd earlier....
    // let scalar_primitive =
    //     elliptic_curve::ScalarPrimitive::<C>::from_slice(&message_bytes).unwrap();
    // let msg_digest = &C::Scalar::from(scalar_primitive).to_bytes();

    info!(
        "Verifying signature result with DamFast: {:?}",
        sig.verify_digest(&message_bytes, &public_key.into())
    );

    // cait sith

    // info!("Combining /w Cait-Sith");
    // let sig = lit_ecdsa_wasm_combine::combiners::cs_curve::combine_signature_shares::<C>(
    //     scalar_shares,
    //     public_key,
    //     presignature_big_r,
    //     msg_hash,
    // );
    //
    // info!("DamFast signature: {}", sig.is_ok());
    //
    // assert!(sig.is_ok());
}

async fn generate_presigns(
    vnc: &VirtualNodeCollection,
    txn_prefix: String,
    signing_scheme: SigningScheme,
    num_nodes: usize,
) -> Vec<Result<Presign, lit_core::error::Error>> {
    let mut v = Vec::new();

    let peers = SimplePeerCollection(vnc.peers().0[..num_nodes].to_vec());
    for node in vnc.nodes.iter().take(num_nodes) {
        let peers = peers.clone();
        let df_ecdsa_state = get_damfast_ecdsa_state(node.tss_state.clone(), signing_scheme);
        let txn_prefix_temp = txn_prefix.clone();
        let threshold = peers.threshold_for_set_testing_only();
        let hash = node.peer.key_hash;
        let jh: JoinHandle<Result<Presign, _>> = tokio::task::spawn(async move {
            let loop_peers = peers.clone();
            let r = match signing_scheme {
                SigningScheme::EcdsaK256Sha256 => df_ecdsa_state
                    .create_presignature_for_peers::<k256::Secp256k1>(
                        &txn_prefix_temp,
                        &mut peers.clone(),
                        threshold,
                    )
                    .await
                    .map(PreSignatureValue::K256),
                SigningScheme::EcdsaP256Sha256 => df_ecdsa_state
                    .create_presignature_for_peers::<p256::NistP256>(
                        &txn_prefix_temp,
                        &mut peers.clone(),
                        threshold,
                    )
                    .await
                    .map(PreSignatureValue::P256),
                SigningScheme::EcdsaP384Sha384 => df_ecdsa_state
                    .create_presignature_for_peers::<p384::NistP384>(
                        &txn_prefix_temp,
                        &mut peers.clone(),
                        threshold,
                    )
                    .await
                    .map(PreSignatureValue::P384),
                _ => panic!("Unsupported signing scheme"),
            };
            let presign = r?;
            Presign::new(presign, hash, &loop_peers)
        });
        v.push(jh);
    }

    // wait for all nodes to complete
    let results = join_all(v).await;
    let mut presigs = Vec::with_capacity(results.len());
    for result in results {
        assert!(result.is_ok());
        presigs.push(result.unwrap());
    }

    presigs
}

fn get_damfast_ecdsa_state(state: Arc<TssState>, signing_scheme: SigningScheme) -> DamFastState {
    DamFastState {
        state,
        signing_scheme,
        dkg_type: DkgType::Standard,
    }
}
