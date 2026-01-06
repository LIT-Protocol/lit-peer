use lit_node_testnet::{
    TestSetupBuilder,
    node_collection::get_node_versions,
    testnet::contracts_repo::{
        self, alias_node_configs_path, latest_wallet_manifest, node_configs_path,
    },
};

use crate::common::{assertions::NetworkIntegrityChecker, version::get_crate_version};

use ethers::types::U256;
use lit_core::utils::binary::bytes_to_hex;
use rand::seq::SliceRandom;
use std::{fs, time::Duration};
use test_case::test_case;
use tracing::info;

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

    // First check if we have the build.
    let release_build_path = format!("./target/{}/debug/lit_node", release_version);
    assert!(
        fs::metadata(&release_build_path).is_ok(),
        "Build does not exist at {}",
        release_build_path
    );

    // Set up a network of nodes running the old build.

    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default()
        .custom_binary_path(Some(release_build_path))
        .force_deploy(true)
        .build()
        .await;

    let num_nodes = validator_collection.validator_count();
    let actions = testnet.actions();
    let realm_id = U256::from(1);
    let starting_epoch = actions.get_current_epoch(realm_id).await;
    let mut next_epoch = starting_epoch + 1;
    let epoch_length = actions.get_epoch_length(realm_id).await.unwrap().as_u64() as usize;

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

        info!("Upgrading node {} to the new build", upgrade_round + 1);

        // Fast forward time to allow nodes to start a DKG to advance to the next epoch.

        actions.increase_blockchain_timestamp(epoch_length).await;

        // After next epoch arrives, run interpolation and decryption tests.
        actions.wait_for_epoch(realm_id, next_epoch).await;
        next_epoch += U256::from(1);

        network_checker.check(&validator_collection, &vec![]).await;

        // Assert node versions.
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
        network_checker.check(&validator_collection, &vec![]).await;

        // Kill the node with the old staker wallet.
        assert!(
            validator_collection
                .stop_node(existing_wallet_with_alias.idx)
                .await
                .is_ok()
        );

        network_checker.check(&validator_collection, &vec![]).await;
    }
}
