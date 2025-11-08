use crate::testnet::ProviderError;
use ethers::{
    middleware::SignerMiddleware,
    signers::{Signer, Wallet},
    types::{H160, U256},
};
use k256::ecdsa::SigningKey;
use lit_blockchain::contracts::payment_delegation::{
    PaymentDelegation, PaymentDelegationErrors, Restriction,
};
use lit_blockchain::util::decode_revert;
use lit_core::utils::binary::bytes_to_hex;
use std::{sync::Arc, time::Duration};
use tracing::{error, info, trace};

use super::actions::Actions;

impl Actions {
    pub async fn fund_wallet(&self, wallet: &Wallet<SigningKey>, amount: &str) {
        let provider = self.deployer_provider();

        let res: Result<(), ProviderError> = provider
            .request(
                "anvil_setBalance",
                [
                    format!("0x{}", bytes_to_hex(wallet.address())),
                    amount.to_string(),
                ],
            )
            .await;

        if let Err(e) = res {
            panic!("Couldn't set balance: {:?}", e);
        }
    }

    pub async fn create_payment_delegation_entry(
        &self,
        payer_wallet: &Wallet<SigningKey>,
        user_we_are_paying_for: H160,
        requests_per_period: u32,
        period_seconds: u32,
        total_max_price: u128,
    ) {
        let payment_delegation_contract_address = self.contracts().payment_delegation.address();
        let provider = self.deployer_provider();

        let client = SignerMiddleware::new(provider.clone(), payer_wallet.clone());
        let payment_delegation_contract =
            PaymentDelegation::new(payment_delegation_contract_address, Arc::new(client));

        let tx = payment_delegation_contract.set_restriction(Restriction {
            total_max_price,
            requests_per_period: U256::from(requests_per_period),
            period_seconds: U256::from(period_seconds),
        });

        let pending_tx = tx.send().await;
        if let Err(e) = pending_tx {
            error!("Problem creating payment delegation restriction: {:?}", e);

            let error_with_type: Option<PaymentDelegationErrors> = e.decode_contract_revert();
            if let Some(error) = error_with_type {
                panic!(
                    "Couldn't create payment delegation restriction - contract revert: {:?}",
                    error
                );
            }

            let decoded = decode_revert(&e, payment_delegation_contract.abi());
            panic!(
                "Couldn't create payment delegation restriction: {:?}",
                decoded
            );
        }

        let pending_tx = pending_tx.unwrap().interval(Duration::from_millis(100));
        let receipt = pending_tx.await.unwrap().expect("No receipt from txn");
        trace!(
            "Payment delegation restriction setting receipt: {:?}",
            receipt
        );

        let tx = payment_delegation_contract.delegate_payments(user_we_are_paying_for);
        let pending_tx = tx.send().await;
        if let Err(e) = pending_tx {
            error!("Problem creating payment delegation: {:?}", e);

            let error_with_type: Option<PaymentDelegationErrors> = e.decode_contract_revert();
            if let Some(error) = error_with_type {
                panic!(
                    "Couldn't create payment delegation - contract revert: {:?}",
                    error
                );
            }

            let decoded = decode_revert(&e, payment_delegation_contract.abi());
            panic!("Couldn't create payment delegation: {:?}", decoded);
        }

        let pending_tx = pending_tx.unwrap().interval(Duration::from_millis(100));
        let receipt = pending_tx.await.unwrap().expect("No receipt from txn");
        trace!("Payment delegation entry creation receipt: {:?}", receipt);

        let payers_and_restrictions = payment_delegation_contract
            .get_payers_and_restrictions(vec![user_we_are_paying_for])
            .call()
            .await
            .unwrap();
        info!(
            "Payers/Restrictions for user {}: {:?}",
            user_we_are_paying_for, payers_and_restrictions
        );
    }
}
