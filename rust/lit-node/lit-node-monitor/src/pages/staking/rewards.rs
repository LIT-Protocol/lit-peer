use crate::utils::datetime::{format_duration, format_timestamp};
use crate::utils::{get_address, get_lit_config};
use ethers_providers::{Http, Provider};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::staking::RewardEpoch;
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

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(
    sortable,
    classes_provider = "BootstrapClassesPreset",
    impl_vec_data_provider
)]
pub struct RewardDetails {
    name: String,
    current: String,
    next: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StakerRewards {
    staker_address: String,
    reward_details: Vec<RewardDetails>,
}

#[component]
pub fn Rewards() -> impl IntoView {
    let data =
        LocalResource::new(|| async move { get_epoch_details(ethers::types::U256::from(1)).await });
    crate::utils::set_header("Rewards Details");
    let reward_data =
        LocalResource::new(
            || async move { get_staker_rewards(ethers::types::U256::from(1)).await },
        );

    view! {
        <Title text="Current Reward Details"/>
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
        {
            move || match reward_data.get().as_deref() {
                None => view! { <p>"Loading..."</p> }.into_any(),
                Some(staker_rewards) => staker_rewards.iter().map(|staker| {
                    view! {
                        <div class="card" >
                            <div class="card-header">
                                <b class="card-title">Rewards for : {staker.staker_address.clone()}</b>
                            </div>
                            <div class="card-body">
                                    <table class="table">
                                        <TableContent rows = staker.reward_details.clone() scroll_container="html"  />
                                    </table>
                            </div>
                        </div>
                        <br/>
                    }
                }).collect_view().into_any()
            }
        }
    }
}

pub async fn get_epoch_details(realm_id: ethers::types::U256) -> Vec<EpochDetails> {
    let address = get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .unwrap();

    let cfg = &get_lit_config();
    let staking = Staking::node_monitor_load(cfg, address).unwrap();
    let epoch_details = staking.epoch(realm_id).call().await;

    let epoch_details = match epoch_details {
        Ok(epoch_details) => epoch_details,
        Err(e) => {
            log::error!("Error getting config: {:?}", e);
            return vec![];
        }
    };

    let rows = vec![
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

pub async fn get_staker_rewards(realm_id: ethers::types::U256) -> Vec<StakerRewards> {
    let mut all_rows = vec![];
    let address = get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .unwrap();

    let cfg = &get_lit_config();
    let staking = Staking::node_monitor_load(cfg, address).unwrap();
    let staking2 = staking.clone();
    let epoch_details = staking.epoch(realm_id).call().await;
    let epoch_details = match epoch_details {
        Ok(epoch_details) => epoch_details,
        Err(e) => {
            log::error!("Error getting config: {:?}", e);
            return vec![];
        }
    };

    let staker_addresses = staking
        .get_validators_in_current_epoch(realm_id)
        .call()
        .await;
    let staker_addresses = match staker_addresses {
        Ok(staker_addresses) => staker_addresses,
        Err(e) => {
            log::error!("Error getting staker addresses: {:?}", e);
            return vec![];
        }
    };

    if staker_addresses.is_empty() {
        return vec![StakerRewards {
            staker_address: "----No Stakers found ------".to_string(),
            reward_details: vec![],
        }];
    }

    let reward_epoch_number = epoch_details.reward_epoch_number;
    let next_reward_epoch_number = epoch_details.next_reward_epoch_number;
    for staker_address in staker_addresses {
        let reward_details = get_reward_details(
            staking2.clone(),
            staker_address,
            reward_epoch_number,
            next_reward_epoch_number,
        )
        .await;
        all_rows.push(StakerRewards {
            staker_address: staker_address.to_string(),
            reward_details,
        });
    }

    all_rows
}

pub async fn get_reward_details(
    staking: Staking<Provider<Http>>,
    staker_address: ethers::types::H160,
    reward_epoch_number: ethers::types::U256,
    next_reward_epoch_number: ethers::types::U256,
) -> Vec<RewardDetails> {
    let mut all_rows = vec![];

    let epoch_rewards = staking
        .get_reward_epoch(staker_address, reward_epoch_number)
        .call()
        .await;
    let epoch_rewards: RewardEpoch = match epoch_rewards {
        Ok(epoch_rewards) => epoch_rewards,
        Err(e) => {
            log::error!("Error getting epoch rewards: {:?}", e);
            return vec![];
        }
    };

    let next_epoch_rewards = staking
        .get_reward_epoch(staker_address, next_reward_epoch_number)
        .call()
        .await;
    let next_epoch_rewards: RewardEpoch = match next_epoch_rewards {
        Ok(next_epoch_rewards) => next_epoch_rewards,
        Err(e) => {
            log::error!("Error getting epoch rewards: {:?}", e);
            return vec![];
        }
    };

    let rows = vec![
        RewardDetails {
            name: "Reward Epoch Number".to_string(),
            current: reward_epoch_number.to_string(),
            next: next_reward_epoch_number.to_string(),
        },
        RewardDetails {
            name: "epochEnd".to_string(),
            current: format_timestamp(epoch_rewards.epoch_end.as_u64()),
            next: format_timestamp(next_epoch_rewards.epoch_end.as_u64()),
        },
        RewardDetails {
            name: "totalStakeWeight".to_string(),
            current: epoch_rewards.total_stake_weight.to_string(),
            next: next_epoch_rewards.total_stake_weight.to_string(),
        },
        RewardDetails {
            name: "totalStakeRewards".to_string(),
            current: epoch_rewards.total_stake_rewards.to_string(),
            next: next_epoch_rewards.total_stake_rewards.to_string(),
        },
        RewardDetails {
            name: "slope".to_string(),
            current: epoch_rewards.slope.to_string(),
            next: next_epoch_rewards.slope.to_string(),
        },
        RewardDetails {
            name: "slopeIncrease".to_string(),
            current: "bad field".to_string(), // epoch_rewards.slope_increase.to_string(),
            next: "bad field".to_string(),    // next_epoch_rewards.slope_increase.to_string(),
        },
        RewardDetails {
            name: "validatorSharePrice".to_string(),
            current: epoch_rewards.validator_share_price.to_string(),
            next: next_epoch_rewards.validator_share_price.to_string(),
        },
        RewardDetails {
            name: "stakeAmount".to_string(),
            current: epoch_rewards.stake_amount.to_string(),
            next: next_epoch_rewards.stake_amount.to_string(),
        },
        RewardDetails {
            name: "validatorSharePriceAtLastUpdate".to_string(),
            current: epoch_rewards
                .validator_share_price_at_last_update
                .to_string(),
            next: next_epoch_rewards
                .validator_share_price_at_last_update
                .to_string(),
        },
        RewardDetails {
            name: "initial".to_string(),
            current: epoch_rewards.initial.to_string(),
            next: next_epoch_rewards.initial.to_string(),
        },
    ];
    all_rows.extend(rows);
    all_rows
}
