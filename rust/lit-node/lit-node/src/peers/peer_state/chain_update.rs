use super::super::PeerState;
use crate::error::unexpected_err;
use crate::{
    error::{EC, Result, blockchain_err, blockchain_err_code},
    utils::eth::EthereumAddress,
};
use ethers::{
    providers::Middleware,
    types::{U64, U256},
};
use lit_blockchain::util::decode_revert;
use std::time::Duration;
use tracing::{Instrument, debug_span, instrument, trace};

#[allow(dead_code)]
impl PeerState {
    // ############ Functions to mutate on-chain peer state using the node wallet ############
    // Note that if we fail to signal ready, it is a problem and we error, but if we fail to lock or advance, probably it is fine, so we just log.
    #[allow(non_snake_case)]
    #[instrument(level = "debug", skip(self), fields(epoch_number = epoch_number.as_u64()))]
    pub async fn rpc_signal_ready_for_next_epoch(
        &self,
        epoch_number: U256,
        realm_id: u64,
    ) -> Result<()> {
        debug!(
            "Signal ready from {} for epoch {}, realm_id {}",
            self.addr,
            epoch_number.as_u64(),
            realm_id
        );
        let realm_id = U256::from(realm_id);
        let func = self
            .staking_contract
            .signal_ready_for_next_epoch(realm_id, epoch_number);

        let gas_estimate = match func
            .estimate_gas()
            .instrument(debug_span!("estimate_gas"))
            .await
        {
            Ok(gas_estimate) => gas_estimate,
            Err(e) => {
                let err_msg = decode_revert(&e, self.staking_contract.abi());
                debug!("Signal ready - gas estimate failed: {}", err_msg);
                return Err(blockchain_err(e, Some(err_msg)));
            }
        };

        let func_with_gas = func.gas(gas_estimate * U256::from(5));
        let tx = match func_with_gas
            .send()
            .instrument(debug_span!("signal_ready_for_next_epoch"))
            .await
        {
            Ok(tx) => tx,
            Err(e) => {
                let err_msg = decode_revert(&e, self.staking_contract.abi());
                return Err(blockchain_err(e, Some(err_msg)));
            }
        };

        // wait for tx to be confirmed
        match tx.await.map_err(|e| blockchain_err(e, None))? {
            Some(receipt) => {
                trace!(
                    "Confirmed signal ready for tx {:?}",
                    receipt.transaction_hash
                );

                if let Some(status) = receipt.status {
                    // if we did get a txn receipt, and it was a success, then we don't really need to confirm we actually signalled ready below, and can early exit.  the chain has already confirmed that the txn took effect.
                    if status == U64::from(1) {
                        return Ok(());
                    } else {
                        error!(
                            "We sent a txn to signal ready with hash {:?}, but it failed with status {:?}",
                            receipt.transaction_hash, status
                        );
                    }
                }
            }
            None => {
                error!(
                    "Failed to get transaction receipt for signal ready for next epoch, but the transaction might still have worked."
                );
            }
        }

        Err(blockchain_err_code(
            "Failed to signal ready for next epoch",
            EC::NodeRpcError,
            None,
        ))
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn rpc_lock_validators_for_next_epoch(&self, realm_id: u64) {
        // let realm_id = *self.chain_data_config_manager.realm_id.read().await;
        let realm_id = U256::from(realm_id);
        if realm_id == U256::from(0) {
            error!("Node is not yet assigned to a realm.  Can not lock validators for next epoch.");
            return;
        }

        {
            let func = self
                .staking_contract
                .lock_validators_for_next_epoch(realm_id);
            let gas_estimate = func.estimate_gas().await;
            match gas_estimate {
                Ok(gas_estimate) => {
                    let func_with_gas = func.gas(gas_estimate * U256::from(2));

                    let result = func_with_gas.send().await;

                    match result {
                        Ok(_) => info!("locked the validators"),
                        Err(e) => {
                            trace!(
                                "failed to lock validators in realm {} for next epoch (only one caller need succeed though) w/ err {:?}",
                                realm_id, e
                            );
                            trace!("{}", decode_revert(&e, self.staking_contract.abi()));
                        }
                    }
                }
                Err(e) => {
                    trace!(
                        "failed to estimate gas to lock validators in realm {} for next epoch (only one caller need succeed though) w/ err {:?}",
                        realm_id, e
                    );
                    trace!("{}", decode_revert(&e, self.staking_contract.abi()));
                }
            }
        }
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn rpc_advance_epoch(&self) {
        let realm_id = match self.peers_in_next_epoch().realm_id() {
            Ok(realm_id) => realm_id,
            Err(e) => {
                error!("Failed to get realm id: {:?}", e);
                return;
            }
        };
        let func = self.staking_contract.advance_epoch(realm_id);
        let gas_estimate = func
            .estimate_gas()
            .instrument(debug_span!("estimate_gas"))
            .await;
        match gas_estimate {
            Ok(gas_estimate) => {
                let func_with_gas = func.gas(gas_estimate * U256::from(5));
                let result = func_with_gas
                    .send()
                    .instrument(debug_span!("advance_epoch"))
                    .await;

                match result {
                    Ok(_) => debug!("advanced the epoch"),
                    Err(e) => {
                        debug!(
                            "failed to advance the epoch (only one caller need succeed though) w/ err {:?}",
                            e
                        );
                        debug!("{}", decode_revert(&e, self.staking_contract.abi()));
                    }
                }
            }
            Err(e) => {
                debug!(
                    "Failed to advance the epoch ( gas estimate failed ) : {}",
                    decode_revert(&e, self.staking_contract.abi())
                );
            }
        }
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn request_to_leave(&self) -> Result<()> {
        let realm_id = self.peers().realm_id()?;
        self.staking_contract
            .request_to_leave_as_node(realm_id)
            .send()
            .instrument(debug_span!("request_to_leave"))
            .await
            .map_err(|e| {
                error!(
                    "Failed to request to leave as node: {:?}",
                    decode_revert(&e, self.staking_contract.abi())
                );
                blockchain_err(e, Some("Failed to request to leave as node".to_string()))
            })?;
        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn request_to_join(&self) -> Result<()> {
        let Some(realm_id) = self.chain_data_config_manager.get_realm_id() else {
            return Err(unexpected_err("No realm id set", None));
        };

        let provider = self.staking_contract.client().provider().clone();
        let wallet_address = self
            .wallet_keys
            .verifying_key()
            .to_eth_address()
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some(
                        "Failed to convert verifying key to eth address during request to join."
                            .to_string(),
                    ),
                )
            })?;
        let balance = provider
            .get_balance(wallet_address, None)
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some(
                        "Failed to get balance of attested node wallet during request to join."
                            .to_string(),
                    ),
                )
            })?;

        if balance.is_zero() {
            return Err(blockchain_err(
                "Aborting request to join as attested node wallet balance is 0.",
                None,
            ));
        }

        let func = self
            .staking_contract
            .request_to_join_as_node(realm_id, self.staker_address);

        let gas_estimate = match func
            .estimate_gas()
            .instrument(debug_span!("estimate_gas"))
            .await
        {
            Ok(gas_estimate) => gas_estimate,
            Err(e) => {
                let msg = decode_revert(&e, self.staking_contract.abi());
                return Err(blockchain_err(e, Some(msg)));
            }
        };
        let func_with_gas = func.gas(gas_estimate * U256::from(5));
        let tx = func_with_gas
            .send()
            .instrument(debug_span!("request_to_join"))
            .await
            .map_err(|e| blockchain_err(e, None))?
            .interval(Duration::from_millis(500));

        match tx.await.map_err(|e| blockchain_err(e, None))? {
            Some(receipt) => {
                trace!(
                    "Confirmed request to join tx {:?}",
                    receipt.transaction_hash
                );
            }
            None => {
                error!("Failed to get transaction receipt for request to join");
            }
        }

        Ok(())
    }
}
