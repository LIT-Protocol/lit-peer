use crate::config::chain::CachedRootKey;
use crate::error::{Result, unexpected_err};
use crate::models::KeySetConfig;
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tss::common::dkg_type::DkgType;
use crate::tss::common::tss_state::TssState;
use crate::tss::dkg::engine::{DkgAfterRestore, DkgEngine};
use crate::tss::dkg::models::Mode;
use crate::version::DataVersionWriter;
use lit_core::error::Unexpected;
use std::collections::HashMap;
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

    #[allow(clippy::too_many_arguments)]
    #[instrument(level = "debug", skip(self, current_peers, new_peers))]
    pub async fn change_epoch(
        &self,
        dkg_id: &str,
        epoch_number: u64,
        shadow_key_opts: (u64, u64),
        realm_id: u64,
        current_peers: &SimplePeerCollection,
        new_peers: &SimplePeerCollection,
        key_sets: &Vec<KeySetConfig>,
    ) -> Result<HashMap<String, Vec<CachedRootKey>>> {
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

        if key_sets.is_empty() {
            error!("No key sets exist to do DKGs");
            return Err(unexpected_err(
                "No key sets exist to do DKGs".to_string(),
                None,
            ));
        }

        for key_set_config in key_sets {
            for (&curve_type, &hd_root_key_count) in &key_set_config.root_key_counts {
                let hd_root_key_count = match self.dkg_type {
                    DkgType::RecoveryParty => 1,
                    DkgType::Standard => hd_root_key_count,
                };
                let epoch_dkg_id = format!("{}.{}.{}", dkg_id, curve_type, self.dkg_type);
                let existing_root_keys = key_set_config
                    .root_keys_by_curve
                    .get(&curve_type)
                    .expect("expected existing root keys but got none")
                    .clone();
                for i in 0..hd_root_key_count {
                    let dkg_id = format!("{}_key_{}", epoch_dkg_id, i);
                    if current_peers.is_empty() {
                        dkg_engine.add_dkg(&dkg_id, &key_set_config.identifier, curve_type, None);
                    } else {
                        let root_key = existing_root_keys.get(i).expect_or_err(format!(
                            "root key missing at index {} for curve: {}",
                            i, curve_type
                        ))?;
                        let key = Some(root_key.clone());
                        dkg_engine.add_dkg(&dkg_id, &key_set_config.identifier, curve_type, key);
                    }
                }
            }
        }
        info!("DKG {} with ID {} started.", self.dkg_type, dkg_id);
        let mode = dkg_engine.execute(dkg_id, realm_id).await?;
        info!(
            "DKG {} with ID {} completed: {:?}",
            self.dkg_type, dkg_id, mode
        );
        let mut root_keys = HashMap::with_capacity(key_sets.len());
        if let Some(m) = mode {
            if m == Mode::Initial {
                let mut key_sets = DataVersionWriter::new_unchecked(
                    &self.tss_state.chain_data_config_manager.key_sets,
                );
                for dkg in dkg_engine.get_dkgs() {
                    match dkg.result {
                        Some(ref result) => {
                            debug!(
                                "DKG for epoch change complete for {} {}.",
                                dkg.dkg_id, dkg.curve_type
                            );
                            let key_set = key_sets
                                .get_mut(&dkg.key_set_id)
                                .expect("How can key set have a DKG but not exist in the key set?");
                            let counts = key_set.root_key_counts[&dkg.curve_type];
                            key_set
                                .root_keys_by_curve
                                .entry(dkg.curve_type)
                                .and_modify(|v: &mut Vec<String>| v.push(result.public_key()))
                                .or_insert_with(|| {
                                    let mut v = Vec::with_capacity(counts);
                                    v.push(result.public_key());
                                    v
                                });
                            root_keys
                                .entry(dkg.key_set_id.clone())
                                .and_modify(|v: &mut Vec<CachedRootKey>| {
                                    v.push(result.dkg_root_key())
                                })
                                .or_insert_with(|| {
                                    let mut v = Vec::with_capacity(counts);
                                    v.push(result.dkg_root_key());
                                    v
                                });
                        }
                        None => {
                            error!("DKG failed!");
                            return Err(unexpected_err("DKG failed", None));
                        }
                    }
                }
                key_sets.commit();
            }
        }
        Ok(root_keys)
    }
}
