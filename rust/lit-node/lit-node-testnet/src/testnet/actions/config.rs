use super::super::contracts::{Contracts, StakingContractGlobalConfig, StakingContractRealmConfig};
use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain::contracts::staking::{ComplaintConfig, Staking, staking};
// use lit_node::peers::peer_reviewer::MAX_COMPLAINT_REASON_VALUE;
pub const MAX_COMPLAINT_REASON_VALUE: u8 = 4;
use lit_node_common::utils::parse_version;
use std::sync::Arc;
use tracing::info;

use super::Actions;

impl Actions {
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

    // shortcut function to update all complaint configs to the same interval and tolerance for testing
    pub async fn update_all_complaint_configs(
        &self,
        interval_secs: Option<u64>,
        tolerance: Option<u64>,
        kick_penalty_percent: Option<u64>,
        kick_penalty_demerits: Option<u64>,
    ) -> Result<()> {
        info!(
            "Updating all complaint reason configs interval_secs to {:?} and tolerance to {:?}",
            interval_secs, tolerance
        );

        let interval_secs = if interval_secs.is_some() {
            Some(U256::from(interval_secs.unwrap()))
        } else {
            None
        };
        let tolerance = if tolerance.is_some() {
            Some(U256::from(tolerance.unwrap()))
        } else {
            None
        };
        let kick_penalty_percent = if kick_penalty_percent.is_some() {
            Some(U256::from(kick_penalty_percent.unwrap()))
        } else {
            None
        };
        let kick_penalty_demerits = if kick_penalty_demerits.is_some() {
            Some(U256::from(kick_penalty_demerits.unwrap()))
        } else {
            None
        };
        for i in 0..=MAX_COMPLAINT_REASON_VALUE {
            let reason = U256::from(i);
            // First, get current chain config for this reason.
            let current_config: lit_blockchain::contracts::staking::ComplaintConfig = self
                .contracts
                .staking
                .complaint_config(reason)
                .call()
                .await
                .map_err(|e| anyhow::anyhow!("unable to get complaint config: {:?}", e))?;

            // Then, set the config with any new values.
            let cc = self.contracts.staking.set_complaint_config(
                reason,
                lit_blockchain::contracts::staking::ComplaintConfig {
                    tolerance: tolerance.unwrap_or(current_config.tolerance),
                    interval_secs: interval_secs.unwrap_or(current_config.interval_secs),
                    kick_penalty_percent: kick_penalty_percent
                        .unwrap_or(current_config.kick_penalty_percent),
                    kick_penalty_demerits: kick_penalty_demerits
                        .unwrap_or(current_config.kick_penalty_demerits),
                },
            );
            if !Contracts::process_contract_call(
                cc,
                format!("updating staking complaint config for reason {:?}", reason).as_str(),
            )
            .await
            {
                return Err(anyhow::anyhow!(
                    "Error updating complaint config for reason {:?}",
                    reason.as_u64()
                ));
            }
        }
        Ok(())
    }

    pub async fn clear_presigns(&self) -> Result<()> {
        let r = self
            .contracts
            .staking
            .emit_clear_offline_phase_data(U256::from(1))
            .call()
            .await;
        if r.is_err() {
            return Err(anyhow::anyhow!(
                "Error clearing presigns: {:?}",
                r.err().unwrap()
            ));
        } else {
            info!("Presigns cleared");
        }
        Ok(())
    }
}
