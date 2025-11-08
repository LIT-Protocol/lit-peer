use crate::utils::context::{WebCallBackContext, get_web_callback_context};
use crate::utils::contract_helper::get_staking_with_signer;
use leptos::prelude::*;
use leptos_meta::*;
use thaw::*;
#[component]
pub fn ValidatorAdmin() -> impl IntoView {
    // let mut gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    // for i in 1..=2 as u64 {
    //     if gs.network_state.len() as u64 <= i {
    //         gs.network_state.push(crate::models::NetworkState::new(i));
    //     }
    // }
    let ctx = get_web_callback_context();
    let staker_address_value = RwSignal::new("".to_string());

    // let shadow_realm_id_value =[RwSignal::new("1".to_string()), RwSignal::new("2".to_string())];
    let realm_id_value = RwSignal::new("1".to_string());
    // let epoch_status_value = RwSignal::new("0".to_string());
    // let epoch_length_value = RwSignal::new("300".to_string());
    crate::utils::set_header("Admin Tools");

    view! {
        <Title text="Admin Tools"/>
        <div class="card" >
            <div class="card-header">
                <b class="card-title">Validator Functions</b>
            </div>
            <div class="card-body">

                <label for="staker_address" class="pt-2">Permitted Stakers:</label>
                <Input input_size=5 attr:id="realm_id" value=realm_id_value />
                <br />
                <label for="staker_address" class="pt-2">Permitted Staker Address:</label>
                <Input input_size=45 attr:id="staker_address" value=staker_address_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Add" </Button>
                <Button on:click={ let ctx = ctx.clone(); move |_| {  rejoin_validator_function(&ctx, &staker_address_value.get()); }}> "Remove" </Button>
                <br />
                <label for="staker_address" class="pt-2">Staker Address:</label>
                <Input input_size=45 attr:id="staker_address" value=staker_address_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Kick" </Button>
                <Button on:click={ let ctx = ctx.clone(); move |_| {  rejoin_validator_function(&ctx, &staker_address_value.get()); }}> "Rejoin" </Button>
                <br />
                <label for="staker_address" class="pt-2">Validator to Slash:</label>
                <Input input_size=45 attr:id="staker_address" value=staker_address_value />
                <label for="staker_address" class="pt-2"> percentage:</label>
                <Input input_size=5 attr:id="staker_address" value=staker_address_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Slash" </Button>
                <br />
                <br />
            </div>
        </div>

        <br />
        <br />

        <div class="card" >
            <div class="card-body">
                <h5 class="card-title">Realm Settings</h5>

                <label for="staker_address" class="pt-2">Realm Id:</label>
                <Input input_size=5 attr:id="realm_id" value=realm_id_value />
                <br />


                <label for="staker_address" class="pt-2">Epoch Length:</label>
                <Input input_size=5 attr:id="realm_id" value=realm_id_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
                <br />

                <label for="staker_address" class="pt-2">Epoch Timeout:</label>
                <Input input_size=5 attr:id="realm_id" value=realm_id_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
                <br />

                <label for="staker_address" class="pt-2">Pending Rejoin Timeout:</label>
                <Input input_size=5 attr:id="realm_id" value=realm_id_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
                <br />

                <label for="staker_address" class="pt-2">Epoch State:</label>
                <Input input_size=5 attr:id="realm_id" value=realm_id_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
                <br />

                <label for="staker_address" class="pt-2">Epoch State:</label>
                <Input input_size=5 attr:id="realm_id" value=realm_id_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
                <br />

                <label for="staker_address" class="pt-2">Kick Penalty Percentage:</label>
                <Input input_size=5 attr:id="realm_id" value=realm_id_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
                <br />

                <label for="staker_address" class="pt-2">Demerit Rejoing Threshold:</label>
                <Input input_size=5 attr:id="realm_id" value=realm_id_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
                <br />

            </div>
        </div>



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
            ctx.show_success("Kicked Validator", format!("Result: {:?}", result).as_str());
        } else {
            let err = result.err();
            ctx.show_error(
                "Failed to kick validator",
                format!("Result: {:?}", err).as_str(),
            );
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

// true
