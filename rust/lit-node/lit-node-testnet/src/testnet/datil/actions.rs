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

    pub async fn get_latest_block_timestamp(&self) -> Result<U256> {
        let block = self
            .deployer_provider()
            .get_block(
                self.deployer_provider()
                    .get_block_number()
                    .await
                    .map_err(|e| anyhow::anyhow!("Error getting block number: {:?}", e))?,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Error getting block: {:?}", e))?
            .ok_or_else(|| anyhow::anyhow!("Error getting block"))?;
        Ok(block.timestamp)
    }

    pub async fn get_epoch_end_time(&self) -> Result<U256> {
        let epoch = self.contracts.staking.epoch().call().await?;
        Ok(epoch.end_time)
    }

    pub async fn set_epoch_end_time(&self, new_end_time: U256) -> Result<()> {
        let cc = self.contracts.staking.set_epoch_end_time(new_end_time);
        if !Contracts::process_contract_call(cc, "set_epoch_end_time").await {
            return Err(anyhow::anyhow!("Error setting epoch end time"));
        }
        Ok(())
    }

    pub async fn set_epoch_end_time_from_now(&self, seconds_from_now: u64) -> Result<()> {
        let current_time = self.get_latest_block_timestamp().await?;
        let new_end_time = current_time + U256::from(seconds_from_now);
        self.set_epoch_end_time(new_end_time).await
    }

    pub async fn get_epoch_length(&self) -> Result<U256> {
        let epoch = self.contracts.staking.epoch().call().await?;
        Ok(epoch.epoch_length)
    }

    pub async fn lit_token_balance(&self, address: Address) -> U256 {
        self.contracts
            .lit_token
            .balance_of(address)
            .call()
            .await
            .unwrap()
    }

    pub async fn get_current_validators(&self) -> Vec<H160> {
        self.contracts
            .staking
            .get_validators_in_current_epoch()
            .call()
            .await
            .expect("Error getting validators from chain")
    }

    pub async fn get_current_validator_structs(&self) -> Vec<Validator> {
        self.contracts
            .staking
            .get_validators_structs_in_current_epoch()
            .call()
            .await
            .expect("Error getting validator structs from chain")
    }

    pub async fn get_validator_struct(&self, staker_address: Address) -> Validator {
        self.contracts
            .staking
            .validators(staker_address)
            .call()
            .await
            .expect("Error getting validator struct from chain")
    }

    pub async fn get_next_validators(&self) -> Vec<H160> {
        self.contracts
            .staking
            .get_validators_in_next_epoch()
            .call()
            .await
            .expect("Error getting next validators from chain")
    }

    pub async fn get_next_validator_structs(&self) -> Vec<Validator> {
        self.contracts
            .staking
            .get_validators_structs_in_next_epoch()
            .call()
            .await
            .expect("Error getting next validator structs from chain")
    }

    pub async fn get_current_validator_count(&self) -> u32 {
        self.get_current_validators().await.len() as u32
    }

    #[doc = "Wait for state to become active again (DKGs run, advance)"]
    pub async fn wait_for_active(&self) {
        info!("Waiting for network to become active again");
        loop {
            let res = self.contracts.staking.state().call().await;
            match res {
                Ok(res) => {
                    if res == 0 {
                        info!("Network is active");
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
                Err(..) => {
                    debug!(
                        "Error checking if validator state is active : {:?}",
                        res.unwrap_err()
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                }
            }
        }

        info!("Sleeping for 2 seconds to make sure nodes sync up with new peer state...");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }

    #[doc = "Wait for state to become locked"]
    pub async fn wait_for_lock(&self) {
        info!("Waiting for nodes to be locked");
        let res = self
            .contracts
            .staking
            .get_validators_in_next_epoch()
            .call()
            .await;

        if res.is_err() {
            panic!(
                "Error getting validators in next epoch: {:?}",
                res.unwrap_err()
            );
        }

        info!("Validators in next epoch: {:?}", res.unwrap());

        loop {
            let res = self.contracts.staking.state().call().await;

            match res {
                Ok(res) => {
                    debug!("State is {:?}", res);
                    if res == 1 {
                        info!("Next validator set is locked");
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
                Err(..) => {
                    info!(
                        "Error checking if validators in next epoch are locked : {:?}",
                        res.unwrap_err()
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                } // _ => tokio::time::sleep(std::time::Duration::from_secs(1)).await,
            }
        }
    }

    pub async fn wait_for_recovery_keys(&self) {
        info!("Waiting for recovery keys!");

        // Check whether the recovery keys are registered on the chain.
        loop {
            if self
                .contracts
                .backup_recovery
                .is_recovery_dkg_completed()
                .call()
                .await
                .unwrap()
            {
                info!("Got recovery keys!");
                break;
            }

            let _r = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    pub async fn wait_for_recovery_status(&self, status: u8) {
        info!(
            "Waiting for the nodes to report status {} to the BackupRecovery contract!",
            status
        );
        // Check whether the nodes reported the status to the contract.
        loop {
            let node_statuses = self
                .contracts
                .backup_recovery
                .get_node_recovery_status()
                .call()
                .await
                .unwrap();

            if node_statuses.iter().all(|x| x.status == status) {
                break;
            }

            let _r = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
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

    #[doc = "Wait for initial epoch to end - a collection of functions to set the state to active and lock validators for next epoch."]
    pub async fn wait_for_initial_epoch(&self) {
        self.start_initial_epoch(true).await
    }

    /// Wait for the initial epoch to end - a collection of functions to set the state to active and lock validators for next epoch.
    pub async fn start_initial_epoch(&self, wait_for_active: bool) {
        let deploy_address = self.deploy_address;
        info!(
            "Starting epoch with validators: {:?}",
            self.contracts
                .staking
                .validators(deploy_address)
                .call()
                .await
                .unwrap()
        );

        info!(
            "Staking state (wait_for_initial_epoch) : {:?}",
            self.contracts.staking.state().call().await
        );

        if wait_for_active {
            self.wait_for_active().await;
        }

        info!("Initial Epoch has started.");
    }

    #[doc = "Lock validators for next epoch"]
    pub async fn lock_validators_for_next_epoch(&self) {
        let state = self.contracts.staking.state().call().await;
        if state.is_err() {
            error!("Error getting state...");
            return;
        }
        info!("Staking state (pre lock) : {:?}", state);

        let lock_func = self.contracts.staking.lock_validators_for_next_epoch();
        let lock_res = lock_func.send().await;
        warn!("Locking validators for next epoch: {:?}", lock_res);
        // assert!(lock_res.is_ok());
        info!(
            "Staking state (post lock) : {:?}",
            self.contracts.staking.state().call().await
        );
    }

    pub async fn set_complaint_reason_config(
        &self,
        reason: U256,
        config: ComplaintConfig,
    ) -> Result<()> {
        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            self.deployer_signing_provider.clone(),
        );

        let cc = staking.set_complaint_config(reason, config);
        if !Contracts::process_contract_call(cc, "set complaint config").await {
            return Err(anyhow::anyhow!("Error setting complaint config"));
        }

        Ok(())
    }

    pub async fn ensure_node_unstaked(
        &self,
        node_account: NodeAccount,
    ) -> Result<NodeStakingStatus> {
        info!("Unstaking node: {:?}", node_account.staker_address);

        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            node_account.signing_provider.clone(),
        );

        let tx = staking.request_to_leave();

        let result = tx.send().await;

        if result.is_err() {
            panic!("Error unstaking node: {:?}", result.unwrap_err());
        }

        Ok(NodeStakingStatus::Unstaked)
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
        // get most recent block timestamp
        let block = self
            .deployer_provider()
            .get_block(self.deployer_provider().get_block_number().await.unwrap())
            .await
            .unwrap()
            .expect("Error getting block");
        let block_timestamp_before = block.timestamp;
        debug!("block_timestamp_before- {}", block_timestamp_before);

        let timestamp = Duration::from_secs(block_timestamp_before.as_u64())
            + Duration::from_secs(seconds_to_increase.try_into().unwrap());
        debug!("timestamp- {}", timestamp.as_secs());

        let res: Result<(), ProviderError> = self
            .deployer_provider()
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
        let mine_block_res: Result<(), ProviderError> = self
            .deployer_provider()
            .request("anvil_mine", [utils::serialize(&1), utils::serialize(&0)])
            .await;
        match mine_block_res {
            Ok(r) => info!("Successfully mined block: {:?}", r),
            Err(e) => {
                info!("Error mining block: {:?}", e);
                panic!("{}", e);
            }
        }

        let block = self
            .deployer_provider()
            .get_block(self.deployer_provider().get_block_number().await.unwrap())
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

    pub async fn get_current_epoch(&self) -> U256 {
        let get_res = self.contracts.staking.epoch().call().await;

        if get_res.is_err() {
            error!("Error in get_epoch: {}", get_res.err().unwrap());
            return U256::zero();
        }
        let epoch = get_res.unwrap();
        let epoch_number = epoch.number;

        epoch_number
    }

    pub async fn wait_for_epoch(&self, epoch: U256) {
        info!(
            "Waiting for epoch {}.  Current epoch is {}.",
            epoch,
            self.get_current_epoch().await
        );
        loop {
            let current_epoch = self.get_current_epoch().await;
            if current_epoch == epoch {
                info!("Advanced! Current epoch is {}.", epoch);
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        info!("Sleeping for 2 seconds to make sure nodes sync up with new peer state...");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}
