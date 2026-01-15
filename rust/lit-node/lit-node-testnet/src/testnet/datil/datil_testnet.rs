use crate::testnet::chain::ChainTrait;
use crate::testnet::datil::contracts::DatilContracts;
use crate::testnet::{NodeAccount, chain::anvil::Anvil};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::types::Address;
use lit_blockchain::resolver::rpc::ENDPOINT_MANAGER;
use lit_node_common::coms_keys::ComsKeys;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

use super::DatilTestnet;
use lit_blockchain::resolver::rpc::RpcHealthcheckPoller;
use lit_blockchain_lite::contracts::pubkey_router::{PubkeyRouter, RootKey};

#[derive(Clone, Debug, Deserialize)]
pub struct DatilNodeAccount {
    pub node_address: Address,
    pub node_address_private_key: ethers::types::H256,
    pub staker_address: Address,
    pub staker_address_private_key: ethers::types::H256,
}

impl DatilTestnet {
    pub async fn new(
        total_num_validators: usize,
        state_cache_path: String,
        contract_resolver_address: Address,
    ) -> Self {
        let datil_chain = Box::new(Anvil::new(
            total_num_validators,
            true,
            Some(state_cache_path.clone()),
        )) as Box<dyn ChainTrait>;
        let process = datil_chain.start_chain().await;

        let mut provider = ENDPOINT_MANAGER
            .get_provider(datil_chain.chain_name())
            .expect(&format!(
                "Error retrieving provider for chain {} - check name and/or rpc_config yaml.",
                datil_chain.chain_name()
            ));

        let provider_mut = Arc::make_mut(&mut provider);
        let provider = Arc::new(provider_mut.set_interval(Duration::from_millis(10)).clone());
        let deployer_signing_provider = datil_chain.deployer().signing_provider.clone();

        let contracts =
            DatilContracts::new(deployer_signing_provider.clone(), contract_resolver_address).await;

        let node_accounts =
            Self::load_node_accounts(datil_chain.chain_name(), datil_chain.chain_id()).await;

        Self {
            process,
            datil_chain,
            provider,
            node_accounts,
            deployer_signing_provider,
            contracts,
        }
    }

    pub fn shutdown(&mut self) {
        self.process.kill().unwrap_or_else(|e| {
            panic!(
                "Datil testnet process {:?} couldn't be killed: {}",
                self.process, e
            )
        });
    }

    // load the node accounts from the datil cache - matches the secrets in the cached state dump file.
    async fn load_node_accounts(chain_name: &str, chain_id: u64) -> Arc<Vec<NodeAccount>> {
        let provider = lit_blockchain::resolver::rpc::ENDPOINT_MANAGER
            .get_provider(chain_name)
            .unwrap();

        let cached_node_accounts_path = "tests/test_data/datil_cache/datil-node-accounts.json";
        let cached_node_accounts = std::fs::read_to_string(cached_node_accounts_path).unwrap();
        let cached_node_accounts: Vec<DatilNodeAccount> =
            serde_json::from_str(&cached_node_accounts).unwrap();

        let mut node_accounts = Vec::new();
        for datil_account in cached_node_accounts {
            let node_address = datil_account.node_address;
            let node_address_private_key = datil_account.node_address_private_key;
            let staker_address = datil_account.staker_address;
            let staker_address_private_key = datil_account.staker_address_private_key;

            let sk =
                SigningKey::from_bytes(k256::FieldBytes::from_slice(&staker_address_private_key.0))
                    .unwrap();
            let staker_wallet = LocalWallet::from(sk).with_chain_id(chain_id);
            let staker_signing_provider =
                Arc::new(SignerMiddleware::new(provider.clone(), staker_wallet));
            let coms_keys = ComsKeys::new();

            let node_account = NodeAccount {
                node_address,
                signing_provider: staker_signing_provider,
                node_address_private_key,
                staker_address_private_key,
                staker_address,
                coms_keys,
            };
            node_accounts.push(node_account);
        }
        info!("Loaded {} node accounts from cache", node_accounts.len());
        Arc::new(node_accounts)
    }

    pub async fn set_root_keys(
        &self,
        src_root_keys: Vec<lit_blockchain::contracts::pubkey_router::RootKey>,
    ) {
        let staking_address = self.contracts.staking.address();
        let func = self
            .contracts
            .pubkey_router
            .admin_reset_root_keys(staking_address);
        let tx = func.send().await.unwrap();
        let _receipt = tx.await.unwrap();
        info!("Called admin_reset_root_keys on the Datil chain to clear root keys");

        let root_keys: Vec<RootKey> = src_root_keys
            .iter()
            .map(|rk| RootKey {
                pubkey: rk.pubkey.clone(),
                key_type: rk.key_type.into(),
            })
            .collect();

        let address = self.contracts.pubkey_router.address();
        info!(
            "Voting for {} root keys on the Datil chain",
            root_keys.len()
        );
        for node_account in self.node_accounts.iter() {
            let sk = SigningKey::from_bytes(k256::FieldBytes::from_slice(
                &node_account.node_address_private_key.0,
            ))
            .unwrap();
            let node_wallet = LocalWallet::from(sk).with_chain_id(self.datil_chain.chain_id());
            let client = Arc::new(SignerMiddleware::new(self.provider.clone(), node_wallet));

            let local_pubkey_router = PubkeyRouter::new(address, client);
            let func = local_pubkey_router.vote_for_root_keys(staking_address, root_keys.clone());
            let tx = func.send().await.unwrap();
            let _receipt = tx.await.unwrap();
            info!(
                "Node {} voted for root keys on the Datil chain",
                node_account.node_address
            );
        }
    }
}
