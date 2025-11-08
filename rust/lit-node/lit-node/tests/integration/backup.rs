use crate::common::peers::get_simple_peer_collection;
use elliptic_curve::Group;
use elliptic_curve::group::GroupEncoding;
use ethers::abi::Address;
use lit_core::utils::binary::bytes_to_hex;
use lit_node::common::key_helper::KeyCache;
use lit_node::peers::peer_state::models::SimplePeerCollection;
use lit_node::tss::common::key_persistence::RECOVERY_DKG_EPOCH;
use lit_node::tss::common::key_share::KeyShare;
use lit_node::tss::common::key_share_commitment::KeyShareCommitments;
use lit_node::tss::common::storage::{
    StorableFile, StorageType, read_key_share_commitments_from_disk, read_key_share_from_disk,
};
use lit_node_core::ethers::prelude::U256;
use lit_node_core::{CompressedBytes, CurveType};
use lit_node_testnet::TestSetupBuilder;
use tracing::info;

/// Tests that decryption shares do not get deleted
/// across epochs and only when the recovery parties download them
#[tokio::test]
async fn verify_restore_decryption_shares_not_deleted() {
    let realm_id = U256::from(1);
    crate::common::setup_logging();

    let (_testnet, validator_collection, _end_user) = TestSetupBuilder::default().build().await;

    let recovery_party_addresses = (0..validator_collection.validators().len())
        .map(|_i| Address::random())
        .collect::<Vec<_>>();

    let tx = validator_collection
        .actions()
        .contracts()
        .backup_recovery
        .register_new_backup_party(recovery_party_addresses);
    let res = tx.send().await.unwrap();
    info!("Registered recovery parties: {:?}", res);

    validator_collection
        .actions()
        .increase_blockchain_timestamp(300)
        .await;

    validator_collection
        .actions()
        .wait_for_epoch(
            realm_id,
            validator_collection
                .actions()
                .get_current_epoch(realm_id)
                .await
                + 1,
        )
        .await;

    // Make sure the recovery DKG has occurred
    validator_collection
        .actions()
        .wait_for_recovery_keys()
        .await;

    validator_collection
        .actions()
        .increase_blockchain_timestamp(300)
        .await;

    validator_collection
        .actions()
        .wait_for_epoch(
            realm_id,
            validator_collection
                .actions()
                .get_current_epoch(realm_id)
                .await
                + 1,
        )
        .await;

    let peers = get_simple_peer_collection(&validator_collection, realm_id)
        .await
        .unwrap();

    let backup_party_state = validator_collection
        .actions()
        .contracts()
        .backup_recovery
        .get_backup_party_state()
        .await
        .unwrap();

    for recovery_key in &backup_party_state.registered_recovery_keys {
        let curve_type = CurveType::try_from(recovery_key.key_type).unwrap();
        let pubkey = hex::encode(&recovery_key.pubkey);
        match curve_type {
            CurveType::BLS | CurveType::BLS12381G1 => {
                check_for_restore_decryption_shares::<blsful::inner_types::G1Projective>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::K256 => {
                check_for_restore_decryption_shares::<k256::ProjectivePoint>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::P256 => {
                check_for_restore_decryption_shares::<p256::ProjectivePoint>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::P384 => {
                check_for_restore_decryption_shares::<p384::ProjectivePoint>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::Ed25519 => {
                check_for_restore_decryption_shares::<vsss_rs::curve25519::WrappedEdwards>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::Ristretto25519 => {
                check_for_restore_decryption_shares::<vsss_rs::curve25519::WrappedRistretto>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::Ed448 => {
                check_for_restore_decryption_shares::<ed448_goldilocks::EdwardsPoint>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::RedJubjub => {
                check_for_restore_decryption_shares::<jubjub::SubgroupPoint>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::RedDecaf377 => {
                check_for_restore_decryption_shares::<decaf377::Element>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
        }
    }
}

async fn check_for_restore_decryption_shares<G>(
    curve_type: CurveType,
    pub_key: &str,
    peers: &SimplePeerCollection,
    realm_id: u64,
) where
    G: Group + GroupEncoding + Default + CompressedBytes,
    G::Scalar: CompressedBytes,
{
    let cache = KeyCache::default();

    for peer in &peers.0 {
        let staker_address = format!("0x{}", bytes_to_hex(peer.staker_address.0));
        let res = read_key_share_from_disk::<KeyShare>(
            curve_type,
            pub_key,
            &staker_address,
            &peer.peer_id,
            RECOVERY_DKG_EPOCH,
            realm_id,
            &cache,
        )
        .await
        .is_ok();
        let storable_file = StorableFile {
            storage_type: StorageType::KeyShare(curve_type),
            pubkey: pub_key.to_string(),
            peer_id: peer.peer_id,
            epoch: RECOVERY_DKG_EPOCH,
            realm_id,
        };
        assert!(
            res,
            "decryption share is missing {}",
            storable_file
                .get_full_path(&staker_address)
                .await
                .unwrap()
                .display()
        );
        let res = read_key_share_commitments_from_disk::<KeyShareCommitments<G>>(
            curve_type,
            pub_key,
            &staker_address,
            &peer.peer_id,
            RECOVERY_DKG_EPOCH,
            realm_id,
            &cache,
        )
        .await
        .is_ok();
        let storable_file = StorableFile {
            storage_type: StorageType::KeyShareCommitment(curve_type),
            pubkey: pub_key.to_string(),
            peer_id: peer.peer_id,
            epoch: RECOVERY_DKG_EPOCH,
            realm_id,
        };
        assert!(
            res,
            "Decryption share commitments are missing {}",
            storable_file
                .get_full_path(&staker_address)
                .await
                .unwrap()
                .display()
        );
    }
}
