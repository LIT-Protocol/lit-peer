use super::serialize;
use crate::testnet::datil::contracts::DatilContracts;
use anyhow::Result;
use ethers::{
    providers::{Middleware, ProviderError},
    types::U256,
};
use tracing::{debug, info};

pub struct Actions {
    datil_contracts: DatilContracts,
}

impl Actions {
    pub fn new(datil_contracts: DatilContracts) -> Self {
        Self { datil_contracts }
    }

    #[doc = "Get the current epoch end time"]
    pub async fn get_epoch_end_time(&self) -> Result<U256> {
        let epoch = self.datil_contracts.staking.epoch().call().await?;
        Ok(epoch.end_time)
    }

    #[doc = "Set the epoch end time from now"]
    pub async fn set_epoch_end_time_from_now(&self, length: U256) -> Result<()> {
        let current_epoch_end_time = self.get_epoch_end_time().await?;
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
            "Setting epoch end time to {}.  Current epoch end time is {}.  Current latest block time is {}",
            n_new_end_time, n_current_epoch_end_time, n_lastet_block_time
        );

        let r = self
            .datil_contracts
            .staking
            .set_epoch_end_time(new_end_time)
            .await;
        if r.is_err() {
            return Err(anyhow::anyhow!("Error setting epoch end time"));
        }
        Ok(())
    }

    #[doc = "Fast forward by a number of blocks"]
    pub async fn fast_forward_blocks(&self, blocks_to_mine: usize) {
        info!("Fast forwarding by {:?} blocks...", blocks_to_mine);

        let deployer_provider = self.datil_contracts.deployer_provider.provider().clone();

        let block_num_before = deployer_provider.get_block_number().await.unwrap();

        let command = "anvil_mine";
        let mine_blocks_res: Result<(), ProviderError> = deployer_provider
            .request(
                command,
                [serialize(&format!("0x{:X}", blocks_to_mine)), serialize(&0)],
            )
            .await;

        match mine_blocks_res {
            Ok(r) => debug!("Successfully mined {:?} blocks: {:?}", blocks_to_mine, r),
            Err(e) => info!(
                "Error mining blocks - you can ignore this on Anvil and look at the below Block Number message to check that it actually fast forwarded {:?}",
                e
            ),
        }

        let block_num_after = deployer_provider.get_block_number().await.unwrap();
        debug!(
            "Block number before fast forwarding: {}, Block number after fast forwarding: {}",
            block_num_before, block_num_after
        );
    }

    pub async fn get_latest_block_timestamp(&self) -> Result<U256> {
        let deployer_provider = self.datil_contracts.deployer_provider.provider().clone();

        let block = deployer_provider
            .get_block(
                deployer_provider
                    .get_block_number()
                    .await
                    .map_err(|e| anyhow::anyhow!("Error getting block number: {:?}", e))?,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Error getting block: {:?}", e))?
            .ok_or_else(|| anyhow::anyhow!("Error getting block"))?;
        Ok(block.timestamp)
    }
}
