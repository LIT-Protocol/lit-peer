use lit_node_testnet::{
    TestSetupBuilder,
    node_collection::get_node_versions,
    testnet::{
        NodeAccount, Testnet,
        contracts_repo::{
            self, WalletManifestItem, alias_node_configs_path, get_alias_manifest_template,
            latest_wallet_manifest, save_alias_manifest,
        },
    },
};

use crate::common::{
    assertions::NetworkIntegrityChecker,
    version::{get_crate_version, update_node_crate_version},
};

use ethers::types::{H160, U256};
use lit_blockchain::{
    contracts::staking::ComplaintConfig,
    resolver::rpc::{ENDPOINT_MANAGER, RpcHealthcheckPoller},
};
use lit_core::utils::binary::bytes_to_hex;
use lit_node::{peers::peer_reviewer::Issue, utils::consensus::get_threshold_count};
use rand::seq::SliceRandom;
use std::{fs, time::Duration};
use test_case::test_case;
use tracing::info;

/// Tests when an inactive validator that comes online with an invalid version, and then the staker requests to join,
/// that the node should eventually be kicked for non-participation.
#[tokio::test]
async fn node_boot_invalid_version() {
    crate::common::setup_logging();
    info!("TEST: node_boot_invalid_version");
    // Set up a network with 6 nodes.
    let num_nodes = 6;
    // set epoch length to 30 mins so it never elapses unless we advance the clock
    let epoch_length = 1800;

    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(num_nodes)
        .build()
        .await;

    let actions = testnet.actions();
    let network_checker = NetworkIntegrityChecker::new(&end_user, &actions).await;

    // Upgrade the node crate to a new version
    let _crate_version_handle = update_node_crate_version("2.9999.9999".to_string());

    let realm_id = U256::from(1);
    // Update version requirements by setting a max version requirement, rendering the new node version invalid.
    let max_version = "2.9999.9998";
    actions
        .set_staking_max_version(realm_id, max_version)
        .await
        .expect("Failed to set max version");

    // Lower the configured threshold for non-participation complaints.
    info!("Lowering the configured threshold for non-participation complaints");
    actions
        .set_complaint_reason_config(
            U256::from(Issue::NonParticipation.value()),
            ComplaintConfig {
                tolerance: U256::from(2),
                interval_secs: U256::from(120),
                kick_penalty_percent: ethers::utils::parse_ether("0.1").unwrap(), // 0.1 ether = 10%
                kick_penalty_demerits: U256::from(10),
            },
        )
        .await
        .expect("Failed to set complaint config");

    // Spin up a new node with the new node version
    info!("Spinning up a new node with the new node version");
    let validator_to_kick = validator_collection
        .add_one(
            false,
            Some(lit_node_testnet::validator::BuildMode::UseNewOrCachedBuild),
            None,
        )
        .await
        .expect("Failed to add new node");
    let staker_address_to_kick = validator_to_kick.account().staker_address;

    // Fast forward time to allow the network to attempt to deal in the new node with the new node version
    // before voting to kick it out due to non-participation.
    info!(
        "Fast forwarding time to allow the network to attempt to deal in the new node with the new node version"
    );
    actions.increase_blockchain_timestamp(epoch_length).await;

    let epoch_number = actions.get_current_epoch(realm_id).await;

    // Wait for kick
    let voting_status = actions
        .wait_for_voting_status_to_kick_validator(
            realm_id,
            epoch_number,
            staker_address_to_kick,
            H160::random(), // For simplicity, we only care about asserting the number of votes.
            get_threshold_count(num_nodes),
            true,
        )
        .await;
    assert!(voting_status.is_ok());

    // Wait for new epoch
    info!("Waiting for epoch 3");
    actions.wait_for_epoch(realm_id, U256::from(3)).await;

    // Run network checks
    info!("Checking network state");
    assert_eq!(
        actions.get_current_validator_count(realm_id).await as usize,
        num_nodes
    );
    network_checker.check(&validator_collection, &vec![]).await;
}

/// Tests the version requirement change such that an active validator is running a node version that is incompatible,
/// so it should request to leave.
#[tokio::test]
async fn active_validator_invalid_version() {
    crate::common::setup_logging();
    info!("TEST: active_validator_invalid_version");
    // Set up a network with 6 nodes.
    let num_nodes = 6;
    // set epoch length to 30 mins so it never elapses unless we advance the clock
    let epoch_length = 1800;

    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(num_nodes)
        .build()
        .await;

    let actions = testnet.actions();
    let network_checker = NetworkIntegrityChecker::new(&end_user, &actions).await;

    // Upgrade the node crate to a new version
    let _crate_version_handle = update_node_crate_version("2.9999.9999".to_string());

    // Spin up a new node with the new node version
    info!("Spinning up a new node with the new node version");
    let new_validator = validator_collection
        .add_one(
            false,
            Some(lit_node_testnet::validator::BuildMode::UseNewOrCachedBuild),
            None,
        )
        .await
        .expect("Failed to add new node");
    let new_validator_staker_address = new_validator.account().staker_address;

    // Fast forward time to allow the network to deal in the new node with the new node version
    info!(
        "Fast forwarding time to allow the network to deal in the new node with the new node version"
    );
    actions.increase_blockchain_timestamp(epoch_length).await;

    let realm_id = U256::from(1);
    // Wait for the new epoch
    info!("Waiting for epoch 3");
    actions.wait_for_epoch(realm_id, U256::from(3)).await;

    // Run network checks
    info!("Checking network state");
    assert_eq!(
        actions.get_current_validator_count(realm_id).await as usize,
        num_nodes + 1
    );
    network_checker.check(&validator_collection, &vec![]).await;

    // Update version requirements by setting a max version requirement, rendering the new node version invalid.
    let max_version = "2.9999.9998";
    actions
        .set_staking_max_version(realm_id, max_version)
        .await
        .expect("Failed to set max version");

    // After some time, fast forward to allow the network to deal out the new node with the new node version.
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    info!(
        "Fast forwarding time to allow the network to deal out the new node with the new node version"
    );
    actions.increase_blockchain_timestamp(epoch_length).await;

    // Wait for the new epoch
    info!("Waiting for epoch 4");
    actions.wait_for_epoch(realm_id, U256::from(4)).await;

    // Run network checks
    info!("Checking network state");
    assert_eq!(
        actions.get_current_validator_count(realm_id).await as usize,
        num_nodes
    );
    network_checker.check(&validator_collection, &vec![]).await;

    // Check that the new node is no longer a validator.
    let active_validators = actions.get_current_validators(realm_id).await;
    assert!(!active_validators.contains(&new_validator_staker_address));
}

/// This test assumes that you have the lit_node builds for the target branches.
/// During local development, there are two ways to get the builds:
/// 1. Run the `build_target_branches` script in the `scripts` directory. (x86 and arm64 builds)
/// 2. Run the `download_builds` script in the `scripts` directory. (x86 builds only)
/// The test will fail if the builds are not found.
#[test_case("origin/release-naga-prod-2025-11-25"; "Upgrade against the latest NAGA-Prod release branch")]
#[tokio::test]
async fn test_version_upgrade_against_old_version(target_branch: &str) {
    crate::common::setup_logging();

    info!(
        "TEST: test_version_upgrade_against_old_version against {}",
        target_branch
    );

    // Get the commit hash that we want the build for.
    let old_build_commit_hash =
        utils::get_target_branch_commit_hash(target_branch).expect("Failed to get commit hash");

    info!("Old build commit hash: {}", old_build_commit_hash);
    // First check if we have the build.
    let old_build_path = format!("./target/test-run/debug/lit_node_{}", old_build_commit_hash);
    assert!(
        fs::metadata(&old_build_path).is_ok(),
        "Build does not exist at {}",
        old_build_path
    );

    // Set up a network of nodes running the old build.

    info!("TEST: node_boot_invalid_version");
    // Set up a network with 6 nodes.
    let num_nodes = 6;
    // set epoch length to 30 mins so it never elapses unless we advance the clock
    let epoch_length = 1800;

    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(num_nodes)
        .force_deploy(true)
        .build()
        .await;

    let actions = testnet.actions();
    let realm_id = U256::from(1);
    let starting_epoch = actions.get_current_epoch(realm_id).await;

    let mut next_epoch = starting_epoch + 1;

    // Keep track of the node versions.

    let complete_node_set = &validator_collection.complete_node_set();
    let initial_node_versions = get_node_versions(&complete_node_set).await;
    info!("Initial node versions: {:?}", initial_node_versions);
    // Assert all node versions are the same.
    assert!(
        initial_node_versions
            .iter()
            .all(|v| v == &initial_node_versions[0])
    );

    let network_checker = NetworkIntegrityChecker::new(&end_user, &actions).await;
    network_checker.check(&validator_collection, &vec![]).await;
    // network_checker
    //     .check_with_drained_presigns(&validator_collection)
    //     .await;

    // First, we shuffle the order of the original staker wallets that we will be gradually adding aliases for.
    let mut wallet_manifest_wallets = latest_wallet_manifest(false);
    wallet_manifest_wallets.shuffle(&mut rand::thread_rng());

    // Keep dealing in new node versions and dealing out old node versions until the entire network is upgraded.
    for upgrade_round in 0..num_nodes {
        info!("Upgrading node {} to the new build", upgrade_round);

        // Prepare manifest and run script to generate and add new alias wallet.
        let alias_node_port = validator_collection.max_port() + 1;
        let existing_wallet_to_add_alias_for = wallet_manifest_wallets[upgrade_round].to_owned();
        generate_wallet_and_add_as_alias(&existing_wallet_to_add_alias_for, alias_node_port).await;
        let existing_wallet_with_alias = existing_wallet_to_add_alias_for;

        // Spin up a new node with the new version and the alias wallet.
        let alias_node_config_path =
            format!("{}/alias_lit_config0.toml", alias_node_configs_path());
        assert!(
            validator_collection
                .add_one_custom(
                    false,
                    alias_node_config_path,
                    &get_latest_alias_node_account(0, &testnet),
                    Some(lit_node_testnet::validator::BuildMode::UseNewOrCachedBuild),
                    1
                )
                .await
                .is_ok()
        );

        // Fast forward time to allow nodes to start a DKG to advance to the next epoch.
        validator_collection
            .actions()
            .increase_blockchain_timestamp(epoch_length)
            .await;

        // After next epoch arrives, run interpolation and decryption tests.
        validator_collection
            .actions()
            .wait_for_epoch(realm_id, next_epoch)
            .await;
        next_epoch += U256::from(1);

        validator_collection.actions().sleep_millis(2000).await; // FIXME : let the nodes all acknowledge the epoch, then  run the tests.   This should be removed once signing across epochs works.

        network_checker
            .check_with_drained_presigns(&validator_collection)
            .await;

        // Assert node versions.
        let complete_node_set = &validator_collection.complete_node_set();
        let mut node_versions = get_node_versions(&complete_node_set).await;
        // Sort the node versions to make it easier to compare.
        node_versions.sort();
        info!(
            "node versions ({:?}) {:?} and initial node versions {:?}",
            node_versions.len(),
            node_versions,
            initial_node_versions
        );
        assert_eq!(node_versions.len(), num_nodes + 1);

        // Get current crate version.
        let current_crate_version = get_crate_version();
        for (i, version) in node_versions.iter().enumerate() {
            if i < (num_nodes - upgrade_round) {
                assert_eq!(version, &initial_node_versions[0]);
            } else {
                assert_eq!(version.to_owned(), current_crate_version);
            }
        }

        // The old staker wallet request to leave the network.
        info!(
            "Requesting to leave the network for staker {:?}",
            existing_wallet_with_alias.staker.address
        );
        contracts_repo::request_to_leave(
            &existing_wallet_with_alias.staker.private_key,
            &format!(
                "0x{}",
                bytes_to_hex(
                    validator_collection
                        .actions()
                        .contracts()
                        .staking
                        .address()
                        .as_bytes()
                )
            ),
        );

        // Fast forward time to allow nodes to start a DKG to advance to the next epoch.
        validator_collection
            .actions()
            .increase_blockchain_timestamp(epoch_length)
            .await;

        // After next epoch arrives, kill node with old version and run network tests.
        validator_collection
            .actions()
            .wait_for_epoch(realm_id, next_epoch)
            .await;
        next_epoch += U256::from(1);
        network_checker
            .check_with_drained_presigns(&validator_collection)
            .await;

        // Kill the node with the old staker wallet.
        assert!(
            validator_collection
                .stop_node(existing_wallet_with_alias.idx)
                .await
                .is_ok()
        );

        network_checker
            .check_with_drained_presigns(&validator_collection)
            .await;
    }
}

fn get_latest_alias_node_account(idx: usize, testnet: &Testnet) -> NodeAccount {
    let latest_alias_wallet_manifest = latest_wallet_manifest(true);
    let provider = ENDPOINT_MANAGER
        .get_provider(testnet.chain_name.clone())
        .expect("Failed to get provider");

    let mut provider_mut = provider.as_ref().clone();
    provider_mut.set_interval(Duration::new(0, 10));
    let provider = std::sync::Arc::new(provider_mut);
    latest_alias_wallet_manifest[idx].map_to_node_account(provider, testnet.chain_id)
}

/// Returns the wallet manifest item that we had added an alias for.
async fn generate_wallet_and_add_as_alias(
    existing_wallet_manifest_item: &WalletManifestItem,
    alias_node_port: usize,
) {
    info!(
        "Using random wallet from manifest to add an alias for: {:?}",
        existing_wallet_manifest_item
    );

    // Generate a new alias manifest by copying from the template and adjusting the values.
    let mut parsed_alias_manifest_template = get_alias_manifest_template();
    info!("Using {:?} as the alias node port", alias_node_port);
    parsed_alias_manifest_template.alias_port = alias_node_port;
    parsed_alias_manifest_template.existing_staker_wallet_private_key =
        existing_wallet_manifest_item.staker.private_key.clone();
    parsed_alias_manifest_template.node_config_ipfs_api_key = std::env::var("IPFS_API_KEY")
        .expect("IPFS_API_KEY not set")
        .to_owned();

    // Write to file.
    save_alias_manifest(&parsed_alias_manifest_template);

    // Now that we have the alias manifest ready, we can run the script.
    contracts_repo::generate_wallet_and_add_as_alias();
}
