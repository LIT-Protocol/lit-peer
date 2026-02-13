use std::net::Ipv4Addr;

use crate::models::GlobalState;
use crate::utils::datetime::{format_timelock, format_timestamp};
use crate::utils::{get_address, get_lit_config, table_classes::TailwindClassesPreset};
use ethers::utils::format_ether;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::staking::Staking;
use serde::{Deserialize, Serialize};
use thaw::{Card, CardHeader, CardPreview};

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(
    sortable,
    classes_provider = "TailwindClassesPreset",
    impl_vec_data_provider
)]

pub struct NodeStakeOverview {
    staker_name: String,
    #[table(skip)]
    node_address: String,
    #[table(skip)]
    staker_address: String,
    last_active_epoch: u64,
    commission_rate: String,
    last_reward_epoch: u64,
    last_realm_id: u64,
    #[table(title = "Delegated Amount")]
    delegated_stake_amount: String,
    #[table(skip)]
    delegated_stake_weight: String,
    #[table(skip)]
    last_reward_epoch_claimed_fixed_cost_rewards: String,
    #[table(skip)]
    last_reward_epoch_claimed_commission: String,
    #[table(title = "Delegating Stakers")]
    unique_delegating_staker_count: u64,
}

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(
    sortable,
    classes_provider = "TailwindClassesPreset",
    impl_vec_data_provider
)]

pub struct StakeRecord {
    #[table(title = "Staker#")]
    index: u32,
    status: String,
    staker: String,
    #[table(skip)]
    id: u64,
    amount: String,
    #[table(title = "Unfreeze")]
    unfreeze_start: u64,
    #[table(title = "Time Lock")]
    time_lock: String,
    #[table(title = "Last Update")]
    last_update_timestamp: String,
    #[table(title = "Reward Claim")]
    last_reward_epoch_claimed: u64,
    initial_share_price: String,
    loaded: bool,
    frozen: bool,
}

#[component]
pub fn StakingDetails() -> impl IntoView {
    let data = LocalResource::new(|| async move { get_staking_records().await });

    crate::utils::set_header("Staking Details");
    view! {
        <Title text="Staking Details"/>
        <Card class="min-w-full">
            <CardHeader>
                <b class="card-title">Node Operator Staking Overview</b>
            </CardHeader>
            <CardPreview class="p-3">
                {move || match data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some((rows, _)) => view! {

                        <table class="table w-full">
                            <TableContent rows = rows.clone() scroll_container="html"  />
                        </table>

                        }.into_any()
                }}
            </CardPreview>
        </Card>
<br/>
<br/>
<br/>
        <Card class="min-w-full">
            <CardHeader>
                <b class="card-title">Stake Records</b>
            </CardHeader>
            <CardPreview class="p-3">
                {move || match data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some((_, rows)) => view! {
                        <table class="table w-full">
                            <TableContent rows = rows.clone() scroll_container="html"  />
                        </table>

                        }.into_any()
                }}
            </CardPreview>
        </Card>



    }
}

pub async fn get_staking_records() -> (Vec<NodeStakeOverview>, Vec<StakeRecord>) {
    let gs = use_context::<GlobalState>().expect("Global State Failed to Load");
        
    let address = get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .unwrap();

    let cfg = &get_lit_config();
    let staking = Staking::node_monitor_load(cfg, address).unwrap();

    let realm_id = ethers::types::U256::from(1);
    let validators = staking.get_all_validators().call().await.unwrap();
    let current_validators = staking
        .get_validators_in_current_epoch(realm_id)
        .call()
        .await
        .unwrap();
    let next_validators = staking
        .get_validators_in_next_epoch(realm_id)
        .call()
        .await
        .unwrap();
    // let mut validators = validators.unwrap();
    // let next_validators = staking.get_validators_in_next_epoch(realm_id).await;
    // let next_validators = next_validators.unwrap();
    // validators.extend(next_validators);
    let mut rows: Vec<StakeRecord> = Vec::new();
    let mut node_stake_overview: Vec<NodeStakeOverview> = Vec::new();
    let mut index = 1;

    log::info!("stakers names: {:?}", gs.staker_names.get());

    // let all_addresses = staking.get_all_validators().call().await.unwrap();
    let all_structs: Vec<lit_blockchain_lite::contracts::staking::Validator> = staking.get_validators_structs(validators.clone()).call().await.unwrap();

    let node_addresses = all_structs.iter().map(|v| v.node_address).collect::<Vec<_>>();
    let mappings: Vec<crate::contracts::staking::AddressMapping> = staking.get_node_staker_address_mappings(node_addresses.clone()).call().await.unwrap();

    log::info!("all_structs: {:?}", all_structs);
    // log::info!("mappings: {:?}", mappings);
    
    for v in all_structs.clone() {
        let ip_address = Ipv4Addr::from(v.ip).to_string();
        let staker_name = gs
            .staker_names
            .get()
            .get(&ip_address)
            .unwrap_or(&ip_address)
            .clone();                

        let node_staker_overview = NodeStakeOverview {
            staker_name: staker_name,
            node_address: v.node_address.to_string(),
            staker_address: mappings.iter().find(|m| m.node_address == v.node_address).unwrap().staker_address.to_string(),
            last_active_epoch: v.last_active_epoch.as_u64(),
            commission_rate: format_ether(v.commission_rate).trim_end_matches('0').trim_end_matches('.').to_string(),
            last_reward_epoch: v.last_reward_epoch.as_u64(),
            last_realm_id: v.last_realm_id.as_u64(),
            delegated_stake_amount: format_ether(v.delegated_stake_amount).trim_end_matches('0').trim_end_matches('.').to_string(),
            delegated_stake_weight: format_ether(v.delegated_stake_weight).trim_end_matches('0').trim_end_matches('.').to_string(),
            last_reward_epoch_claimed_fixed_cost_rewards: format_ether(v.last_reward_epoch_claimed_fixed_cost_rewards).trim_end_matches('0').trim_end_matches('.').to_string(),
            last_reward_epoch_claimed_commission: format_ether(v.last_reward_epoch_claimed_commission).trim_end_matches('0').trim_end_matches('.').to_string(),
            unique_delegating_staker_count: v.unique_delegating_staker_count.as_u64(),
        };
        node_stake_overview.push(node_staker_overview);
    }

    for v in validators {
        let records = staking.get_stake_records_for_user(v, v).await;
        let records = records.unwrap();
        let _units = "ether";
        let status = if current_validators.contains(&v) {
            "Current".to_string()
        } else if next_validators.contains(&v) {
            "Next".to_string()
        } else {
            "Inactive".to_string()
        };
        for r in records {
            if r.loaded {
                let node_address = match mappings.iter().find(|m| m.staker_address == v) {
                    Some(mapping) => mapping.node_address,
                    None => v,
                };
            
                let staker_name =  match  all_structs.iter().find(|av| av.node_address == node_address) {
                    Some(validator_struct) => {
                        let ip_address = Ipv4Addr::from(validator_struct.ip).to_string();
                        log::info!("ip_address: {:?}", ip_address);
                        let staker_name = gs
                            .staker_names
                            .get()
                            .get(&ip_address)
                            .unwrap_or(&v.to_string())
                            .clone();
        
                        staker_name
                    } 
                    None => {
                        v.to_string()
                    }
                };
                
                rows.push(StakeRecord {
                    index: index,
                    status: status.clone(),
                    staker: staker_name,
                    id: r.id.as_u64(),
                    amount: format_ether(r.amount).trim_end_matches('0').trim_end_matches('.').to_string(),
                    unfreeze_start: r.unfreeze_start.as_u64(),
                    time_lock: format_timelock(r.time_lock.as_u64()),
                    last_update_timestamp: format_timestamp(r.last_update_timestamp.as_u64()),
                    last_reward_epoch_claimed: r.last_reward_epoch_claimed.as_u64(),
                    initial_share_price: format_ether(r.initial_share_price).trim_end_matches('0').trim_end_matches('.').to_string(),
                    loaded: r.loaded,
                    frozen: r.frozen,
                });
            }
        }
        index += 1;
    }
    (node_stake_overview, rows)
}
