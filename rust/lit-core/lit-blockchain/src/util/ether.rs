use ethers::prelude::Address;
use ethers::types::TransactionReceipt;
use ethers::utils::keccak256;
use serde_json::{Map, Value};

use crate::error::{Result, conversion_err};

const BLOCK_GAS_LIMIT: u64 = 30_000_000;

/// This method is provided primarily for errors/logging.
pub fn transaction_receipt_to_serde(txn_rec: &TransactionReceipt) -> Value {
    let mut map: Map<String, Value> = Map::new();
    map.insert("transaction_hash".into(), Value::String(format!("{}", txn_rec.transaction_hash)));
    if let Some(status) = txn_rec.status {
        map.insert("status".into(), Value::String(format!("{status}")));
    }

    Value::Object(map)
}

/// Logic borrowed from ethers utils::secret_key_to_address
pub fn public_key_to_address(public_key: &[u8]) -> Result<Address> {
    if public_key[0] != 0x04 {
        return Err(conversion_err(
            "public_key_to_address given public key without 0x04 prefix", None,
        ));
    }

    let hash = keccak256(&public_key[1..]);

    Ok(Address::from_slice(&hash[12..]))
}

pub mod middleware {
    use alloy::{
        signers::{Signer, local::PrivateKeySigner},
        sol_types::eip712_domain,
    };
    use async_trait::async_trait;
    use ethers::{
        contract::ContractError,
        core::k256::ecdsa::SigningKey,
        providers::{Middleware, MiddlewareError, PendingTransaction},
        types::{
            BlockId, Bytes, Eip1559TransactionRequest, U256, transaction::eip2718::TypedTransaction,
        },
        utils::secret_key_to_address,
    };
    use thiserror::Error;

    use crate::contracts::forwarder;

    use super::BLOCK_GAS_LIMIT;

    pub mod alloy_structs {
        use alloy::sol;
        use serde::Serialize;

        sol! {
            #[derive(Debug, Serialize)]
            struct ForwardRequest {
                address from;
                address to;
                uint256 value;
                uint256 gas;
                uint256 nonce;
                bytes data;
            }
        }
    }

    #[derive(Debug)]
    pub struct EIP2771GasRelayerMiddleware<M> {
        inner: M,
        /// This is the signer that will sign the meta-transaction. This is NOT the signer that
        /// will send the transaction to the Forwarder contract.
        transaction_signer: SigningKey,
        forwarder_with_gas_signer: forwarder::Forwarder<M>,
    }

    impl<M> EIP2771GasRelayerMiddleware<M> {
        pub fn new(
            inner: M, transaction_signer: SigningKey,
            forwarder_with_gas_signer: forwarder::Forwarder<M>,
        ) -> Self {
            Self { inner, transaction_signer, forwarder_with_gas_signer }
        }
    }

    #[derive(Error, Debug)]
    pub enum EIP2771GasRelayerMiddlewareError<M: Middleware> {
        #[error("{0}")]
        SignerError(String),

        #[error("{0}")]
        MiddlewareError(M::Error),

        #[error("{0}")]
        ContractRevert(String),

        #[error("{0}")]
        ContractError(ContractError<M>),

        #[error("Failed to get nonce")]
        FailedToGetNonce(String),

        #[error("{0}")]
        FailedToEstimateGas(M::Error),

        #[error("Missing chain ID")]
        MissingChainID(String),

        #[error("Missing to address")]
        MissingToAddress,

        #[error("Missing data")]
        MissingData,

        #[error("Conversion error")]
        ConversionError(String),

        #[error("Unsupported transaction type")]
        UnsupportedTransactionType,
    }

    impl<M> MiddlewareError for EIP2771GasRelayerMiddlewareError<M>
    where
        M: Middleware,
    {
        type Inner = M::Error;

        fn from_err(src: M::Error) -> Self {
            EIP2771GasRelayerMiddlewareError::MiddlewareError(src)
        }

        fn as_inner(&self) -> Option<&Self::Inner> {
            match self {
                EIP2771GasRelayerMiddlewareError::MiddlewareError(e) => Some(e),
                EIP2771GasRelayerMiddlewareError::ContractError(e) => e.as_middleware_error(),
                EIP2771GasRelayerMiddlewareError::FailedToEstimateGas(e) => Some(e),
                _ => None,
            }
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    impl<M> Middleware for EIP2771GasRelayerMiddleware<M>
    where
        M: Middleware,
    {
        type Error = EIP2771GasRelayerMiddlewareError<M>;
        type Provider = M::Provider;
        type Inner = M;

        fn inner(&self) -> &M {
            &self.inner
        }

        async fn send_transaction<Tx: Into<TypedTransaction> + Send + Sync>(
            &self, tx: Tx, block: Option<BlockId>,
        ) -> Result<PendingTransaction<'_, Self::Provider>, Self::Error> {
            // Get the nonce for the transaction signer
            let transaction_signer_address = secret_key_to_address(&self.transaction_signer);
            let nonce = self
                .forwarder_with_gas_signer
                .get_nonce(transaction_signer_address)
                .call()
                .await
                .map_err(|e| EIP2771GasRelayerMiddlewareError::FailedToGetNonce(e.to_string()))?;

            let typed_tx = tx.into();

            // Estimate the gas needed for the transaction.
            let gas = self
                .inner()
                .estimate_gas(&typed_tx, block)
                .await
                .map_err(|e| EIP2771GasRelayerMiddlewareError::FailedToEstimateGas(e))?;
            // Apply a multiplier to the gas estimate for safety. This is capped at 30M.
            let gas = {
                if gas * U256::from(5) > U256::from(BLOCK_GAS_LIMIT) {
                    U256::from(BLOCK_GAS_LIMIT)
                } else {
                    gas * U256::from(5)
                }
            };

            let typed_tx: Eip1559TransactionRequest = match typed_tx {
                TypedTransaction::Legacy(legacy_tx) => {
                    // Map to Eip1559TransactionRequest
                    let mut eip1559_tx = Eip1559TransactionRequest::default();
                    eip1559_tx.from = legacy_tx.from;
                    eip1559_tx.to = legacy_tx.to;
                    eip1559_tx.value = legacy_tx.value;
                    eip1559_tx.gas = legacy_tx.gas;
                    eip1559_tx.nonce = legacy_tx.nonce;
                    eip1559_tx.data = legacy_tx.data;
                    eip1559_tx.chain_id = legacy_tx.chain_id;
                    eip1559_tx
                }
                TypedTransaction::Eip1559(tx) => tx,
                _ => return Err(EIP2771GasRelayerMiddlewareError::UnsupportedTransactionType),
            };

            // Get the signature over the typed data
            // Here, we use alloy to generate the typed data signature because there is a bug in ethers-rs that causes
            // the encoding for the data field (Bytes) to be incorrect.
            let signature = {
                let chain_id = {
                    match typed_tx.chain_id {
                        Some(chain_id) => chain_id.as_u64(),
                        None => {
                            let chain_id = self.inner().get_chainid().await.map_err(|e| {
                                EIP2771GasRelayerMiddlewareError::MissingChainID(e.to_string())
                            })?;
                            chain_id.as_u64()
                        }
                    }
                };

                let alloy_domain = eip712_domain! {
                    name: "GSNv2 Forwarder",
                    version: "0.0.1",
                    chain_id: chain_id,
                    verifying_contract: alloy::primitives::Address::new(self.forwarder_with_gas_signer.address().0),
                };

                let alloy_struct = alloy_structs::ForwardRequest {
                    from: alloy::primitives::Address::new(transaction_signer_address.0),
                    to: alloy::primitives::Address::new(
                        typed_tx
                            .to
                            .clone()
                            .ok_or(EIP2771GasRelayerMiddlewareError::MissingToAddress)?
                            .as_address()
                            .ok_or(EIP2771GasRelayerMiddlewareError::ConversionError(
                                "To is not an address".to_string(),
                            ))?
                            .0,
                    ),
                    value: alloy::primitives::U256::from_limbs(
                        U256::from(typed_tx.value.unwrap_or(U256::from(0))).0,
                    ),
                    gas: alloy::primitives::U256::from_limbs(gas.0),
                    nonce: alloy::primitives::U256::from_limbs(U256::from(nonce).0),
                    data: alloy::primitives::Bytes::from(
                        typed_tx
                            .data
                            .clone()
                            .ok_or(EIP2771GasRelayerMiddlewareError::MissingData)?
                            .to_vec(),
                    ),
                };

                // Use the meta wallet to sign the request
                let meta_signer: PrivateKeySigner =
                    PrivateKeySigner::from_signing_key(self.transaction_signer.clone());
                let alloy_sig = meta_signer
                    .sign_typed_data(&alloy_struct, &alloy_domain)
                    .await
                    .map_err(|e| EIP2771GasRelayerMiddlewareError::SignerError(e.to_string()))?;
                Bytes::from(alloy_sig.as_bytes())
            };

            let forwarder_execute_req = forwarder::ForwardRequest {
                from: transaction_signer_address,
                to: typed_tx
                    .to
                    .ok_or(EIP2771GasRelayerMiddlewareError::MissingToAddress)?
                    .as_address()
                    .ok_or(EIP2771GasRelayerMiddlewareError::ConversionError(
                        "To is not an address".to_string(),
                    ))?
                    .to_owned(),
                value: typed_tx.value.unwrap_or(U256::from(0)),
                gas,
                nonce,
                data: typed_tx.data.ok_or(EIP2771GasRelayerMiddlewareError::MissingData)?,
            };

            let fn_call =
                self.forwarder_with_gas_signer.execute(forwarder_execute_req, signature).gas(gas);
            let tx = fn_call.send().await;

            match tx {
                Err(e) => {
                    match e.decode_contract_revert::<forwarder::ForwarderErrors>().ok_or(
                        EIP2771GasRelayerMiddlewareError::ConversionError(
                            "Failed to decode contract revert".to_string(),
                        ),
                    )? {
                        forwarder::ForwarderErrors::SignatureDoesNotMatch(chain_e) => {
                            return Err(EIP2771GasRelayerMiddlewareError::ContractRevert(
                                chain_e.to_string(),
                            ));
                        }
                        forwarder::ForwarderErrors::TransactionRevertedSilently(chain_e) => {
                            return Err(EIP2771GasRelayerMiddlewareError::ContractRevert(
                                chain_e.to_string(),
                            ));
                        }
                        _ => {
                            return Err(EIP2771GasRelayerMiddlewareError::ContractError(e));
                        }
                    }
                }
                Ok(tx) => Ok(PendingTransaction::new(tx.tx_hash(), self.inner().provider())),
            }
        }
    }
}
