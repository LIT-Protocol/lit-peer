use crate::utils::{get_address, get_lit_config, table_classes::TailwindClassesPreset};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::{pubkey_router::PubkeyRouter, staking::Staking};
use serde::{Deserialize, Serialize};
use thaw::{Card, CardHeader, CardPreview};
#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(
    sortable,
    classes_provider = "TailwindClassesPreset",
    impl_vec_data_provider
)]
pub struct RootKeys {
    keytype: String,
    pubkey: String,
}
#[component]
pub fn RootKeys() -> impl IntoView {
    let data = LocalResource::new(|| async move { get_root_keys().await });

    crate::utils::set_header("Root Keys");
    view! {
        <Title text="Root Keys"/>
        {move || match data.get().as_deref() {
            None => view! { <p>"Loading..."</p> }.into_any(),
            Some(rows) => {
                let rows = rows.to_vec();
                rows.into_iter().map(|key_set|
                    view! {
                        <Card class="min-w-full">
                            <CardHeader>
                                <b class="card-title">Root Keys - {key_set.0.clone()}</b>
                            </CardHeader>
                            <CardPreview class="p-3">
                                    <table class="table w-full">
                                        <TableContent rows = key_set.1.clone() scroll_container="html"  />
                                    </table>
                            </CardPreview>
                        </Card>
                    }
                ).collect_view().into_any()
            }
        }}
    }
}

pub async fn get_root_keys() -> Vec<(String, Vec<RootKeys>)> {
    use crate::contracts::pubkey_router::RootKey;

    let pubkey_router_address = get_address(crate::contracts::PUB_KEY_ROUTER_CONTRACT)
        .await
        .unwrap();
    let staking_contract_address = get_address(crate::contracts::STAKING_CONTRACT)
        .await
        .unwrap();

    let cfg = &get_lit_config();
    let pubkey_router = PubkeyRouter::node_monitor_load(cfg, pubkey_router_address).unwrap();

    let root_keys = pubkey_router
        .get_root_keys(staking_contract_address, DEFAULT_KEY_SET_NAME.to_string())
        .call()
        .await;

    let key_configs = staking.key_sets().call().await;
    let key_configs = match key_configs {
        Ok(key_configs) => key_configs,
        Err(e) => {
            log::error!("Error getting key configs: {:?}", e);
            return vec![];
        }
    };

    let mut keysets: Vec<(String, Vec<RootKeys>)> = vec![];

    for key_config in key_configs {
        let root_keys = pubkey_router
            .get_root_keys(staking_contract_address, key_config.identifier.to_string())
            .call()
            .await;

        log::info!("Root keys: {:?}", root_keys);

        let root_keys = match root_keys {
            Ok(root_keys) => root_keys as Vec<RootKey>,
            Err(e) => {
                log::error!("Error getting root_keys: {:?}", e);
                return vec![];
            }
        };

        let mut rows: Vec<RootKeys> = vec![];

        log::info!("Got Root keys!");

        for root_key in &root_keys {
            log::info!("Root Key: {:?}", root_key);
            let keytype = match root_key.key_type.as_u32() {
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
            };
            let pubkey = format!("0x{}", hex::encode(root_key.pubkey.clone()));
            rows.push(RootKeys { keytype, pubkey });
        }

        keysets.push((key_config.identifier.to_string(), rows));
    }

    keysets
}
