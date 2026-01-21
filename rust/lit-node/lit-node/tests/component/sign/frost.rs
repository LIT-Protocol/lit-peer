use crate::common::interpolation::read_key_share_from_disk_from_test_harness;
use crate::component::dkg::initial_dkg;
use futures::future::join_all;
use lit_frost::{
    Identifier, Scheme, SignatureShare, SigningCommitments, SigningShare, VerifyingKey,
    VerifyingShare,
};
use lit_node::peers::peer_state::models::SimplePeer;
use lit_node::tss::common::key_share::KeyShare;
use lit_node::tss::frost::FrostState;
use lit_node_core::PeerId;
use lit_node_core::SigningScheme;
use lit_sdk::signature::signing_scheme_to_frost_scheme;
use test_case::test_case;
use tokio::task::JoinHandle;
use tracing::info;

#[test_case(SigningScheme::SchnorrEd25519Sha512; "Sign using Ed25519")]
#[test_case(SigningScheme::SchnorrK256Sha256;  "Sign using K256")]
#[test_case(SigningScheme::SchnorrP256Sha256;  "Sign using P256")]
#[test_case(SigningScheme::SchnorrP384Sha384;  "Sign using P384")]
#[test_case(SigningScheme::SchnorrRistretto25519Sha512;  "Sign using Ristretto")]
#[test_case(SigningScheme::SchnorrEd448Shake256;  "Sign using Ed448")]
#[test_case(SigningScheme::SchnorrRedJubjubBlake2b512;  "Sign using RedJubJub")]
#[test_case(SigningScheme::SchnorrK256Taproot;  "Sign using Taproot")]
#[test_case(SigningScheme::SchnorrRedDecaf377Blake2b512;  "Sign using RedDecaf377")]
#[test_case(SigningScheme::SchnorrkelSubstrate; "Sign using Schnorrkel")]
#[tokio::test]
#[doc = "Test Frost signatures using a only 2 virtual nodes."]
async fn sign_lower_threshold(signing_scheme: SigningScheme) {
    crate::common::setup_logging();

    info!("Starting test: sign with frost using {:?}", &signing_scheme);
    let num_nodes = 2;
    let aggregation_scheme: Scheme = signing_scheme_to_frost_scheme(signing_scheme).unwrap();
    let curve_type = signing_scheme.curve_type();

    let (vnc, pubkey, epoch, signing_peers) = initial_dkg(curve_type, num_nodes).await;
    let threshold = signing_peers.threshold_for_set_testing_only();

    let txn_prefix = "FROST_SIGN";
    let message = "Hello world!".as_bytes();

    let mut v = Vec::with_capacity(signing_peers.0.len());
    let mut verifying_key = Option::<VerifyingKey>::None;
    for signing_node in &signing_peers.0 {
        let node = vnc
            .nodes
            .iter()
            .find(|n| n.peer.peer_id == signing_node.peer_id)
            .unwrap();
        let frost_state = FrostState::new(node.tss_state.clone(), signing_scheme);

        let (_, secret_share, vk, _) =
            load_frost_key_share(&signing_node, &pubkey, epoch, signing_scheme).await;
        if verifying_key.is_none() {
            verifying_key = Some(vk.clone());
        }

        let loop_peers = signing_peers.clone();

        let jh: JoinHandle<(
            Identifier,
            SignatureShare,
            SigningCommitments,
            VerifyingShare,
        )> = tokio::task::spawn(async move {
            let sig_share = frost_state
                .sign_internal(
                    txn_prefix,
                    &loop_peers,
                    message,
                    signing_scheme,
                    &vk,
                    &secret_share,
                    threshold,
                )
                .await;
            sig_share.expect("error from frost state sig_share")
        });
        v.push(jh);
    }

    // wait for all nodes to complete
    let results = join_all(v).await;

    let mut signing_commitments: Vec<(Identifier, SigningCommitments)> =
        Vec::with_capacity(results.len());
    let mut signature_shares: Vec<(Identifier, SignatureShare)> = Vec::with_capacity(results.len());
    let mut signer_pubkeys: Vec<(Identifier, VerifyingShare)> = Vec::with_capacity(results.len());
    for result in &results {
        assert!(result.is_ok());
        let (identifier, signature_share, commitments, verifying_share) =
            result.as_ref().expect("error unwrapping result");

        let identifier = identifier.clone();
        signing_commitments.push((identifier.clone(), commitments.clone()));
        signature_shares.push((identifier.clone(), signature_share.clone()));
        signer_pubkeys.push((identifier, verifying_share.clone()));
    }

    tracing::info!("Signature shares: {:?}", signature_shares);
    tracing::info!("Signing commitments: {:?}", signing_commitments);
    tracing::info!("Signer pubkeys: {:?}", signer_pubkeys);

    let verifying_key = verifying_key.expect("verifying key not found");
    // Aggregate also verifies the signature
    // so if this passes, the signature is valid
    let signature = aggregation_scheme
        .aggregate(
            message,
            &signing_commitments,
            &signature_shares,
            &signer_pubkeys,
            &verifying_key,
        )
        .expect("error aggregating signature");

    tracing::info!("Signature: {:?}", signature);
}

#[test_case(SigningScheme::SchnorrEd25519Sha512; "Sign using Ed25519")]
#[test_case(SigningScheme::SchnorrK256Sha256;  "Sign using K256")]
#[test_case(SigningScheme::SchnorrP256Sha256;  "Sign using P256")]
#[test_case(SigningScheme::SchnorrP384Sha384;  "Sign using P384")]
#[test_case(SigningScheme::SchnorrRistretto25519Sha512;  "Sign using Ristretto")]
#[test_case(SigningScheme::SchnorrEd448Shake256;  "Sign using Ed448")]
#[test_case(SigningScheme::SchnorrRedJubjubBlake2b512;  "Sign using RedJubJub")]
#[test_case(SigningScheme::SchnorrK256Taproot;  "Sign using Taproot")]
#[test_case(SigningScheme::SchnorrRedDecaf377Blake2b512;  "Sign using RedDecaf377")]
#[test_case(SigningScheme::SchnorrkelSubstrate; "Sign using Schnorrkel")]
#[tokio::test]
#[doc = "Test Frost signatures using a set of virtual nodes."]
async fn sign_with_pubkey(signing_scheme: SigningScheme) {
    crate::common::setup_logging();
    info!("Starting test: sign with frost using {:?}", &signing_scheme);
    let num_nodes = 5;
    let aggregation_scheme: Scheme = signing_scheme_to_frost_scheme(signing_scheme).unwrap();
    let curve_type = signing_scheme.curve_type();

    let (vnc, pubkey, epoch, signing_peers) = initial_dkg(curve_type, num_nodes).await;
    let threshold = signing_peers.threshold_for_set_testing_only();

    let txn_prefix = "FROST_SIGN";
    let message = "Hello world!".as_bytes();

    let mut v = Vec::with_capacity(signing_peers.0.len());
    let mut verifying_key = Option::<VerifyingKey>::None;
    for signing_node in &signing_peers.0 {
        let node = vnc
            .nodes
            .iter()
            .find(|n| n.peer.peer_id == signing_node.peer_id)
            .unwrap();
        let frost_state = FrostState::new(node.tss_state.clone(), signing_scheme);

        let (_, secret_share, vk, _) =
            load_frost_key_share(&signing_node, &pubkey, epoch, signing_scheme).await;
        if verifying_key.is_none() {
            verifying_key = Some(vk.clone());
        }

        let loop_peers = signing_peers.clone();

        let jh: JoinHandle<(
            Identifier,
            SignatureShare,
            SigningCommitments,
            VerifyingShare,
        )> = tokio::task::spawn(async move {
            let sig_share = frost_state
                .sign_internal(
                    txn_prefix,
                    &loop_peers,
                    message,
                    signing_scheme,
                    &vk,
                    &secret_share,
                    threshold,
                )
                .await;
            sig_share.expect("error from frost state sig_share")
        });
        v.push(jh);
    }

    // wait for all nodes to complete
    let results = join_all(v).await;

    let mut signing_commitments: Vec<(Identifier, SigningCommitments)> =
        Vec::with_capacity(results.len());
    let mut signature_shares: Vec<(Identifier, SignatureShare)> = Vec::with_capacity(results.len());
    let mut signer_pubkeys: Vec<(Identifier, VerifyingShare)> = Vec::with_capacity(results.len());
    for result in &results {
        assert!(result.is_ok());
        let (identifier, signature_share, commitments, verifying_share) =
            result.as_ref().expect("error unwrapping result");

        let identifier = identifier.clone();
        signing_commitments.push((identifier.clone(), commitments.clone()));
        signature_shares.push((identifier.clone(), signature_share.clone()));
        signer_pubkeys.push((identifier, verifying_share.clone()));
    }

    tracing::info!("Signature shares: {:?}", signature_shares);
    tracing::info!("Signing commitments: {:?}", signing_commitments);
    tracing::info!("Signer pubkeys: {:?}", signer_pubkeys);

    let verifying_key = verifying_key.expect("verifying key not found");
    // Aggregate also verifies the signature
    // so if this passes, the signature is valid
    let signature = aggregation_scheme
        .aggregate(
            message,
            &signing_commitments,
            &signature_shares,
            &signer_pubkeys,
            &verifying_key,
        )
        .expect("error aggregating signature");

    tracing::info!("Signature: {:?}", signature);
}

pub async fn load_frost_key_share(
    peer: &SimplePeer,
    pubkey: &str,
    epoch: u64,
    signing_scheme: SigningScheme,
) -> (PeerId, SigningShare, VerifyingKey, usize) {
    let curve_type = signing_scheme.curve_type();
    let realm_id = 1;
    let key_share = read_key_share_from_disk_from_test_harness::<KeyShare>(
        peer, pubkey, epoch, curve_type, realm_id,
    )
    .await
    .expect("Failed to load key share");

    let scheme: Scheme = signing_scheme_to_frost_scheme(signing_scheme).unwrap();
    let signing_share = SigningShare {
        scheme,
        value: key_share.secret_as_bytes().unwrap(),
    };
    let public_key = VerifyingKey {
        scheme,
        value: key_share.public_key_as_bytes().unwrap(),
    };
    (
        key_share.peer_id,
        signing_share,
        public_key,
        key_share.threshold,
    )
}
