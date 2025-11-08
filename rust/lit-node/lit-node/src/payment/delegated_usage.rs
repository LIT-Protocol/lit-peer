use crate::config::chain::ChainDataConfigManager;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

// Refactored from the RLI
pub struct DelegatedUsageDB {
    // stores config for defaults like rate limit window, etc.
    pub chain_data_config_manager: Arc<ChainDataConfigManager>,
    // maps user address to the number of times they have used the endpoint under a delegation context
    pub delegation_uses_map: Cache<Vec<u8>, u32>,
}

impl DelegatedUsageDB {
    // False positive warning from the lint.
    #[allow(dead_code)]
    pub fn default_with_chain_data_config_manager(
        chain_data_config_manager: Arc<ChainDataConfigManager>,
    ) -> Self {
        Self {
            chain_data_config_manager,
            // 7 day TTL
            delegation_uses_map: Cache::builder()
                .time_to_live(Duration::from_secs(7 * 24 * 60 * 60))
                .build(),
        }
    }
}
