use crate::common::ecdsa::simple_single_sign_with_hd_key;
use crate::common::recovery_party::SiweSignature;
use chrono::{Duration, Utc};
use ethers::prelude::{H160, LocalWallet, Signer, U256};
use ethers::types::Address;
use lit_blockchain::contracts::backup_recovery::BackupRecoveryState;
use lit_core::config::CFG_ADMIN_OVERRIDE_NAME;
use lit_core::utils::binary::bytes_to_hex;
use lit_node::common::key_helper::KeyCache;
use lit_node::endpoints::auth_sig::LITNODE_ADMIN_RES;
use lit_node::peers::peer_state::models::{NetworkState, SimplePeer, SimplePeerCollection};
use lit_node::tss::common::key_persistence::RECOVERY_DKG_EPOCH;
use lit_node::tss::common::key_share::KeyShare;
use lit_node::tss::common::key_share_commitment::KeyShareCommitments;
use lit_node::tss::common::restore::NodeRecoveryStatus;
use lit_node::tss::common::storage::{
    StorableFile, StorageType, read_key_share_commitments_from_disk, read_key_share_from_disk,
};
use lit_node_core::{Blinders, CompressedBytes, CurveType, JsonAuthSig, PeerId, SigningScheme};
use lit_node_testnet::TestSetupBuilder;
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;
use lit_rust_crypto::{
    blsful, decaf377, ed448_goldilocks,
    group::{Group, GroupEncoding},
    jubjub,
    k256::{
        self,
        ecdsa::{SigningKey, VerifyingKey},
    },
    p256, p384, pallas, vsss_rs,
};
use reqwest::Client;
use semver::Version;
use sha3::{Keccak256, digest::Digest};
use std::collections::HashMap;
use std::path::PathBuf;
use test_case::test_case;
use tokio::task::JoinSet;
use tracing::info;

const TARBALL_NAME: &str = "lit_backup_encrypted_keys.tar.gz";

// How the numbers are set is very important to test different thresholds.
// In this test, 5 nodes run a recovery DKG for 4 recovery party members.
// 3 of these 4 members help 3 of these 5 nodes to restore the network.

#[test_case(5, 3, 4, 3; "3 out of 5 nodes, 3 out of 4 members")]
#[tokio::test]
async fn end_to_end_backup_with_recovery_tool_test(
    number_of_nodes_before: usize,
    number_of_nodes_after: usize,
    recovery_party_size: usize,
    recovery_party_active_members: usize,
) {
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024) // 32MB stack
        .spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(end_to_end_test(
                    number_of_nodes_before,
                    number_of_nodes_after,
                    recovery_party_size,
                    recovery_party_active_members,
                ));
        })
        .unwrap()
        .join()
        .unwrap();
}

async fn end_to_end_test(
    number_of_nodes_before: usize,
    number_of_nodes_after: usize,
    recovery_party_size: usize,
    recovery_party_active_members: usize,
) {
    let realm_id = U256::from(1);
    let admin_signing_key = create_node_operator_admin_signing_key().await;

    crate::common::setup_logging();
    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(number_of_nodes_before)
        .build()
        .await;

    let pubkey = end_user.first_pkp().pubkey.clone();

    let backup_directory = create_recovery_directory();

    let lrts = create_recovery_parties(
        recovery_party_size,
        &testnet,
        &validator_collection,
        &backup_directory,
    )
    .await;
    let recovery_party_addresses = lrts
        .iter()
        .map(|lrt| lrt.wallet_address)
        .collect::<Vec<_>>();

    validator_collection
        .actions()
        .wait_for_epoch(realm_id, U256::from(2))
        .await;

    let tx = validator_collection
        .actions()
        .contracts()
        .backup_recovery
        .register_new_backup_party(recovery_party_addresses.clone());
    let res = tx.send().await.unwrap();
    info!("Registered recovery parties: {:?}", res);

    // Fast-forward the network by 300 seconds, and wait for the new node to be active - effectively waiting for the next epoch.
    validator_collection
        .actions()
        .increase_blockchain_timestamp(300)
        .await;

    // Wait for DKG to start and then finish, by effectively waiting for the epoch change - nodes become active once more.
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

    sign_with_all_curves(&validator_collection, &end_user, pubkey.clone(), true).await;

    download_decryption_key_shares_to_local_lit_recovery_tools(
        &testnet,
        &mut validator_collection,
        &lrts,
    )
    .await;

    // Use the admin endpoint to get the backup and blinders
    let client = reqwest::ClientBuilder::new()
        .tls_sni(false)
        .build()
        .unwrap();

    let downloaded_blinders = node_operator_perform_backup(
        &admin_signing_key,
        &testnet,
        &validator_collection,
        &backup_directory,
        number_of_nodes_after,
    )
    .await;
    info!("Downloaded blinders : {:?}", downloaded_blinders);
    // stop old nodes but leave the test net up. Setting the network to restore state
    // should stop all the nodes
    info!("Setting network state to Restore");
    validator_collection
        .actions()
        .set_epoch_state(realm_id, NetworkState::Restore as u8)
        .await
        .unwrap();

    info!(
        "Making sure that {} nodes are offline",
        number_of_nodes_before
    );
    for i in 0..number_of_nodes_before {
        let validator = validator_collection.get_validator_by_idx_mut(i);
        assert!(validator.is_node_offline());
    }

    // Since we're using the exact same contract state as before the nodes got shut down, we need to
    // allow the nodes to register their attested wallets on their next boot.
    let actions = validator_collection.actions();
    let current_validators = actions.get_current_validators(realm_id).await;
    actions
        .admin_set_register_attested_wallet_disabled_for_validators(current_validators, false)
        .await
        .expect("Failed to set register attested wallet disabled for validators");

    // nodes start in restore mode and reuse the same testnet
    info!("Restarting the nodes");
    let validator_collection2 = ValidatorCollection::builder()
        .num_staked_nodes(number_of_nodes_after)
        .pause_network_while_building(false)
        .build(&testnet)
        .await
        .expect("Failed to build validator collection");

    // give time for the nodes to start up and enter the "restore" state.
    validator_collection2.actions().sleep_millis(2000).await;

    info!("Depositing to the nodes");
    // deposit_to_wallet(validator_collection2.actions()).await;
    let backup_state: BackupRecoveryState = validator_collection2
        .actions()
        .contracts()
        .backup_recovery
        .get_backup_party_state()
        .await
        .unwrap();

    info!("Recovery party state: {:?}", backup_state);
    let session_id = backup_state.session_id.to_string();

    // Blinders need to be in-place before the key backups are uploaded
    info!("Uploading blinders to nodes");
    upload_blinders_to_nodes(
        &admin_signing_key,
        &testnet,
        &downloaded_blinders,
        &validator_collection2,
    )
    .await;

    // Key backups need to be in-place before the decryption shares are uploaded
    info!("Uploading backups to nodes");
    upload_key_backups_to_nodes(
        &admin_signing_key,
        &testnet,
        &client,
        &validator_collection2,
        &backup_directory,
    )
    .await;

    let recover_command = lit_recovery::args::Commands::Recover {
        directory: backup_directory.clone(),
        session_id: session_id.clone(),
    };
    for lrt in &lrts[..recovery_party_active_members] {
        info!(
            "Wallet {} uploading decryption shares",
            hex::encode(lrt.wallet_address.as_bytes())
        );
        lrt.child.command(recover_command.clone()).await.unwrap();
    }
    info!("Decryption shares uploaded");

    // Wait until all keys are restored
    validator_collection
        .actions()
        .wait_for_recovery_status(NodeRecoveryStatus::AllKeysAreRestored as u8)
        .await;
    info!("All the nodes restored all the keys!");

    // Signing will fail until a reshare DKG occurs
    // which remaps the old peer ids to the new peer ids
    info!("Set NetworkState to NextValidatorSetLocked");
    validator_collection2
        .actions()
        .set_epoch_state(realm_id, NetworkState::NextValidatorSetLocked as u8)
        .await
        .unwrap();

    // Advance the epoch one to ensure the DKG can be performed successfully then sign again
    // Fast-forward the network by 300 seconds, and wait for the new node to be active
    // - effectively waiting for the next epoch.
    validator_collection2
        .actions()
        .increase_blockchain_timestamp(300)
        .await;

    // Wait for DKG to start and then finish, by effectively waiting for the epoch change
    // - nodes become active once more.
    validator_collection2
        .actions()
        .wait_for_epoch(
            realm_id,
            validator_collection2
                .actions()
                .get_current_epoch(realm_id)
                .await
                + 1,
        )
        .await;

    // The first DKG after restore was a special case.
    // Now test that the usual DKGs after that also work.

    // Advance the epoch one to ensure the DKG can be performed successfully then sign again
    // Fast-forward the network by 300 seconds, and wait for the new node to be active
    // - effectively waiting for the next epoch.
    validator_collection2
        .actions()
        .increase_blockchain_timestamp(300)
        .await;

    // Wait for DKG to start and then finish, by effectively waiting for the epoch change
    // - nodes become active once more.
    validator_collection2
        .actions()
        .wait_for_epoch(
            realm_id,
            validator_collection2
                .actions()
                .get_current_epoch(realm_id)
                .await
                + 1,
        )
        .await;

    sign_with_all_curves(&validator_collection2, &end_user, pubkey.clone(), true).await;
}

async fn upload_key_backups_to_nodes(
    admin_signing_key: &SigningKey,
    testnet: &Testnet,
    client: &Client,
    validator_collection: &ValidatorCollection,
    backup_directory: &PathBuf,
) {
    let validators = validator_collection.get_active_validators().await.unwrap();
    let mut join_set = JoinSet::new();
    // Download the backups and blinders
    for &validator in validators.iter() {
        let public_address = validator.public_address();
        let chain_id = testnet.chain_id;
        let client = client.clone();
        let admin_signing_key = admin_signing_key.clone();
        let backup_directory = backup_directory.clone();
        join_set.spawn(async move {
            let url = format!("http://{}", public_address.clone());
            let auth_sig =
                generate_admin_auth_sig(&admin_signing_key, chain_id, &url, &public_address);
            let json_body = serde_json::to_string(&auth_sig.auth_sig).unwrap();

            let tar_file = backup_directory.join(format!("{}{}", public_address, TARBALL_NAME));
            let file = tokio::fs::File::open(tar_file).await.unwrap();

            info!("Uploading backup for validator {}", public_address);
            let response = client
                .post(format!("{}/web/admin/set_key_backup", url))
                .header("Content-Type", "application/octet-stream")
                .header(
                    "x-auth-sig",
                    data_encoding::BASE64URL.encode(json_body.as_bytes()),
                )
                .body(file)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            debug!("Response: {}", response);
            let response: serde_json::Value = serde_json::from_str(&response).unwrap();
            let success = response.as_object().unwrap().get("success");
            let success = if let Some(success) = success {
                success.as_str().map(|s| s == "true").unwrap_or_default()
            } else {
                false
            };
            (public_address, success)
        });
    }
    while let Some(node_info) = join_set.join_next().await {
        let (public_address, success) = node_info.unwrap();
        info!("Node {} received tar backup: {}", public_address, success);
        assert!(success);
    }
}

async fn upload_blinders_to_nodes(
    admin_signing_key: &SigningKey,
    testnet: &Testnet,
    downloaded_blinders: &HashMap<String, Blinders>,
    validator_collection2: &ValidatorCollection,
) {
    let validators = validator_collection2.get_active_validators().await.unwrap();
    let mut join_set = JoinSet::new();

    for &validator in validators.iter() {
        let public_address = validator.public_address();
        let admin_signing_key = admin_signing_key.clone();
        let chain_id = testnet.chain_id;
        let blinders = downloaded_blinders[&public_address];

        join_set.spawn(async move {
            // Send the blinders to the node operators
            let url = format!("http://{}/web/admin/set_blinders", public_address);
            let auth_sig =
                generate_admin_auth_sig(&admin_signing_key, chain_id, &url, &public_address);

            info!(
                "{} Sending blinders: {}",
                public_address,
                serde_json::to_string_pretty(&blinders).unwrap()
            );

            let response = lit_sdk::admin::SetBlindersRequest::new()
                .url_prefix(lit_sdk::UrlPrefix::Http)
                .public_address(public_address.clone())
                .request(lit_sdk::admin::SetBlindersData { auth_sig, blinders })
                .build()
                .unwrap()
                .send()
                .await
                .unwrap();

            info!("Response: {:?}", response);
            public_address
        });
    }
    while let Some(node_info) = join_set.join_next().await {
        let public_address = node_info.unwrap();
        info!("Node {} received blinders", public_address);
    }
}

async fn node_operator_perform_backup(
    admin_signing_key: &SigningKey,
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
    backup_directory: &PathBuf,
    number_of_nodes: usize,
) -> HashMap<String, Blinders> {
    let validators = validator_collection.get_active_validators().await.unwrap();
    let mut join_set = JoinSet::new();
    let epoch = validator_collection
        .actions()
        .get_current_epoch(U256::from(1))
        .await
        .as_u64();
    // Download the backups and blinders
    for (i, &validator) in validators.iter().enumerate() {
        if i >= number_of_nodes {
            // Optimize the test by not downloading more than necessary
            break;
        }
        let public_address = validator.public_address();
        let chain_id = testnet.chain_id;
        let admin_signing_key = admin_signing_key.clone();
        let backup_directory = backup_directory.clone();
        join_set.spawn(async move {
            let url = format!("http://{}", public_address);
            let auth_sig =
                generate_admin_auth_sig(&admin_signing_key, chain_id, &url, &public_address);

            info!("Getting backup for validator {}", public_address);

            let blinders_response = lit_sdk::admin::GetBlindersRequest::new()
                .url_prefix(lit_sdk::UrlPrefix::Http)
                .public_address(public_address.clone())
                .request(auth_sig.clone())
                .build()
                .unwrap()
                .send()
                .await
                .unwrap();
            let blinders = blinders_response.results();

            debug!(
                "{} Downloaded Blinders: {}",
                public_address,
                serde_json::to_string_pretty(&blinders).unwrap()
            );

            info!("Downloading backup from '{}'. This may take awhile.", url);
            let node_tar_name = format!("{}{}", public_address, TARBALL_NAME);
            let file = async_std::fs::File::create(backup_directory.join(node_tar_name))
                .await
                .unwrap();
            let _response = lit_sdk::admin::GetKeyBackupRequest::new()
                .url_prefix(lit_sdk::UrlPrefix::Http)
                .public_address(public_address.clone())
                .request(lit_sdk::admin::GetKeyBackupParameters {
                    auth: auth_sig,
                    epoch,
                })
                .build()
                .unwrap()
                .download(file)
                .await
                .unwrap();

            (public_address, *blinders)
        });
    }
    let mut downloaded_blinders = HashMap::<String, Blinders>::with_capacity(validators.len());
    while let Some(node_info) = join_set.join_next().await {
        let (public_address, blinders) = node_info.unwrap();
        downloaded_blinders.insert(public_address, blinders);
    }

    downloaded_blinders
}

async fn download_decryption_key_shares_to_local_lit_recovery_tools(
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
    lrts: &[LitRecoveryTool],
) {
    let config_command = lit_recovery::args::Commands::SetConfig {
        address: format!(
            "0x{}",
            hex::encode(
                validator_collection
                    .actions()
                    .contracts()
                    .contract_resolver
                    .address()
                    .as_bytes()
            )
        ),
        rpc_url: format!("http://{}", testnet.rpcurl),
        chain_id: testnet.chain_id,
        env: 0,
    };
    for lrt in lrts {
        lrt.child.command(config_command.clone()).await.unwrap();
        lrt.child
            .command(lit_recovery::args::Commands::DownloadShare)
            .await
            .unwrap();
        lrt.child
            .command(lit_recovery::args::Commands::UploadPublicKey)
            .await
            .unwrap();
    }

    let realm_id = U256::from(1);
    let mut peers = SimplePeerCollection::default();
    let addresses = validator_collection
        .validators()
        .iter()
        .map(|v| v.account().staker_address)
        .collect();
    let mappings = validator_collection
        .actions()
        .get_node_attested_pubkey_mappings(&addresses)
        .await
        .unwrap();
    for i in 0..validator_collection.validator_count() {
        let validator = validator_collection.get_validator_by_idx(i);
        let attested_wallet = mappings[i].as_ref().unwrap();
        let mut wallet_public_key_bytes = vec![4u8; 65];
        attested_wallet
            .x
            .to_big_endian(&mut wallet_public_key_bytes[1..33]);
        attested_wallet
            .y
            .to_big_endian(&mut wallet_public_key_bytes[33..65]);

        peers.0.push(SimplePeer {
            socket_address: validator.public_address(),
            peer_id: PeerId::from_slice(wallet_public_key_bytes.as_slice()).unwrap(),
            staker_address: validator.account().staker_address,
            key_hash: 0,
            kicked: false,
            version: Version::new(1, 0, 0),
            realm_id,
        });
    }

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
                check_for_lingering_keys::<blsful::inner_types::G1Projective>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::K256 => {
                check_for_lingering_keys::<k256::ProjectivePoint>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::P256 => {
                check_for_lingering_keys::<p256::ProjectivePoint>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::P384 => {
                check_for_lingering_keys::<p384::ProjectivePoint>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::Ed25519 => {
                check_for_lingering_keys::<vsss_rs::curve25519::WrappedEdwards>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::Ristretto25519 => {
                check_for_lingering_keys::<vsss_rs::curve25519::WrappedRistretto>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::Ed448 => {
                check_for_lingering_keys::<ed448_goldilocks::EdwardsPoint>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::RedJubjub => {
                check_for_lingering_keys::<jubjub::SubgroupPoint>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::RedDecaf377 => {
                check_for_lingering_keys::<decaf377::Element>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await;
            }
            CurveType::RedPallas => {
                check_for_lingering_keys::<pallas::Point>(
                    curve_type,
                    &pubkey,
                    &peers,
                    realm_id.as_u64(),
                )
                .await
            }
        }
    }
}

async fn check_for_lingering_keys<G>(
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
        .is_err();
        let storable_file = StorableFile {
            storage_type: StorageType::KeyShare(curve_type),
            pubkey: pub_key.to_string(),
            peer_id: peer.peer_id,
            epoch: RECOVERY_DKG_EPOCH,
            realm_id,
        };
        assert!(
            res,
            "Lingering decryption share that should've been deleted {}",
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
        .is_err();
        let storable_file = StorableFile {
            storage_type: StorageType::KeyShareCommitment(curve_type),
            pubkey: pub_key.to_string(),
            peer_id: peer.peer_id,
            epoch: RECOVERY_DKG_EPOCH,
            realm_id,
        };
        assert!(
            res,
            "Lingering decryption share commitments that should've been deleted {}",
            storable_file
                .get_full_path(&staker_address)
                .await
                .unwrap()
                .display()
        );
    }
}

fn create_recovery_directory() -> PathBuf {
    let mut backup_directory = std::env::current_dir().unwrap();
    backup_directory.push("recovery_state");

    if backup_directory.exists() {
        std::fs::remove_dir_all(&backup_directory).unwrap();
    }

    if !backup_directory.exists() {
        std::fs::create_dir(&backup_directory).unwrap();
    }
    backup_directory
}

async fn create_node_operator_admin_signing_key() -> SigningKey {
    let admin_signing_key = SigningKey::random(&mut rand::rngs::OsRng);
    let admin_address = admin_signing_key.to_eth_address_str();

    tokio::fs::write(
        format!("./{}.toml", CFG_ADMIN_OVERRIDE_NAME),
        format!(
            r#"[node]
admin_address = "{}"
    "#,
            admin_address
        ),
    )
    .await
    .unwrap();

    info!(
        "Starting backup recovery test with admin_address = {}",
        admin_address
    );
    admin_signing_key
}

async fn create_recovery_parties(
    num_nodes: usize,
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
    backup_directory: &PathBuf,
) -> Vec<LitRecoveryTool> {
    info!("Creating recovery parties");
    let mut lrts = Vec::with_capacity(5);
    for i in 0..num_nodes {
        let share_db_name = format!("sdb{}.db3", i);
        let file = format!("recovery_{}", i);
        let keyring_file = backup_directory.join(file);
        let share_db_path = backup_directory.join(share_db_name);
        let lrt = start_lit_recovery_tool(keyring_file, share_db_path).await;

        let lrt_wallet = LocalWallet::from_bytes(&lrt.child.get_secret_for_wallet().await.unwrap())
            .unwrap()
            .with_chain_id(testnet.chain_id);

        validator_collection
            .actions()
            .fund_wallet(&lrt_wallet, "100000000000000000000")
            .await;
        info!(
            "Lit Recovery Tool wallet_address: {}, wallet_key = {}",
            hex::encode(lrt.wallet_address.as_bytes()),
            lrt.wallet_key
        );
        lrts.push(lrt);
    }
    lrts
}

async fn sign_with_all_curves(
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
    pubkey: String,
    expected_result: bool,
) {
    info!("Signing with all curves");
    for scheme in [
        SigningScheme::Bls12381G1ProofOfPossession,
        SigningScheme::SchnorrEd25519Sha512,
        SigningScheme::SchnorrK256Sha256,
        SigningScheme::SchnorrP256Sha256,
        SigningScheme::SchnorrP384Sha384,
        SigningScheme::SchnorrRistretto25519Sha512,
        SigningScheme::SchnorrEd448Shake256,
        SigningScheme::SchnorrRedJubjubBlake2b512,
        SigningScheme::SchnorrK256Taproot,
        SigningScheme::SchnorrRedDecaf377Blake2b512,
        SigningScheme::SchnorrkelSubstrate,
        SigningScheme::EcdsaK256Sha256,
        SigningScheme::EcdsaP256Sha256,
        SigningScheme::EcdsaP384Sha384,
    ] {
        assert_eq!(
            simple_single_sign_with_hd_key(
                &validator_collection,
                &end_user,
                pubkey.clone(),
                scheme,
                &vec![]
            )
            .await,
            expected_result,
            "Failed to sign before recovery."
        );
    }
}

struct LitRecoveryTool {
    pub child: lit_recovery::LitRecovery,
    pub wallet_key: String,
    pub wallet_address: Address,
}

async fn start_lit_recovery_tool(backup_directory: PathBuf, share_db: PathBuf) -> LitRecoveryTool {
    let recovery = lit_recovery::LitRecovery::new(
        Some(backup_directory),
        Some("a".to_string()),
        Some(share_db),
        Some(PathBuf::from(".")),
    )
    .await
    .unwrap();

    let verifying_key = recovery.info().await.verifying_key;
    let wallet_key = hex::encode(verifying_key.to_encoded_point(false).as_bytes());
    let wallet_address = verifying_key.to_eth_address();

    LitRecoveryTool {
        child: recovery,
        wallet_key,
        wallet_address,
    }
}

fn generate_admin_auth_sig(
    signing_key: &SigningKey,
    chain_id: u64,
    uri: &str,
    domain: &str,
) -> lit_node_core::AdminAuthSig {
    let address = signing_key.to_eth_address_str();
    let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.fZ").to_string();
    let expiration = (Utc::now() + Duration::days(1)) // Sets 1 day expiration
        .format("%Y-%m-%dT%H:%M:%S%.fZ")
        .to_string();
    let siwe_message = siwe::Message {
        domain: domain.parse().unwrap(),
        address: signing_key.to_eth_address().0,
        statement: None,
        uri: uri.parse().unwrap(),
        version: siwe::Version::V1,
        chain_id,
        nonce: "AAAAAAAAAAAAAAAAA".into(),
        issued_at: now.parse().unwrap(),
        expiration_time: Some(expiration.parse().unwrap()),
        not_before: None,
        request_id: None,
        resources: vec![LITNODE_ADMIN_RES.parse().unwrap()],
    };
    let signed_message = siwe_message.to_string();

    let (signature, recovery_id) = signing_key.sign_siwe(signed_message.as_bytes());
    let mut buffer = [0u8; 65];
    buffer[..64].copy_from_slice(&signature.to_bytes());
    buffer[64] = recovery_id.to_byte();
    lit_node_core::AdminAuthSig {
        auth_sig: JsonAuthSig::new(
            hex::encode(&buffer),
            "web3.eth.personal.sign".to_string(),
            signed_message,
            address,
            None,
        ),
    }
}

trait EthereumAddress {
    fn to_eth_address_str(&self) -> String {
        let address = fmt_address(&self.to_eth_address().0);
        let mut buffer = String::new();
        buffer.push('0');
        buffer.push('x');
        buffer.push_str(&String::from_utf8(address.to_vec()).unwrap());
        buffer
    }

    fn to_eth_address(&self) -> Address;
}

impl EthereumAddress for VerifyingKey {
    fn to_eth_address(&self) -> Address {
        let pub_key_pt = self.to_encoded_point(false);
        let digest = keccak256(&pub_key_pt.as_bytes()[1..]);
        let last_20 = <[u8; 20]>::try_from(&digest[12..]).unwrap();
        H160::from_slice(&last_20)
    }
}

impl EthereumAddress for SigningKey {
    fn to_eth_address(&self) -> Address {
        let public_key = self.verifying_key();
        public_key.to_eth_address()
    }
}

fn fmt_address(bytes: &[u8; 20]) -> [u8; 40] {
    let mut buffer = [0u8; 40];
    hex::encode_to_slice(bytes, &mut buffer).unwrap();

    let checksum = keccak256(&buffer);

    for i in 0..buffer.len() {
        let byte = checksum[i / 2];
        let nibble = 0xf & if i & 1 == 0 { byte >> 4 } else { byte };
        if nibble >= 8 {
            buffer[i] = buffer[i].to_ascii_uppercase();
        }
    }
    buffer
}

fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::default();
    hasher.update(bytes);
    hasher.finalize().into()
}
