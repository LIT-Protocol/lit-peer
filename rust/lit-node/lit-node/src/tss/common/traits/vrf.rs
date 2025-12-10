use crate::error::Result;
use crate::tss::common::curve_state::CurveState;
use elliptic_curve::{Group, group::GroupEncoding};
use lit_vrf::Proof;

#[allow(dead_code)]
#[async_trait::async_trait]
pub trait Vrf<G>
where
    G: Group + GroupEncoding + Default,
{
    fn curve_state(&self) -> CurveState;

    async fn prove(&self, message: &[u8]) -> Result<Proof<G>>;

    async fn verify(&self, message: &[u8], proof: Proof<G>) -> Result<()>;
}
