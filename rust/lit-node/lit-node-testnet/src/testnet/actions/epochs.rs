use super::super::contracts::Contracts;
use crate::node_collection::ensure_min_node_epoch;
use anyhow::Result;
use ethers::prelude::*;
// use lit_node::peers::peer_reviewer::MAX_COMPLAINT_REASON_VALUE;
pub const MAX_COMPLAINT_REASON_VALUE: u8 = 4;
use tracing::{debug, error, info, warn};

use super::Actions;

impl Actions {
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
}
