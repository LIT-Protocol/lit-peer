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
    pub num_nodes: usize,
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

    // First check if we have the build.
    let release_build_path = format!("./target/{}/debug/lit_node", release_version);
    assert!(
        fs::metadata(&release_build_path).is_ok(),
        "Build does not exist at {}",
        release_build_path
    );

    if use_old_chain_state {
        // TODO: Implement old chain state setup, by passing a parameter to the chain state data.
    }

    // Set up a network of nodes running the old build.
    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default()
        .custom_binary_path(Some(release_build_path))
        .force_deploy(true)
        .build()
        .await;

    let num_nodes = validator_collection.validator_count();
    let actions = testnet.actions();
    let vc = validator_collection.clone();

    let complete_node_set = validator_collection.complete_node_set();
    let initial_node_versions = get_node_versions(&complete_node_set).await;
    let network_checker = NetworkIntegrityChecker::new(&end_user, &actions).await;

    let mut upgrade_step_data = UpgradeStepData {
        upgrade_round: 0,
        num_nodes,
        initial_node_versions,
        complete_node_set,
        realm_id: U256::from(1),
        epoch_length: actions
            .get_epoch_length(U256::from(1))
            .await
            .unwrap()
            .as_u64() as usize,
    };

    info!(
        "Initial node versions: {:?}",
        upgrade_step_data.initial_node_versions
    );
    // Assert all node versions are the same.
    assert!(
        upgrade_step_data
            .initial_node_versions
            .iter()
            .all(|v| v == &upgrade_step_data.initial_node_versions[0])
    );

    info!("Validating initial network state");
    network_checker
        .check(&mut validator_collection, &vec![])
        .await;

    // Keep dealing in new node versions and dealing out old node versions until the entire network is upgraded.
    for upgrade_round in 0..num_nodes {
        info!("Upgrading node {} to the new build", upgrade_round + 1);
        upgrade_step_data.upgrade_round = upgrade_round;

        // select a random validator to upgrade
        let validator = validator_collection.get_validator_by_index_as_mut(upgrade_round);

        // request to leave
        validator
            .request_to_leave(&actions)
            .await
            .expect("Failed to request to leave");
        validator.stop_node().expect("Failed to stop node");

        // advance and validate
        advance_and_validate_step(&actions, &network_checker, &vc, &upgrade_step_data).await;
        network_checker.check(&vc, &vec![]).await;

        // start the node with a new binary
        validator
            .start_node(false, true)
            .await
            .expect("Failed to start node");
        // request to join
        validator
            .request_to_join(&actions, U256::from(1))
            .await
            .expect("Failed to request to join");

        // test that we can advance and validate the step
        advance_and_validate_step(&actions, &network_checker, &vc, &upgrade_step_data).await;
    }
}

async fn advance_and_validate_step(
    actions: &Actions,
    network_checker: &NetworkIntegrityChecker,
    validator_collection: &ValidatorCollection,
    data: &UpgradeStepData,
) {
    let current_epoch = actions.get_current_epoch(data.realm_id).await;
    let next_epoch = current_epoch + 1;
    actions
        .increase_blockchain_timestamp(data.epoch_length)
        .await;

    // After next epoch arrives, run interpolation and decryption tests.
    actions.wait_for_epoch(data.realm_id, next_epoch).await;

    network_checker.check(validator_collection, &vec![]).await;

    // Assert node versions.
    let mut node_versions = get_node_versions(&data.complete_node_set).await;
    // Sort the node versions to make it easier to compare.
    node_versions.sort();
    info!(
        "node versions ({:?}) {:?} and initial node versions {:?}",
        node_versions.len(),
        node_versions,
        data.initial_node_versions
    );
    assert_eq!(node_versions.len(), data.num_nodes + 1);

    // Get current crate version.
    let current_crate_version = get_crate_version();
    for (i, version) in node_versions.iter().enumerate() {
        if i < (data.num_nodes - data.upgrade_round) {
            assert_eq!(version, &data.initial_node_versions[0]);
        } else {
            assert_eq!(version.to_owned(), current_crate_version);
        }
    }
}
