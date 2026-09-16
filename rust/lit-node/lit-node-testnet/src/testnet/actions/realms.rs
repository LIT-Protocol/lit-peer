use anyhow::Result;
use ethers::prelude::*;
use std::time::Duration;
use tracing::info;

use super::Actions;

impl Actions {
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
}
