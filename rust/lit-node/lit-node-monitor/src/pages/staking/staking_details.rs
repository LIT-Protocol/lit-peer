use crate::utils::datetime::{format_timelock, format_timestamp};
use crate::utils::{get_address, get_lit_config};
use ethers::utils::format_units;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::staking::Staking;
use serde::{Deserialize, Serialize};

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(
    sortable,
    classes_provider = "BootstrapClassesPreset",
    impl_vec_data_provider
)]
pub struct StakeRecord {
    #[table(title = "Staker#")]
    index: u32,
    status: String,
    staker: String,
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
        <Title text="Contracts"/>
        <div class="card" >
            <div class="card-header">
                <b class="card-title">Node Operator Staking Overview</b>
            </div>
            <div class="card-body">
                {move || match data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(rows) => view! {

                        <table class="table">
                            <TableContent rows = rows.clone() scroll_container="html"  />
                        </table>

                        }.into_any()
                }}
            </div>
        </div>

    }
}

pub async fn get_staking_records() -> Vec<StakeRecord> {
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
    let mut index = 1;

    for v in validators {
        let records = staking.get_stake_records_for_user(v, v).await;
        let records = records.unwrap();
        let units = "ether";
        let status = if current_validators.contains(&v) {
            "Current".to_string()
        } else if next_validators.contains(&v) {
            "Next".to_string()
        } else {
            "Inactive".to_string()
        };
        for r in records {
            if r.loaded {
                rows.push(StakeRecord {
                    index: index,
                    status: status.clone(),
                    staker: v.to_string(),
                    id: r.id.as_u64(),
                    amount: format!("{:.2e}", r.amount.as_u64()),
                    unfreeze_start: r.unfreeze_start.as_u64(),
                    time_lock: format_timelock(r.time_lock.as_u64()),
                    last_update_timestamp: format_timestamp(r.last_update_timestamp.as_u64()),
                    last_reward_epoch_claimed: r.last_reward_epoch_claimed.as_u64(),
                    initial_share_price: format!("{:.2e}", r.initial_share_price.as_u64()),
                    loaded: r.loaded,
                    frozen: r.frozen,
                });
            }
        }
        index += 1;
    }
    rows
}
