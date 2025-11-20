use crate::error::blockchain_err;
use crate::models::KeySetConfig;
use crate::peers::PeerState;
use crate::version::DataVersionReader;
use lit_node_core::CurveType;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CurveState {
    pub peer_state: Arc<PeerState>,
    pub curve_type: CurveType,
    pub key_set_identifier: Option<String>,
}

impl CurveState {
    pub fn new(
        peer_state: Arc<PeerState>,
        curve_type: CurveType,
        key_set_identifier: Option<String>,
    ) -> Self {
        Self {
            peer_state,
            curve_type,
            key_set_identifier,
        }
    }

    pub fn root_keys(&self) -> lit_core::error::Result<Vec<String>> {
        Ok(match &self.key_set_identifier {
            None => {
                let default_key_set = DataVersionReader::read_field_unchecked(
                    &self.peer_state.chain_data_config_manager.generic_config,
                    |generic_config| generic_config.default_key_set.clone(),
                );
                match &default_key_set {
                    Some(key_set_id) => self.get_root_keys_by_key_set_id(key_set_id)?,
                    None => DataVersionReader::read_field_unchecked(
                        &self.peer_state.chain_data_config_manager.key_sets,
                        |key_sets| {
                            Ok::<Vec<String>, lit_core::error::Error>(
                                key_sets
                                    .values()
                                    .find(|&config| valid_key_set(config, self.curve_type))
                                    .ok_or_else(|| {
                                        blockchain_err(
                                            format!(
                                                "No key set with curve type {} exists",
                                                self.curve_type
                                            ),
                                            None,
                                        )
                                    })?
                                    .root_keys_by_curve[&self.curve_type]
                                    .clone(),
                            )
                        },
                    )?,
                }
            }
            Some(key_set_id) => self.get_root_keys_by_key_set_id(key_set_id)?,
        })
    }

    fn get_root_keys_by_key_set_id(
        &self,
        key_set_id: &str,
    ) -> lit_core::error::Result<Vec<String>> {
        DataVersionReader::read_field_unchecked(
            &self.peer_state.chain_data_config_manager.key_sets,
            |key_sets| {
                let config = key_sets.get(key_set_id).ok_or_else(|| {
                    blockchain_err(
                        format!("No key set with identifier {} exists", key_set_id),
                        None,
                    )
                })?;
                if valid_key_set(config, self.curve_type) {
                    Ok(config.root_keys_by_curve[&self.curve_type].clone())
                } else {
                    Err(blockchain_err(
                        format!(
                            "Key set with identifier {} does not contain any root keys with curve type {}",
                            key_set_id, self.curve_type
                        ),
                        None,
                    ))
                }
            },
        )
    }
}

fn valid_key_set(config: &KeySetConfig, curve_type: CurveType) -> bool {
    config.root_keys_by_curve.contains_key(&curve_type)
        && config.root_key_counts.contains_key(&curve_type)
        && config.root_key_counts[&curve_type] > 0
        && !config.root_keys_by_curve[&curve_type].is_empty()
}
