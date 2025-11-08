use ethers::middleware::Middleware;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, PendingTransaction, Provider};
use ethers::signers::{Signer, Wallet};
use ethers::types::{Bytes, TxHash, U256};
use k256::ecdsa::SigningKey;
use lit_blockchain::util::ether::middleware::EIP2771GasRelayerMiddleware;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

use lit_blockchain::contracts::ledger::Ledger;
use lit_blockchain::contracts::{default_local_client, default_local_client_no_wallet};
use lit_core::config::{LitConfig, ReloadableLitConfig};

use crate::error::{Result, unexpected_err};
use crate::payment::{
    batches::Batch, payed_endpoint::PayedEndpoint, payment_tracker::NodeCapacityConfig,
    payment_tracker::PaymentTracker,
};
use crate::peers::PeerState;
use crate::utils::contract::get_ledger_contract_with_gas_relay;
use crate::utils::contract::get_price_feed_contract;
use crate::utils::contract::get_price_feed_contract_with_gas_relay;
use lit_node_common::config::{CFG_KEY_PAYMENT_INTERVAL_MS_DEFAULT, LitNodeConfig};

// Main Batch Payment Processor.
pub async fn batch_payment_processor(
    mut quit_rx: mpsc::Receiver<bool>,
    cfg: Arc<ReloadableLitConfig>,
    payment_tracker: Arc<PaymentTracker>,
    peer_state: Arc<PeerState>,
) {
    let interval_ms = cfg
        .load_full()
        .payment_interval_ms()
        .unwrap_or(CFG_KEY_PAYMENT_INTERVAL_MS_DEFAULT) as u64;
    let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    info!(
        "Starting: Batch Payment Processor (will try every {}s)",
        interval.period().as_secs()
    );

    let mut failed_batches = vec![];

    loop {
        // Check if we should quit, or continue.
        tokio::select! {
            _ = quit_rx.recv() => {
                info!("Stopped: Batch Payment Processor");
                return;
            }
            _ = interval.tick() => {
                // Continue below.
            }
        }

        let cfg = cfg.load_full();

        //  Submit payment batches.
        let batches = payment_tracker.batches().take_batches_for_payment().await;
        if batches.is_empty() {
            continue;
        }

        let meta_signing_key = peer_state.wallet_keys.signing_key().clone();
        let client = match default_local_client(&cfg, None) {
            Ok(client) => client,
            Err(e) => {
                error!("Failed to load default client");
                continue;
            }
        };

        let provider = match default_local_client_no_wallet(&cfg) {
            Ok(provider) => provider,
            Err(e) => {
                error!("Failed to load default provider");
                continue;
            }
        };

        let ledger = match get_ledger_contract_with_gas_relay(&cfg, meta_signing_key).await {
            Ok(ledger) => ledger,
            Err(e) => {
                error!("Failed to get the ledger contract: {:?}", e);
                continue;
            }
        };

        failed_batches =
            charge_for_batches(&ledger, &client, &provider, batches, failed_batches).await;

        if !failed_batches.is_empty() {
            error!("Failed to charge for batches: {:?}", failed_batches);
        }
    }
}

// Usage Percentage Batch Payment Processor.
pub async fn usage_processor(
    mut quit_rx: mpsc::Receiver<bool>,
    cfg: Arc<ReloadableLitConfig>,
    payment_tracker: Arc<PaymentTracker>,
    peer_state: Arc<PeerState>,
) {
    let interval_ms = cfg
        .load_full()
        .payment_interval_ms()
        .unwrap_or(CFG_KEY_PAYMENT_INTERVAL_MS_DEFAULT) as u64;
    let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    info!(
        "Starting: Price Feed thread (will try every {}s)",
        interval.period().as_secs()
    );

    let mut last_reported_percentage = 0;

    let meta_signer_key = peer_state.wallet_keys.signing_key().clone();
    // Set initial usage
    if let Err(e) = set_usage_percentage(&cfg.load_full(), 0, meta_signer_key).await {
        error!("Failed to set usage percentage: {:?}", e);
    }

    loop {
        // Check if we should quit, or continue.
        tokio::select! {
            _ = quit_rx.recv() => {
                info!("Stopped: Price Feed thread");
                return;
            }
            _ = interval.tick() => {
                // Continue below.
            }
        }

        let cfg = cfg.load_full();

        // 1. Update the node capacity config.
        let cfg_for_capacity = cfg.clone();
        let payment_tracker_for_capacity = payment_tracker.clone();
        if let Ok(price_feed) = get_price_feed_contract(&cfg_for_capacity).await {
            match price_feed.get_node_capacity_config().call().await {
                Ok(config) => {
                    trace!("Retrieved node capacity config: {:?}", config);
                    let node_capacity_config = NodeCapacityConfig {
                        pkp_sign_max_concurrency: config.pkp_sign_max_concurrency.as_u64(),
                        enc_sign_max_concurrency: config.enc_sign_max_concurrency.as_u64(),
                        lit_action_max_concurrency: config.lit_action_max_concurrency.as_u64(),
                        sign_session_key_max_concurrency: config
                            .sign_session_key_max_concurrency
                            .as_u64(),
                        global_max_capacity: config.global_max_capacity.as_u64(),
                    };
                    payment_tracker_for_capacity.update_node_capacity_config(node_capacity_config);
                }
                Err(e) => {
                    error!("Failed to get node capacity config: {:?}", e);
                }
            }
        }

        // 2. Update the usage percentage.
        let usage_percentage = payment_tracker.get_usage_percentage();
        if usage_percentage != last_reported_percentage {
            // Report usage percentage
            let meta_signer_key = peer_state.wallet_keys.signing_key().clone();
            let cfg_for_usage = cfg.clone();
            tokio::task::spawn(async move {
                if let Err(e) =
                    set_usage_percentage(&cfg_for_usage, usage_percentage, meta_signer_key).await
                {
                    error!("Failed to set usage percentage: {:?}", e);
                }
            });

            last_reported_percentage = usage_percentage;
        }
    }
}

async fn set_usage_percentage(
    cfg: &LitConfig,
    percentage: u64,
    meta_signer_key: SigningKey,
) -> Result<()> {
    let price_feed_contract = get_price_feed_contract_with_gas_relay(cfg, meta_signer_key).await?;
    let product_ids = PayedEndpoint::get_all_product_ids();
    price_feed_contract
        .set_usage(U256::from(percentage), product_ids)
        .send()
        .await
        .map(|_| ())
        .map_err(|e| {
            let err_msg = format!("Cannot set the usage percentage: {:?}", e);
            unexpected_err(e, Some(err_msg))
        })
}

#[allow(clippy::type_complexity)]
async fn charge_for_batches(
    ledger: &Ledger<
        EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    >,
    client: &SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
    provider: &Arc<Provider<Http>>,
    new_batches: Vec<Batch>,
    old_batches: Vec<Batch>,
) -> Vec<Batch> {
    let mut failed_batches = vec![];

    for batch in old_batches.into_iter().chain(new_batches.into_iter()) {
        let id = batch.id();

        // Only relevant for new batches
        if batch.is_empty() {
            trace!("Batch {} is empty, skipping it.", batch.id());
            continue;
        }

        // Only relevant for old bathces
        if let Some(tx_hash) = batch.get_tx_hash() {
            let pending_tx = PendingTransaction::new(tx_hash, provider);
            if pending_tx.await.is_ok() {
                debug!("Successfully charged for Batch {}", batch.id());
                continue;
            }
        }

        // Either no pending transaction found or it failed.
        if let Err(batch) = charge_for_batch(ledger, client, batch).await {
            // Failed again. If the max trial count is not exceeded, save the batch.
            // Only relevant for old batches.
            if batch.max_trial_count_exceeded() {
                error!(
                    "Max trial count is exceeded for Batch {}: {:?}",
                    batch.id(),
                    batch
                );
            } else {
                failed_batches.push(batch);
            }
        }
    }

    failed_batches
}

#[allow(clippy::type_complexity)]
async fn charge_for_batch(
    ledger: &Ledger<
        EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    >,
    client: &SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
    mut batch: Batch,
) -> std::result::Result<(), Batch> {
    let (tx_hash, signed_tx) = match get_signed_tx_and_tx_hash(ledger, client, &batch).await {
        Ok((tx_hash, signed_tx)) => (tx_hash, signed_tx),
        Err(e) => {
            batch.increment_counter();
            warn!(
                "Failed to create the transaction for Batch {}: {:?}",
                batch.id(),
                e
            );
            return Err(batch);
        }
    };

    match send_signed_transaction(client, signed_tx).await {
        Ok(()) => {
            debug!("Successfully charged for Batch {}", batch.id());
            Ok(())
        }
        Err(e) => {
            warn!("Failed to charge for Batch {}: {:?}", batch.id(), e);
            batch.set_tx_hash_and_increment_counter(Some(tx_hash));
            Err(batch)
        }
    }
}

#[allow(clippy::type_complexity)]
async fn get_signed_tx_and_tx_hash(
    ledger: &Ledger<
        EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    >,
    client: &SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
    batch: &Batch,
) -> std::result::Result<(TxHash, Bytes), String> {
    let (addresses, prices) = batch.into_vecs();
    let cu = ledger.charge_users(addresses, prices, batch.id());

    // The following line would be a much shorter way to send the transaction;
    // but if it fails, we don't get a tx_hash back. Hence, we have this whole function.
    // let tx = cu.send().await.map_err(|e| e.to_string())?;

    let mut typed_tx = cu.tx.clone();
    client
        .fill_transaction(&mut typed_tx, None)
        .await
        .map_err(|e| e.to_string())?;
    let signature = client
        .signer()
        .sign_transaction(&typed_tx)
        .await
        .map_err(|e| e.to_string())?;

    let tx_hash = typed_tx.hash(&signature);
    let signed_tx = typed_tx.rlp_signed(&signature);
    Ok((tx_hash, signed_tx))
}

async fn send_signed_transaction(
    client: &SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
    signed_tx: Bytes,
) -> std::result::Result<(), String> {
    let pending_tx = client
        .send_raw_transaction(signed_tx)
        .await
        .map_err(|e| e.to_string())?;

    match pending_tx.await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
