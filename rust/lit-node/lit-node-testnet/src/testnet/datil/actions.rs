use crate::testnet::datil::contracts::Contracts;
use crate::testnet::{NodeAccount, WhichTestnet};
use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::utils;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain_datil::contracts::pubkey_router::RootKey;
use lit_blockchain_datil::contracts::staking::ComplaintConfig;
use lit_blockchain_datil::contracts::staking::{Staking, Validator};
use lit_core::utils::binary::bytes_to_hex;
use lit_node_common::models::NodeStakingStatus;
use lit_node_core::CurveType;
// use lit_node::peers::peer_reviewer::MAX_COMPLAINT_REASON_VALUE;
pub const MAX_COMPLAINT_REASON_VALUE: u8 = 4;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, warn};

pub struct RootKeyConfig {
    pub curve_type: CurveType,
    pub count: usize,
}

#[derive(Clone, Debug)]
pub struct Actions {
    contracts: Contracts,
    deployer_signing_provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    which_testnet: WhichTestnet,
    deploy_address: Address,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NetworkState {
    Active = 0,
    NextValidatorSetLocked = 1,
    ReadyForNextEpoch = 2,
    Unlocked = 3,
    Paused = 4,
    Restore = 5,
    Unknown = 255,
}

impl From<u8> for NetworkState {
    fn from(value: u8) -> Self {
        match value {
            0 => NetworkState::Active,
            1 => NetworkState::NextValidatorSetLocked,
            2 => NetworkState::ReadyForNextEpoch,
            3 => NetworkState::Unlocked,
            4 => NetworkState::Paused,
            5 => NetworkState::Restore,
            _ => NetworkState::Unknown,
        }
    }
}

impl Actions {
    pub fn new(
        contracts: Contracts,
        deployer_signing_provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        which_testnet: WhichTestnet,
        deploy_address: Address,
    ) -> Self {
        Self {
            contracts,
            deployer_signing_provider,
            which_testnet,
            deploy_address,
        }
    }

    pub fn deployer_signing_provider(
        &self,
    ) -> Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
        self.deployer_signing_provider.clone()
    }

    pub fn deployer_provider(&self) -> Arc<Provider<Http>> {
        self.deployer_signing_provider.inner().clone()
    }

    pub fn contracts(&self) -> &Contracts {
        &self.contracts
    }

    pub async fn lit_token_balance(&self, address: Address) -> U256 {
        self.contracts
            .lit_token
            .balance_of(address)
            .call()
            .await
            .unwrap()
    }

    pub async fn get_all_root_keys(&self) -> Option<Vec<RootKey>> {
        let staking_address = self.contracts.staking.address();
        let root_keys = self
            .contracts
            .pubkey_router
            .get_root_keys(staking_address)
            .call()
            .await
            .unwrap();

        if !root_keys.is_empty() {
            info!("Got root keys!");
            tracing::trace!("Root keys: {:?}", root_keys);
            return Some(root_keys);
        } else {
            info!("No root keys yet for contract {:?}", staking_address);
        }

        None
    }

    pub async fn get_root_keys(&self, curve_type: CurveType) -> Option<Vec<String>> {
        let all_root_keys = self.get_all_root_keys().await;
        all_root_keys.as_ref()?;
        let all_root_keys: Vec<RootKey> = all_root_keys.unwrap();

        let root_keys: Vec<String> = all_root_keys
            .iter()
            .filter(|k| k.key_type == curve_type.into())
            .map(|k| bytes_to_hex(k.pubkey.clone()))
            .collect::<Vec<String>>();

        Some(root_keys)
    }
}
