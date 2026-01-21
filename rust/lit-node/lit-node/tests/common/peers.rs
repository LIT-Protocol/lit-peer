use anyhow::Result;
use ethers::types::U256;
use lit_node::peers::peer_state::models::SimplePeerCollection;
use lit_node::{config::chain::ChainDataConfigManager, peers::peer_state::models::SimplePeer};
use lit_node_core::PeerId;
use lit_node_testnet::testnet::actions::Actions;
use lit_node_testnet::validator::ValidatorCollection;
use rand::Rng;
use semver::Version;
use tracing::debug;

/// This function returns a random `node_collection` index within the deterministic subset.
pub async fn get_random_peer_within_deterministic_subset(actions: &Actions) -> Result<SimplePeer> {
    // Get the sorted peers from chain.
    let sorted_validators = get_sorted_peers(actions, U256::from(1))
        .await
        .map_err(|e| anyhow::anyhow!("failed to get sorted peers: {:?}", e))?;

    // Get a random address within the deterministic subset.
    let mut rng = rand::thread_rng();
    let random_idx = rng.gen_range(0..sorted_validators.len());
    let random_peer = sorted_validators
        .get(random_idx)
        .expect("failed to get random peer");
    debug!(
        "Random peer within deterministic subset chosen: {:?}",
        random_peer
    );

    Ok(random_peer.to_owned())
}

pub async fn get_sorted_peers(actions: &Actions, realm_id: U256) -> Result<Vec<SimplePeer>> {
    // Get the validators in the current epoch.
    let current_validators = actions.get_current_validator_structs(realm_id).await;
    let node_addresses = current_validators
        .iter()
        .map(|v| v.node_address)
        .collect::<Vec<_>>();
    // Get the kicked validators
    let kicked_validators = actions
        .contracts()
        .staking
        .get_kicked_validators(realm_id)
        .await
        .map_err(|e| anyhow::anyhow!("failed to get kicked validators: {:?}", e))?;

    // Get the address mapping.
    let address_mapping = actions
        .contracts()
        .staking
        .get_node_staker_address_mappings(node_addresses)
        .call()
        .await
        .expect("failed to get node staker address mappings");

    // Get the sorted peers.
    let sorted_validators = ChainDataConfigManager::sort_and_filter_validators(
        current_validators,
        kicked_validators,
        address_mapping,
        realm_id,
    );

    // Map into peer socket addresses.
    Ok(sorted_validators.iter().map(SimplePeer::from).collect())
}

pub async fn get_simple_peer_collection(
    validator_collection: &ValidatorCollection,
    realm_id: U256,
) -> Result<SimplePeerCollection> {
    let mut peers = SimplePeerCollection::default();
    let addresses = validator_collection
        .validators()
        .iter()
        .map(|v| v.account().staker_address)
        .collect();
    let mappings = validator_collection
        .actions()
        .get_node_attested_pubkey_mappings(&addresses)
        .await?;

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
            peer_id: PeerId::from_slice(wallet_public_key_bytes.as_slice())?,
            staker_address: validator.account().staker_address,
            key_hash: 0,
            kicked: false,
            version: Version::new(1, 0, 0),
            realm_id,
        });
    }
    Ok(peers)
}
