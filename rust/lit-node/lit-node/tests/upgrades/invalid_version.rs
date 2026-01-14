use lit_node_testnet::TestSetupBuilder;

use crate::common::{assertions::NetworkIntegrityChecker, version::update_node_crate_version};

use ethers::types::{H160, U256};
use lit_blockchain::contracts::staking::ComplaintConfig;
use lit_node::{peers::peer_reviewer::Issue, utils::consensus::get_threshold_count};
use tracing::info;

/// Tests when an inactive validator that comes online with an invalid version, and then the staker requests to join,
/// that the node should eventually be kicked for non-participation.
#[tokio::test]
async fn node_boot_invalid_version() {
    crate::common::setup_logging();
    info!("TEST: node_boot_invalid_version");
    // set epoch length to 30 mins so it never elapses unless we advance the clock

    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default().build().await;

    let realm_id = U256::from(1);
    let epoch_length = testnet
        .actions()
        .get_epoch_length(realm_id)
        .await
        .unwrap()
        .as_u64() as usize;
    let num_nodes = validator_collection.validator_count();
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
