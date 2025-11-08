use crate::utils::context::{WebCallBackContext, get_web_callback_context};
use crate::utils::contract_helper::get_staking_with_signer;
use leptos::prelude::*;
use thaw::*;
#[component]
pub fn QuickFunctions() -> impl IntoView {
    let ctx = get_web_callback_context();
    let staker_address_value = RwSignal::new("".to_string());

    view! {
        <br />
        <h5 class="card-title">Quick Functions</h5>
        <label for="staker_address" class="pt-2">Staker Address:</label>
        <Input input_size=45 attr:id="staker_address" value=staker_address_value />
        <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Kick" </Button>
        <Button on:click={ let ctx = ctx.clone(); move |_| {  rejoin_validator_function(&ctx, &staker_address_value.get()); }}> "Rejoin" </Button>
        <Button on:click={ let ctx = ctx.clone(); move |_| {  add_validator_function(&ctx, &staker_address_value.get()); }}> "Add to next epoch" </Button>
        <br />

    }
}

fn kick_validator_function(ctx: &WebCallBackContext, staker_address: &str) -> impl IntoView {
    log::info!("Kick Validator | staker_address: {:?}", staker_address);
    let staker_address = staker_address.to_string();
    let ctx = ctx.clone();
    leptos::task::spawn_local(async move {
        let (staking, from) = get_staking_with_signer(&ctx).await;

        let validator_staker_address = hex::decode(staker_address.replace("0x", "")).unwrap();
        let validator_staker_address =
            ethers::types::H160::from_slice(validator_staker_address.as_slice());

        log::info!(
            "Kick Validator | validator_staker_address: {:?}",
            validator_staker_address
        );

        let function_call = staking
            .admin_kick_validator_in_next_epoch(validator_staker_address)
            .from(from);

        let result = function_call.send().await;

        log::info!("Kick Validator | initial result: {:?}", result);

        if result.is_ok() {
            let tx = result.unwrap();
            let result = tx.await;
            log::info!("Kick Validator network | tx result: {:?}", result);
        }
    });

    // true
}

fn rejoin_validator_function(ctx: &WebCallBackContext, staker_address: &str) -> impl IntoView {
    log::info!("Rejoin Validator | staker_address: {:?}", staker_address);
    let staker_address = staker_address.to_string();
    let ctx = ctx.clone();
    leptos::task::spawn_local(async move {
        let (staking, from) = get_staking_with_signer(&ctx).await;

        let validator_staker_address = hex::decode(staker_address.replace("0x", "")).unwrap();
        let validator_staker_address =
            ethers::types::H160::from_slice(validator_staker_address.as_slice());

        log::info!(
            "Rejoin Validator | validator_staker_address: {:?}",
            validator_staker_address
        );

        let realm_id = ethers::types::U256::from(1);
        let function_call = staking
            .admin_rejoin_validator(realm_id, validator_staker_address)
            .from(from);

        let result = function_call.send().await;

        log::info!("Rejoin Validator | initial result: {:?}", result);

        if result.is_ok() {
            let tx = result.unwrap();
            let result = tx.await;
            log::info!("Rejoin Validator network | tx result: {:?}", result);
        }
    });

    // true
}

fn add_validator_function(ctx: &WebCallBackContext, staker_address: &str) -> impl IntoView {
    log::info!("Add Validator | staker_address: {:?}", staker_address);
    let staker_address = staker_address.to_string();
    let ctx = ctx.clone();
    leptos::task::spawn_local(async move {
        let (staking, from) = get_staking_with_signer(&ctx).await;

        let validator_staker_address = hex::decode(staker_address.replace("0x", "")).unwrap();
        let validator_staker_address =
            ethers::types::H160::from_slice(validator_staker_address.as_slice());

        let realm_id = ethers::types::U256::from(1);
        let function_call = staking
            .request_to_join_as_admin(realm_id, validator_staker_address)
            .from(from);

        let result = function_call.send().await;

        log::info!("Add Validator | initial result: {:?}", result);

        if result.is_ok() {
            let tx = result.unwrap();
            let result = tx.await;
            log::info!("Add Validator network | tx result: {:?}", result);
        }
    });

    // true
}
