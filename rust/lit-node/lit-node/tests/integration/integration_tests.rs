use crate::common::{
    assertions::NetworkIntegrityChecker, peers::get_random_peer_within_deterministic_subset,
    setup_logging,
};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::types::{H160, U256};
use futures::future::{BoxFuture, join_all};
use lit_blockchain::{
    contracts::staking::{Staking, StakingEvents, Validator, staking},
    resolver::rpc::{ENDPOINT_MANAGER, RpcHealthcheckPoller},
};
use lit_core::utils::binary::bytes_to_hex;
use lit_node::{
    peers::peer_reviewer::{Issue, MAX_COMPLAINT_REASON_VALUE},
    utils::consensus::get_threshold_count,
};
use lit_node_testnet::validator::remove_node_keys;
use lit_node_testnet::{TestSetupBuilder, end_user::EndUser};
use lit_node_testnet::{
    node_collection::choose_random_indices,
    testnet::{
        NodeAccount, Testnet,
        contracts::{ComplaintConfig, Contracts, StakingContractRealmConfig},
        contracts_repo::default_staker_ip_addresses,
    },
    validator::ValidatorCollection,
};
use std::{collections::HashMap, sync::Arc, time::Duration};
use test_case::test_case;
use tracing::info;

/// Tests a node being kicked after locking and signaling ready but before the epoch advances.
/// The nodes will re-lock and retry the DKG / reshare with the new locked set
#[tokio::test]
async fn retry_after_signaling_ready_test() {
    setup_logging();

    info!("TEST: retry_after_signaling_ready_test");

    // Start a new node collection and wait for the DKG to complete
    // and root keys to be voted for.

    const NUM_NODES: usize = 7;
    let epoch_length = 300;
    let (testnet, validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(NUM_NODES)
        .epoch_length(epoch_length)
        .chain_polling_interval("5000".to_string())
        .build()
        .await;

    // get max node by staker address
    let node_staker_address_to_kick = testnet
        .node_accounts
        .iter()
        .max_by(|a, b| a.staker_address.cmp(&b.staker_address))
        .unwrap()
        .staker_address;

    info!(
        "Node to kick: {:?}\n All nodes: {:?}",
        node_staker_address_to_kick,
        &testnet
            .node_accounts
            .iter()
            .map(|a| a.staker_address)
            .collect::<Vec<_>>()
    );

    // Assert that the network works
    let network_checker =
        NetworkIntegrityChecker::new(&end_user, &validator_collection.actions()).await;
    network_checker.check(&validator_collection, &vec![]).await;

    let realm_id = U256::from(1);
    let retries_before_kicking = validator_collection
        .actions()
        .contracts()
        .staking
        .epoch(realm_id)
        .call()
        .await
        .unwrap()
        .retries;
    assert!(
        retries_before_kicking == U256::from(0),
        "Retries before kicking should be 0 but it is {:?}",
        retries_before_kicking,
    );

    let epoch = validator_collection
        .actions()
        .contracts()
        .staking
        .epoch(realm_id)
        .call()
        .await
        .unwrap();
    info!(
        "Epoch before advancing timestamp and kicking is {}",
        epoch.number
    );

    // as soon as any node has signalled ready, kick that node
    let mut provider = ENDPOINT_MANAGER
        .get_provider(testnet.chain_name.clone())
        .expect("Error retrieving provider - check name and yaml.");
    let provider_mut = Arc::make_mut(&mut provider);
    let provider = Arc::new(provider_mut.set_interval(Duration::from_millis(10)).clone());
    let sk = SigningKey::from_bytes(
        testnet
            .deploy_account
            .staker_address_private_key
            .as_bytes()
            .into(),
    )
    .unwrap();
    let wallet = LocalWallet::from(sk).with_chain_id(testnet.chain_id);
    let staking_clone = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
        validator_collection.actions().contracts().staking.address(),
        Arc::new(SignerMiddleware::new(provider, wallet)),
    );
    let handle = tokio::spawn(async move {
        // Subscribe to staking events
        let events = staking_clone.events();
        let mut stream = events.stream().await.unwrap();
        while let Some(Ok(event)) = stream.next().await {
            match event {
                StakingEvents::ReadyForNextEpochFilter(ready_event) => {
                    info!("Received ReadyForNextEpochFilter event: {:?}", ready_event);

                    //let staker_address_to_kick = ready_event.staker;
                    //FIXME:  we're doing this to avoid an incompatibility with Cait-Sith.
                    let staker_address_to_kick = node_staker_address_to_kick;
                    // kick this validator

                    let func =
                        staking_clone.admin_kick_validator_in_next_epoch(staker_address_to_kick);
                    let func_with_gas = func
                        .gas(U256::from(500000))
                        .gas_price(U256::from(1073088832));

                    let tx = func_with_gas.send().await.unwrap();
                    match tx.await {
                        Ok(receipt) => {
                            info!(
                                "Successfully kicked validator: {:?} - receipt: {:?}",
                                staker_address_to_kick, receipt
                            );
                        }
                        Err(e) => {
                            info!("Failed to kick validator: {:?}", e);
                        }
                    }

                    // confirm that the retry counter has increased
                    let retries_after_kicking =
                        staking_clone.epoch(realm_id).call().await.unwrap().retries;
                    assert!(
                        retries_after_kicking == U256::from(1),
                        "Retries after kicking should be 1 but it is {:?}",
                        retries_after_kicking,
                    );

                    // confirm that a node was kicked
                    let validator_count = staking_clone
                        .get_validators_in_next_epoch(realm_id)
                        .call()
                        .await
                        .unwrap()
                        .len();
                    assert!(
                        validator_count == NUM_NODES - 1,
                        "There should be 1 less validator in the next epoch since we kicked one.  But there are {} nodes",
                        validator_count
                    );

                    break;
                }
                _ => {
                    info!("Received event: {:?}", event);
                }
            }
        }
    });

    validator_collection.actions().sleep_millis(2000).await;
    let actions_clone = validator_collection.actions().clone();
    let advance_handle = tokio::spawn(async move {
        // advance timestamp so that nodes can lock and advance
        actions_clone.increase_blockchain_timestamp(300).await;
        info!("Increased blockchain timestamp by 300 seconds");
    });

    join_all(vec![handle, advance_handle]).await;

    validator_collection
        .actions()
        .wait_for_active(realm_id)
        .await;
    let epoch = validator_collection
        .actions()
        .contracts()
        .staking
        .epoch(realm_id)
        .call()
        .await
        .unwrap();
    info!("Network is active again.  Epoch is {}", epoch.number);
    // sleep for a few secs so the nodes get their peers again
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    // how many nodes are in the current set?
    let current_validators = validator_collection
        .actions()
        .get_current_validator_structs(realm_id)
        .await;
    info!("Current validators length: {:?}", current_validators.len());
    assert!(
        current_validators.len() == NUM_NODES - 1,
        "There should be 1 less validator in the current node set since we kicked one but there are {} nodes",
        current_validators.len(),
    );
    let next_validators = validator_collection
        .actions()
        .get_next_validators(realm_id)
        .await;
    info!("Next validators length: {:?}", next_validators.len());
    assert!(
        current_validators.len() == NUM_NODES - 1,
        "There should be 1 less validator in the next node set since we kicked one"
    );
    network_checker.check(&validator_collection, &vec![]).await;
}

/// This test is used to test when a staked node never wakes up, which results in it getting kicked
/// before the rest of the network successfully advances to the new epoch.
#[tokio::test]
async fn one_node_never_wakes() {
    setup_logging();

    info!("TEST: one_node_never_wakes");

    // Start the node collection
    let num_nodes = 6;
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(num_nodes)
        .force_deploy(true)
        .build()
        .await;

    let mut complaint_reason_to_config = HashMap::<U256, ComplaintConfig>::new();
    complaint_reason_to_config.insert(
        U256::from(Issue::Unresponsive.value()),
        ComplaintConfig::builder().tolerance(U256::from(3)).build(),
    );

    let _testnet_contracts = Testnet::setup_contracts(
        &mut testnet,
        None,
        Some(
            StakingContractRealmConfig::builder()
                .complaint_reason_to_config(complaint_reason_to_config)
                .realm_id(U256::from(1))
                .build(),
        ),
    )
    .await
    .expect("Failed to setup contracts");

    let validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(num_nodes)
        .num_asleep_initially(1)
        .wait_for_root_keys(false)
        .wait_initial_epoch(false)
        .build(&testnet)
        .await
        .expect("Failed to build validator collection");

    info!("Waiting for sleeping node to get kicked");
    let validator_asleep = *validator_collection
        .asleep_nodes()
        .first()
        .expect("No node is asleep");

    let realm_id = U256::from(1);
    let staker_address_to_kick = validator_asleep.account().staker_address;
    let epoch_number = validator_collection
        .actions()
        .get_current_epoch(realm_id)
        .await;

    info!("Sleeping validator: {:?}", staker_address_to_kick);

    let voting_status = validator_collection
        .actions()
        .wait_for_voting_status_to_kick_validator(
            realm_id,
            epoch_number,
            staker_address_to_kick,
            H160::random(), // For simplicity, we only care about asserting the number of votes.
            1, // In the genesis epoch, the number of votes required to kick a node is 1.
            true,
        )
        .await;
    assert!(voting_status.is_ok());

    // Assert that the node that was asleep got kicked
    let voting_status = voting_status.unwrap();
    info!("Voting status: {:?}", voting_status);
    assert!(voting_status.votes.as_usize() >= 1);

    // After the node is kicked, wait for the DKG to complete
    validator_collection
        .actions()
        .wait_for_initial_epoch(realm_id)
        .await;

    // Assert that the current validator set is 1 less.
    let current_validators = validator_collection
        .actions()
        .get_current_validators(realm_id)
        .await;
    info!("Current validators: {:?}", current_validators);

    assert_eq!(current_validators.len(), num_nodes - 1);
}

/// This tests that a node goes offline and online again and can resume operation with the rest of the network of nodes.
#[tokio::test]
async fn node_restarts() {
    setup_logging();

    info!("TEST: node_restarts ( 6 nodes )");

    let num_nodes = 6;
    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(num_nodes)
        .chain_polling_interval("5000".to_string())
        .build()
        .await;

    let actions = testnet.actions();

    // Run network checks
    let network_checker = NetworkIntegrityChecker::new(&end_user, &actions).await;
    network_checker.check(&validator_collection, &vec![]).await;

    let random_node_idx_to_shutdown = validator_collection
        .stop_random_node()
        .await
        .expect("Failed to stop random node");

    // Wait for 5 seconds before starting the node back up.  the node should not be kicked, since this is shorter than the complaint tolerance.
    info!(
        "Waiting for 5 seconds before re-starting the offline node at idx {:?}",
        random_node_idx_to_shutdown
    );
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Start the node back up
    info!("Restarting the shutdown node");
    validator_collection
        .start_node(random_node_idx_to_shutdown)
        .await
        .expect("Failed to start node");
    info!("Node restarted");

    // give the node like, 5 seconds to come back online
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Run network checks
    network_checker.check(&validator_collection, &vec![]).await;
    info!("Network integrity check passed after node came back online");

    // Fast forward time to allow nodes to start a DKG to advance to the next epoch.
    actions.increase_blockchain_timestamp(300).await;

    // After next epoch arrives, run interpolation and decryption tests.
    let realm_id = U256::from(1);
    actions.wait_for_lock(realm_id).await;
    actions.wait_for_active(realm_id).await;
    network_checker.check(&validator_collection, &vec![]).await;
}

/// This tests that a node registers an invalid port and gets kicked.
#[test_case(1; "with invalid port")]
/// This tests that a node registers the same IP and port as another node and gets kicked.
#[test_case(2; "with same IP and port")]
#[tokio::test]
async fn one_node_conflicting_networking_info(test_case: usize) {
    setup_logging();

    info!("TEST: one_node_conflicting_networking_info");

    let num_nodes = 6;
    let (random_node_idx_to_be_kicked, new_staked_port_of_node_to_be_kicked) = match test_case {
        1 => {
            // Choose a random node index to stake with an invalid port.
            let random_node_idx = choose_random_indices(num_nodes, 1)
                .iter()
                .cloned()
                .collect::<Vec<usize>>()[0];

            (random_node_idx, "5555".to_owned())
        }
        2 => {
            // Randomly choose an impersonator and a victim.
            let random_node_indices: Vec<usize> = choose_random_indices(num_nodes, 2)
                .iter()
                .cloned()
                .collect();
            let random_node_idx_impersonater = random_node_indices[0];
            let random_node_idx_impersonated = random_node_indices[1];
            let default_ip_addresses = default_staker_ip_addresses(7470, num_nodes);
            let new_port_of_impersonator = default_ip_addresses[random_node_idx_impersonated]
                .split(':')
                .collect::<Vec<&str>>()[1];

            (
                random_node_idx_impersonater,
                new_port_of_impersonator.to_owned(),
            )
        }
        _ => panic!("Invalid test case"),
    };

    // Start the node collection
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(num_nodes)
        .staker_account_setup_mapper(Box::new(move |args: (usize, NodeAccount, Contracts)| {
            let random_node_idx_to_be_kicked_clone = random_node_idx_to_be_kicked;
            let new_staked_port_of_node_to_be_kicked_clone =
                new_staked_port_of_node_to_be_kicked.clone();

            Box::pin(async move {
                if args.0 == random_node_idx_to_be_kicked_clone {
                    // Send a TX to chain to update the staker information with an invalid port.
                    let staker_provider = args.1.signing_provider;
                    let staking = Staking::<
                        SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
                    >::new(
                        args.2.staking.address(), staker_provider.clone()
                    );

                    let validator: Validator = staking
                        .validators(args.1.staker_address)
                        .call()
                        .await
                        .expect("Failed to get staker config");
                    let new_staked_port = new_staked_port_of_node_to_be_kicked_clone
                        .parse::<u32>()
                        .expect("Failed to parse port");
                    let update_cc = staking.set_ip_port_node_address(
                        validator.ip,
                        validator.ipv_6,
                        new_staked_port,
                        validator.node_address,
                    );

                    Contracts::process_contract_call(
                        update_cc,
                        "setting staker port to invalid port",
                    )
                    .await;

                    info!(
                        "Successfully updated staker ({}) {:?}: port {:?} -> {:?}",
                        args.0, args.1.staker_address, validator.port, new_staked_port
                    );
                }

                Ok(())
            }) as BoxFuture<'static, Result<(), anyhow::Error>>
        }))
        .build()
        .await;

    let _testnet_contracts = Testnet::setup_contracts(&mut testnet, None, None)
        .await
        .expect("Failed to setup contracts");

    let validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(num_nodes)
        .wait_for_root_keys(false)
        .wait_initial_epoch(false)
        .build(&testnet)
        .await
        .expect("Failed to build validator collection");

    let actions = testnet.actions();

    // Update complaint config to be 3 to speed up test.
    // This is intentionally not set before the validator collection is built since only 1 vote is
    // needed to kick a node in the genesis epoch, and we don't want complaints due to each node being
    // spun up during building the validator collection.
    info!("Updating complaint config to be 3 to speed up test.");
    let mut complaint_reason_to_config = HashMap::<U256, ComplaintConfig>::new();
    complaint_reason_to_config.insert(
        U256::from(Issue::Unresponsive.value()),
        ComplaintConfig::builder().tolerance(U256::from(3)).build(),
    );
    complaint_reason_to_config.insert(
        U256::from(Issue::IncorrectInfo.value()),
        ComplaintConfig::builder().tolerance(U256::from(3)).build(),
    );

    let realm_id = U256::from(1);

    assert!(
        actions
            .update_staking_realm_config(
                StakingContractRealmConfig::builder()
                    .complaint_reason_to_config(complaint_reason_to_config)
                    .realm_id(realm_id)
                    .build()
            )
            .await
            .is_ok()
    );

    info!(
        "Waiting for node {} to be kicked",
        random_node_idx_to_be_kicked
    );
    let staker_address_to_kick = testnet.node_accounts[random_node_idx_to_be_kicked].staker_address;
    let epoch_number = actions.get_current_epoch(realm_id).await;
    let voting_status = actions
        .wait_for_voting_status_to_kick_validator(
            realm_id,
            epoch_number,
            staker_address_to_kick,
            H160::random(), // For simplicity, we only care about asserting the number of votes.
            1, // In the genesis epoch, the number of votes required to kick a node is 1.
            true,
        )
        .await;
    assert!(voting_status.is_ok());

    // After the node is kicked, wait for the DKG to complete
    let epoch_number = actions.get_current_epoch(realm_id).await;
    actions.wait_for_epoch(realm_id, epoch_number + 1).await;

    // Assert that the current validator set is 1 less.
    let current_validators = actions.get_current_validators(realm_id).await;
    info!("Current validators: {:?}", current_validators);
    assert_eq!(current_validators.len(), num_nodes - 1);

    let validator_epochs = validator_collection.get_validator_epochs().await;
    info!("Validator epochs: {:?}", validator_epochs);

    let mut end_user = EndUser::new(&testnet);
    end_user.fund_wallet_default_amount().await;
    let _pkp_info = end_user.new_pkp().await;

    // Run network checks
    let network_checker = NetworkIntegrityChecker::new(&end_user, &testnet.actions()).await;
    network_checker.check(&validator_collection, &vec![]).await;
}

#[tokio::test]
async fn node_restarts_without_key_material() {
    setup_logging();

    info!("TEST: node_restarts_without_key_material");

    let num_nodes = 6;
    let realm_id = U256::from(1);
    let epoch_length = 1800;
    // set epoch length to 30 mins so it never elapses unless we advance the clock
    let (testnet, mut validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(num_nodes)
        .epoch_length(epoch_length)
        .build()
        .await;

    let actions = testnet.actions();

    // Lower the configured interval for complaints to reduce possibility of any kicks.
    info!("Lowering the complaint interval to 15s for all complaints");
    for i in 1..=MAX_COMPLAINT_REASON_VALUE {
        actions
            .set_complaint_reason_config(
                U256::from(i),
                staking::ComplaintConfig {
                    tolerance: U256::from(10),
                    interval_secs: U256::from(15),
                    kick_penalty_percent: ethers::utils::parse_ether("0.1").unwrap(), // 0.1 ether = 10%
                    kick_penalty_demerits: U256::from(10),
                },
            )
            .await
            .unwrap_or_else(|_| panic!("Failed to set complaint config for reason {}", i));
    }

    let network_checker = NetworkIntegrityChecker::new(&end_user, &actions).await;
    let starting_epoch = actions.get_current_epoch(realm_id).await;

    // Get random idx within the node collection
    let random_peer_to_shutdown = get_random_peer_within_deterministic_subset(&actions)
        .await
        .expect("Failed to get random node peer within deterministic subset");

    let random_peer_port = random_peer_to_shutdown
        .socket_address
        .split(':')
        .collect::<Vec<_>>()[1];
    validator_collection
        .stop_node_by_port(random_peer_port)
        .await
        .expect("Failed to stop node");

    // Remove node keys
    remove_node_keys(format!(
        "0x{}",
        bytes_to_hex(random_peer_to_shutdown.staker_address.as_bytes())
    ));

    // Start the node back up. Due to the node not having any key material, the node will
    // crash from not being able to register the new attested wallet.
    info!("Restarting the shutdown node");
    let start_res = validator_collection
        .start_node_by_port(random_peer_port)
        .await;
    assert!(start_res.is_err());

    // Wait for complaint_interval_secs to pass for the complaints cache to clear so that
    // we can test for the node being kicked for being unresponsive.
    actions
        .wait_for_complaint_cache_to_clear()
        .await
        .expect("Failed to wait for complaint cache to clear");

    // Use a sufficiently long complaint interval to mark the node as non-participating.
    actions
        .set_complaint_reason_config(
            U256::from(Issue::Unresponsive.value()),
            staking::ComplaintConfig {
                tolerance: U256::from(2),
                interval_secs: U256::from(300),
                kick_penalty_percent: ethers::utils::parse_ether("0.1").unwrap(), // 0.1 ether = 10%
                kick_penalty_demerits: U256::from(10),
            },
        )
        .await
        .expect("Failed to set complaint config");

    // Wait until the node is kicked.
    info!("Waiting for node to get kicked");
    let voting_status = actions
        .wait_for_voting_status_to_kick_validator(
            realm_id,
            starting_epoch,
            random_peer_to_shutdown.staker_address,
            H160::random(), // For simplicity, we only care about asserting the number of votes.
            get_threshold_count(validator_collection.size()),
            true,
        )
        .await;
    assert!(voting_status.is_ok());

    info!("Fast forwarding time to allow nodes to start a DKG to advance to the next epoch.");
    // Fast forward time to allow nodes to start a DKG to advance to the next epoch.
    actions.increase_blockchain_timestamp(epoch_length).await;

    // After next epoch arrives, run network tests.
    actions.wait_for_epoch(realm_id, starting_epoch + 1).await;
    network_checker.check(&validator_collection, &vec![]).await;
}

#[tokio::test]
async fn register_attested_wallet() {
    setup_logging();

    info!("TEST: register_attested_wallet");

    // Start a new node collection. No need to wait for DKG to complete.
    let realm_id = U256::from(1);
    const NUM_NODES: usize = 3;
    let mut testnet = Testnet::builder()
        .num_staked_and_joined_validators(NUM_NODES)
        .force_deploy(true)
        .build()
        .await;

    let _testnet_contracts = Testnet::setup_contracts(
        &mut testnet,
        None,
        Some(
            StakingContractRealmConfig::builder()
                .epoch_length(Some(U256::from(300)))
                .realm_id(U256::from(1))
                .build(),
        ),
    )
    .await
    .expect("Failed to setup contracts");

    let actions = testnet.actions();

    // Assert that the node addresses and operator addresses are the same.
    let next_validator_structs = actions.get_next_validator_structs(realm_id).await;
    info!("Next validator structs: {:?}", next_validator_structs);
    for validator_struct in next_validator_structs {
        assert_eq!(
            validator_struct.node_address,
            validator_struct.operator_address
        );
    }

    let _validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(NUM_NODES)
        .build(&testnet)
        .await
        .expect("Failed to build validator collection");

    // Assert that the node addresses and operator addresses are now different.
    let next_validator_structs = actions.get_next_validator_structs(realm_id).await;
    for validator_struct in next_validator_structs {
        assert_ne!(
            validator_struct.node_address,
            validator_struct.operator_address
        );
    }
}
