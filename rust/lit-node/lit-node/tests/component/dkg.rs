use super::utils::virtual_node_collection::{VirtualNode, VirtualNodeCollection};
use crate::common::interpolation::{get_secret_and_shares, interpolate_secret};
use ed448_goldilocks::EdwardsPoint;
use elliptic_curve::{Group, group::GroupEncoding};
use ethers::types::{H160, U256};
use futures::future::join_all;
use lit_blockchain::contracts::backup_recovery::RecoveredPeerId;
use lit_core::utils::binary::bytes_to_hex;
use lit_node::common::key_helper::KeyCache;
use lit_node::config::chain::CachedRootKey;
use lit_node::peers::peer_state::models::SimplePeerCollection;
use lit_node::tss::common::dkg_type::DkgType;
use lit_node::tss::common::key_share::KeyShare;
use lit_node::tss::common::storage::{
    delete_key_share_commitments_older_than_epoch, read_key_share_from_disk,
    write_key_share_to_cache_only,
};
use lit_node::tss::dkg::engine::{DkgAfterRestore, DkgAfterRestoreData, DkgEngine};
use lit_node::utils::key_share_proof::{compute_key_share_proofs, verify_key_share_proofs};
use lit_node::version::DataVersionWriter;
use lit_node_core::CompressedBytes;
use lit_node_core::CurveType;
use lit_node_core::PeerId;
use std::collections::HashMap;
use test_case::test_case;
use tokio::task::JoinHandle;
use tracing::{error, info};
use vsss_rs::curve25519::{WrappedEdwards, WrappedRistretto};

// The following tests show how components can be tested in isolation.
#[test_case(CurveType::K256; "K256 Key generation")]
#[test_case(CurveType::BLS; "BLS Key generation")]
#[test_case(CurveType::Ed25519; "Ed25519 Key generation")]
#[test_case(CurveType::Ristretto25519; "Ristretto25519 Key generation")]
#[test_case(CurveType::Ed448; "Ed448 Key generation")]
#[test_case(CurveType::P256; "P256 Key generation")]
#[test_case(CurveType::P384; "P384 Key generation")]
#[test_case(CurveType::RedJubjub; "RedJubjub Key generation")]
#[test_case(CurveType::RedDecaf377; "RedDecaf377 Key generation")]
#[test_case(CurveType::BLS12381G1; "Bls12381G1 Key Generation")]
#[tokio::test]
#[doc = "Test that a DKG can be run on a set of virtual nodes."]
pub async fn dkg_only(curve_type: CurveType) {
    initial_dkg(curve_type, 3).await;
}

#[test_case(CurveType::K256; "K256 Key Share Proofs")]
#[test_case(CurveType::BLS; "BLS Key Share Proofs")]
#[test_case(CurveType::Ed25519; "Ed25519 Key Share Proofs")]
#[test_case(CurveType::Ristretto25519; "Ristretto25519 Key Share Proofs")]
#[test_case(CurveType::Ed448; "Ed448 Key Share Proofs")]
#[test_case(CurveType::P256; "P256 Key Share Proofs")]
#[test_case(CurveType::P384; "P384 Key Share Proofs")]
#[test_case(CurveType::RedJubjub; "RedJubjub Key Share Proofs")]
#[test_case(CurveType::RedDecaf377; "RedDecaf377 Key Share Proofs")]
#[test_case(CurveType::BLS12381G1; "Bls12381G1 Key Share Proofs")]
#[tokio::test]
pub async fn dkg_and_key_share_proofs(curve_type: CurveType) {
    let (mut vnc, pubkey, epoch, current_peers) = initial_dkg(curve_type, 3).await;
    let mut root_keys_map = HashMap::<CurveType, Vec<String>>::new();
    {
        for node in &mut vnc.nodes {
            let mut peers_in_current_epoch = DataVersionWriter::new_unchecked(
                &node
                    .tss_state
                    .peer_state
                    .chain_data_config_manager
                    .peers
                    .peers_for_current_epoch,
            );
            peers_in_current_epoch.epoch_number = epoch;
            peers_in_current_epoch.commit();
            let mut root_keys = DataVersionWriter::new_unchecked(
                &node.tss_state.chain_data_config_manager.root_keys,
            );
            root_keys.push(CachedRootKey {
                curve_type,
                public_key: pubkey.clone(),
            });
            root_keys.commit();
            root_keys_map
                .entry(curve_type)
                .and_modify(|v| v.push(pubkey.clone()))
                .or_insert(vec![pubkey.clone()]);
        }
    }

    let mut proofs = Vec::with_capacity(3);

    for node in &vnc.nodes {
        let res = compute_key_share_proofs(
            "test",
            &root_keys_map,
            &node.tss_state.addr,
            &current_peers,
            1,
            epoch,
        )
        .await;
        assert!(res.is_ok());
        proofs.push(res.map_err(|e| e.to_string()));
    }

    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                continue;
            }
            let proofs = proofs.clone();
            let key_share_proofs = proofs[i].as_ref().unwrap();
            let res = verify_key_share_proofs(
                &root_keys_map,
                "test",
                &vnc.nodes[i].tss_state.addr,
                &vnc.nodes[i].addr,
                &vnc.nodes[i].tss_state.peer_state.hex_staker_address(),
                &key_share_proofs,
                &current_peers,
                epoch,
                1,
            )
            .await;
            assert!(res.is_ok());
            let curves = res.unwrap();
            let result = curves.get(&curve_type).unwrap();
            assert!(result.is_ok());
        }
    }
}

#[test_case(k256::ProjectivePoint::default(), CurveType::K256; "K256 Refresh")]
#[test_case(blsful::inner_types::G1Projective::default(), CurveType::BLS; "BLS Refresh")]
#[test_case(WrappedEdwards::default(), CurveType::Ed25519; "Ed25519 Refresh")]
#[test_case(WrappedRistretto::default(), CurveType::Ristretto25519; "Ristretto25519 Refresh")]
#[test_case(EdwardsPoint::default(), CurveType::Ed448; "Ed448 Refresh")]
#[test_case(p256::ProjectivePoint::default(), CurveType::P256; "P256 Refresh")]
#[test_case(p384::ProjectivePoint::default(), CurveType::P384; "P384 Refresh")]
#[test_case(jubjub::SubgroupPoint::default(), CurveType::RedJubjub; "RedJubjub Refresh")]
#[test_case(decaf377::Element::default(), CurveType::RedDecaf377; "RedDecaf377 Refresh")]
#[test_case(blsful::inner_types::G1Projective::default(), CurveType::BLS12381G1; "Bls12381G1 Key Generation")]
#[tokio::test]
pub async fn dkg_and_refresh<G>(_g: G, curve_type: CurveType)
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: From<PeerId> + CompressedBytes,
{
    crate::common::setup_logging();
    let num_nodes = 5;
    // initial setup
    let (mut vnc, pubkey, epoch, _current_peers) = initial_dkg(curve_type, num_nodes).await;
    // do a refresh
    let (next_epoch, _current_peers) = refresh_dkg::<G>(curve_type, &vnc, &pubkey, epoch).await;
    assert_eq!(epoch + 1, next_epoch);
    vnc.shutdown().await;
}

// For a reshare the public key resolves to the original public key
#[test_case(k256::ProjectivePoint::default(), CurveType::K256, 3, [1,0].to_vec() ; "K256 add node, keep threshold")]
#[test_case(blsful::inner_types::G1Projective::default(), CurveType::BLS, 3, [1, 0].to_vec() ; "BLS add node, keep threshold")]
#[test_case(blsful::inner_types::G1Projective::default(), CurveType::BLS12381G1, 3, [1, 0].to_vec(); "Bls12381G1 add node, keep threshold")]
#[test_case(WrappedEdwards::default(), CurveType::Ed25519, 3, [1, 0].to_vec(); "Ed25519 add node, keep threshold")]
#[test_case(WrappedRistretto::default(), CurveType::Ristretto25519, 3, [1, 0].to_vec(); "Ristretto25519 add node, keep threshold")]
#[test_case(ed448_goldilocks::EdwardsPoint::default(), CurveType::Ed448, 3, [1, 0].to_vec(); "Ed448 add node, keep threshold")]
#[test_case(p256::ProjectivePoint::default(), CurveType::P256, 3, [1, 0].to_vec(); "P256 add node, keep threshold")]
#[test_case(p384::ProjectivePoint::default(), CurveType::P384, 3, [1, 0].to_vec(); "P384 add node, keep threshold")]
#[test_case(jubjub::SubgroupPoint::default(), CurveType::RedJubjub, 3, [1, 0].to_vec(); "RedJubjub add node, keep threshold")]
#[test_case(decaf377::Element::default(), CurveType::RedDecaf377, 3, [1, 0].to_vec(); "RedDecaf377 add node, keep threshold")]
// #[test_case( CurveType::K256, 4, [-2,0].to_vec() ; "ECDSA remove node, keep threshold")]
// #[test_case( CurveType::BLS, 4, [-2,0].to_vec() ; "BLS remove node, keep threshold")]
// #[test_case( CurveType::K256, 4, [-1,0].to_vec() ; "ECDSA first node, keep threshold")]
// #[test_case( CurveType::K256, 4, [1,0].to_vec() ; "ECDSA add node, change threshold")]
#[test_case(k256::ProjectivePoint::default(), CurveType::K256, 5, [-1,0].to_vec() ; "K256 remove node, change threshold")]
#[test_case(blsful::inner_types::G1Projective::default(), CurveType::BLS, 5, [-1,0].to_vec() ; "BLS remove node, change threshold")]
#[test_case(blsful::inner_types::G1Projective::default(), CurveType::BLS12381G1, 5, [-1,0].to_vec() ; "BLS12381G1 remove node, change threshold")]
// #[test_case( CurveType::K256, 3, [1,2,0].to_vec() ; "ECDSA add two nodes")]
// #[test_case( CurveType::K256, 6, [-2,-2,0].to_vec() ; "ECDSA remove two nodes")]
// #[test_case( CurveType::K256, 6, [4,4,-2,-2,0].to_vec() ; "ECDSA add two nodes and remove two nodes")]
// #[test_case( CurveType::K256, 5, [1,2,1,2,1,2,1,2,1,2,0].to_vec() ; "ECDSA add 10 nodes to 5 node network")]
// #[test_case( CurveType::K256, 10, [-1,-1,-5,0,-4,0,-3,0,-1,0,3,0,-1,5,5,0].to_vec() ; "ECDSA reshare batch: 10 r3 a1 a1 a1 a1 r3")]
// For a reshare the public key resolves to the original public key
// #[test_case( CurveType::K256, 5, [1,1,0,-4,0,-3,0,-1,0,4,3,0,1,5,5,0].to_vec() ; "ECDSA reshare batch: 5 a2 r1 r1 r1 a2 a3")]
#[tokio::test]
pub async fn dkg_and_reshare<G>(
    _g: G,
    curve_type: CurveType,
    num_nodes: usize,
    node_changes: Vec<i8>,
) where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: From<PeerId> + CompressedBytes,
{
    crate::common::setup_logging();
    info!(
        "curve_type: {}, num_nodes: {}, node_changes: {:?}",
        curve_type, num_nodes, node_changes
    );
    struct EpochHistory {
        epoch: u64,
        threshold: usize,
        peers: SimplePeerCollection,
    }

    let mut epoch_history: Vec<EpochHistory> = vec![];

    // initial setup
    let (mut vnc, pubkey, epoch, current_peers) = initial_dkg(curve_type, num_nodes).await;

    let history = EpochHistory {
        epoch,
        threshold: current_peers.threshold_for_set_testing_only(),
        peers: current_peers.clone(),
    };
    epoch_history.push(history);

    for node_change in node_changes {
        let index = if node_change.unsigned_abs() as usize > vnc.nodes.len() {
            vnc.nodes.len()
        } else {
            node_change.unsigned_abs() as usize
        };

        if node_change > 0 {
            let _added = vnc.insert_node(index).await;
        } else if node_change < 0 {
            let _removed = vnc.remove_node(index).await;
        }

        if node_change == 0 {
            let existing_peers = epoch_history.iter().last().unwrap().peers.clone();
            let epoch = epoch_history.iter().last().unwrap().epoch;

            let (next_epoch, current_peers) =
                reshare_dkg::<G>(curve_type, &vnc, &existing_peers, &pubkey, epoch).await;

            // save it!
            let history = EpochHistory {
                epoch: next_epoch,
                threshold: current_peers.threshold_for_set_testing_only(),
                peers: current_peers.clone(),
            };
            epoch_history.push(history);
            assert_eq!(epoch + 1, next_epoch);
        }
    }

    for history in epoch_history {
        info!(
            "Epoch: {}, Threshold: {}, Peers [{}]: {:?}",
            history.epoch,
            history.threshold,
            history.peers.0.len(),
            history.peers.debug_addresses()
        );
    }

    vnc.shutdown().await;
}

#[test_case(k256::ProjectivePoint::default(), CurveType::K256, 5, 5; "K256 restore 5 nodes")]
#[test_case(blsful::inner_types::G1Projective::default(), CurveType::BLS, 5, 3;  "BLS restore 5 nodes")]
#[test_case(blsful::inner_types::G1Projective::default(), CurveType::BLS12381G1, 3, 3; "Bls12381G1 restore 3 nodes")]
#[test_case(WrappedEdwards::default(), CurveType::Ed25519, 5, 4; "Ed25519 restore 5 nodes")]
#[test_case(WrappedRistretto::default(), CurveType::Ristretto25519, 5, 3; "Ristretto25519 restore 5 nodes")]
#[test_case(ed448_goldilocks::EdwardsPoint::default(), CurveType::Ed448, 3, 3; "Ed448 restore 3 nodes")]
#[test_case(p256::ProjectivePoint::default(), CurveType::P256, 3, 3; "P256 restore 3 nodes")]
#[test_case(p384::ProjectivePoint::default(), CurveType::P384, 3, 3; "P384 restore 3 nodes")]
#[test_case(jubjub::SubgroupPoint::default(), CurveType::RedJubjub, 5, 4; "RedJubjub restore 5 nodes")]
#[test_case(decaf377::Element::default(), CurveType::RedDecaf377, 3, 3; "RedDecaf377 restore 3 nodes")]
#[tokio::test]
pub async fn dkg_after_restore<G>(
    _g: G,
    curve_type: CurveType,
    num_nodes_before: usize,
    num_nodes_after: usize,
) where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: From<PeerId> + CompressedBytes,
{
    crate::common::setup_logging();
    let mut vnc_before = VirtualNodeCollection::new(num_nodes_before).await;
    let staker_addresses = vnc_before.staker_addresses();
    let current_peers = SimplePeerCollection(vec![]);
    let next_peers = vnc_before.peers();
    let dkg_id = "TEST_DKG_1_1.";
    let realm_id = 1;
    let threshold = next_peers.threshold_for_set_testing_only();
    let mut join_set = tokio::task::JoinSet::new();

    for node in vnc_before.nodes.iter() {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let mut dkg_engine = DkgEngine::new(
            node.tss_state.clone(),
            DkgType::Standard,
            1,
            threshold,
            (1, realm_id),
            &current_peers,
            &next_peers,
            DkgAfterRestore::False,
        );
        for i in 0..2 {
            let dkg_id = format!("{}{}_key_{}", dkg_id, curve_type, i + 1);
            dkg_engine.add_dkg(&dkg_id, curve_type, None);
        }
        join_set.spawn(async move {
            let r = dkg_engine.execute(dkg_id, realm_id).await;
            info!("change epoch result: {:?}", r);
            let _ = r.expect("error from dkg manager change epoch");
            let root_keys = dkg_engine.get_dkgs().collect::<Vec<_>>();
            assert_eq!(root_keys.len(), 2);
            root_keys
                .iter()
                .map(|r| r.result().unwrap().public_key())
                .collect::<Vec<_>>()
        });
    }

    let mut root_keys = Vec::new();

    while let Some(node_info) = join_set.join_next().await {
        let result = node_info.expect("error from dkg engine");
        if root_keys.is_empty() {
            root_keys = result;
        } else {
            assert_eq!(root_keys, result, "nodes generated different root keys");
        }
    }

    let current_peers = vnc_before.peers();
    let mut initial_secrets = Vec::with_capacity(root_keys.len());
    for pubkey in &root_keys {
        let secret = interpolate_secret(curve_type, &current_peers, pubkey, 2, realm_id).await;
        initial_secrets.push(secret);
    }
    vnc_before.shutdown().await;

    let mut vnc_after = VirtualNodeCollection {
        nodes: Vec::new(),
        testnet: vnc_before.testnet,
    };

    for i in 0..num_nodes_after {
        let port = (7470 + num_nodes_before + i) as u16;
        // the key hash must be unique, so if we're using 0 as the staker address, we generate a random one
        let staker_address = staker_addresses.get(i).unwrap().clone();
        vnc_after.add_node_internal(port, staker_address).await;
    }
    // setup all the background channels
    vnc_after.update_internal_state().await;

    let mut recovered_peer_ids = vec![];
    let mut recovery_key_cache = KeyCache::default();
    for (old_node, new_node) in vnc_before.nodes.iter().zip(vnc_after.nodes.iter()) {
        restore(
            old_node,
            new_node,
            &root_keys,
            curve_type,
            2,
            &mut recovery_key_cache,
        )
        .await;
        recovered_peer_ids.push(RecoveredPeerId {
            node_address: H160::random(),
            old_peer_id: U256::from(old_node.peer.peer_id),
            new_peer_id: U256::from(new_node.peer.peer_id),
        });
    }

    let dkg_id = "TEST_DKG_1_2.";
    join_set = tokio::task::JoinSet::new();

    let next_peers = vnc_after.peers();
    let threshold = next_peers.threshold_for_set_testing_only();
    for node in vnc_after.nodes.iter() {
        // assume this wait is because the join set starts executing immediately on creation
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let mut dkg_engine = DkgEngine::new(
            node.tss_state.clone(),
            DkgType::Standard,
            2,
            threshold,
            (2, realm_id),
            &current_peers,
            &next_peers,
            DkgAfterRestore::True(DkgAfterRestoreData {
                peers: recovered_peer_ids.clone(),
                key_cache: recovery_key_cache.clone(),
            }),
        );
        for (i, pubkey) in root_keys.iter().enumerate() {
            let dkg_id = format!("{}{}_key_{}", dkg_id, curve_type, i + 1);
            dkg_engine.add_dkg(&dkg_id, curve_type, Some(pubkey.clone()));
        }
        join_set.spawn(async move {
            let r = dkg_engine.execute(dkg_id, realm_id).await;
            info!("change epoch result: {:?}", r);
            let _ = r.expect("error from dkg manager change epoch");
            let root_keys = dkg_engine.get_dkgs().collect::<Vec<_>>();
            assert_eq!(root_keys.len(), 2);
            root_keys
                .iter()
                .map(|r| r.result().unwrap().public_key())
                .collect::<Vec<_>>()
        });
    }

    while let Some(node_info) = join_set.join_next().await {
        let _ = node_info.expect("error from dkg engine");
    }

    for (i, pubkey) in root_keys.iter().enumerate() {
        let secret = interpolate_secret(curve_type, &next_peers, pubkey, 3, realm_id).await;
        assert_eq!(
            secret, initial_secrets[i],
            "secrets do not match after restore"
        );
    }
}

#[tokio::test]
pub async fn dkg_only_all_curves() {
    crate::common::setup_logging();
    info!("Starting dkg test");
    let num_nodes = 7;
    let epoch = 1;
    let vnc = VirtualNodeCollection::new(num_nodes).await;

    let current_peers = SimplePeerCollection(vec![]);
    let pubkeys = dkg_all_curves(&vnc, epoch, &current_peers).await;

    info!("Generated {} pubkeys", pubkeys.len());
    assert_eq!(pubkeys.len(), 20);
}

async fn restore(
    old_node: &VirtualNode,
    new_node: &VirtualNode,
    root_keys: &[String],
    curve_type: CurveType,
    epoch: u64,
    cache: &mut KeyCache,
) {
    let realm_id = 1;
    let mut read_cache = KeyCache::default();
    for pub_key in root_keys {
        let key_share: KeyShare = read_key_share_from_disk(
            curve_type,
            pub_key,
            &old_node.hex_staker_address,
            &old_node.peer.peer_id,
            epoch,
            realm_id,
            &mut read_cache,
        )
        .await
        .expect("key share not found");

        delete_key_share_commitments_older_than_epoch(
            curve_type,
            pub_key,
            &old_node.hex_staker_address,
            &old_node.peer.peer_id,
            epoch + 1,
            realm_id,
            &mut read_cache,
        )
        .await
        .expect("failed to remove the key share commitments");

        write_key_share_to_cache_only(
            curve_type,
            pub_key,
            &new_node.peer.peer_id,
            &new_node.hex_staker_address,
            epoch,
            realm_id,
            cache,
            &key_share,
        )
        .await
        .expect("write key share to disk failed");
    }
}

pub async fn initial_dkg(
    curve_type: CurveType,
    num_nodes: usize,
) -> (VirtualNodeCollection, String, u64, SimplePeerCollection) {
    crate::common::setup_logging();
    info!("Starting dkg test {}", curve_type.to_string());
    let epoch = 1;
    let realm_id = 1;
    let mut vnc = VirtualNodeCollection::new(num_nodes).await;
    vnc.update_cdm_realm_id(1).await;

    let peers = vnc.peers();

    let current_peers = SimplePeerCollection(vec![]);
    let pubkey = dkg(&vnc, curve_type, epoch, None, &current_peers).await;
    let epoch = epoch + 1;

    info!("Generated {} pubkey: {:?}", curve_type.to_string(), pubkey);
    let initial_secret = interpolate_secret(curve_type, &peers, &pubkey, epoch, realm_id).await;
    let peers = vnc.peers();

    info!(
        "Initial interpolated secret: {:?}",
        bytes_to_hex(&initial_secret.to_bytes())
    );

    (vnc, pubkey, epoch, peers)
}

pub async fn refresh_dkg<G>(
    curve_type: CurveType,
    vnc: &VirtualNodeCollection,
    pubkey: &String,
    epoch: u64,
) -> (u64, SimplePeerCollection)
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: From<PeerId> + CompressedBytes,
{
    let peers = vnc.peers();
    let realm_id = 1;
    // get the secret & shares before doing the DKG (these keyshares aren't deleted, but for good form!)
    let (initial_secret, initial_shares) =
        get_secret_and_shares::<G>(curve_type, pubkey, &peers, epoch, realm_id).await;

    // the current/next peers are the same, which causes a refresh
    let current_peers = peers.clone();
    let _result = dkg(vnc, curve_type, epoch, Some(pubkey.clone()), &current_peers).await;
    let epoch = epoch + 1;

    let (refresh_secret, refresh_shares) =
        get_secret_and_shares::<G>(curve_type, pubkey, &peers, epoch, realm_id).await;

    let msg = format!(
        "Interpolated Secret (pre/post): {:?} / {:?}",
        bytes_to_hex(&initial_secret.to_bytes()),
        bytes_to_hex(&refresh_secret.to_bytes())
    );
    match initial_secret == refresh_secret {
        true => info!("{}", msg),
        false => error!("{}", msg),
    }
    assert_eq!(initial_secret, refresh_secret);

    assert!(
        initial_shares
            .iter()
            .zip(refresh_shares)
            .all(|(a, b)| a != &b)
    );

    let peers = vnc.peers();
    (epoch, peers)
}

pub async fn reshare_dkg<G>(
    curve_type: CurveType,
    vnc: &VirtualNodeCollection,
    current_peers: &SimplePeerCollection,
    pubkey: &String,
    epoch: u64,
) -> (u64, SimplePeerCollection)
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: From<PeerId> + CompressedBytes,
{
    let realm_id = 1;
    // get the SimplePeer before doing the DKG (these keyshares aren't deleted, but for good form!)
    let (initial_secret, _initial_shares) =
        get_secret_and_shares::<G>(curve_type, pubkey, current_peers, epoch, realm_id).await;

    // the current/next peers are the same, which causes a refresh
    let _result = dkg(vnc, curve_type, epoch, Some(pubkey.clone()), current_peers).await;
    let epoch = epoch + 1;

    let (reshare_secret, _reshare_shares) =
        get_secret_and_shares::<G>(curve_type, pubkey, &vnc.peers(), epoch, realm_id).await;

    let msg = format!(
        "Interpolated Secret (pre/post): {:?} / {:?}",
        bytes_to_hex(&initial_secret.to_bytes()),
        bytes_to_hex(&reshare_secret.to_bytes())
    );
    match initial_secret == reshare_secret {
        true => info!("{}", msg),
        false => error!("{}", msg),
    }
    assert_eq!(initial_secret, reshare_secret);

    let peers = vnc.peers();
    (epoch, peers)
}

pub async fn dkg(
    vnc: &VirtualNodeCollection,
    curve_type: CurveType,
    epoch: u64,
    pubkey: Option<String>,
    current_peers: &SimplePeerCollection,
) -> String {
    // setup virtual nodes
    let next_peers = vnc.peers();
    let realm_id = 1;
    info!("Starting DKG on virtual nodes.");
    info!("DKG Included peers: {:?}", next_peers.debug_addresses());

    let mut v = Vec::new();
    let dkg_id = "TEST_DKG_1_1.KEYTYPE";
    // start keygen on all nodes
    let threshold = next_peers.threshold_for_set_testing_only();
    for node in vnc.nodes.iter() {
        // this is a representation of what happens - but is not exhaustive

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let mut dkg_engine = DkgEngine::new(
            node.tss_state.clone(),
            DkgType::Standard,
            epoch,
            threshold,
            (epoch, realm_id),
            current_peers,
            &next_peers,
            DkgAfterRestore::False,
        );
        dkg_engine.add_dkg(dkg_id, curve_type, pubkey.clone());

        let jh: JoinHandle<String> = tokio::task::spawn(async move {
            let r = dkg_engine.execute(dkg_id, realm_id).await;
            info!("change epoch result: {:?}", r);
            let _ = r.expect("error from dkg manager change epoch");
            let root_keys = dkg_engine.get_dkgs().collect::<Vec<_>>();
            assert_eq!(root_keys.len(), 1);
            let root_key = root_keys[0].result().expect("a result");
            root_key.public_key()
        });
        v.push(jh);
    }

    // wait for all nodes to complete
    let results = join_all(v).await;
    for result in &results {
        assert!(result.is_ok()); // weak test, but better than nothing!
    }

    info!("{:?}", results);
    // Public keys should be the same for all the nodes
    for index in 0..(results.len() - 1) {
        assert_eq!(
            results[index].as_ref().unwrap(),
            results[index + 1].as_ref().unwrap(),
            "index: {}, index + 1: {}",
            index,
            index + 1,
        );
    }

    info!(
        "Finished DKG for {:?} nodes with key type {:?} with results : {:?}",
        next_peers.0.len(),
        curve_type,
        results
    );

    // return the pubkey value generated, or boolean as string for refresh.
    results[0]
        .as_ref()
        .expect("first result is not ok")
        .to_string()
}

pub async fn dkg_all_curves(
    vnc: &VirtualNodeCollection,
    epoch: u64,
    current_peers: &SimplePeerCollection,
) -> Vec<String> {
    // setup virtual nodes
    let next_peers = vnc.peers();
    let realm_id = 1;
    info!("Starting DKG on virtual nodes.");
    info!("DKG Included peers: {:?}", next_peers.debug_addresses());

    let mut v = Vec::new();
    let dkg_id = "TEST_DKG_1_1.";
    // start keygen on all nodes
    let threshold = next_peers.threshold_for_set_testing_only();
    for node in vnc.nodes.iter() {
        // this is a representation of what happens - but is not exhaustive

        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let mut dkg_engine = DkgEngine::new(
            node.tss_state.clone(),
            DkgType::Standard,
            epoch,
            threshold,
            (epoch, realm_id),
            current_peers,
            &next_peers,
            DkgAfterRestore::False,
        );
        for curve_type in CurveType::into_iter() {
            for i in 0..2 {
                let dkg_id = format!("{}{}_key_{}", dkg_id, curve_type, i);
                dkg_engine.add_dkg(&dkg_id, curve_type, None);
            }
        }

        let jh: JoinHandle<Vec<String>> = tokio::task::spawn(async move {
            let r = dkg_engine.execute(dkg_id, realm_id).await;
            info!("change epoch result: {:?}", r);
            let _ = r.expect("error from dkg manager change epoch");
            let root_keys = dkg_engine.get_dkgs().collect::<Vec<_>>();
            assert_eq!(root_keys.len(), 20);
            root_keys
                .iter()
                .map(|r| r.result().unwrap().public_key())
                .collect::<Vec<_>>()
        });
        v.push(jh);
    }

    // wait for all nodes to complete
    let results = join_all(v).await;
    for result in &results {
        assert!(result.is_ok()); // weak test, but better than nothing!
    }

    info!("{:?}", results);
    // Public keys should be the same for all the nodes
    for index in 0..(results.len() - 1) {
        assert_eq!(
            results[index].as_ref().unwrap(),
            results[index + 1].as_ref().unwrap(),
            "index: {}, index + 1: {}",
            index,
            index + 1,
        );
    }

    // return the pubkey value generated, or boolean as string for refresh.
    results[0].as_ref().expect("result is not ok").clone()
}
