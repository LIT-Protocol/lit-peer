use std::fmt::Debug;

use crate::error::Result;
use lit_node_core::LitResourceAbility;

pub trait SessionCapabilityObject: Debug {
    fn verify_capabilities_for_resource(
        &self,
        requested_lit_resource_ability: &LitResourceAbility,
    ) -> Result<()>;
}
