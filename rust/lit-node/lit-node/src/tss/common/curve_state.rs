use crate::tss::common::traits::dkg::BasicDkg;
use crate::tss::common::tss_state::TssState;
use lit_node_core::CurveType;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CurveState {
    pub state: Arc<TssState>,
    pub curve_type: CurveType,
}

#[async_trait::async_trait]
impl BasicDkg for CurveState {
    fn tss_state(&self) -> Arc<TssState> {
        self.state.clone()
    }

    fn curve_type(&self) -> CurveType {
        self.curve_type
    }
}
