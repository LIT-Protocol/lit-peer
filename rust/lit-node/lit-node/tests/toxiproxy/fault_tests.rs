use chrono::Duration;
use lit_node::utils::consensus::get_threshold_count;
use lit_node_core::CurveType;
use lit_node_testnet::TestSetupBuilder;
use std::collections::HashMap;

use crate::common::assertions::NetworkIntegrityChecker;
use crate::common::faults::{
    FAULT_TEST_CHATTER_CLIENT_TIMEOUT_SECS, FaultType,
    generate_and_save_proxy_mappings_for_local_testing,
    inject_fault_between_all_sources_to_random_target, inject_latency_fault, setup_proxies,
};

use crate::common::networking::get_local_url_from_port;
use crate::common::setup_logging;
use lit_node_testnet::testnet::contracts::{ComplaintConfig, StakingContractRealmConfig};

use ethers::types::U256;
use lit_node::peers::peer_reviewer::Issue;
use lit_node_common::proxy_mapping::ClientProxyMapping;
use once_cell::sync::Lazy;
use test_case::test_case;

use tracing::{debug, info};

const FAULT_TEST_NUM_NODES: usize = 5;
const STARTING_PORT: usize = 7470;

static PROXY_MAPPINGS: Lazy<ClientProxyMapping> = Lazy::new(|| {
    generate_and_save_proxy_mappings_for_local_testing(FAULT_TEST_NUM_NODES, STARTING_PORT).unwrap()
});

fn setup() {
    setup_logging();

    // Set up proxies
    setup_proxies(&PROXY_MAPPINGS);
}

/// This tests that when the link between node 0 and node 1 has a fault in one direction, it results in
/// node 1 voting to kick node 0 and vice versa during the second DKG.
#[tokio::test]
pub async fn single_link_fault_transient_oneway() {
    let realm_id = U256::from(1);
    setup();

    info!("Starting single_link_fault_transient_oneway test");

    let epoch_length = 100;

    let (testnet, validator_collection, _end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(FAULT_TEST_NUM_NODES)
        .epoch_length(epoch_length)
        .is_fault_test(true)
        .build()
        .await;

    let actions = testnet.actions();

    info!("Injecting fault");
    // Inject fault between node 1 and node 0
    inject_latency_fault(
        get_local_url_from_port(7470 + 1),
        get_local_url_from_port(7470),
        Duration::seconds(i64::try_from(FAULT_TEST_CHATTER_CLIENT_TIMEOUT_SECS + 2).unwrap())
            .num_milliseconds()
            .try_into()
            .unwrap(),
        0,
        1.0,
    );

    // Update complaint config to be 1 to speed up test.
    // This is intentionally not set before the validator collection is built since only 1 vote is
    // needed to kick a node in the genesis epoch, and we don't want complaints due to each node being
    // spun up during building the validator collection.
    info!("Updating complaint config");
    let mut complaint_reason_to_config = HashMap::<U256, ComplaintConfig>::new();
    complaint_reason_to_config.insert(
        U256::from(Issue::Unresponsive.value()),
        ComplaintConfig::builder()
            .tolerance(U256::from(1))
            .interval_secs(U256::from(5))
            .build(),
    );
    // Make all complaint types have short intervals.
    complaint_reason_to_config.insert(
        U256::from(Issue::NonParticipation.value()),
        ComplaintConfig::builder()
            .interval_secs(U256::from(5))
            .build(),
    );
    complaint_reason_to_config.insert(
        U256::from(Issue::IncorrectInfo.value()),
        ComplaintConfig::builder()
            .interval_secs(U256::from(5))
            .build(),
    );
    complaint_reason_to_config.insert(
        U256::from(Issue::KeyShareValidationFailure(CurveType::BLS).value()),
        ComplaintConfig::builder()
            .interval_secs(U256::from(5))
            .build(),
    );

    assert!(
        actions
            .update_staking_realm_config(
                StakingContractRealmConfig::builder()
                    .complaint_reason_to_config(complaint_reason_to_config)
                    .build()
            )
            .await
            .is_ok()
    );
    assert!(actions.wait_for_complaint_cache_to_clear().await.is_ok());

    info!("Complaint configurations have been set");

    // Fast forward time so that nodes can lock and advance the genesis epoch.
    actions.increase_blockchain_timestamp(300).await;

    // Get staker address of the validator to be kicked (node 0)
    let node_0_staker_address = testnet.node_accounts[0].staker_address;
    let node_0_address = validator_collection
        .get_validator_by_account(&testnet.node_accounts[0])
        .unwrap()
        .node_address();

    // Get staker address of the validator voting to kick (node 1)
    let node_1_staker_address = testnet.node_accounts[1].staker_address;
    let node_1_address = validator_collection
        .get_validator_by_account(&testnet.node_accounts[1])
        .unwrap()
        .node_address();
    info!(
        "Waiting for staker {} at {} to vote to kick staker {} at {}",
        node_1_staker_address, node_1_address, node_0_staker_address, node_0_address
    );

    let epoch = actions.get_current_epoch(realm_id).await;
    // Assert that, eventually, there is 1 vote from node 1 voting to kick node 0.
    let voting_status = actions
        .wait_for_voting_status_to_kick_validator(
            realm_id,
            epoch,
            node_0_staker_address,
            node_1_staker_address,
            1,
            false,
        )
        .await;

    // test to see if the voting status took too long to get the result

    let voting_status = match voting_status.is_ok() {
        true => voting_status,
        false => {
            let epoch = actions.get_current_epoch(realm_id).await;
            actions
                .wait_for_voting_status_to_kick_validator(
                    realm_id,
                    epoch,
                    node_0_staker_address,
                    node_1_staker_address,
                    1,
                    false,
                )
                .await
        }
    };

    assert!(voting_status.is_ok());

    let voting_status = voting_status.unwrap();
    assert_eq!(voting_status.votes.as_usize(), 1);
    assert!(voting_status.did_voter_vote_to_kick_validator);
}

/// This tests that the nodes are able to DKG and sign when a single node is semi-faulty.
#[tokio::test]
async fn single_node_semi_faulty() {
    setup();
    let realm_id = U256::from(1);

    info!("TEST: single_node_semi_faulty");

    // Inject faults
    let faulty_node_port = inject_fault_between_all_sources_to_random_target(
        FaultType::LatencyAroundClientTimeout,
        STARTING_PORT,
        FAULT_TEST_NUM_NODES,
    );

    info!("Faulty node port: {}", faulty_node_port);

    // Start a new node collection
    let (testnet, validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(FAULT_TEST_NUM_NODES)
        .is_fault_test(true)
        .build()
        .await;

    // Todo -> should nodes be able to report to the SDK about the peers that they have validated with ?
    // This would be useful at the SDK level to avoid making requests to a peer set that can't communicate within itself....
    if testnet.is_from_cache {
        // because the testnet is cached, and we have a multi-second fault, we need to sleep a bit to let the nodes eventually respond to liveness checks.
        // the 12 seconds accounts for 3s of latency, 3s of jitter, and an additional 6 seconds as a buffer to ensure all nodes have sufficient time to respond to liveness checks under fault conditions.
        testnet.actions().sleep_millis(12000).await;
    }

    let faulty_node = validator_collection
        .get_validator_by_port(faulty_node_port)
        .unwrap();
    let validators_to_include = &vec![faulty_node];
    // Assert that the node didn't get kicked
    let current_validators = validator_collection
        .actions()
        .get_current_validators(realm_id)
        .await;
    info!("Node collection get current validators");
    assert_eq!(current_validators.len(), FAULT_TEST_NUM_NODES);

    // Assert that the network can still perform operations
    let network_checker =
        NetworkIntegrityChecker::new(&end_user, validator_collection.actions()).await;
    network_checker
        .check(&validator_collection, validators_to_include)
        .await;
}

/// This tests that the nodes are able to DKG and sign when a single node is faulty
/// during the second DKG.
#[test_case(FaultType::LatencyBelowClientTimeout, false; "with latency below client timeout")]
#[test_case(FaultType::SlowTcpClosing, false; "with slow tcp closing")]
#[test_case(FaultType::TimeoutAboveClientTimeout, true; "with timeout above client timeout")]
#[test_case(FaultType::Slicer, false; "with slicer")]
#[tokio::test]
pub async fn single_node_fault_after_first_dkg_during_second_dkg(
    fault_type: FaultType,
    is_faulty_node_kicked: bool,
) {
    setup();
    let realm_id = U256::from(1);
    info!("Starting test with fault type: {:?}", fault_type);

    // Start staking and Initial DKG
    let (testnet, validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(FAULT_TEST_NUM_NODES)
        .is_fault_test(true)
        .build()
        .await;

    let actions = testnet.actions();
    let starting_epoch = actions.get_current_epoch(realm_id).await;
    debug!("Starting epoch: {}", starting_epoch);
    assert!(
        starting_epoch == U256::from(2),
        "Starting epoch should be 2"
    );

    // Update complaint config to be 3 to speed up test. Allow some time for nodes to sync up.
    // This is intentionally not set before the first epoch advancement since only 1 vote is
    // needed to kick a node in the genesis epoch - we don't want that.
    let mut complaint_reason_to_config = HashMap::<U256, ComplaintConfig>::new();
    complaint_reason_to_config.insert(
        U256::from(Issue::Unresponsive.value()),
        ComplaintConfig::builder().tolerance(U256::from(3)).build(),
    );

    assert!(
        actions
            .update_staking_realm_config(
                StakingContractRealmConfig::builder()
                    .complaint_reason_to_config(complaint_reason_to_config)
                    .build()
            )
            .await
            .is_ok()
    );
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let network_checker = NetworkIntegrityChecker::new(&end_user, &actions).await;

    // Inject faults before the next DKG starts for all connections upstream to a randomly
    // selected node.
    let random_faulty_node_port = inject_fault_between_all_sources_to_random_target(
        fault_type,
        STARTING_PORT,
        FAULT_TEST_NUM_NODES,
    );

    // fast forward timestamp so that nodes can lock and advance.
    actions.increase_blockchain_timestamp(300).await;

    actions.wait_for_lock(realm_id).await; // wait for lock, triggering DKG
    let epoch_number = actions.get_current_epoch(realm_id).await;
    info!(
        "Validator set locked for Epoch 3 - we are in Epoch {}",
        epoch_number
    );
    assert!(
        epoch_number == U256::from(2),
        "Epoch number should still be 2"
    );
    if is_faulty_node_kicked {
        info!(
            "Waiting for faulty node with port {} to be kicked",
            random_faulty_node_port
        );
        // Assert that, eventually, there are a threshold number (7) of votes from nodes voting to kick the randomly chosen node with faults.
        let random_faulty_node_idx = random_faulty_node_port - STARTING_PORT;
        let first_non_faulty_node_idx = {
            let mut first_non_faulty_node_idx = 99usize;
            for i in 0..FAULT_TEST_NUM_NODES {
                if i != random_faulty_node_idx {
                    first_non_faulty_node_idx = i;
                    break;
                }
            }
            first_non_faulty_node_idx
        };
        let staker_address_to_kick = testnet.node_accounts[random_faulty_node_idx].staker_address;
        let staker_address_of_non_faulty_node =
            testnet.node_accounts[first_non_faulty_node_idx].staker_address;
        let epoch_number = actions.get_current_epoch(realm_id).await;
        let get_voting_status_res = actions
            .wait_for_voting_status_to_kick_validator(
                realm_id,
                epoch_number,
                staker_address_to_kick,
                staker_address_of_non_faulty_node,
                get_threshold_count(FAULT_TEST_NUM_NODES),
                true,
            )
            .await;
        assert!(get_voting_status_res.is_ok());
        info!(
            "Faulty node with port {} and staker_address {} is kicked",
            random_faulty_node_port, staker_address_to_kick
        );
    }

    // Wait for network to advance to new epoch after DKG
    let current_epoch = actions.get_current_epoch(realm_id).await;
    actions.wait_for_epoch(realm_id, current_epoch + 1).await;

    info!(
        "Waited for network to advance to new epoch after DKG.  Current epoch: {}",
        current_epoch
    );

    // Run network checks.
    network_checker.check(&validator_collection, &vec![]).await;
}
