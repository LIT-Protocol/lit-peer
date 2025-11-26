use super::super::validators::get_validators;
use crate::utils::{
    context::{WebCallBackContext, get_web_callback_context},
    contract_helper::{get_lit_token, get_staking},
};
use ethers::types::H160;
use ethers_providers::{Http, Middleware, Provider};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_struct_table::*;
use lit_blockchain_lite::contracts::lit_token::LITToken;
use serde::{Deserialize, Serialize};

#[derive(TableRow, Clone, Serialize, Deserialize)]
#[table(
    sortable,
    classes_provider = "BootstrapClassesPreset",
    impl_vec_data_provider
)]
pub struct ContractBalance {
    name: String,
    balance: String,
    total_supply: String,
}

#[derive(TableRow, Clone, Serialize, Deserialize, PartialEq)]
#[table(
    sortable,
    classes_provider = "BootstrapClassesPreset",
    impl_vec_data_provider
)]
pub struct WalletBalance {
    owner: String,
    socket_address: String,
    wallet_address: String,
    lit: String,
    gas: String,
}

#[component]
pub fn Wallets() -> impl IntoView {
    let data = LocalResource::new(move || {
        let ctx = get_web_callback_context();
        async move { get_wallet_balances(&ctx).await }
    });

    view! {
        <Title text="Balance Details"/>
        <div class="card" >
            <div class="card-header">
                <b class="card-title">Node / NodeOp / EOA Wallets</b>
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

pub async fn get_wallet_balances(ctx: &WebCallBackContext) -> Vec<WalletBalance> {
    let staking = get_staking(ctx).await;
    let lit_token = get_lit_token(ctx).await;

    let validators = get_validators(&staking, true, 1).await;
    let mut rows: Vec<WalletBalance> = Vec::new();

    let provider = Provider::<Http>::try_from(ctx.active_network.chain_url.clone())
        .expect("could not instantiate HTTP Provider");

    for v in validators {
        let socket_address = v.socket_address;

        let owner = "NodeOp (Main Staker)".to_string();
        let wallet_address = v.staker_address;
        let (gas, lit) =
            get_gas_and_lit(wallet_address.clone(), lit_token.clone(), provider.clone()).await;

        rows.push(WalletBalance {
            owner,
            socket_address: format!("[{}]", socket_address.clone()),
            wallet_address,
            lit,
            gas,
        });

        let owner = "Node".to_string();
        let wallet_address = v.wallet_address;
        let (gas, lit) =
            get_gas_and_lit(wallet_address.clone(), lit_token.clone(), provider.clone()).await;

        rows.push(WalletBalance {
            owner,
            socket_address,
            wallet_address,
            lit,
            gas,
        });
    }

    let local_account = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"; // get from MetaMask ?
    let owner = "EOA".to_string();
    let socket_address = "n/a".to_string();
    let wallet_address = local_account.to_string();

    let (gas, lit) = get_gas_and_lit(
        local_account.to_string(),
        lit_token.clone(),
        provider.clone(),
    )
    .await;

    rows.push(WalletBalance {
        owner,
        socket_address,
        wallet_address,
        lit,
        gas,
    });

    // let realm_id = ethers::types::U256::from(1);
    // // let staking = Staking::load_with_signer(cfg, address, None).unwrap();
    // let staking = Staking::node_monitor_load(cfg, address).unwrap();

    rows
}

pub async fn get_gas_and_lit(
    wallet_address: String,
    lit_token: LITToken<Provider<Http>>,
    provider: Provider<Http>,
) -> (String, String) {
    let bytes = hex::decode(wallet_address.replace("0x", "")).unwrap();
    let wallet_address_h160 = H160::from_slice(&bytes);

    let gas = provider
        .get_balance(wallet_address_h160, None)
        .await
        .unwrap();
    let gas = gas.as_u128() as f64 / 1e18;
    let gas = gas.to_string();

    let lit = lit_token.balance_of(wallet_address_h160).call().await;
    let lit = lit.unwrap();
    let lit = lit.as_u128() as f64 / 1e18;
    let lit = lit.to_string();

    (gas, lit)
}
