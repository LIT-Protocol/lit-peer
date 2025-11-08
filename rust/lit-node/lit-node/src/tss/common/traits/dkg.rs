use crate::version::DataVersionReader;
use lit_node_core::CurveType;
use std::{fmt::Debug, sync::Arc};

#[async_trait::async_trait]
pub trait BasicDkg: Debug + Send + Sync {
    fn tss_state(&self) -> Arc<crate::tss::common::tss_state::TssState>;
    fn curve_type(&self) -> CurveType;
    async fn root_keys(&self) -> Vec<String> {
        let curve_type = self.curve_type();
        DataVersionReader::read_field_unchecked(
            &self.tss_state().chain_data_config_manager.root_keys,
            |crk| {
                crk.iter()
                    .filter_map(|k| match k.curve_type == curve_type {
                        true => Some(k.public_key.clone()),
                        false => None,
                    })
                    .collect()
            },
        )
    }
}
