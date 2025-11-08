use crate::utils::{get_address, get_lit_config};
use ethers::types::H160;
use leptos::prelude::*;

#[component]
pub fn NetWorkStatusAtBlock(realm_id: u64, block_number: u64) -> impl IntoView {
    let data =
        LocalResource::new(move || async move { get_network_status(realm_id, block_number).await });

    let loading_msg = format!("Loading network status at block {}...", block_number);
    // let l = move || data.get().unwrap();

    move || {
        match data.get().as_deref() {
        Some(d) => view! {
            <div class="row">
                <table class="table">
                    <tbody>
                        <tr>
                            <td>Network Status</td>
                            <td>{d.0.clone()}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <td>Epoch</td>
                            <td>{d.1}</td>
                            <td></td>
                        </tr>
                        <tr>
                            <td> Validators</td>
                            <td>{d.2.len()}</td>
                            <td>{d.2.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")}</td>
                        </tr>
                        <tr>
                            <td>Kicked Validators</td>
                            <td>{d.3.len()}</td>
                            <td>{d.3.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")}</td>
                        </tr>
                        <tr>
                            <td>Next Validators</td>
                            <td>{d.4.len()}</td>
                            <td>{d.4.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        }
        .into_any(),
        None => view! {
            <div>
                {loading_msg.clone()}
            </div>
        }
        .into_any(),
    }
    }
}

pub async fn get_network_status(
    realm_id: u64,
    block_number: u64,
) -> (String, u64, Vec<H160>, Vec<H160>, Vec<H160>) {
    let realm_id = ethers::types::U256::from(realm_id);
    let address = match get_address(crate::contracts::STAKING_CONTRACT).await {
        Ok(address) => address,
        Err(e) => {
            log::warn!("Error getting staking contract address: {:?}", e);
            return ("".to_string(), 0, vec![], vec![], vec![]);
        }
    };
    let cfg = &get_lit_config();
    let staking = crate::contracts::staking::Staking::node_monitor_load(cfg, address).unwrap();

    let mut epoch_details = staking.epoch(realm_id);
    epoch_details.block = Some(block_number.into());
    let epoch_details = epoch_details.call().await.unwrap();

    let epoch_number = epoch_details.number;
    let mut state_call = staking.state(realm_id);
    state_call.block = Some(block_number.into());
    let state = state_call.call().await.unwrap();
    let network_state = match state {
        0 => "Active".to_string(),
        1 => "NextValidatorSetLocked".to_string(),
        2 => "ReadyForNextEpoch".to_string(),
        3 => "Unlocked".to_string(),
        4 => "Paused".to_string(),
        5 => "Restore".to_string(),
        _ => "Unknown".to_string(),
    };

    let mut kicked_validators = staking.get_kicked_validators(realm_id);
    kicked_validators.block = Some(block_number.into());
    let kicked_validators = kicked_validators.call().await.unwrap();

    let mut validators_call = staking.get_validators_in_current_epoch(realm_id);
    validators_call.block = Some(block_number.into());
    let validators = validators_call.call().await.unwrap();

    let mut next_validators_call = staking.get_validators_in_next_epoch(realm_id);
    next_validators_call.block = Some(block_number.into());
    let next_validators = next_validators_call.call().await.unwrap();

    // format!(
    //     "Network Status: {} at epoch# {} | Validators: {} | Kicked Validators: {}",
    //     network_state,
    //     epoch_number,
    //     validators.len(),
    //     kicked_validators.len()
    // )
    (
        network_state,
        epoch_number.as_u64(),
        validators,
        kicked_validators,
        next_validators,
    )
}
