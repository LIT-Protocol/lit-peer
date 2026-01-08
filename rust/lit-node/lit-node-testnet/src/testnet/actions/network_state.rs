use super::super::NodeAccount;
use super::super::contracts::Contracts;
use crate::models::VotingStatusToKickValidator;
use crate::node_collection::handshake_returns_keys;
use crate::testnet::actions::NetworkState;
use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain::contracts::staking::Staking;
use lit_node_common::models::NodeStakingStatus;
// use lit_node::peers::peer_reviewer::MAX_COMPLAINT_REASON_VALUE;
pub const MAX_COMPLAINT_REASON_VALUE: u8 = 4;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info};

use super::Actions;

impl Actions {
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

    pub async fn wait_for_root_keys(&self, realm_id: U256, keyset_id: &str) -> bool {
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
            if self.get_root_keys(1, keyset_id).await.is_some()
                && self.get_root_keys(2, keyset_id).await.is_some()
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

    /// Wait for number of votes to kick validator to reach the expected value.
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
}
