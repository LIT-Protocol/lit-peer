use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain::contracts::pubkey_router::PubkeyRouter;
use lit_blockchain::contracts::staking::Staking;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::debug;
use tracing::info;

use ethers::providers::Http;
use ethers::providers::StreamExt;

pub async fn listen_for_events(
    staking_contract: Arc<Staking<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>>,
    pubkey: Arc<PubkeyRouter<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>>,
    mut quit_rx: mpsc::Receiver<bool>,
) -> anyhow::Result<()> {
    let staking_events = staking_contract.events();
    let mut staking_stream = staking_events
        .stream()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create event stream: {}", e))?;

    let pubkey_events = pubkey.events();
    let mut pubkey_stream = pubkey_events
        .stream()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create event stream: {}", e))?;

    info!("Starting TESTNET event listener");

    loop {
        tokio::select! {
            _ = quit_rx.recv() => {
                // info!("Received quit signal, exiting event listener");
               // break;
            }

            event = staking_stream.next() => {
                debug!("EMIT(Staking): {:?}", event);
            }

            event = pubkey_stream.next() => {
                debug!("EMIT(PubkeyRouter): {:?}", event);
            }
        }
    }
    // Ok(())
}
