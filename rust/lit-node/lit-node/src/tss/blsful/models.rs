use crate::tss::common::{dkg_type::DkgType, tss_state::TssState};

use lit_node_core::SigningScheme;
use std::sync::Arc;

#[derive(Debug)]
pub struct BlsState {
    pub state: Arc<TssState>,
    pub dkg_type: DkgType,
    pub signing_scheme: SigningScheme,
}

impl BlsState {
    pub fn new(state: Arc<TssState>, signing_scheme: SigningScheme) -> Self {
        Self::new_with_dkg_type(state, signing_scheme, DkgType::Standard)
    }

    pub fn new_with_dkg_type(
        state: Arc<TssState>,
        signing_scheme: SigningScheme,
        dkg_type: DkgType,
    ) -> Self {
        BlsState {
            state,
            signing_scheme,
            dkg_type,
        }
    }
}
