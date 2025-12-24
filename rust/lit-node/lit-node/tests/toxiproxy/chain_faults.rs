use crate::common::faults::{
    disable_fault_channel_direct, generate_and_save_proxy_mappings_for_local_chain_testing,
    get_random_faulty_node_port, setup_proxies,
};
use crate::common::networking::get_local_url_from_port;
use crate::common::setup_logging;
use ethers::types::U256;
use lit_node_common::proxy_mapping::ClientProxyMapping;
use lit_node_testnet::TestSetupBuilder;
use once_cell::sync::Lazy;
use tracing::info;

const FAULT_TEST_NUM_NODES: usize = 5;
const STARTING_PORT: usize = 7470;
const ANVIL_PORT: usize = 8545;
static PROXY_MAPPINGS: Lazy<ClientProxyMapping> = Lazy::new(|| {
    generate_and_save_proxy_mappings_for_local_chain_testing(FAULT_TEST_NUM_NODES, STARTING_PORT)
        .unwrap()
});

fn setup() {
    setup_logging();
    // Set up proxies
    setup_proxies(&PROXY_MAPPINGS);
}

#[ignore]
#[tokio::test]
async fn single_node_disconnected_from_chain() {
    setup();

    info!("TEST: single_node_disconnects_from_chain");

    let faulty_node_port =
        get_random_faulty_node_port(STARTING_PORT, STARTING_PORT + FAULT_TEST_NUM_NODES);
    info!("Faulty node port: {}", faulty_node_port);

    let fault_node_url = get_local_url_from_port(faulty_node_port);
    let anvil_url = get_local_url_from_port(ANVIL_PORT);

    // setup and instantly disable the fault, since we need to hook the nodes config, which gets cached.
    // inject_timeout_fault_direct(
    //     fault_node_url.clone(),
    //     anvil_url.clone(),
    //     TIMEOUT_MS,
    //     TOXICITY,
    //     true,
    // );
    // disable_fault_direct(fault_node_url.clone(), anvil_url.clone(), true);

    // Start a new node collection
    let (testnet, validator_collection, _end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(FAULT_TEST_NUM_NODES)
        .is_fault_test(true)
        .build()
        .await;

    let actions = testnet.actions().clone();

    // wait for a few seconds to led the nodes chat with each other.
    actions.sleep_millis(5000).await;

    // This looks odd, but instead of settting a fault within Toxiproxy, we're going to disable
    // the entire channel, effectively disconnecting the node from the chain.
    disable_fault_channel_direct(fault_node_url.clone(), anvil_url.clone(), true);

    // wait for a minute to observe the effects of the fault.
    actions.sleep_millis(60000).await;

    let realm_id = U256::from(1);
    let epoch = actions.get_current_epoch(realm_id).await;
    info!("Current epoch: {}", epoch);
    let seconds_to_increase = 300;
    actions
        .increase_blockchain_timestamp(seconds_to_increase)
        .await;

    let next_epoch = epoch + U256::from(1);
    info!("Next epoch: {}", next_epoch);
    actions.wait_for_epoch(realm_id, next_epoch).await;
    info!("Advanced to next epoch: {}", next_epoch);

    info!("Validator length: {}", validator_collection.size());
}
