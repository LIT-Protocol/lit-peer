use crate::utils::{get_address, get_lit_config};
use ethers::providers::Provider;
use ethers::types::U256;
use ethers_providers::Http;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::price_feed::{LitActionPriceConfig, PriceFeed};
use serde::{Deserialize, Serialize};
#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(
    sortable,
    classes_provider = "BootstrapClassesPreset",
    impl_vec_data_provider
)]
pub struct PricingConfig {
    name: String,
    value: String,
    #[table(title = "Exponential Notation")]
    evalue: String,
    notes: String,
}

#[component]
pub fn Pricing() -> impl IntoView {
    crate::utils::set_header("Network Configuration");

    let price_feed_data = LocalResource::new(|| async move { get_price_feed().await });

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
                            <TableContent rows = rows.0.clone() scroll_container="html"  />
                        </table>
                        }.into_any()
                }}
            </div>
        </div>
        <br />
        <div class="card" >
            <div class="card-header">
                <b class="card-title">Lit Action Individual Prices</b>
            </div>

            <div class="card-body">
                {move || match price_feed_data.get().as_deref() {
                    None => view! { <p>"Loading..."</p> }.into_any(),
                    Some(rows) => view! {
                        <table class="table">
                            <TableContent rows = rows.1.clone() scroll_container="html"  />
                        </table>
                        }.into_any()
                }}
            </div>

        </div>
        <br />
        ** Note: the Lit Action price in Base Network Prices is the price reported to the client for transaction estimation purposes. The actual price is the sum of the individual prices ( and their frequency ) calculated during the run of the Lit Action.
    }
}

async fn get_price_feed_contract() -> PriceFeed<Provider<Http>> {
    let address = get_address(crate::contracts::PRICE_FEED_CONTRACT)
        .await
        .unwrap();
    let cfg = &get_lit_config();
    PriceFeed::node_monitor_load(cfg, address).unwrap()
}

pub async fn get_price_feed() -> (Vec<PricingConfig>, Vec<PricingConfig>) {
    let price_feed = get_price_feed_contract().await;
    let product_ids = vec![U256::from(0), U256::from(1), U256::from(2), U256::from(3)];

    let product_id_desc = vec![
        "Encryption Sign",
        "Lit Action",
        "PKP Sign",
        "Session Key Sign",
    ];
    let config = price_feed.base_network_prices(product_ids).call().await;

    let config = match config {
        Ok(config) => config,
        Err(e) => {
            log::error!("Error getting price feed: {:?}", e);
            return (vec![], vec![]);
        }
    };

    let mut rows1 = vec![];
    let mut sig_price = U256::from(0);
    for (i, price) in config.iter().enumerate() {
        if i == 2 {
            sig_price = price.clone();
        }
        rows1.push(PricingConfig {
            name: product_id_desc[i].to_string(),
            value: price.to_string(),
            evalue: format!("{:.2e}", price.as_u128()),
            notes: "".to_string(),
        });
    }

    let la_prices: Vec<LitActionPriceConfig> = price_feed
        .get_lit_action_price_configs()
        .call()
        .await
        .unwrap();

    let la_price_desc = vec![
        "baseAmount",
        "runtimeLength",
        "memoryUsage",
        "codeLength",
        "responseLength",
        "signatures",
        "broadcasts",
        "contractCalls",
        "callDepth",
        "decrypts",
        "fetches",
    ];
    let mut rows2 = vec![];
    let mut lit_action_sig_price: U256;
    for (i, price) in la_prices.iter().enumerate() {
        let mut notes = "".to_string();
        if price.price_component == 5 {
            lit_action_sig_price = price.price;
            if lit_action_sig_price > sig_price {
                notes = "This is more expensive than the base network price for a signature."
                    .to_string();
            } else if lit_action_sig_price < sig_price {
                notes = "This is less expensive than the base network price for a signature."
                    .to_string();
            }
        }
        rows2.push(PricingConfig {
            name: la_price_desc[price.price_component as usize].to_string(),
            value: price.price.to_string(),
            evalue: format!("{:.2e}", price.price.as_u128()),
            notes,
        });
    }

    (rows1, rows2)
}
