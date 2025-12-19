use crate::utils::{get_address, get_lit_config};
use ethers::types::U256;
use leptos::prelude::*;
use leptos_meta::*;
use lit_blockchain_lite::contracts::     staking::{KeySetConfig, Staking};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct KeySetMonitorConfig {
    pub identifier: String,
    pub description: String,
    pub minimum_threshold: usize,
    pub monetary_value: usize,
    pub complete_isolation: bool,
    pub realms: Vec<u32>,
    pub root_keys_by_curve: Vec<(String, u32)>,
    pub recovery_party_members: Vec<String>,
}

#[component]
pub fn Keysets() -> impl IntoView {
    let data = LocalResource::new(|| async move { get_key_set_configs().await });

    crate::utils::set_header("Key Sets");
    view! {
        <Title text="Key Sets"/>
        <div class="card" >
           {   move || match data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(rows) =>
                    rows.iter().map(|(identifier, key_set_config)|
                        view! {
                            <div class="card" >
                                <div class="card-header">
                                    <b class="card-title">KeySet - {identifier.clone()}</b>
                                </div>
                                <div class="card-body">
                                    <table class="table">
                                        <tr>
                                            <td>Identifier</td>
                                            <td>{identifier.clone()}</td>
                                        </tr>
                                        <tr>
                                            <td>Description</td>
                                            <td>{key_set_config.description.clone()}</td>
                                        </tr>
                                        <tr>
                                            <td>Minimum Threshold</td>
                                            <td>{key_set_config.minimum_threshold.clone()}</td>
                                        </tr>
                                        <tr>
                                            <td>Monetary Value</td>
                                            <td>{key_set_config.monetary_value.clone()}</td>
                                        </tr>
                                        <tr>
                                            <td>Complete Isolation</td>
                                            <td>{key_set_config.complete_isolation.clone()}</td>
                                        </tr>
                                        <tr>
                                            <td>Realms</td>
                                            <td>{key_set_config.realms.iter().map(|realm| realm.to_string()).collect::<Vec<String>>().join(", ")}</td>
                                        </tr>
                                        <tr>
                                            <td>Root Keys by Curve</td>
                                            <td>{key_set_config.root_keys_by_curve.iter().map(|(curve, count)| format!("{}: {}", curve, count)).collect::<Vec<String>>().join(", ")}</td>
                                        </tr>
                                        <tr>
                                            <td>Recovery Party Members</td>
                                            <td>{key_set_config.recovery_party_members.iter().map(|member| member.clone()).collect::<Vec<String>>().join(", ")}</td>
                                        </tr>
                                    </table>
                                </div>
                            </div>
                        }).collect_view().into_any()
                }
        }
        </div>
    }
}

pub async fn get_key_set_configs() -> Vec<(String, KeySetMonitorConfig)> {

    let staking_contract_address = get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .unwrap();

    let cfg = &get_lit_config();

    let staking = Staking::node_monitor_load(cfg, staking_contract_address).unwrap();

    let key_configs  = staking.key_sets().call().await;
    let key_configs: Vec<KeySetConfig> = match key_configs {
        Ok(key_configs) => key_configs,
        Err(e) => {
            log::error!("Error getting key configs: {:?}", e);
            return vec![];
        }
    };

    let mut key_set_monitor_configs: Vec<(String, KeySetMonitorConfig)> = vec![];

    for key_config in key_configs {

        let mut pos = 0;
        let mut root_keys_by_curve: Vec<(String, u32)> = vec![];
        for curve in key_config.curves {
            let curve_type = get_curve_type(curve);
            let curve_count = key_config.counts[pos].as_u32();
            root_keys_by_curve.push((curve_type, curve_count));
            pos += 1;
        }

        let key_set_config = KeySetMonitorConfig {
                identifier: key_config.identifier.to_string(),
                description: key_config.description.to_string(),
                minimum_threshold: key_config.minimum_threshold as usize,
                monetary_value: key_config.monetary_value as usize,
                complete_isolation: key_config.complete_isolation,
                realms: key_config.realms.iter().map(|realm| realm.as_u32() ).collect(),
                root_keys_by_curve: root_keys_by_curve,
                recovery_party_members: key_config.recovery_party_members.iter().map(|member| member.to_string()).collect(),
            };

        key_set_monitor_configs.push((key_config.identifier.to_string(), key_set_config));
    }

    key_set_monitor_configs
}

pub fn get_curve_type(curve: U256) -> String {
    match curve.as_u32() {
        1 => "BLS".to_string(),
        2 => "K256".to_string(),
        3 => "Ed25519".to_string(),
        4 => "Ed448".to_string(),
        5 => "Ristretto25519".to_string(),
        6 => "P256".to_string(),
        7 => "P384".to_string(),
        8 => "RedJubJub".to_string(),
        9 => "RedDecaf377".to_string(),
        10 => "BLS12381G1".to_string(),
        _ => "Unknown".to_string(),
    }
}