use crate::models::GlobalState;
use crate::utils::{get_address, get_lit_config};
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::U256;
use leptos::context::use_context;
use leptos::prelude::*;

pub fn poll_network(realm_id: u64) {
    let _data = LocalResource::new(move || do_poll(realm_id));
}

pub async fn do_poll(realm_id: u64) {
    let mut gs = use_context::<GlobalState>().expect("Global State Failed to Load");

    let address = get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .unwrap();
    let cfg = &get_lit_config();
    let staking = crate::contracts::staking::Staking::node_monitor_load(cfg, address).unwrap();

    let epoch_details = staking.epoch(U256::from(realm_id)).call().await.unwrap();
    let number = epoch_details.number;
    let network_state = match staking.state(U256::from(realm_id)).call().await.unwrap() {
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

    let index = gs.index_for_realm_id(realm_id);

    log::info!(
        "Polling Network | realm_id: {}, index: {}, epoch: {}, network_state: {}, block: {}",
        realm_id,
        index,
        number,
        network_state,
        block_number
    );

    gs.network_state.get()[index].epoch.set(number.as_u64());
    gs.network_state.get()[index]
        .network_state
        .set(network_state);
    gs.block.set(block_number);
}
