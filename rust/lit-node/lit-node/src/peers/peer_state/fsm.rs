use super::super::PeerState;
use super::super::peer_state::models::NetworkState;
use crate::error::{Result, blockchain_err, unexpected_err};
use ethers::types::{Address, H160, U256};
use lit_blockchain::config::LitBlockchainConfig;
use lit_blockchain::resolver::rpc::RpcHealthcheckPoller;
use lit_blockchain::util::decode_revert;
use std::collections::BTreeSet;
use std::iter::FromIterator;
use tracing::{instrument, trace};

#[allow(dead_code)]
impl PeerState {
    #[doc = "Get the current epoch along with length, number and retries."]
    #[instrument(level = "debug", skip(self))]
    pub async fn get_epoch(&self, realm_id: u64) -> Result<(U256, U256, U256, U256)> {
        // let realm_id = self.current_or_next_realm_id().await?;
        let realm_id = U256::from(realm_id);
        let epoch = self
            .staking_contract
            .epoch(realm_id)
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;

        Ok((
            epoch.epoch_length,
            epoch.number,
            epoch.end_time,
            epoch.retries,
        ))
    }

    #[doc = "Get the current block number."]
    #[instrument(level = "debug", skip(self))]
    pub async fn get_block_number(&self) -> Result<ethers::types::U64> {
        use ethers::providers::Middleware;
        let cfg = self.lit_config.load_full();
        let chain = cfg.blockchain_chain_name()?;
        let wallet = lit_blockchain::contracts::load_wallet(&cfg, None)?;
        let provider =
            lit_blockchain::resolver::rpc::ENDPOINT_MANAGER.get_provider(chain.as_str())?;

        provider
            .get_block_number()
            .await
            .map_err(|e| unexpected_err(e, Some("Failed to get block number from chain".into())))
    }

    pub async fn validators_for_next_epoch_locked(&self, realm_id: u64) -> Result<bool> {
        let state = self.network_state(realm_id).await?;
        Ok(state == NetworkState::NextValidatorSetLocked)
    }

    pub async fn validators_in_active_state(&self, realm_id: u64) -> Result<bool> {
        let state = self.network_state(realm_id).await?;
        Ok(state == NetworkState::Active)
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn network_state(&self, realm_id: u64) -> Result<NetworkState> {
        let realm_id = U256::from(realm_id);
        let state = self
            .staking_contract
            .state(realm_id)
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;

        Ok(NetworkState::from(state))
    }

    #[instrument(level = "debug", skip(self), fields(addr = addr.to_string()))]
    pub async fn get_ready_signal(&self, realm_id: u64, addr: H160) -> Result<bool> {
        let realm_id = U256::from(realm_id);
        let ready_signal = self
            .staking_contract
            .ready_for_next_epoch(realm_id, addr)
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;

        Ok(ready_signal)
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn current_validator_count_for_consensus(&self, realm_id: u64) -> Result<U256> {
        let realm_id = U256::from(realm_id);
        let count = self
            .staking_contract
            .current_validator_count_for_consensus(realm_id)
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;

        Ok(count)
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn next_validator_count_for_consensus(&self, realm_id: u64) -> Result<U256> {
        let realm_id = U256::from(realm_id);
        let count = self
            .staking_contract
            .next_validator_count_for_consensus(realm_id)
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;

        Ok(count)
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn get_count_of_validators_ready_for_next_epoch(
        &self,
        realm_id: u64,
    ) -> Result<U256> {
        let realm_id = U256::from(realm_id);
        let count = self
            .staking_contract
            .count_of_next_validators_ready_for_next_epoch(realm_id)
            .call()
            .await
            .map_err(|e| blockchain_err(e, None))?;
        Ok(count)
    }

    pub async fn current_or_next_realm_id(&self, realm_id: u64) -> Result<U256> {
        let realm_id = U256::from(realm_id);
        match self.peers().realm_id() {
            Ok(realm_id) => Ok(realm_id),
            Err(e) => self.peers_in_next_epoch().realm_id(),
        }
    }

    pub fn part_of_validators_union(&self) -> bool {
        // TODO?: Store in CDM cache
        let vin_current = BTreeSet::from_iter(self.validators_in_current_epoch());
        let vin_next = BTreeSet::from_iter(self.validators_in_next_epoch());
        let vi_union_of_epochs = vin_current.union(&vin_next);

        let res = vi_union_of_epochs
            .filter(|v| !v.is_kicked)
            .map(|v| v.address)
            .collect::<Vec<Address>>()
            .contains(&self.node_address());
        trace!(
            "Node: {:?} is part of union: {:?}",
            self.node_address(),
            res
        );
        res
    }

    pub async fn get_threshold_for_node_count(&self, count: usize) -> Result<U256> {
        self.staking_contract
            .get_threshold(U256::from(count))
            .call()
            .await
            .map_err(|e|
                blockchain_err(
                    decode_revert(&e, self.staking_contract.abi()),
                    Some(format!("Unable to contact chain to get threshold for node count - original error {:?}", e)),
                )
            )
    }
}
