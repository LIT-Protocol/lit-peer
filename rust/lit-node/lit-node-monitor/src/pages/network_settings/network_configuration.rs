use crate::utils::datetime::{format_duration, format_timelock};
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
        <div class="card" >
            <div class="card-header">
                <b class="card-title">Global Network Configuration</b>
            </div>
            <div class="card-body">

                {move || match global_data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(rows) => view! {
                        <table class="table">
                            <TableContent rows = rows.clone() scroll_container="html"  />
                        </table>
                        }.into_any()
                }}
            </div>
            <div class="card-body">
                <h5 class="card-title">Realm #1 Configuration</h5>

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

pub async fn get_realm_config(realm_id: ethers::types::U256) -> Vec<NetworkConfig> {
    let address = get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .unwrap();
    let cfg = &get_lit_config();
    let staking = Staking::node_monitor_load(cfg, address).unwrap();
    let config = staking.realm_config(realm_id).call().await;

    let config = match config {
        Ok(config) => config,
        Err(e) => {
            log::error!("Error getting config: {:?}", e);
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
        NetworkConfig {
            name: "rpc_health_check_enabled".to_string(),
            value: config.rpc_healthcheck_enabled.to_string(),
        },
        NetworkConfig {
            name: "permitted_validators_on".to_string(),
            value: config.permitted_validators_on.to_string(),
        },
        // NetworkConfig {
        //     name: "min_epoch_for_rewards".to_string(),
        //     value: config.min_epoch_for_rewards.to_string(),
        // },
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

    let config = match config {
        Ok(config) => config,
        Err(e) => {
            log::error!("Error getting global config: {:?}", e);
            return vec![];
        }
    };

    let key_types = staking.get_key_types().call().await.unwrap();

    //     uint256 tokenRewardPerTokenPerEpoch;
    // // the key type of the node.  // 1 = BLS, 2 = ECDSA.  Not doing this in an enum so we can add more keytypes in the future without redeploying.
    // uint256[] keyTypes;
    // // don't start the DKG or let nodes leave the validator set
    // // if there are less than this many nodes
    // uint256 minimumValidatorCount;
    // /// thunderhead
    // uint256 rewardEpochDuration;
    // uint256 maxTimeLock;
    // uint256 minTimeLock;
    // uint256 bmin; // Minimum reward budget (in basis points, i.e., 0.1%)
    // uint256 bmax; // Maximum reward budget (in basis points, i.e., 0.5%)
    // uint256 k; // Kink parameter for rewards (e.g., 0.5)
    // uint256 p; // Power parameter (e.g., 0.5)
    // bool enableStakeAutolock; // if true, stake will be autolocked
    // bool permittedStakersOn;
    // uint256 tokenPrice;
    // uint256 profitMultiplier;
    // uint256 usdCostPerMonth;
    // uint256 maxEmissionRate;
    // uint256 minStakeAmount;
    // uint256 maxStakeAmount;
    // uint256 minSelfStake;
    // uint256 minSelfStakeTimelock;

    let rows = vec![
        NetworkConfig {
            name: "key_types".to_string(),
            value: key_types
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
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
            value: config.bmin.to_string(),
        },
        NetworkConfig {
            name: "bmax".to_string(),
            value: config.bmax.to_string(),
        },
        NetworkConfig {
            name: "k".to_string(),
            value: config.k.to_string(),
        },
        NetworkConfig {
            name: "p".to_string(),
            value: config.p.to_string(),
        },
        NetworkConfig {
            name: "enable_stake_autolock".to_string(),
            value: config.enable_stake_autolock.to_string(),
        },
        NetworkConfig {
            name: "token_price".to_string(),
            value: config.token_price.to_string(),
        },
        NetworkConfig {
            name: "profit_multiplier".to_string(),
            value: config.profit_multiplier.to_string(),
        },
        NetworkConfig {
            name: "usd_cost_per_month".to_string(),
            value: config.usd_cost_per_month.to_string(),
        },
        NetworkConfig {
            name: "max_emission_rate".to_string(),
            value: config.max_emission_rate.to_string(),
        },
        NetworkConfig {
            name: "min_stake_amount".to_string(),
            value: config.min_stake_amount.to_string(),
        },
        NetworkConfig {
            name: "max_stake_amount".to_string(),
            value: config.max_stake_amount.to_string(),
        },
        NetworkConfig {
            name: "min_self_stake".to_string(),
            value: config.min_self_stake.to_string(),
        },
        NetworkConfig {
            name: "min_self_stake_timelock".to_string(),
            value: format_timelock(config.min_self_stake_timelock.as_u64()),
        },
        NetworkConfig {
            name: "token_reward_per_token_per_epoch".to_string(),
            value: config.token_reward_per_token_per_epoch.to_string(),
        },
    ];
    rows
}
