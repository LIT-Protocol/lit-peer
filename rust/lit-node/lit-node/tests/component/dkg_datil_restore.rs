use super::utils::virtual_node_collection::VirtualNodeCollection;
use crate::common::interpolation::CurveScalar;
use crate::common::interpolation::interpolate_secret_from_shares;
use crate::common::interpolation::interpolate_secret;
use async_std::path::PathBuf;
use elliptic_curve::{Group, group::GroupEncoding};
use ethers::types::{H160, U256};
use lit_blockchain::contracts::backup_recovery::RecoveredPeerId;
use lit_node::common::key_helper::{KeyCache, KeyCacheType};
use lit_node::common::storage::do_read_from_disk;
use lit_node::tasks::fsm::epoch_change::ShadowOptions;
use lit_node::tss::common::dkg_type::DkgType;
use lit_node::tss::common::key_share::KeyShare;
use lit_node::tss::common::storage::write_key_share_to_cache_only;
use lit_node::tss::dkg::engine::{DkgAfterRestore, DkgAfterRestoreData, DkgEngine};
use lit_node::tss::util::DEFAULT_KEY_SET_NAME;
use lit_node_core::{CompressedBytes, CompressedHex};
use lit_node_core::CurveType;
use lit_node_core::PeerId;
use serde::{Deserialize, Serialize};
use test_case::test_case;
use tracing::info;
use vsss_rs::{DefaultShare, IdentifierPrimeField};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatilKeyShare {
    pub hex_private_share: String,
    pub hex_public_key: String,
    pub curve_type: u8,
    pub index: u16,
    pub threshold: u16,
    pub total_shares: u16,
    pub txn_prefix: String,
}

#[test_case(k256::ProjectivePoint::default(), CurveType::K256, 3; "K256 restore 3 nodes")]
// #[test_case(blsful::inner_types::G1Projective::default(), CurveType::BLS,  3;  "BLS restore 5 nodes")]
#[tokio::test]
pub async fn datil_dkg_after_restore<G>(_g: G, curve_type: CurveType, num_nodes: usize)
where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: From<PeerId> + CompressedBytes,
{
    crate::common::setup_logging();
    let vnc = VirtualNodeCollection::new(num_nodes).await;
    let staker_addresses = vnc.staker_addresses();
    let current_peers = vnc.peers();
    let realm_id = 1;

    let peers_for_keyshare = current_peers
        .0
        .iter()
        .map(|p| p.peer_id)
        .collect::<Vec<_>>();
    // let initial_secrets = vec!["0303755e79a7f05df251fe322e50ea96dc191e8352d3d3bb31c6aef44c5a6558b3".to_string()];
    let root_keys =
        vec!["03f1d2820b40e00b7206a5d1c15bf52f4bcac39119e6964cfe8c4e73c35e857f51".to_string()];
    let epoch = 1;
    let mut recovered_peer_ids = vec![];
    let recovery_key_cache = KeyCache::default();
    let mut initial_shares = vec![];
    for (i, new_node) in vnc.nodes.iter().enumerate() {
        let share_index = i + 1;
        let empty_cache = KeyCache::default();
        let path = format!("./tests/test_data/datil_restore_component/Key-H-2-03f1d2820b40e00b7206a5d1c15bf52f4bcac39119e6964cfe8c4e73c35e857f51-{}-H-2.cbor", share_index).to_string();
        let path = PathBuf::from(path);
        let datil_key_share =
            do_read_from_disk::<DatilKeyShare>(&path, &empty_cache, KeyCacheType::Protected)
                .await
                .expect("failed to read key share from disk");

        let identity_id = datil_key_share.index + 1;
        let share = DefaultShare {
            identifier: IdentifierPrimeField(k256::Scalar::from(identity_id as u64)),
            value: IdentifierPrimeField(k256::Scalar::from_uncompressed_hex(&datil_key_share.hex_private_share.clone()).unwrap()),
        };
        initial_shares.push(share);

        info!("Datil key share: {:?}", datil_key_share);
        let naga_key_share = KeyShare {
            hex_private_share: datil_key_share.hex_private_share.clone(),
            hex_public_key: datil_key_share.hex_public_key.clone(),
            curve_type: CurveType::try_from(datil_key_share.curve_type).unwrap(),
            threshold: datil_key_share.threshold as usize,
            total_shares: datil_key_share.total_shares as usize,
            peer_id: new_node.peer.peer_id,
            txn_prefix: datil_key_share.txn_prefix.clone(),
            realm_id: realm_id,
            peers: peers_for_keyshare.clone(),
        };
        // info!("Datil key share: {:?}", key_share);
        write_key_share_to_cache_only(
            curve_type,
            &datil_key_share.hex_public_key,
            &new_node.peer.peer_id,
            &new_node.hex_staker_address,
            epoch,
            realm_id,
            &recovery_key_cache,
            &naga_key_share,
        )
        .await
        .expect("write key share to cache failed");

        recovered_peer_ids.push(RecoveredPeerId {
            node_address: staker_addresses[i],
            old_peer_id: U256::from(identity_id),
            new_peer_id: U256::from(new_node.peer.peer_id),
        });
    }

    let dkg_id = "TEST_DKG_1_2.";
    let mut join_set = tokio::task::JoinSet::new();

    let next_peers = vnc.peers();
    let threshold = 3;
    for node in vnc.nodes.iter() {
        // assume this wait is because the join set starts executing immediately on creation
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let shadow_key_opts = ShadowOptions::new(false, 2, realm_id, 2, realm_id);
        let mut dkg_engine = DkgEngine::new(
            node.tss_state.clone(),
            DkgType::Standard,
            epoch,
            threshold,
            &shadow_key_opts,
            &current_peers,
            &next_peers,
            DkgAfterRestore::True(DkgAfterRestoreData {
                peers: recovered_peer_ids.clone(),
                key_cache: recovery_key_cache.clone(),
                is_datil_restore: true,
            }),
        );
        for (i, pubkey) in root_keys.iter().enumerate() {
            let dkg_id = format!("{}{}_key_{}", dkg_id, curve_type, i + 1);
            dkg_engine.add_dkg(
                &dkg_id,
                DEFAULT_KEY_SET_NAME,
                curve_type,
                Some(pubkey.clone()),
            );
        }
        join_set.spawn(async move {
            let r = dkg_engine.execute(dkg_id, realm_id).await;
            info!("change epoch result: {:?}", r);
            let _ = r.expect("error from dkg manager change epoch");
            // let root_keys = dkg_engine.get_dkgs().collect::<Vec<_>>();
            // assert_eq!(root_keys.len(), 1);
            // root_keys
            //     .iter()
            //     .map(|r| r.result().unwrap().public_key())
            //     .collect::<Vec<_>>()
        });
    }

    while let Some(node_info) = join_set.join_next().await {
        let _ = node_info.expect("error from dkg engine");
    }

    let epoch = epoch + 1;
    for (_i, pubkey) in root_keys.iter().enumerate() {
        let naga_secret = interpolate_secret(curve_type, &next_peers, pubkey, epoch, realm_id).await;
        let datil_secret = interpolate_secret_from_shares::<k256::ProjectivePoint>(threshold, &initial_shares);
        info!("Naga Secret: {:?}", naga_secret);
        info!("Datil Secret: {:?}", datil_secret);
        assert_eq!(naga_secret, CurveScalar::K256(datil_secret), "secrets do not match after restore");
    }
}


#[ignore]
#[test]
fn test_interpolate_secrets() {
    
    let threshold = 3;
    let share1 = k256::Scalar::from_uncompressed_hex("8c803a0d76e20437e4b5ba37cc896cf3dceca20ce8e426c8b6ea98e45d6a7d9b").unwrap();
    let share2 = k256::Scalar::from_uncompressed_hex("49ebd62ce07f46a63e05ce91fb3edd463b1ba579a83cfa88d89e2ab8ee2f7861").unwrap();
    let share3 = k256::Scalar::from_uncompressed_hex("8182c7a788c704dc544ccbbdc18451ee3b8430ca394c0f7ea1090a1b98429546").unwrap();
    let shares = vec![
        DefaultShare {
            identifier: IdentifierPrimeField(k256::Scalar::from(2 as u64)),
            value: IdentifierPrimeField(share1),
        },
        DefaultShare {
            identifier: IdentifierPrimeField(k256::Scalar::from(3 as u64)),
            value: IdentifierPrimeField(share2),
        },
        DefaultShare {
            identifier: IdentifierPrimeField(k256::Scalar::from(4 as u64)),
            value: IdentifierPrimeField(share3),
        },
    ];
    let secret = interpolate_secret_from_shares::<k256::ProjectivePoint>(threshold, &shares);
    println!("Secret: {:?}", secret);
    // 0x802B01E05FA6F2B268FA4BF835CE995291DD791180C40271E804A0CBE75E6BEA
}