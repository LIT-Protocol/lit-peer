use crate::utils::context::{WebCallBackContext, get_web_callback_context};
use crate::utils::contract_helper::get_staking_with_signer;
use ethers::types::H160;
use leptos::prelude::*;
use leptos_meta::*;
use thaw::*;
#[component]
pub fn Realms() -> impl IntoView {
    // let mut gs = use_context::<GlobalState>().expect("Global State Failed to Load");
    // for i in 1..=2 as u64 {
    //     if gs.network_state.len() as u64 <= i {
    //         gs.network_state.push(crate::models::NetworkState::new(i));
    //     }
    // }
    let ctx = get_web_callback_context();
    // let staker_address_value = RwSignal::new("".to_string());

    let shadow_realm_id_value = [
        RwSignal::new("1".to_string()),
        RwSignal::new("2".to_string()),
    ];
    let realm_id_value = RwSignal::new("1".to_string());
    let epoch_status_value = RwSignal::new("0".to_string());
    let epoch_length_value = RwSignal::new("300".to_string());
    crate::utils::set_header("Admin Tools");

    view! {
        <Title text="Admin Tools"/>
        // <div class="card" >
        //     <div class="card-body">
        //         <h5 class="card-title">Validator Functions</h5>

        //         <label for="staker_address" class="pt-2">Permitted Stakers:</label>
        //         <Input input_size=5 attr:id="realm_id" value=realm_id_value />
        //         <br />
        //         <label for="staker_address" class="pt-2">Permitted Staker Address:</label>
        //         <Input input_size=45 attr:id="staker_address" value=staker_address_value />
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Add" </Button>
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  rejoin_validator_function(&ctx, &staker_address_value.get()); }}> "Remove" </Button>
        //         <br />
        //         <label for="staker_address" class="pt-2">Staker Address:</label>
        //         <Input input_size=45 attr:id="staker_address" value=staker_address_value />
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Kick" </Button>
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  rejoin_validator_function(&ctx, &staker_address_value.get()); }}> "Rejoin" </Button>
        //         <br />
        //         <label for="staker_address" class="pt-2">Validator to Slash:</label>
        //         <Input input_size=45 attr:id="staker_address" value=staker_address_value />
        //         <label for="staker_address" class="pt-2"> percentage:</label>
        //         <Input input_size=5 attr:id="staker_address" value=staker_address_value />
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Slash" </Button>
        //         <br />
        //         <br />
        //     </div>
        // </div>

        // <br />
        // <br />

        // <div class="card" >
        //     <div class="card-body">
        //         <h5 class="card-title">Realm Settings</h5>

        //         <label for="staker_address" class="pt-2">Realm Id:</label>
        //         <Input input_size=5 attr:id="realm_id" value=realm_id_value />
        //         <br />


        //         <label for="staker_address" class="pt-2">Epoch Length:</label>
        //         <Input input_size=5 attr:id="realm_id" value=realm_id_value />
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
        //         <br />

        //         <label for="staker_address" class="pt-2">Epoch Timeout:</label>
        //         <Input input_size=5 attr:id="realm_id" value=realm_id_value />
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
        //         <br />

        //         <label for="staker_address" class="pt-2">Pending Rejoin Timeout:</label>
        //         <Input input_size=5 attr:id="realm_id" value=realm_id_value />
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
        //         <br />

        //         <label for="staker_address" class="pt-2">Epoch State:</label>
        //         <Input input_size=5 attr:id="realm_id" value=realm_id_value />
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
        //         <br />

        //         <label for="staker_address" class="pt-2">Epoch State:</label>
        //         <Input input_size=5 attr:id="realm_id" value=realm_id_value />
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
        //         <br />

        //         <label for="staker_address" class="pt-2">Kick Penalty Percentage:</label>
        //         <Input input_size=5 attr:id="realm_id" value=realm_id_value />
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
        //         <br />

        //         <label for="staker_address" class="pt-2">Demerit Rejoing Threshold:</label>
        //         <Input input_size=5 attr:id="realm_id" value=realm_id_value />
        //         <Button on:click={ let ctx = ctx.clone(); move |_| {  kick_validator_function(&ctx, &staker_address_value.get()); }}> "Set" </Button>
        //         <br />

        //     </div>
        // </div>


        <br />
        <br />

        <div class="card" >
            <div class="card-header">
                <b class="card-title">New Realms</b>
            </div>
            <div class="card-body">

                <label for="add_realm" class="pt-2">Add Realm:</label>
                <Button on:click={ let ctx = ctx.clone(); move |_| { add_realm_function( &ctx); } }> "Add Realm" </Button>
                <br />
                <label for="staker_address" class="pt-2">Source Realm Id:</label>
                <Input input_size=5 attr:id="src_realm_id" value=shadow_realm_id_value[0] />
                <label for="staker_address" class="pt-2">Destination Realm Id:</label>
                <Input input_size=5 attr:id="dst_realm_id" value=shadow_realm_id_value[1] />
                <Button on:click={ let ctx = get_web_callback_context(); move |_| { setup_shadow_splicing( &ctx); } }> "Setup Shadow Splicing" </Button>
                <br />
                <label for="add_realm" class="pt-2">Check Shadow validators for realm #2:</label>
                <Button on:click={ let ctx = ctx.clone(); move |_| { check_shadow_validators( &ctx); } }> "Check Shadow Validators" </Button>


                <hr />

                <h5 class="card-title">Adjust Realms</h5>

                <label class="pt-2">Set realm state to:</label>
                <Combobox value=epoch_status_value placeholder="Set Status ">
                    <ComboboxOption value="0" text="Active" />
                    <ComboboxOption value="1" text="NextValidatorSetLocked" />
                    <ComboboxOption value="2" text="ReadyForNextEpoch" />
                    <ComboboxOption value="3" text="Unlocked" />
                    <ComboboxOption value="4" text="Paused" />
                    <ComboboxOption value="5" text="Restore" />
                </Combobox>
                <label class="pt-2">for realm:</label>
                 <Combobox value=realm_id_value placeholder="for realm" >
                    <ComboboxOption value="1" text="1" />
                    <ComboboxOption value="2" text="2" />
                </Combobox>
                <Button on:click={ let ctx = ctx.clone(); move |_| { set_epoch_status_function( &ctx, realm_id_value.get(), epoch_status_value.get()); } }> "Set Status" </Button>
                <br />


                <label class="pt-2">Set Epoch Length   :</label>
                <Input input_size=5 attr:id="epoch_length" value=epoch_length_value />
                <Button on:click={ let ctx = ctx.clone(); move |_| { set_epoch_length_function( &ctx, epoch_length_value.get().parse::<u64>().unwrap()); } }> "Set Epoch Length" </Button>
                <br />


            </div>
        </div>


    }
}

fn add_realm_function(ctx: &WebCallBackContext) {
    log::info!("Add Realm !");
    let ctx = ctx.clone();
    leptos::task::spawn_local(async move {
        let (staking, from) = get_staking_with_signer(&ctx).await;
        ctx.show_info("Adding Realm", "Attempting to add realm.");
        let function = staking.add_realm().from(from);
        let result = function.send().await;

        if result.is_ok() {
            ctx.show_success("Added Realm", format!("Result: {:?}", result).as_str());
        } else {
            let err = result.err();
            ctx.show_error("Failed to add realm", format!("Result: {:?}", err).as_str());
        }
    });
}

fn set_epoch_status_function(ctx: &WebCallBackContext, realm_id: String, new_state: String) {
    log::info!("Set Epoch State !");
    log::info!("Realm Id: {:?}", realm_id);
    log::info!("New State: {:?}", new_state);
    let ctx = ctx.clone();
    leptos::task::spawn_local(async move {
        let (staking, from) = get_staking_with_signer(&ctx).await;
        let realm_id = ethers::types::U256::from(realm_id.parse::<u64>().unwrap());

        let new_state = match new_state.as_str() {
            "Active" => 0,
            "NextValidatorSetLocked" => 1,
            "ReadyForNextEpoch" => 2,
            "Unlocked" => 3,
            "Paused" => 4,
            "Restore" => 5,
            _ => 0,
        };
        ctx.show_info(
            "Network Status",
            &format!("Attempting to update epoch state for realm {}.", realm_id),
        );
        let function = staking.set_epoch_state(realm_id, new_state).from(from);
        let result = function.send().await;

        if result.is_ok() {
            ctx.show_success(
                "Updated Epoch State",
                format!("Result: {:?}", result).as_str(),
            );
        } else {
            let err = result.err();
            ctx.show_error(
                "Failed to update epoch state",
                format!("Result: {:?}", err).as_str(),
            );
        }
    });
}

fn set_epoch_length_function(ctx: &WebCallBackContext, epoch_length: u64) {
    log::info!("Set Epoch Length !");
    let ctx = ctx.clone();
    leptos::task::spawn_local(async move {
        let (staking, from) = get_staking_with_signer(&ctx).await;
        let realm_id = ethers::types::U256::from(2);
        ctx.show_info(
            "Attempting to set epoch length",
            &format!(
                "Attempting to set epoch length to {} for realm {}.",
                epoch_length, realm_id
            ),
        );
        let function = staking
            .set_epoch_length(realm_id, ethers::types::U256::from(epoch_length))
            .from(from);
        let result = function.send().await;

        if result.is_ok() {
            ctx.show_success(
                "Updated Epoch Length",
                format!("Result: {:?}", result).as_str(),
            );
        } else {
            let err = result.err();
            ctx.show_error(
                "Failed to update epoch length",
                format!("Result: {:?}", err).as_str(),
            );
        }
    });
}

fn check_shadow_validators(ctx: &WebCallBackContext) {
    log::info!("Check Shadow Validators !");
    let ctx = ctx.clone();
    leptos::task::spawn_local(async move {
        let (staking, _from) = get_staking_with_signer(&ctx).await;
        let realm_id = ethers::types::U256::from(2);
        let shadow_validators = staking.get_shadow_validators(realm_id).call().await;

        let validators = shadow_validators.unwrap();

        if validators.is_empty() {
            ctx.show_warning(
                "No Shadow Validators",
                "No shadow validators found for realm 2",
            );
            return;
        }

        for staker_address in validators {
            let result = staking
                .is_active_shadow_validator(realm_id, staker_address)
                .call()
                .await;
            let shadow_realm_id = staking
                .get_shadow_realm_id_for_staker_address(staker_address)
                .call()
                .await;

            ctx.show_info(
                &format!("Shadow Validator {}", staker_address),
                &format!(
                    "{} is an active shadow validator: {:?} /n Shadow Realm Id: {:?}",
                    staker_address, result, shadow_realm_id
                ),
            );
        }
    });
}

fn setup_shadow_splicing(ctx: &WebCallBackContext) {
    log::info!("Setup shadow splicing !");
    let ctx = ctx.clone();
    leptos::task::spawn_local(async move {
        let (staking, from) = get_staking_with_signer(&ctx).await;
        let src_realm_id = ethers::types::U256::from(1);
        let dst_realm_id = ethers::types::U256::from(2);

        let reserve_addresses = staking.get_all_reserve_validators().call().await;
        let reserve_addresses = reserve_addresses.unwrap();
        let validators = staking
            .get_validators_structs(reserve_addresses)
            .call()
            .await;
        let validators = validators.unwrap();

        let node_addresses = validators
            .iter()
            .map(|v| v.node_address)
            .collect::<Vec<H160>>();

        let mappings: Vec<crate::contracts::staking::AddressMapping> = staking
            .get_node_staker_address_mappings(node_addresses)
            .call()
            .await
            .unwrap();

        let new_stakers = validators
            .iter()
            .map(|v| {
                mappings
                    .iter()
                    .find(|m| m.node_address == v.node_address)
                    .unwrap()
                    .staker_address
            })
            .collect::<Vec<H160>>();

        log::info!("New Stakers: {:?}", new_stakers);

        let function_call = staking
            .admin_setup_shadow_splicing(src_realm_id, dst_realm_id, new_stakers)
            .from(from);
        let result = function_call.send().await;

        if result.is_ok() {
            ctx.show_success(
                "Setup Shadow Splicing",
                format!("Result: {:?}", result).as_str(),
            );
        } else {
            let err = result.err();
            ctx.show_error(
                "Failed to setup shadow splicing",
                format!("Result: {:?}", err).as_str(),
            );
        }
    });
}
