use crate::utils::{get_address, get_lit_config};
use ethers::types::U256;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::price_feed::{LitActionPriceConfig, PriceFeed};
use serde::{Deserialize, Serialize};
use ethers_providers::Http;
use ethers::providers::Provider;
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
pub fn Pricing() -> impl IntoView {
    crate::utils::set_header("Network Configuration");


    let price_feed_data = LocalResource::new(|| async move { get_price_feed().await });
    let lit_action_prices_data = LocalResource::new(|| async move { get_lit_action_prices().await });
    view! {
        <Title text="Pricing"/>
        <div class="card" >
            <div class="card-header">
                <b class="card-title">Base Network Prices</b>
            </div>

            <div class="card-body">
                {move || match price_feed_data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(rows) => view! {
                        <table class="table">
                            <TableContent rows = rows.clone() scroll_container="html"  />
                        </table>
                        }.into_any()
                }}
            </div>
        </div>
        <div class="card" >
            <div class="card-header">
                <b class="card-title">Base Network Prices</b>
            </div>

            <div class="card-body">
                {move || match lit_action_prices_data.get().as_deref() {
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

async fn get_price_feed_contract() -> PriceFeed<Provider<Http>> {
    let address = get_address(crate::contracts::PRICE_FEED_CONTRACT)
        .await
        .unwrap();
    let cfg = &get_lit_config();
    PriceFeed::node_monitor_load(cfg, address).unwrap()
}

pub async fn get_price_feed() -> Vec<NetworkConfig> {
    
    let price_feed = get_price_feed_contract().await;
    let product_ids = vec![U256::from(0), U256::from(1), U256::from(2), U256::from(3)];

   
    let product_id_desc = vec!["Encryption Sign", "Lit Action", "PKP Sign", "Session Key Sign"];
    let config = price_feed.base_network_prices(product_ids).call().await;

    let config = match config {
        Ok(config) => config,
        Err(e) => {
            log::error!("Error getting price feed: {:?}", e);
            return vec![];
        }
    };

    let mut rows = vec![];
    for (i, price) in config.iter().enumerate() {
        rows.push(NetworkConfig {
            name: product_id_desc[i].to_string(),
            value: price.to_string(),
        });
    }

    rows
}

pub async fn get_lit_action_prices() -> Vec<NetworkConfig> {
    let price_feed = get_price_feed_contract().await;
    let la_prices: Vec<LitActionPriceConfig> = price_feed.get_lit_action_price_configs().call().await.unwrap();

    let la_price_desc = vec!["baseAmount", "runtimeLength", "memoryUsage", "codeLength", "responseLength", "signatures", "broadcasts", "contractCalls", "callDepth", "decrypts", "fetches"];
    let mut rows = vec![];
    for (i, price) in la_prices.iter().enumerate() {
        rows.push(NetworkConfig {
            name: la_price_desc[price.price_component as usize].to_string(),
            value: price.price.to_string(),
        });
    }

    rows
}