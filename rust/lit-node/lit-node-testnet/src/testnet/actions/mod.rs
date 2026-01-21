use crate::testnet::datil::contracts::DatilContracts;

use super::WhichTestnet;
use super::contracts::Contracts;
use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::utils;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::signers::Wallet;
// use lit_node::peers::peer_reviewer::MAX_COMPLAINT_REASON_VALUE;
pub const MAX_COMPLAINT_REASON_VALUE: u8 = 4;
use std::time::Duration;
use std::{fmt::Display, sync::Arc};
use tracing::{debug, info};

pub mod config;
pub mod epochs;
pub mod keysets;
pub mod network_state;
pub mod payment_delegation;
pub mod realms;
pub mod validators;
#[derive(Clone, Debug)]
pub struct Actions {
    contracts: Contracts,
    datil_contracts: DatilContracts,
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

impl Display for NetworkState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Actions {
    pub fn new(
        contracts: Contracts,
        datil_contracts: DatilContracts,
        deployer_signing_provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        which_testnet: WhichTestnet,
        deploy_address: Address,
    ) -> Self {
        Self {
            contracts,
            datil_contracts,
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

    pub fn datil_contracts(&self) -> &DatilContracts {
        &self.datil_contracts
    }

    pub async fn lit_token_balance(&self, address: Address) -> U256 {
        self.contracts
            .lit_token
            .balance_of(address)
            .call()
            .await
            .unwrap()
    }

    pub async fn sleep_random_millis(&self, min: u64, max: u64) {
        use rand::Rng;
        let millis = rand::thread_rng().gen_range(min..max);
        info!("Sleeping a test for {} millis.", millis);
        tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
    }

    #[doc = "Sleep for a number of milliseconds"]
    pub async fn sleep_millis(&self, millis: u64) {
        info!("Sleeping a test for {} millis.", millis);
        tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
    }

    #[doc = "Fast forward by a number of blocks"]
    pub async fn increase_blockchain_timestamp(&self, seconds_to_increase: usize) {
        let deployer_provider = self.deployer_provider().clone();
        Self::do_increase_blockchain_timestamp(deployer_provider, seconds_to_increase).await;
    }

    pub async fn do_increase_blockchain_timestamp(
        deployer_provider: Arc<Provider<Http>>,
        seconds_to_increase: usize,
    ) {
        // get most recent block timestamp
        let block = deployer_provider
            .get_block(deployer_provider.get_block_number().await.unwrap())
            .await
            .unwrap()
            .expect("Error getting block");
        let block_timestamp_before = block.timestamp;
        debug!("block_timestamp_before- {}", block_timestamp_before);

        let timestamp = Duration::from_secs(block_timestamp_before.as_u64())
            + Duration::from_secs(seconds_to_increase.try_into().unwrap());
        debug!("timestamp- {}", timestamp.as_secs());

        let res: Result<(), ProviderError> = deployer_provider
            .request("evm_setNextBlockTimestamp", [timestamp.as_secs()])
            .await;

        match res {
            Ok(r) => info!(
                "Successfully increased blockchain timestamp by {:?} seconds: {:?}",
                seconds_to_increase, r
            ),
            Err(e) => {
                info!("Error increasing blockchain timestamp: {:?}", e);
                panic!("{}", e);
            }
        }

        // mine a block
        let mine_block_res: Result<(), ProviderError> = deployer_provider
            .request("anvil_mine", [utils::serialize(&1), utils::serialize(&0)])
            .await;
        match mine_block_res {
            Ok(r) => info!("Successfully mined block: {:?}", r),
            Err(e) => {
                info!("Error mining block: {:?}", e);
                panic!("{}", e);
            }
        }

        let block = deployer_provider
            .get_block(deployer_provider.get_block_number().await.unwrap())
            .await
            .unwrap()
            .expect("Error getting block");
        let block_timestamp_after = block.timestamp;
        debug!("block_timestamp_after- {}", block_timestamp_after);
    }

    #[doc = "Fast forward by a number of blocks"]
    pub async fn fast_forward_blocks(&self, blocks_to_mine: usize) {
        info!("Fast forwarding by {:?} blocks...", blocks_to_mine);
        let command = match self.which_testnet {
            WhichTestnet::Anvil => "anvil_mine",
            WhichTestnet::Hardhat => "hardhat_mine",
            _ => panic!("Unsupported network for fastforwarding blocks!"),
        };

        let block_num_before = self.deployer_provider().get_block_number().await.unwrap();

        let mine_blocks_res: Result<(), ProviderError> = self
            .deployer_provider()
            .request(
                command,
                [
                    utils::serialize(&format!("0x{:X}", blocks_to_mine)),
                    utils::serialize(&0),
                ],
            )
            .await;

        match mine_blocks_res {
            Ok(r) => debug!("Successfully mined {:?} blocks: {:?}", blocks_to_mine, r),
            Err(e) => info!(
                "Error mining blocks - you can ignore this on Anvil and look at the below Block Number message to check that it actually fast forwarded {:?}",
                e
            ),
        }

        let block_num_after = self.deployer_provider().get_block_number().await.unwrap();
        debug!(
            "Block number before fast forwarding: {}, Block number after fast forwarding: {}",
            block_num_before, block_num_after
        );
    }
}
