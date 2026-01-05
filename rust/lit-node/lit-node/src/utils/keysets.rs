use crate::error::unexpected_err_code;
use crate::error::{EC, Result};
use crate::models::KeySetConfig;
use crate::{config::chain::ChainDataConfigManager, version::DataVersionReader};

pub fn get_default_keyset_id(cdm: &ChainDataConfigManager) -> Result<String> {
    let keysets = DataVersionReader::read_field_unchecked(&cdm.key_sets, |key_sets| {
        key_sets.values().cloned().collect::<Vec<_>>()
    });

    let default_keyset_id =
        DataVersionReader::read_field_unchecked(&cdm.generic_config, |generic_config| {
            generic_config.default_key_set.clone()
        });

    let default_keyset_id = match default_keyset_id {
        Some(keyset_id) => keyset_id,
        None => {
            return Err(unexpected_err_code(
                "Default keyset not found in configuration.",
                EC::NodeNoKeysetIdFound,
                None,
            ));
        }
    };

    if !key_set_id_exists(cdm, &default_keyset_id) {
        return Err(unexpected_err_code(
            "The default keyset was not found in the keysets list.",
            EC::NodeNoKeysetIdFound,
            None,
        ));
    };

    Ok(default_keyset_id)
}

pub fn key_set_id_exists(cdm: &ChainDataConfigManager, key_set_id: &str) -> bool {
    let keysets = DataVersionReader::read_field_unchecked(&cdm.key_sets, |key_sets| {
        key_sets.values().cloned().collect::<Vec<_>>()
    });

    keysets.iter().any(|keyset| keyset.identifier == key_set_id)
}

pub fn get_key_set_by_id(cdm: &ChainDataConfigManager, key_set_id: &str) -> Result<KeySetConfig> {
    let keysets = DataVersionReader::read_field_unchecked(&cdm.key_sets, |key_sets| {
        key_sets.values().cloned().collect::<Vec<_>>()
    });
    let key_set = keysets
        .iter()
        .find(|keyset| keyset.identifier == key_set_id)
        .ok_or_else(|| {
            unexpected_err_code(
                format!("Key set with id {} not found", key_set_id),
                EC::NodeNoKeysetIdFound,
                None,
            )
        })?;
    Ok(key_set.clone())
}
