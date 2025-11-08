use crate::utils::context::WebCallBackContext;
use crate::utils::contract_helper::get_pubkey;
use crate::utils::contract_helper::get_staking;
use ethers::providers::StreamExt;
use std::error::Error;

pub async fn listen_for_events(ctx: &WebCallBackContext) -> Result<bool, Box<dyn Error>> {
    let staking_contract = get_staking(ctx).await;
    let pubkey = get_pubkey(ctx).await;

    let staking_events = staking_contract.events();
    let mut staking_stream = staking_events.stream().await?;

    let pubkey_events = pubkey.events();
    let mut pubkey_stream = pubkey_events.stream().await?;

    log::info!("Starting event listener");

    loop {
        tokio::select! {
            event = staking_stream.next() => {
                log::info!("EMIT(Staking): {:?}", event);
                ctx.show_info("Staking Event", format!("{:?}", event).as_str());
            }

            event = pubkey_stream.next() => {
                log::info!("EMIT(PubkeyRouter): {:?}", event);
                ctx.show_info("PubkeyRouter Event", format!("{:?}", event).as_str());
            }
        }
    }
}
