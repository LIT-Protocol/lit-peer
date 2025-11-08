use blsful::inner_types::{Group, GroupEncoding};
use lit_node_testnet::{
    end_user::EndUser,
    testnet::{NodeAccount, Testnet, WhichTestnet, contracts::StakingContractRealmConfig},
    validator::ValidatorCollection,
};

use crate::common::{assertions::NetworkIntegrityChecker, setup_logging};

use ethers::types::U256;
use lit_core::utils::binary::bytes_to_hex;
use lit_node::common::key_helper::KeyCache;
use lit_node::peers::peer_state::models::{SimplePeer, SimplePeerCollection};
use lit_node::tss::common::key_persistence::KeyPersistence;
use lit_node::tss::common::key_share_commitment::KeyShareCommitments;
use lit_node::tss::common::storage::read_key_share_commitments_from_disk;
use lit_node_core::{CompressedBytes, CurveType, PeerId};
use network_state::{NetworkState, get_next_random_network_state};
use semver::Version;
use tracing::info;

fn setup() {
    setup_logging();
}

#[tokio::test]
async fn test_many_epochs() {
    setup();

    info!("TEST: test_many_epochs");

    const NUM_EPOCHS: usize = 15;
    const MAX_VALIDATORS: usize = 12;
    const MIN_VALIDATORS: usize = 5;
    const INITIAL_VALIDATORS: usize = 7;
    const EPOCH_LENGTH: u64 = 3000;

    // Generate test plan
    let mut test_plan: Vec<NetworkState> = Vec::new();
    // Seed the first network state
    test_plan.push(NetworkState::new(2, INITIAL_VALIDATORS, 0, 0));
    for idx in 1..NUM_EPOCHS {
        let prev_validator_count = test_plan[idx - 1].get_validator_count();
        let network_state = get_next_random_network_state(
            MIN_VALIDATORS,
            MAX_VALIDATORS,
            prev_validator_count,
            test_plan[idx - 1].get_epoch_number() + 1,
        );
        test_plan.push(network_state);
    }

    // Print the test plan
    info!("Test plan:");
    for network_state in &test_plan {
        info!("{}", network_state);
    }

    // Setup the network
    let mut testnet = Testnet::builder()
        .which_testnet(WhichTestnet::Anvil)
        .num_staked_and_joined_validators(INITIAL_VALIDATORS)
        .num_staked_only_validators((MAX_VALIDATORS * 2) - INITIAL_VALIDATORS)
        .build()
        .await;

    info!("Setting up contracts");
    let _testnet_contracts = Testnet::setup_contracts(
        &mut testnet,
        None,
        Some(
            StakingContractRealmConfig::builder()
                .epoch_length(Some(U256::from(EPOCH_LENGTH)))
                .max_presign_count(U256::from(0))
                .min_presign_count(U256::from(0))
                .build(),
        ),
    )
    .await
    .expect("Failed to setup contracts");

    let actions = testnet.actions();

    info!("Building validator collection");
    let mut validator_collection = ValidatorCollection::builder()
        .num_staked_nodes(MAX_VALIDATORS * 2) // this is doubled since the entire set can request to leave and a new one requests to join
        // explicitly indicate that the indices between INITIAL_VALIDATORS and (MAX_VALIDATORS * 2) are asleep as a vec
        .asleep_initially_override(Some((INITIAL_VALIDATORS..(MAX_VALIDATORS * 2)).collect()))
        .build(&testnet)
        .await
        .expect("Failed to build validator collection");

    info!(
        "Validator collection: {:?}",
        validator_collection.addresses()
    );

    let mut end_user = EndUser::new(&testnet);
    end_user.fund_wallet_default_amount().await;
    end_user.new_pkp().await.expect("Failed to mint PKP");

    let network_checker = NetworkIntegrityChecker::new(&end_user, &actions).await;

    info!("Network is set up, time to start the test plan");
    let realm_id = U256::from(1);
    for i in 1..test_plan.len() {
        let start_time = std::time::Instant::now();
        let next_network_state = &test_plan[i];
        info!(
            "Transitioning from [{}] to [{}]",
            test_plan[i - 1],
            next_network_state
        );

        // Request to leave
        info!("Validators are requesting to leave");
        let node_accounts_requested_to_leave: Vec<NodeAccount> = {
            let validators = validator_collection
                .random_validators_request_to_leave(next_network_state.get_validators_dealt_out())
                .await
                .expect("Failed to request to leave");
            validators.iter().map(|v| v.account().clone()).collect()
        };

        // Request to join
        info!("Validators are requesting to join");
        validator_collection
            .random_validators_request_to_join(next_network_state.get_validators_dealt_in(), 1)
            .await
            .expect("Failed to request to join");

        // Fast forward time so nodes can lock, DKG and advance
        info!("Fast forwarding time");
        actions
            .increase_blockchain_timestamp(EPOCH_LENGTH as usize)
            .await;

        // Wait until network has advanced to the next epoch
        info!(
            "Waiting for epoch to advance to {}",
            next_network_state.get_epoch_number()
        );
        actions
            .wait_for_epoch(realm_id, U256::from(next_network_state.get_epoch_number()))
            .await;

        // Assert the new network state is as expected
        info!("Checking network state");
        assert_eq!(
            actions.get_current_validator_count(realm_id).await as usize,
            next_network_state.get_validator_count()
        );

        network_checker.check(&validator_collection, &vec![]).await;

        // Wait until the round timeout has passed so that ongoing TSS operations can finish before shutting down nodes.
        info!("Waiting for some time before shutting down nodes");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        // Shutdown the nodes that have requested to leave
        info!("Shutting down nodes that have requested to leave");
        validator_collection
            .stop_nodes_with_accounts(node_accounts_requested_to_leave)
            .expect("Failed to stop nodes");

        let duration = start_time.elapsed();
        info!(
            "Time taken for epoch {} to complete: {:?} seconds.",
            next_network_state.get_epoch_number(),
            duration.as_secs()
        );
    }

    // Check that lingering key shares have been removed
    let mut peers = SimplePeerCollection::default();
    let addresses = validator_collection
        .validators()
        .iter()
        .map(|v| v.account().staker_address)
        .collect();
    let mappings = actions
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
    for curve_type in CurveType::into_iter() {
        let root_keys = actions.get_root_keys(curve_type as u8, None).await.unwrap();
        for pub_key in &root_keys {
            match curve_type {
                CurveType::BLS | CurveType::BLS12381G1 => {
                    check_for_lingering_keys::<blsful::inner_types::G1Projective>(
                        curve_type,
                        pub_key,
                        &peers,
                        realm_id.as_u64(),
                    )
                    .await;
                }
                CurveType::K256 => {
                    check_for_lingering_keys::<k256::ProjectivePoint>(
                        curve_type,
                        pub_key,
                        &peers,
                        realm_id.as_u64(),
                    )
                    .await;
                }
                CurveType::P256 => {
                    check_for_lingering_keys::<p256::ProjectivePoint>(
                        curve_type,
                        pub_key,
                        &peers,
                        realm_id.as_u64(),
                    )
                    .await;
                }
                CurveType::P384 => {
                    check_for_lingering_keys::<p384::ProjectivePoint>(
                        curve_type,
                        pub_key,
                        &peers,
                        realm_id.as_u64(),
                    )
                    .await;
                }
                CurveType::Ed25519 => {
                    check_for_lingering_keys::<vsss_rs::curve25519::WrappedEdwards>(
                        curve_type,
                        pub_key,
                        &peers,
                        realm_id.as_u64(),
                    )
                    .await;
                }
                CurveType::Ristretto25519 => {
                    check_for_lingering_keys::<vsss_rs::curve25519::WrappedRistretto>(
                        curve_type,
                        pub_key,
                        &peers,
                        realm_id.as_u64(),
                    )
                    .await;
                }
                CurveType::Ed448 => {
                    check_for_lingering_keys::<ed448_goldilocks::EdwardsPoint>(
                        curve_type,
                        pub_key,
                        &peers,
                        realm_id.as_u64(),
                    )
                    .await;
                }
                CurveType::RedJubjub => {
                    check_for_lingering_keys::<jubjub::SubgroupPoint>(
                        curve_type,
                        pub_key,
                        &peers,
                        realm_id.as_u64(),
                    )
                    .await;
                }
                CurveType::RedDecaf377 => {
                    check_for_lingering_keys::<decaf377::Element>(
                        curve_type,
                        pub_key,
                        &peers,
                        realm_id.as_u64(),
                    )
                    .await;
                }
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
    let key_helper = KeyPersistence::<G>::new(curve_type);
    let cache = KeyCache::default();
    for peer in &peers.0 {
        let staker_address = format!("0x{}", bytes_to_hex(peer.staker_address.0));
        for epoch in 1..=13 {
            let res = key_helper
                .read_key(
                    pub_key,
                    &peer.peer_id,
                    epoch,
                    &staker_address,
                    realm_id,
                    &cache,
                )
                .await;
            assert!(res.is_err());

            let res = read_key_share_commitments_from_disk::<KeyShareCommitments<G>>(
                curve_type,
                pub_key,
                &staker_address,
                &peer.peer_id,
                epoch,
                realm_id,
                &cache,
            )
            .await;
            assert!(res.is_err());
        }
        for epoch in 14..=5 {
            let res = key_helper
                .read_key(
                    pub_key,
                    &peer.peer_id,
                    epoch,
                    &staker_address,
                    realm_id,
                    &cache,
                )
                .await;
            assert!(res.is_ok());

            let res = read_key_share_commitments_from_disk::<KeyShareCommitments<G>>(
                curve_type,
                pub_key,
                &staker_address,
                &peer.peer_id,
                epoch,
                realm_id,
                &cache,
            )
            .await;
            assert!(res.is_ok());
        }
    }
}

mod network_state {
    use lit_node::utils::consensus::get_threshold_count;
    use rand::Rng;
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct NetworkState {
        epoch_number: usize,
        validator_count: usize,
        validators_dealt_in: usize,
        validators_dealt_out: usize,
    }

    impl NetworkState {
        pub fn new(
            epoch_number: usize,
            validator_count: usize,
            validators_dealt_in: usize,
            validators_dealt_out: usize,
        ) -> Self {
            Self {
                epoch_number,
                validator_count,
                validators_dealt_in,
                validators_dealt_out,
            }
        }

        pub fn get_epoch_number(&self) -> usize {
            self.epoch_number
        }

        pub fn get_validator_count(&self) -> usize {
            self.validator_count
        }

        pub fn get_validators_dealt_in(&self) -> usize {
            self.validators_dealt_in
        }

        pub fn get_validators_dealt_out(&self) -> usize {
            self.validators_dealt_out
        }

        pub fn get_threshold(&self) -> usize {
            get_threshold_count(self.validator_count)
        }
    }

    impl fmt::Display for NetworkState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Epoch: {}, Validators: {} (+{}/-{}), Threshold: {}",
                self.epoch_number,
                self.validator_count,
                self.validators_dealt_in,
                self.validators_dealt_out,
                self.get_threshold()
            )
        }
    }

    /// Given the previous validator count, randomize the next valid network state. The priority is to
    /// find (interesting) dealt_in / dealt_out combinations before checking whether it is a valid change.
    pub fn get_next_random_network_state(
        minimum_validators: usize,
        maximum_validators: usize,
        prev_validator_count: usize,
        epoch_number: usize,
    ) -> NetworkState {
        let rng = &mut lit_node_testnet::rand::thread_rng();

        let mut valid_dealt_in_and_out;
        loop {
            // We want to make sure that we're not dealing out such that < threshold of current validators remain in the new epoch!
            let validators_dealt_out = rng
                .gen_range(0..=(prev_validator_count - get_threshold_count(prev_validator_count)));
            let validators_dealt_in = rng
                .gen_range(0..(maximum_validators - prev_validator_count + validators_dealt_out));

            // The change is valid if the new validator count is within the bounds, inclusive.
            let new_validator_count =
                prev_validator_count + validators_dealt_in - validators_dealt_out;
            valid_dealt_in_and_out = new_validator_count >= minimum_validators
                && new_validator_count <= maximum_validators;
            if valid_dealt_in_and_out {
                return NetworkState::new(
                    epoch_number,
                    new_validator_count,
                    validators_dealt_in,
                    validators_dealt_out,
                );
            }
        }
    }
}
