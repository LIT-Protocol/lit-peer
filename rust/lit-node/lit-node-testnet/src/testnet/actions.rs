use super::contracts::{Contracts, StakingContractGlobalConfig, StakingContractRealmConfig};
use super::{NodeAccount, WhichTestnet};
use crate::models::VotingStatusToKickValidator;
use crate::node_collection::{ensure_min_node_epoch, handshake_returns_keys};
use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::utils;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain::contracts::pubkey_router::RootKey;
use lit_blockchain::contracts::staking::{ComplaintConfig, UncompressedK256Key, staking};
use lit_blockchain::contracts::{
    lit_token::lit_token::LITToken,
    staking::{Staking, StakingErrors, Validator},
};
use lit_core::utils::binary::bytes_to_hex;
use lit_node_common::models::NodeStakingStatus;
// use lit_node::peers::peer_reviewer::MAX_COMPLAINT_REASON_VALUE;
pub const MAX_COMPLAINT_REASON_VALUE: u8 = 4;
use super::PeerItem;
use lit_node_common::utils::parse_version;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, warn};

const DEFAULT_TIMELOCK_SECONDS: u64 = 86400 * 120; // 1 day
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

    pub async fn get_epoch_end_time(&self, realm_id: U256) -> Result<U256> {
        let epoch = self.contracts.staking.epoch(realm_id).call().await?;
        Ok(epoch.end_time)
    }

    pub async fn set_epoch_end_time(&self, realm_id: U256, new_end_time: U256) -> Result<()> {
        let cc = self
            .contracts
            .staking
            .set_epoch_end_time(realm_id, new_end_time);
        if !Contracts::process_contract_call(cc, "set_epoch_end_time").await {
            return Err(anyhow::anyhow!("Error setting epoch end time"));
        }
        Ok(())
    }

    pub async fn set_epoch_end_time_from_now(&self, realm_id: U256, length: U256) -> Result<()> {
        let current_epoch_end_time = self.get_epoch_end_time(realm_id).await?;
        let lastest_block_time = self.get_latest_block_timestamp().await?;
        let new_end_time = lastest_block_time + U256::from(length);

        use chrono::{DateTime, Utc};

        let n_current_epoch_end_time =
            DateTime::<Utc>::from_timestamp(current_epoch_end_time.as_u64() as i64, 0)
                .expect("Invalid Unix timestamp")
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();

        let n_new_end_time = DateTime::<Utc>::from_timestamp(new_end_time.as_u64() as i64, 0)
            .expect("Invalid Unix timestamp")
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let n_lastet_block_time =
            DateTime::<Utc>::from_timestamp(lastest_block_time.as_u64() as i64, 0)
                .expect("Invalid Unix timestamp");

        debug!(
            "Setting epoch end time to {} for realm {}.  Current epoch end time is {}.  Current latest block time is {}",
            n_new_end_time, realm_id, n_current_epoch_end_time, n_lastet_block_time
        );

        self.set_epoch_end_time(realm_id, new_end_time).await
    }

    pub async fn set_epoch_length(&self, realm_id: U256, epoch_length: U256) -> Result<()> {
        let cc = self
            .contracts
            .staking
            .set_epoch_length(realm_id, epoch_length);
        let r = Contracts::process_contract_call(cc, "set_epoch_length").await;
        if !r {
            return Err(anyhow::anyhow!("Error setting epoch length! "));
        }
        Ok(())
    }

    pub async fn get_epoch_length(&self, realm_id: U256) -> Result<U256> {
        let epoch = self.contracts.staking.epoch(realm_id).call().await?;
        Ok(epoch.epoch_length)
    }

    pub async fn set_epoch_state(&self, realm_id: U256, state: u8) -> Result<()> {
        let cc = self.contracts.staking.set_epoch_state(realm_id, state);
        let r = Contracts::process_contract_call(cc, "set_epoch_state").await;
        if !r {
            return Err(anyhow::anyhow!("Error setting epoch state! "));
        }
        Ok(())
    }

    pub async fn add_realm(&self) -> Result<u64> {
        let tx = self.contracts.staking.add_realm();
        let result = tx
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Error sending tx to add realm! {:?}", e))?;
        let _result = result
            .log_msg("add_realm")
            .await
            .map_err(|e| anyhow::anyhow!("Error waiting for successful add realm tx! {:?}", e))?;
        let new_num_realms = self.contracts.staking.num_realms().call().await?;

        Ok(new_num_realms.as_u64())
    }

    pub async fn lit_token_balance(&self, address: Address) -> U256 {
        self.contracts
            .lit_token
            .balance_of(address)
            .call()
            .await
            .unwrap()
    }

    pub async fn get_current_validators(&self, realm_id: U256) -> Vec<H160> {
        self.contracts
            .staking
            .get_validators_in_current_epoch(realm_id)
            .call()
            .await
            .expect("Error getting validators from chain")
    }

    pub async fn get_current_validator_structs(&self, realm_id: U256) -> Vec<Validator> {
        self.contracts
            .staking
            .get_validators_structs_in_current_epoch(realm_id)
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

    pub async fn get_next_validators(&self, realm_id: U256) -> Vec<H160> {
        self.contracts
            .staking
            .get_validators_in_next_epoch(realm_id)
            .call()
            .await
            .expect("Error getting next validators from chain")
    }

    pub async fn get_next_validator_structs(&self, realm_id: U256) -> Vec<Validator> {
        self.contracts
            .staking
            .get_validators_structs_in_next_epoch(realm_id)
            .call()
            .await
            .expect("Error getting next validator structs from chain")
    }

    pub async fn get_current_validator_count(&self, realm_id: U256) -> u32 {
        self.get_current_validators(realm_id).await.len() as u32
    }

    pub async fn send_approve_and_stake(
        &self,
        staker: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    ) -> Result<()> {
        // give some tokens to the staker

        let deployer_balance = self
            .contracts
            .lit_token
            .balance_of(self.deploy_address)
            .call()
            .await?;
        info!("Deployer balance is {}", deployer_balance);

        info!(
            "Balance before send: {:?}",
            self.lit_token_balance(staker.address()).await
        );

        let amount_to_send = ethers::utils::parse_units(4, 18).unwrap().into();
        let r = self
            .contracts
            .lit_token
            .transfer(staker.address(), amount_to_send);

        let res = r
            .send()
            .await
            .unwrap()
            .interval(Duration::from_millis(500))
            .await;
        if let Err(e) = res {
            panic!("Error sending LIT tokens: {:?}", e);
        }

        info!(
            "Balance after send: {:?}",
            self.lit_token_balance(staker.address()).await
        );

        let lit_token = LITToken::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            self.contracts.lit_token.address(),
            staker.clone(),
        );

        // spender is the deployed staking balances contract
        let spender = self.contracts.staking.address();
        let amount_to_approve = ethers::utils::parse_units(2, 18).unwrap().into();
        let r = lit_token.approve(spender, amount_to_approve);
        let r = r.send().await;
        if r.is_err() {
            panic!("Error Approving ERC20 : {:?}", r);
        }

        let receipt = r.unwrap().await;
        if receipt.is_err() {
            panic!("(Receipt) Error Approving ERC20 : {:?}", receipt);
        }

        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            staker.clone(),
        );

        let stake_amount = staking.min_self_stake().call().await?;

        info!("Staking from {:?}", staker.address(),);

        let r = staking.stake(
            stake_amount,
            U256::from(DEFAULT_TIMELOCK_SECONDS),
            staker.address(),
        );

        let r = r.send().await;
        if let Err(e) = r {
            debug!(
                "Error doing stake.  Revert: {:?}",
                lit_blockchain::util::decode_revert(&e, staking.abi())
            );

            let revert: Option<StakingErrors> = e.decode_contract_revert();
            match revert {
                Some(r) => {
                    return Err(anyhow::anyhow!(
                        "Error doing stake: {:?}.  Revert: {:?}",
                        e,
                        r
                    ));
                }
                None => {
                    return Err(anyhow::anyhow!(
                        "Error doing stake: {:?}.  Could not decode revert reason.  Revert: {:?}",
                        &e,
                        lit_blockchain::util::decode_revert(&e, staking.abi())
                    ));
                }
            }
        }

        // make sure it's fully mined so we don't accidently advance then lock the next epoch before the user has actually staked
        let _receipt = r.unwrap().interval(Duration::from_millis(500)).await;

        Ok(())
    }

    pub async fn send_request_to_join(
        &self,
        realm_id: U256,
        staker: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        _ip: u32,
        _port: u32,
        node_info: &PeerItem,
    ) -> Result<()> {
        info!(
            "Staking from {:?} for with node_address {:?} - PeerItem {:?}",
            staker.address(),
            node_info.node_address,
            node_info
        );

        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            staker.clone(),
        );

        info!(
            "request to join with sender pub key: {:?}",
            U256::from_big_endian(&node_info.sender_public_key[..])
        );

        let r = staking.request_to_join(realm_id);

        let r = r.send().await;
        if let Err(e) = r {
            debug!(
                "Error doing request_to_join for {:}.  Revert: {:?}",
                node_info.addr,
                lit_blockchain::util::decode_revert(&e, staking.abi())
            );

            let revert: Option<StakingErrors> = e.decode_contract_revert();
            match revert {
                Some(r) => {
                    return Err(anyhow::anyhow!(
                        "Error doing request_to_join {:} : {:?}.  Revert: {:?}",
                        node_info.addr,
                        e,
                        r
                    ));
                }
                None => {
                    return Err(anyhow::anyhow!(
                        "Error doing request_to_join {:} : {:?}.  Could not decode revert reason.  Revert: {:?}",
                        node_info.addr,
                        &e,
                        lit_blockchain::util::decode_revert(&e, staking.abi())
                    ));
                }
            }
        }

        // make sure it's fully mined so we don't accidently advance then lock the next epoch before the user has actually staked
        let _receipt = r.unwrap().interval(Duration::from_millis(500)).await;

        Ok(())
    }

    #[doc = "Wait for state to become active again (DKGs run, advance)"]
    pub async fn wait_for_active(&self, realm_id: U256) {
        info!("Waiting for network to become active again");
        loop {
            let res = self.contracts.staking.state(realm_id).call().await;
            match res {
                Ok(res) => {
                    match res {
                        0 => {
                            info!("Network is active");
                            break;
                        }
                        5 => {
                            info!("Network is in recovery mode");
                            break;
                        }
                        _ => {} // Wait for active or recovery mode
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
                Err(..) => {
                    debug!(
                        "Error checking if validator state is active : {:?}",
                        res.unwrap_err()
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }

        info!("Sleeping for 3 seconds to make sure nodes sync up with new peer state...");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    #[doc = "Wait for state to become locked"]
    pub async fn wait_for_lock(&self, realm_id: U256) {
        info!("Waiting for nodes to be locked");
        let res = self
            .contracts
            .staking
            .get_validators_in_next_epoch(realm_id)
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
            let res = self.contracts.staking.state(realm_id).call().await;

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
                }
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

    pub async fn wait_for_root_keys(&self, realm_id: U256, keyset_id: Option<String>) -> bool {
        info!("Waiting for root keys!");

        let res = self.contracts.staking.state(realm_id).call().await;
        match res {
            Ok(res) => {
                match res {
                    0 => {}           // Network is active, therefore root keys will be created
                    5 => return true, // Network is in recovery mode, therefore root keys will not be created directly, but restored
                    _ => return false,
                }
            }
            Err(..) => {
                return false;
            }
        }

        // First, check whether the root keys are registered on the chain.
        // hardcoded to BLS = 1, ECDSA = 2
        loop {
            if self.get_root_keys(1, keyset_id.clone()).await.is_some()
                && self.get_root_keys(2, keyset_id.clone()).await.is_some()
            {
                break;
            }
            let _r = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        // Then, wait until the nodes have synced the latest chain state.
        loop {
            if handshake_returns_keys(self, realm_id).await {
                break;
            }
            let _r = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        true
    }

    pub async fn get_root_keys(
        &self,
        curve_type: u8,
        keyset_id: Option<String>,
    ) -> Option<Vec<String>> {
        let all_root_keys = self.get_all_root_keys(keyset_id).await;

        all_root_keys.as_ref()?;
        let all_root_keys: Vec<RootKey> = all_root_keys.unwrap();

        let root_keys: Vec<String> = all_root_keys
            .iter()
            .filter(|k| k.key_type == U256::from(curve_type))
            .map(|k| bytes_to_hex(k.pubkey.clone()))
            .collect::<Vec<String>>();

        Some(root_keys)
    }

    pub async fn get_all_root_keys(&self, keyset_id: Option<String>) -> Option<Vec<RootKey>> {
        let keyset_id = keyset_id.unwrap_or("naga-keyset1".to_string());
        let staking_address = self.contracts.staking.address();
        let root_keys = self
            .contracts
            .pubkey_router
            .get_root_keys(staking_address, keyset_id)
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

    /// Wait for number of votes to kick validator to reach the expected value.
    ///
    /// Note that the actual number of votes to kick validator may be greater than the expected value.
    pub async fn wait_for_voting_status_to_kick_validator(
        &self,
        realm_id: U256,
        epoch_number: U256,
        validator_to_kick_staker_address: Address,
        voter_staker_address: Address,
        expected_num_votes_to_kick_validator: usize,
        expect_validator_kicked: bool,
    ) -> Result<VotingStatusToKickValidator> {
        loop {
            let epoch = self.contracts().staking.epoch(realm_id).call().await;
            if epoch.is_err() {
                error!("Error getting epoch: {:?}", epoch.unwrap_err());
                return Err(anyhow::anyhow!("Error getting epoch"));
            }
            let epoch = epoch.unwrap();
            let current_epoch = epoch.number;

            if current_epoch > epoch_number {
                info!(
                    "Current epoch: {:?}, expected epoch: {:?}",
                    current_epoch, epoch_number
                );
                return Err(anyhow::anyhow!(
                    "Current epoch is greater than the expected epoch"
                ));
            }

            let (votes, voter_voted) = self
                .contracts
                .staking
                .get_voting_status_to_kick_validator(
                    realm_id,
                    epoch_number,
                    validator_to_kick_staker_address,
                    voter_staker_address,
                )
                .await?;

            info!(
                "votes: {:?}  / expected_num_votes_to_kick_validator: {:?}",
                votes, expected_num_votes_to_kick_validator
            );

            if votes.as_usize() >= expected_num_votes_to_kick_validator {
                let mut kicked_validators = vec![];
                // Wait 3 seconds to make sure the node is actually kicked.
                for sec in 0..10 {
                    // is the node actually kicked?
                    kicked_validators = self
                        .contracts
                        .staking
                        .get_kicked_validators(realm_id)
                        .await?;
                    if kicked_validators.contains(&validator_to_kick_staker_address) {
                        break;
                    }
                    info!(
                        "Waiting {} up to 10 seconds to discover which validator was kicked.",
                        sec + 1
                    );
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }

                info!("kicked_validators: {:?}", kicked_validators);
                info!(
                    "validator_to_kick_staker_address: {:?}",
                    validator_to_kick_staker_address
                );

                if expect_validator_kicked {
                    assert!(
                        kicked_validators.contains(&validator_to_kick_staker_address),
                        "Validator {:?} is not in the set of kicked validators: {:?}",
                        validator_to_kick_staker_address,
                        kicked_validators
                    );
                    // verify that the node isn't in the set anymore
                    let validators = self
                        .contracts
                        .staking
                        .get_validators_in_next_epoch(realm_id)
                        .await?;
                    assert!(
                        !validators.contains(&validator_to_kick_staker_address),
                        "Validator {:?} is still in the set of validators: {:?}",
                        validator_to_kick_staker_address,
                        validators
                    );
                }

                return Ok(VotingStatusToKickValidator {
                    votes,
                    did_voter_vote_to_kick_validator: voter_voted,
                });
            }

            // Wait for 1 second before checking again.
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    #[doc = "Wait for initial epoch to end - a collection of functions to set the state to active and lock validators for next epoch."]
    pub async fn wait_for_initial_epoch(&self, realm_id: U256) {
        self.start_initial_epoch(realm_id, true).await
    }

    /// Wait for the initial epoch to end - a collection of functions to set the state to active and lock validators for next epoch.
    pub async fn start_initial_epoch(&self, realm_id: U256, wait_for_active: bool) {
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
            self.contracts.staking.state(realm_id).call().await
        );

        if wait_for_active {
            self.wait_for_active(realm_id).await;
        }

        info!("Initial Epoch has started.");
    }

    #[doc = "Lock validators for next epoch"]
    pub async fn lock_validators_for_next_epoch(&self, realm_id: U256) {
        let state = self.contracts.staking.state(realm_id).call().await;
        if state.is_err() {
            error!("Error getting state...");
            return;
        }
        info!("Staking state (pre lock) : {:?}", state);

        let lock_func = self
            .contracts
            .staking
            .lock_validators_for_next_epoch(realm_id);
        let lock_res = lock_func.send().await;
        warn!("Locking validators for next epoch: {:?}", lock_res);
        // assert!(lock_res.is_ok());
        info!(
            "Staking state (post lock) : {:?}",
            self.contracts.staking.state(realm_id).call().await
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

    pub async fn set_staking_min_version(&self, realm_id: U256, min_version: &str) -> Result<()> {
        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            self.deployer_signing_provider.clone(),
        );

        let min_version = parse_version(min_version)?;
        let cc = staking.set_min_version(realm_id, min_version);
        if !Contracts::process_contract_call(cc, "set minimum version").await {
            return Err(anyhow::anyhow!("Error setting min version"));
        }

        Ok(())
    }

    pub async fn set_staking_max_version(&self, realm_id: U256, max_version: &str) -> Result<()> {
        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            self.deployer_signing_provider.clone(),
        );

        let max_version = parse_version(max_version)?;
        let cc = staking.set_max_version(realm_id, max_version);
        if !Contracts::process_contract_call(cc, "set maximum version").await {
            return Err(anyhow::anyhow!("Error setting max version"));
        }

        Ok(())
    }

    pub async fn admin_set_register_attested_wallet_disabled_for_validators(
        &self,
        validator_addresses: Vec<H160>,
        disabled: bool,
    ) -> Result<()> {
        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            self.contracts.staking.address(),
            self.deployer_signing_provider.clone(),
        );

        for validator_address in validator_addresses {
            let cc = staking
                .admin_set_validator_register_attested_wallet_disabled(validator_address, disabled);
            if !Contracts::process_contract_call(cc, "set register attested wallet disabled").await
            {
                return Err(anyhow::anyhow!(
                    "Error setting register attested wallet disabled for validator"
                ));
            }
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

    pub async fn get_current_epoch(&self, realm_id: U256) -> U256 {
        let get_res = self.contracts.staking.epoch(realm_id).call().await;

        if get_res.is_err() {
            error!("Error in get_epoch: {}", get_res.err().unwrap());
            return U256::zero();
        }
        let epoch = get_res.unwrap();
        let epoch_number = epoch.number;

        epoch_number
    }

    pub async fn wait_for_epoch(&self, realm_id: U256, epoch: U256) {
        info!(
            "Waiting for epoch {}.  Current epoch is {}.",
            epoch,
            self.get_current_epoch(realm_id).await
        );
        loop {
            let current_epoch = self.get_current_epoch(realm_id).await;
            if current_epoch == epoch {
                info!("Advanced! Current epoch is {}.", epoch);
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        // Ensure all nodes have reached the expected epoch
        let min_epoch = epoch.as_u64();

        loop {
            let all_nodes_at_epoch = ensure_min_node_epoch(self, realm_id, min_epoch).await;
            if all_nodes_at_epoch {
                info!("All nodes have reached epoch {}", min_epoch);
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    pub async fn ensure_node_staked_and_joined(
        &self,
        realm_id: U256,
        node_account: &NodeAccount,
        node_addr: &str,
        node_port: usize,
    ) -> Result<NodeStakingStatus> {
        let node_signer = node_account.signing_provider.clone();

        info!(
            "Checking if node {} is already staked...",
            node_signer.address()
        );

        // stake if not already
        let is_staked = self
            .contracts
            .staking
            .check_staking_amounts(node_account.staker_address)
            .call()
            .await;
        if let Ok(is_staked) = is_staked {
            if is_staked {
                info!("Node {} is already staked!", node_signer.address());
            } else {
                info!("Node {} is not staked.  Staking...", node_signer.address());
                self.send_approve_and_stake(node_signer.clone()).await?;
            }
        }

        // request to join if not already
        let next_validators = self
            .contracts
            .staking
            .get_validators_in_next_epoch(realm_id)
            .call()
            .await?;
        let is_joined = next_validators.contains(&node_account.staker_address);
        if !is_joined {
            info!("Node {} is not joined.  Joining...", node_signer.address());
            let peer_item = PeerItem {
                addr: node_addr.to_string(),
                node_address: node_account.node_address,
                sender_public_key: node_account.coms_keys.sender_public_key().to_bytes(),
                receiver_public_key: node_account.coms_keys.receiver_public_key().to_bytes(),
                staker_address: node_account.staker_address,
            };

            self.send_request_to_join(
                realm_id,
                node_signer,
                2130706433u32,
                node_port as u32,
                &peer_item,
            )
            .await?;
        }

        Ok(NodeStakingStatus::StakedAndJoined)
    }

    pub async fn update_staking_global_config(
        &self,
        staking_global_config: StakingContractGlobalConfig,
    ) -> Result<()> {
        Contracts::update_staking_global_config(
            self.contracts.staking.clone(),
            staking_global_config,
        )
        .await
    }

    pub async fn update_staking_realm_config(
        &self,
        staking_realm_config: StakingContractRealmConfig,
    ) -> Result<()> {
        Contracts::update_staking_realm_config(self.contracts.staking.clone(), staking_realm_config)
            .await
    }

    /// This function waits until the complaints cache completely clears.
    pub async fn wait_for_complaint_cache_to_clear(&self) -> Result<()> {
        // Get the maximum configured complaint interval from the staking contract.
        let mut max_complaint_interval_secs = U256::zero();

        for i in 1..=MAX_COMPLAINT_REASON_VALUE {
            let complaint_config: staking::ComplaintConfig = self
                .contracts
                .staking
                .complaint_config(U256::from(i))
                .call()
                .await
                .map_err(|e| anyhow::anyhow!("Error getting complaint config: {:?}", e))?;

            if complaint_config.interval_secs > max_complaint_interval_secs {
                max_complaint_interval_secs = complaint_config.interval_secs;
            }
        }
        info!(
            "Sleeping for {:?} seconds to allow complaints cache to clear",
            max_complaint_interval_secs
        );
        tokio::time::sleep(std::time::Duration::from_secs(
            max_complaint_interval_secs.as_u64(),
        ))
        .await;

        Ok(())
    }

    pub async fn get_node_attested_pubkey_mappings(
        &self,
        node_addresses: &Vec<H160>,
    ) -> Result<Vec<Option<UncompressedK256Key>>> {
        // Get the node's attested pubkey mappings from the staking contract
        let pubkey_mappings = self
            .contracts
            .staking
            .get_node_attested_pub_key_mappings(node_addresses.clone())
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("Error getting node attested pubkey mappings: {:?}", e))?;

        // Turn into a map
        let pubkey_mappings = pubkey_mappings
            .into_iter()
            .map(|m| (m.node_address, m.pub_key))
            .collect::<HashMap<_, _>>();

        // Return the pubkey mappings for each node address
        Ok(node_addresses
            .into_iter()
            .map(|node_address| pubkey_mappings.get(&node_address).cloned())
            .collect())
    }

    pub async fn set_state_to_paused(&self, realm_id: u64) {
        let state = NetworkState::Paused as u8;
        let realm_id = U256::from(realm_id);
        let cc = self.contracts.staking.set_epoch_state(realm_id, state);
        if !Contracts::process_contract_call(cc, "set state to paused").await {
            panic!("Error setting state to paused");
        }
    }

    pub async fn set_state_to_active(&self, realm_id: u64) {
        let state = NetworkState::Active as u8;
        let realm_id = U256::from(realm_id);
        let cc = self.contracts.staking.set_epoch_state(realm_id, state);
        if !Contracts::process_contract_call(cc, "set state to active").await {
            panic!("Error setting state to active");
        }
    }

    pub async fn set_state(&self, realm_id: u64, state: NetworkState) {
        let state = state as u8;
        let realm_id = U256::from(realm_id);
        let cc = self.contracts.staking.set_epoch_state(realm_id, state);
        if !Contracts::process_contract_call(cc, "set state").await {
            panic!("Error setting state to {:?}", state);
        }
    }

    pub async fn set_state_to_next_validator_set_locked(&self, realm_id: u64) {
        let state = NetworkState::NextValidatorSetLocked as u8;
        let realm_id = U256::from(realm_id);
        let cc = self.contracts.staking.set_epoch_state(realm_id, state);
        if !Contracts::process_contract_call(cc, "set state to next validator set locked").await {
            panic!("Error setting state to next validator set locked");
        }
    }

    pub async fn get_state(&self, realm_id: u64) -> NetworkState {
        let realm_id = U256::from(realm_id);
        let state = self.contracts.staking.state(realm_id).call().await;
        if state.is_err() {
            panic!("Error getting state: {:?}", state.err().unwrap());
        }
        NetworkState::from(state.unwrap() as u8)
    }

    pub async fn setup_shadow_splicing(
        &self,
        source_realm_id: u64,
        target_realm_id: u64,
        target_validators: Vec<H160>,
    ) -> Result<()> {
        let source_realm_id = U256::from(source_realm_id);
        let target_realm_id = U256::from(target_realm_id);

        let tx = self.contracts.staking.admin_setup_shadow_splicing(
            source_realm_id,
            target_realm_id,
            target_validators,
        );
        let result = tx
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Error sending tx to setup shadow splicing! {:?}", e))?;
        let _result = result.log_msg("setup_shadow_splicing").await.map_err(|e| {
            anyhow::anyhow!(
                "Error waiting for successful setup shadow splicing tx! {:?}",
                e
            )
        })?;
        Ok(())
    }

    pub async fn wait_for_shadow_splicing_to_complete(
        &self,
        realm_id: u64,
        expected_validators: Vec<H160>,
    ) -> Result<()> {
        let realm_id = U256::from(realm_id);

        let count = expected_validators.len();
        info!(
            "Waiting for shadow splicing to complete... expecting {} validators.",
            count
        );
        loop {
            let mut found_validators: Vec<H160> = Vec::new();

            let validators = self
                .contracts
                .staking
                .get_validators_in_current_epoch(realm_id)
                .call()
                .await?;

            for validator in validators {
                if !expected_validators.contains(&validator) {
                    info!(
                        "Validator {} is not in the expected validators list.",
                        validator
                    );
                } else {
                    found_validators.push(validator);
                }
            }

            if found_validators.len() == count {
                info!("Shadow splicing has been completed.");
                break;
            }

            info!(
                "Waiting for shadow splicing to complete...  Found {} of {} validators.   Current validators: {:?}",
                found_validators.len(),
                count,
                found_validators
            );
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        Ok(())
    }

    pub async fn add_second_keyset(&self, realm_id: U256) -> Result<()> {
        let update_key_set_request = staking::KeySetConfig {
            minimum_threshold: 3,
            monetary_value: 0,
            complete_isolation: false,
            identifier: "naga_keyset_2".to_string(),
            description: "Naga Keyset 2".to_string(),
            realms: vec![realm_id],
            curves: vec![
                U256::from(1),
                U256::from(2),
                U256::from(3),
                U256::from(4),
                U256::from(5),
                U256::from(6),
                U256::from(7),
                U256::from(8),
                U256::from(9),
                U256::from(10),
            ],
            counts: vec![
                U256::from(1),
                U256::from(2),
                U256::from(2),
                U256::from(2),
                U256::from(2),
                U256::from(2),
                U256::from(2),
                U256::from(2),
                U256::from(2),
                U256::from(2),
            ],
            recovery_session_id: Bytes::from_static(&[]),
        };

        let cc = self.contracts.staking.set_key_set(update_key_set_request);
        let result = cc
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Error sending tx to add second keyset! {:?}", e))?;
        let _result = result.log_msg("add_second_keyset").await.map_err(|e| {
            anyhow::anyhow!("Error waiting for successful add second keyset tx! {:?}", e)
        })?;
        info!("Second keyset added successfully");
        Ok(())
    }
}
