use crate::error::unexpected_err_code;
use crate::error::{EC, Result};
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

    let default_keyset = match keysets
        .iter()
        .find(|keyset| keyset.identifier == default_keyset_id)
    {
        Some(keyset) => keyset.identifier.clone(),
        None => {
            return Err(unexpected_err_code(
                "The default keyset was not found in the keysets list.",
                EC::NodeNoKeysetIdFound,
                None,
            ));
        }
    };

    Ok(default_keyset)
}
