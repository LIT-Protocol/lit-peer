use crate::common::faults::{
    disable_chain_for_random_faulty_node, enable_chain_for_node,
    generate_and_save_proxy_mappings_for_local_chain_testing, setup_proxies,
};
use crate::common::setup_logging;
use ethers::types::U256;
use lit_node_common::proxy_mapping::ClientProxyMapping;
use lit_node_testnet::TestSetupBuilder;
use once_cell::sync::Lazy;
use tracing::info;

const FAULT_TEST_NUM_NODES: usize = 5;
const STARTING_PORT: usize = 7470;
static PROXY_MAPPINGS: Lazy<ClientProxyMapping> = Lazy::new(|| {
    generate_and_save_proxy_mappings_for_local_chain_testing(FAULT_TEST_NUM_NODES, STARTING_PORT)
        .unwrap()
});

fn setup() {
    setup_logging();
    // Set up proxies
    setup_proxies(&PROXY_MAPPINGS);
}

#[tokio::test]
async fn kick_node_who_loses_chain_connection() {
    setup();

    info!("TEST: kick_node_who_loses_chain_connection");
    let realm_id = U256::from(1);
    let seconds_to_increase = 300;

    // Start a new node collection
    let (testnet, _validator_collection, _end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(FAULT_TEST_NUM_NODES)
        .is_fault_test(true)
        .build()
        .await;

    let actions = testnet.actions().clone();

    // wait for a few seconds to let the nodes chat with each other.
    actions.sleep_millis(1000).await;

    let faulty_node_port =
        disable_chain_for_random_faulty_node(STARTING_PORT, FAULT_TEST_NUM_NODES);
    info!("Faulty node port: {}", faulty_node_port);

    assert!(
        actions
            .update_all_complaint_configs(Some(50), Some(2), None, Some(1))
            .await
            .is_ok()
    );

    // Update the epoch, forcing a kick due to DKG non-participation.
    let epoch = actions.get_current_epoch(realm_id).await;
    info!("Current epoch: {}", epoch);
    actions
        .increase_blockchain_timestamp(seconds_to_increase)
        .await;

    let next_epoch = epoch + U256::from(1);
    info!("Next epoch: {}", next_epoch);
    actions.wait_for_epoch(realm_id, next_epoch).await;
    info!("Advanced to next epoch: {}", next_epoch);

    // Test to see if our validator was kicked.
    let validator_structs = actions.get_current_validator_structs(realm_id).await;
    assert!(
        validator_structs
            .iter()
            .find(|v| v.port == faulty_node_port as u32)
            .is_none()
    );
}

#[tokio::test]
async fn auto_rejoin_faulty_node() {
    setup();

    info!("TEST: auto_rejoin_faulty_node");
    let realm_id = U256::from(1);
    let seconds_to_increase = 300;

    // Start a new node collection
    let (testnet, _validator_collection, _end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(FAULT_TEST_NUM_NODES)
        .is_fault_test(true)
        .build()
        .await;

    let actions = testnet.actions().clone();

    // wait for a few seconds to led the nodes chat with each other.
    actions.sleep_millis(1000).await;

    let faulty_node_port =
        disable_chain_for_random_faulty_node(STARTING_PORT, FAULT_TEST_NUM_NODES);
    info!("Faulty node port: {}", faulty_node_port);

    assert!(
        actions
            .update_all_complaint_configss(Some(50), Some(2), None, Some(1))
            .await
            .is_ok()
    );

    // Update the epoch, forcing a kick due to DKG non-participation.
    let epoch = actions.get_current_epoch(realm_id).await;
    info!("Current epoch: {}", epoch);
    actions
        .increase_blockchain_timestamp(seconds_to_increase)
        .await;

    let next_epoch = epoch + U256::from(1);
    info!("Next epoch: {}", next_epoch);
    actions.wait_for_epoch(realm_id, next_epoch).await;
    info!("Advanced to next epoch: {}", next_epoch);

    // Test to see if our validator was kicked.

    // wait for the kicked node to try to call rejion.
    enable_chain_for_node(faulty_node_port);
    actions.sleep_millis(3000).await;

    let epoch = actions.get_current_epoch(realm_id).await;
    info!("Current epoch: {}", epoch);
    actions
        .increase_blockchain_timestamp(seconds_to_increase)
        .await;

    let next_epoch = epoch + U256::from(1);
    info!("Next epoch: {}", next_epoch);
    actions.wait_for_epoch(realm_id, next_epoch).await;
    info!("Advanced to next epoch: {}", next_epoch);
}
