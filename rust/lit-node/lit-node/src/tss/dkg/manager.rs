use crate::config::chain::CachedRootKey;
use crate::error::{Result, unexpected_err};
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tss::common::dkg_type::DkgType;
use crate::tss::common::tss_state::TssState;
use crate::tss::dkg::engine::{DkgAfterRestore, DkgEngine};
use crate::tss::dkg::models::Mode;
use crate::tss::util::DEFAULT_KEY_SET_NAME;
use lit_core::error::Unexpected;
use lit_node_core::CurveType;
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct DkgManager {
    pub tss_state: Arc<TssState>,
    pub dkg_type: DkgType,
    pub next_dkg_after_restore: DkgAfterRestore,
}

impl DkgManager {
    pub fn new(tss_state: Arc<TssState>, dkg_type: DkgType) -> Self {
        Self {
            tss_state,
            dkg_type,
            next_dkg_after_restore: DkgAfterRestore::False,
        }
    }

    #[instrument(level = "debug", skip(self, current_peers, new_peers))]
    pub async fn change_epoch(
        &self,
        dkg_id: &str,
        epoch_number: u64,
        shadow_key_opts: (u64, u64),
        realm_id: u64,
        current_peers: &SimplePeerCollection,
        new_peers: &SimplePeerCollection,
    ) -> Result<Vec<CachedRootKey>> {
        let key_set_config = self
            .tss_state
            .peer_state
            .staking_contract
            .get_key_set(DEFAULT_KEY_SET_NAME.to_string())
            .call()
            .await
            .map_err(|e| unexpected_err(e, None))?;

        let mut root_keys: Vec<CachedRootKey> = Vec::new();
        let threshold = self
            .tss_state
            .peer_state
            .get_threshold_for_node_count(new_peers.0.len())
            .await?
            .as_usize();
        let mut dkg_engine = DkgEngine::new(
            self.tss_state.clone(),
            self.dkg_type,
            epoch_number,
            threshold,
            shadow_key_opts,
            current_peers,
            new_peers,
            self.next_dkg_after_restore.clone(),
        );
        let chain_root_keys = if current_peers.is_empty() {
            Vec::new()
        } else {
            self.root_keys()
        };
        for (curve_type, hd_root_key_count) in key_set_config
            .curves
            .iter()
            .zip(key_set_config.counts.iter())
        {
            let curve_type =
                CurveType::try_from(*curve_type).map_err(|e| unexpected_err(e, None))?;
            let hd_root_key_count = match self.dkg_type {
                DkgType::RecoveryParty => 1,
                DkgType::Standard => hd_root_key_count.as_usize(),
            };

            let epoch_dkg_id = format!("{}.{}.{}", dkg_id, curve_type, self.dkg_type);
            let existing_root_keys = if current_peers.is_empty() {
                Vec::new()
            } else {
                chain_root_keys
                    .iter()
                    .filter_map(|k| match k.curve_type == curve_type {
                        true => Some(k.public_key.clone()),
                        false => None,
                    })
                    .collect()
            };
            for i in 0..hd_root_key_count {
                if current_peers.is_empty() {
                    let dkg_id = format!("{}_key_{}", epoch_dkg_id, i);
                    dkg_engine.add_dkg(&dkg_id, curve_type, None);
                } else {
                    let root_key = existing_root_keys.get(i).expect_or_err(format!(
                        "root key missing at index {} for curve: {}",
                        i, curve_type
                    ))?;
                    let dkg_id = format!("{}_key_{}", epoch_dkg_id, i);
                    dkg_engine.add_dkg(&dkg_id, curve_type, Some(root_key.clone()));
                }
            }
        }
        info!("DKG {} with ID {} started.", self.dkg_type, dkg_id);
        let mode = dkg_engine.execute(dkg_id, realm_id).await?;
        info!(
            "DKG {} with ID {} completed: {:?}",
            self.dkg_type, dkg_id, mode
        );
        if let Some(m) = mode
            && m == Mode::Initial
        {
            for dkg in dkg_engine.get_dkgs() {
                match dkg.result {
                    Some(ref result) => {
                        debug!(
                            "DKG for epoch change complete for {} {}.",
                            dkg.dkg_id, dkg.curve_type
                        );
                        root_keys.push(result.dkg_root_key());
                    }
                    None => {
                        error!("DKG failed!");
                        return Err(unexpected_err("DKG failed", None));
                    }
                }
            }
        }
        Ok(root_keys)
    }

    pub fn root_keys(&self) -> Vec<CachedRootKey> {
        self.tss_state.chain_data_config_manager.root_keys()
    }
}
