pub mod context;
pub mod contract_helper;
pub mod datetime;
pub mod polling;
pub mod rpc_calls;
pub mod sdk_models;
pub mod table_classes;

use crate::models::{GlobalState, NetworkConfig};
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{H160, U256};
use ethers::utils::keccak256;
use leptos::context::use_context;
use leptos::prelude::*;
use lit_blockchain_lite::contracts;
use std::error::Error;

pub fn set_header(header: &str) {
    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    gs.page_header.set(header.to_string());
}

pub async fn get_hex_encoded_address(contract_name: &str) -> Result<String, Box<dyn Error>> {
    let address = get_address(contract_name).await?;
    Ok(format!("0x{}", hex::encode(address)))
}

pub async fn get_address(contract_name: &str) -> Result<H160, Box<dyn Error>> {
    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    get_address_with_network(&gs.active_network(), contract_name).await
}

pub async fn get_address_with_network(
    network: &NetworkConfig,
    contract_name: &str,
) -> Result<H160, Box<dyn Error>> {
    let resolver_contract_address = network.resolver_contract.replace("0x", "");
    let address = &hex::decode(resolver_contract_address).unwrap();
    let address = ethers::types::H160::from_slice(address.as_slice());
    let cfg = &get_lit_config_with_network(&network);
    let resolver = contracts::contract_resolver::ContractResolver::node_monitor_load(cfg, address)?;
    let env = network.environment;
    let typ = keccak256(contract_name.as_bytes());

    let address = resolver.get_contract(typ, env).call().await?;
    Ok(address)
}

pub fn get_lit_config() -> contracts::NodeMonitorLitConfig {
    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    let network = gs.active_network();
    get_lit_config_with_network(&network)
}

pub fn get_lit_config_with_network(network: &NetworkConfig) -> contracts::NodeMonitorLitConfig {
    contracts::NodeMonitorLitConfig {
        blockchain_chain_id: 1,
        rpc_url: network.chain_url.clone(),
        wallet_key: None,
    }
}

pub fn poll_network() {
    let _data = LocalResource::new(do_poll);
}

pub async fn do_poll() {
    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    let _realm_id = ethers::types::U256::from(1);

    let address = match get_address(crate::contracts::STAKING_CONTRACT).await {
        Ok(address) => address,
        Err(e) => {
            log::warn!("Error getting staking contract address: {:?}", e);
            return;
        }
    };
    let cfg = &get_lit_config();
    let staking = crate::contracts::staking::Staking::node_monitor_load(cfg, address).unwrap();

    let epoch_details = staking.epoch(U256::from(1)).call().await.unwrap();
    let number = epoch_details.number;
    let network_state = match staking.state(U256::from(1)).call().await.unwrap() {
        0 => "Active".to_string(),
        1 => "NextValidatorSetLocked".to_string(),
        2 => "ReadyForNextEpoch".to_string(),
        3 => "Unlocked".to_string(),
        4 => "Paused".to_string(),
        5 => "Restore".to_string(),
        _ => "Unknown".to_string(),
    };

    use std::convert::TryFrom;
    let provider = Provider::<Http>::try_from(gs.active_network().chain_url.as_str())
        .expect("could not instantiate HTTP Provider");
    let block_number = provider.get_block_number().await.unwrap();
    let block_number = block_number.as_u64();

    gs.network_state.get()[0].epoch.set(number.as_u64());
    gs.network_state.get()[0].network_state.set(network_state);
    gs.block.set(block_number);
}

pub fn make_nav_url(resource_url: &str) -> String {
    format!("{}{}", base_path(), resource_url)
}

pub fn base_path() -> &'static str {
    #[cfg(feature = "naga-test")]
    return "/monitor/naga-test";
    #[cfg(feature = "naga-prod")]
    return "/monitor/naga-prod";
    #[cfg(feature = "naga-dev")]
    return "/monitor/naga-dev";
    #[cfg(feature = "naga-staging")]
    return "/monitor/naga-staging";
    #[cfg(feature = "internalDev")]
    return "/monitor/internalDev";

    ""
}
