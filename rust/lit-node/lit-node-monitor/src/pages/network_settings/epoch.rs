use crate::utils::datetime::{format_duration, format_timestamp};
use crate::utils::{get_address, get_lit_config};
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
pub struct EpochDetails {
    name: String,
    value: String,
}

#[component]
pub fn Epoch() -> impl IntoView {
    let data =
        LocalResource::new(|| async move { get_epoch_details(ethers::types::U256::from(1)).await });

    crate::utils::set_header("Epoch Details");

    let data2 =
        LocalResource::new(|| async move { get_epoch_details(ethers::types::U256::from(2)).await });

    view! {
        <Title text="Epoch Details"/>
        <div class="card" >
            <div class="card-header">
                <b class="card-title">Realm #1 Epoch Details</b>
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
        <br/>

        <div class="card" >
            <div class="card-header">
                <b class="card-title">Realm #2 Epoch Details</b>
            </div>
            <div class="card-body">

                {move || match data2.get().as_deref() {
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

pub async fn get_epoch_details(realm_id: ethers::types::U256) -> Vec<EpochDetails> {
    let address = get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .unwrap();
    let cfg = &get_lit_config();
    let staking = Staking::node_monitor_load(cfg, address).unwrap();
    let epoch_details = staking.epoch(realm_id).call().await;
    let state = staking.state(realm_id).call().await;

    let epoch_details = match epoch_details {
        Ok(epoch_details) => epoch_details,
        Err(e) => {
            log::error!("Error getting config: {:?}", e);
            return vec![];
        }
    };

    let state = match state {
        Ok(state) => state,
        Err(e) => {
            log::error!("Error getting state: {:?}", e);
            return vec![];
        }
    };

    let rows = vec![
        EpochDetails {
            name: "Network State".to_string(),
            value: state.to_string(),
        },
        EpochDetails {
            name: "number".to_string(),
            value: epoch_details.number.to_string(),
        },
        EpochDetails {
            name: "reward_epoch_number".to_string(),
            value: epoch_details.reward_epoch_number.to_string(),
        },
        EpochDetails {
            name: "next_reward_epoch_number".to_string(),
            value: epoch_details.next_reward_epoch_number.to_string(),
        },
        EpochDetails {
            name: "start_time".to_string(),
            value: format_timestamp(epoch_details.start_time.as_u64()),
        },
        EpochDetails {
            name: "end_time".to_string(),
            value: format_timestamp(epoch_details.end_time.as_u64()),
        },
        EpochDetails {
            name: "epoch_length".to_string(),
            value: format_duration(epoch_details.epoch_length.as_u64()),
        },
        EpochDetails {
            name: "retries".to_string(),
            value: epoch_details.retries.to_string(),
        },
        EpochDetails {
            name: "timeout".to_string(),
            value: epoch_details.timeout.to_string(),
        },
    ];
    rows
}
