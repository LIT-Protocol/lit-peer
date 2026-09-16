// note - this file contains code that can be deleted after the version upgrade tests are enabled.
// we put these here, because they may have some custom logic that is more complex than just checking the version.

use crate::peers::peer_state::models::SimplePeerCollection;

pub fn peers_not_at_version_2_1_8(peers: &SimplePeerCollection) -> bool {
    peers.has_version_lower_than("2.1.8")
}
