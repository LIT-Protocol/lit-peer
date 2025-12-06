use std::{collections::HashMap, net::Ipv4Addr};

use crate::{
    models::GlobalState,
    utils::{get_address, get_lit_config},
};
use chrono::{NaiveDateTime, TimeDelta};
use ethers::types::H160;
use leptos::prelude::*;
use lit_blockchain_lite::contracts::staking::{AddressMapping, Validator};

#[component]
pub fn NetWorkStatusAtBlock(realm_id: u64, block_number: u64, block_time: String) -> impl IntoView {
    log::info!("block_time: {:?}", block_time);
    let block_time = NaiveDateTime::parse_from_str(&block_time, "%Y%m%d %H:%M:%S").unwrap();
    log::info!("block_time (naive): {:?}", block_time);
    let data = LocalResource::new(move || async move {
        get_network_status(realm_id, block_number, block_time).await
    });

    let loading_msg = format!("Loading network status at block {}...", block_number);
    // let l = move || data.get().unwrap();

    move || {
        match data.get().as_deref() {
        Some(d) => view! {
            <div class="row">
                <table class="table">
                    <tbody>
                        <tr>
                            <td>Network Status</td>
                            <td>{d.0.clone()}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <td>Epoch</td>
                            <td>{d.1}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <td> Validators</td>
                            <td>{d.2.len()}</td>
                            <td>{d.2.join(", ")}</td>
                        </tr>
                        <tr>
                            <td>Kicked Validators</td>
                            <td>{d.3.len()}</td>
                            <td>{d.3.join(", ")}</td>
                        </tr>
                        <tr>
                            <td>Next Validators</td>
                            <td>{d.4.len()}</td>
                            <td>{d.4.join(", ")}</td>
                        </tr>
                        <tr>
                            <td>Complaints</td>
                            <td>0</td>
                            <td></td>
                        </tr>
                        <tr>
                            <td>GCP Logs</td>
                            <td><a href={d.5.clone()} target="_blank">Google Cloud Project Logs</a></td>
                            <td></td>
                        </tr>
                    </tbody>
                </table>
            </div>
        }
        .into_any(),
        None => view! {
            <div>
                {loading_msg.clone()}
            </div>
        }
        .into_any(),
    }
    }
}

pub async fn get_network_status(
    realm_id: u64,
    block_number: u64,
    block_time: NaiveDateTime,
) -> (String, u64, Vec<String>, Vec<String>, Vec<String>, String) {
    let realm_id = ethers::types::U256::from(realm_id);
    let address = match get_address(crate::contracts::STAKING_CONTRACT).await {
        Ok(address) => address,
        Err(e) => {
            log::warn!("Error getting staking contract address: {:?}", e);
            return ("".to_string(), 0, vec![], vec![], vec![], "".to_string());
        }
    };

    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    let staker_names = gs.staker_names.get_untracked();

    let cfg = &get_lit_config();
    let staking = crate::contracts::staking::Staking::node_monitor_load(cfg, address).unwrap();

    let validators = staking.get_all_validators().call().await.unwrap();
    let validator_structs = staking
        .get_validators_structs(validators.clone())
        .call()
        .await
        .unwrap();
    let mappings = staking
        .get_node_staker_address_mappings(
            validator_structs
                .iter()
                .map(|v| v.node_address.clone())
                .collect::<Vec<H160>>(),
        )
        .call()
        .await
        .unwrap();

    let mut epoch_details = staking.epoch(realm_id);
    epoch_details.block = Some(block_number.into());
    let epoch_details = epoch_details.call().await.unwrap();

    let epoch_number = epoch_details.number;
    let mut state_call = staking.state(realm_id);
    state_call.block = Some(block_number.into());
    let state = state_call.call().await.unwrap();
    let network_state = match state {
        0 => "Active".to_string(),
        1 => "NextValidatorSetLocked".to_string(),
        2 => "ReadyForNextEpoch".to_string(),
        3 => "Unlocked".to_string(),
        4 => "Paused".to_string(),
        5 => "Restore".to_string(),
        _ => "Unknown".to_string(),
    };

    let mut kicked_validators = staking.get_kicked_validators(realm_id);
    kicked_validators.block = Some(block_number.into());
    let kicked_validators = kicked_validators.call().await.unwrap();

    let kicked_validators = kicked_validators
        .iter()
        .map(|v| get_staker_name(&staker_names, &validator_structs, &mappings, *v))
        .collect::<Vec<String>>();

    let mut validators_call = staking.get_validators_in_current_epoch(realm_id);
    validators_call.block = Some(block_number.into());
    let validators = validators_call.call().await.unwrap();

    let validators = validators
        .iter()
        .map(|v| get_staker_name(&staker_names, &validator_structs, &mappings, *v))
        .collect::<Vec<String>>();

    let mut next_validators_call = staking.get_validators_in_next_epoch(realm_id);
    next_validators_call.block = Some(block_number.into());
    let next_validators = next_validators_call.call().await.unwrap();

    let next_validators = next_validators
        .iter()
        .map(|v| get_staker_name(&staker_names, &validator_structs, &mappings, *v))
        .collect::<Vec<String>>();

    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    let subnet_id = gs.active_network().subnet_id;
    let cursor_timestamp = block_time.format("%Y-%m-%dT%H:%M:%S.%fZ").to_string();
    let time_delta = TimeDelta::minutes(3);
    let start_time = block_time.checked_sub_signed(time_delta).unwrap();
    let end_time = block_time.checked_add_signed(time_delta).unwrap();
    let start_time = start_time.format("%Y-%m-%dT%H:%M:%S.%fZ").to_string();
    let end_time = end_time.format("%Y-%m-%dT%H:%M:%S.%fZ").to_string();
    let gcp_logs_url = format!(
        "https://console.cloud.google.com/logs/query;query=jsonPayload.guest_subnet%3D%22{}%22;
cursorTimestamp={};
startTime={};
endTime={}?
project=quickstart-1572387045298",
        subnet_id, cursor_timestamp, start_time, end_time
    );

    (
        network_state,
        epoch_number.as_u64(),
        validators,
        kicked_validators,
        next_validators,
        gcp_logs_url,
    )
}

pub fn get_staker_name(
    common_addresses: &HashMap<String, String>,
    validator_structs: &Vec<Validator>,
    mappings: &Vec<AddressMapping>,
    address: H160,
) -> String {
    let node_address = mappings
        .iter()
        .find(|m| m.staker_address == address)
        .unwrap()
        .node_address;
    let validator_struct = validator_structs
        .iter()
        .find(|v| v.node_address == node_address)
        .unwrap();
    let ip_address = Ipv4Addr::from(validator_struct.ip).to_string();

    common_addresses
        .get(&ip_address)
        .unwrap_or(&address.to_string())
        .clone()
}
