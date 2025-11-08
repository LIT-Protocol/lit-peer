use elliptic_curve::group::{Group, GroupEncoding};
use lit_node_core::PeerId;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Mode {
    Initial,
    NewPeer,      // For new Peers
    RefreshPeer,  // For existing Peers where the new and old peer sets are the same.
    ExistingPeer, // For survivors in a reshare
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Initial => write!(f, "Initial"),
            Mode::NewPeer => write!(f, "NewPeer"),
            Mode::RefreshPeer => write!(f, "RefreshPeer"),
            Mode::ExistingPeer => write!(f, "ExistingPeer"),
        }
    }
}

/// The output of the DKG protocol
#[derive(Debug, Clone)]
pub struct DkgOutput<G: Group + GroupEncoding + Default> {
    /// The public key
    pub pk: G,
    /// The secret key share
    pub share: G::Scalar,
    /// The public key share commitments to other participants
    pub key_share_commitments: Vec<G>,
    /// The key share index
    pub peer_id: PeerId,
}
