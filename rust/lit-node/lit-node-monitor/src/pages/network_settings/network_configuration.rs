use crate::utils::datetime::{format_duration, format_timelock};
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
pub struct NetworkConfig {
    name: String,
    value: String,
}

#[component]
pub fn NetworkConfiguration() -> impl IntoView {
    crate::utils::set_header("Network Configuration");

    let global_data = LocalResource::new(|| async move { get_global_config().await });
    let data =
        LocalResource::new(|| async move { get_realm_config(ethers::types::U256::from(1)).await });

    view! {
        <Title text="Network Configuration"/>
        <Card class="min-w-full">
            <CardHeader>
                <b class="card-title">Global Network Configuration</b>
            </CardHeader>
            <CardPreview class="p-3">

                {move || match global_data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(rows) => view! {
                        <table class="table w-full">
                            <TableContent rows = rows.clone() scroll_container="html"  />
                        </table>
                        }.into_any()
                }}
            </CardPreview>
            <CardPreview class="p-3">
                <h5 class="card-title">Realm #1 Configuration</h5>

                {move || match data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(rows) => view! {
                        <table class="table w-full">
                            <TableContent rows = rows.clone() scroll_container="html"  />
                        </table>
                        }.into_any()
                }}
            </CardPreview>

        </Card>
    }
}

pub async fn get_realm_config(realm_id: ethers::types::U256) -> Vec<NetworkConfig> {
    let address = get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .unwrap();
    let cfg = &get_lit_config();
    let staking = Staking::node_monitor_load(cfg, address).unwrap();
    let config = staking.realm_config(realm_id).call().await;

    let config: lit_blockchain_lite::contracts::staking::RealmConfig = match config {
        Ok(config) => config,
        Err(e) => {
            log::error!("Error getting realm config: {:?}", e);
            return vec![];
        }
    };

    let rows = vec![
        NetworkConfig {
            name: "max_concurrent_requests".to_string(),
            value: config.max_concurrent_requests.to_string(),
        },
        NetworkConfig {
            name: "max_presign_count".to_string(),
            value: config.max_presign_count.to_string(),
        },
        NetworkConfig {
            name: "min_presign_count".to_string(),
            value: config.min_presign_count.to_string(),
        },
        NetworkConfig {
            name: "peer_checking_interval_secs".to_string(),
            value: config.peer_checking_interval_secs.to_string(),
        },
        NetworkConfig {
            name: "max_presign_concurrency".to_string(),
            value: config.max_presign_concurrency.to_string(),
        },
        // Deprecated and now unused
        NetworkConfig {
            name: "rpc_health_check_enabled".to_string(),
            value: config.rpc_healthcheck_enabled.to_string(),
        },
        NetworkConfig {
            name: "min_epoch_for_rewards".to_string(),
            value: config.min_epoch_for_rewards.to_string(),
        },
        NetworkConfig {
            name: "permitted_validators_on".to_string(),
            value: config.permitted_validators_on.to_string(),
        },
        NetworkConfig {
            name: "default_key_set".to_string(),
            value: config.default_key_set.to_string(),
        },
    ];
    rows
}

pub async fn get_global_config() -> Vec<NetworkConfig> {
    let address = get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .unwrap();
    let cfg = &get_lit_config();
    let staking = Staking::node_monitor_load(cfg, address).unwrap();
    let config = staking.global_config().call().await;

    let config: lit_blockchain_lite::contracts::staking::GlobalConfig = match config {
        Ok(config) => config,
        Err(e) => {
            log::error!("Error getting global config: {:?}", e);
            return vec![];
        }
    };

    let rows = vec![
        NetworkConfig {
            name: "token_reward_per_token_per_epoch".to_string(),
            value: format_ether(config.token_reward_per_token_per_epoch)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "minimum_validator_count".to_string(),
            value: config.minimum_validator_count.to_string(),
        },
        NetworkConfig {
            name: "reward_epoch_duration".to_string(),
            value: format_duration(config.reward_epoch_duration.as_u64()),
        },
        NetworkConfig {
            name: "max_time_lock".to_string(),
            value: format_timelock(config.max_time_lock.as_u64()),
        },
        NetworkConfig {
            name: "min_time_lock".to_string(),
            value: format_timelock(config.min_time_lock.as_u64()),
        },
        NetworkConfig {
            name: "bmin".to_string(),
            value: format_ether(config.bmin)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "bmax".to_string(),
            value: format_ether(config.bmax)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "k".to_string(),
            value: format_ether(config.k)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "p".to_string(),
            value: format_ether(config.p)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "enable_stake_autolock".to_string(),
            value: config.enable_stake_autolock.to_string(),
        },
        NetworkConfig {
            name: "token_price".to_string(),
            value: format_ether(config.token_price)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "profit_multiplier".to_string(),
            value: format_ether(config.profit_multiplier)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "usd_cost_per_month".to_string(),
            value: format_ether(config.usd_cost_per_month)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "max_emission_rate".to_string(),
            value: format_ether(config.max_emission_rate)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "min_stake_amount".to_string(),
            value: format_ether(config.min_stake_amount)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "max_stake_amount".to_string(),
            value: format_ether(config.max_stake_amount)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "min_self_stake".to_string(),
            value: format_ether(config.min_self_stake)
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string(),
        },
        NetworkConfig {
            name: "min_self_stake_timelock".to_string(),
            value: format_timelock(config.min_self_stake_timelock.as_u64()),
        },
        NetworkConfig {
            name: "min_validator_count_to_clamp_minimum_threshold".to_string(),
            value: config
                .min_validator_count_to_clamp_minimum_threshold
                .to_string(),
        },
        NetworkConfig {
            name: "min_threshold_to_clamp_at".to_string(),
            value: config.min_threshold_to_clamp_at.to_string(),
        },
        NetworkConfig {
            name: "vote_to_advance_time_out".to_string(),
            value: config.vote_to_advance_time_out.to_string(),
        },
    ];
    rows
}
