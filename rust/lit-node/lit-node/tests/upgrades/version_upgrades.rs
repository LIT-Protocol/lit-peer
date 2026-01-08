use crate::common::{assertions::NetworkIntegrityChecker, version::get_crate_version};
use ethers::types::U256;
use lit_node_core::NodeSet;
use lit_node_testnet::{
    TestSetupBuilder, node_collection::get_node_versions, testnet::actions::Actions,
    validator::ValidatorCollection,
};
use std::fs;
use test_case::test_case;
use tracing::info;

struct UpgradeStepData {
    pub upgrade_round: usize,
    pub initial_node_count: usize,
    pub initial_node_versions: Vec<String>,
    pub complete_node_set: Vec<NodeSet>,
    pub realm_id: U256,
    pub epoch_length: usize,
}

/// This test assumes that you have the lit_node builds for the target branches.
/// During local development, there are two ways to get the builds:
/// 1. Run the `build_target_branches` script in the `scripts` directory. (x86 and arm64 builds)
/// 2. Run the `download_builds` script in the `scripts` directory. (x86 builds only)
/// The test will fail if the builds are not found.
#[test_case("2.1.5", false; "Upgrade against the latest NAGA-Prod release branch, assuming chain state was updated manually.")]
#[tokio::test]
async fn test_version_upgrade_against_old_version(
    release_version: &str,
    use_old_chain_state: bool,
) {
    crate::common::setup_logging();
    info!("TEST: Upgrade against release: {}", release_version);

    let standard_binary_path = "./target/test-run/debug/lit_node";
    // First check if we have the build.
    let old_build_path = format!("./target/test-run/debug/lit_node_{}", old_build_commit_hash);
    assert!(
        fs::metadata(&old_build_path).is_ok(),
        "Build does not exist at {}",
        old_build_path
    );

    if use_old_chain_state {
        // TODO: Implement old chain state setup, by passing a parameter to the chain state data.
    }

    // Set up a network of nodes running the old build.
    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default()
        .custom_binary_path(Some(release_build_path))
        .max_presign_count(0)
        .min_presign_count(0)
        .force_deploy(true)
        .build()
        .await;

    let initial_node_count = validator_collection.validator_count();
    let actions = testnet.actions();
    let vc = validator_collection.clone();

    // Keep track of the node versions.
    let complete_node_set = &validator_collection.complete_node_set();
    let initial_node_versions = get_node_versions(&complete_node_set).await;
    info!("Initial node versions: {:?}", initial_node_versions);
    // Assert all node versions are the same.
    assert!(
        upgrade_step_data
            .initial_node_versions
            .iter()
            .all(|v| v == &upgrade_step_data.initial_node_versions[0])
    );

    info!("Validating initial network state");
    network_checker.check(&vc, &vec![]).await;

    // Keep dealing in new node versions and dealing out old node versions until the entire network is upgraded.
    for upgrade_round in 0..initial_node_count {
        info!("Upgrading node {} to the new build", upgrade_round + 1);
        upgrade_step_data.upgrade_round = upgrade_round;

        // select a random validator to upgrade
        let validator = validator_collection.get_validator_by_index_as_mut(upgrade_round);

        // request to leave
        validator
            .request_to_leave(&actions)
            .await
            .expect("Failed to request to leave");

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

/// This test assumes that you have the lit_node builds for the target branches.
/// During local development, there are two ways to get the builds:
/// 1. Run the `build_target_branches` script in the `scripts` directory. (x86 and arm64 builds)
/// 2. Run the `download_builds` script in the `scripts` directory. (x86 builds only)
/// The test will fail if the builds are not found.
#[test_case("origin/release-naga-prod-2025-11-25"; "Upgrade against the latest NAGA-Prod release branch")]
#[tokio::test]
async fn test_version_upgrade_against_old_version_with_new_stakers(target_branch: &str) {
    crate::common::setup_logging();

    info!("TEST: Upgrade against (new): {}", target_branch);

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

    let num_nodes = 5;
    // Set up a network of nodes running the old build.

    info!("TEST: test_version_upgrade_against_old_version");
    // Set up a network with 6 nodes.
    // set epoch length to 30 mins so it never elapses unless we advance the clock
    let epoch_length = 1800;

    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default()
        .custom_binary_path(Some(old_build_path))
        // .force_deploy(true)  // make sure all the initial chain data comes from the old build.
        .num_staked_and_joined_validators(num_nodes)
        .num_staked_only_validators(num_nodes)
        .start_staked_only_validators(false)
        .build()
        .await;

    let num_nodes = validator_collection.validator_count();
    let actions = testnet.actions();
    let realm_id = U256::from(1);
    let starting_epoch = actions.get_current_epoch(realm_id).await;
    let mut next_epoch = starting_epoch + 1;

    let current_crate_version = get_crate_version();

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

    // Keep dealing in new node versions and dealing out old node versions until the entire network is upgraded.
    for upgrade_round in 0..num_nodes {
        let validator = validator_collection.get_validator_by_idx(upgrade_round);
        info!(
            "Requesting to leave the network for staker {:?}",
            validator.account().staker_address
        );
        assert!(validator.request_to_leave(&actions).await.is_ok());

        info!("Upgrading node {} to the new build", upgrade_round);

        let node_account = &testnet.node_accounts[upgrade_round + num_nodes];
        let validator_idx = upgrade_round + num_nodes;
        let node_config_file_path =
            format!("{}/lit_config{:?}.toml", node_configs_path(), validator_idx);

        assert!(
            validator_collection
                .add_one_custom(
                    false,
                    node_config_file_path,
                    node_account,
                    Some(lit_node_testnet::validator::BuildMode::UseNewOrCachedBuild),
                    1
                )
                .await
                .is_ok()
        );

        // Fast forward time to allow nodes to start a DKG to advance to the next epoch.
        actions.increase_blockchain_timestamp(epoch_length).await;

        // After next epoch arrives, run interpolation and decryption tests.
        actions.wait_for_epoch(realm_id, next_epoch).await;
        next_epoch += U256::from(1);

    network_checker.check(validator_collection, &vec![]).await;

    if nodes_removed == 0 {
        let mut node_versions = get_node_versions(&data.complete_node_set).await;
        // Assert node versions.
        assert_eq!(node_versions.len() - nodes_removed, data.initial_node_count);

        // Sort the node versions to make it easier to compare.
        node_versions.sort();
        info!(
            "node versions ({:?}) {:?} and initial node versions {:?}",
            node_versions.len(),
            node_versions,
            data.initial_node_versions
        );

        // Get current crate version.
        // let current_crate_version = get_crate_version();
        // for (i, version) in node_versions.iter().enumerate() {
        //     if i < (data.initial_node_count - data.upgrade_round) {
        //         assert_eq!(version, &data.initial_node_versions[0]);
        //     } else {
        //         assert_eq!(version.to_owned(), current_crate_version);
        //     }
        // }
    }
}
