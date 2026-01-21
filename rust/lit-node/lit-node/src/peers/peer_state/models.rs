use crate::{
    error::{Result, unexpected_err},
    models::PeerValidator,
};
use derive_more::Display;
use ethers::types::H160;
use lit_node_core::{NodeSet, PeerId};
use rocket::serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerValidatorStatus {
    Entering, // Not in current, but in locked next
    Exiting,  // in current, but not in locked next
    Survivor, // in both
    Unknown,
}

#[derive(Debug, Display, PartialEq, Eq, Clone)]
#[display("{:?}", self)]
pub enum NetworkState {
    Active = 0,
    NextValidatorSetLocked = 1,
    ReadyForNextEpoch = 2,
    Unlocked = 3,
    Paused = 4,
    Restore = 5,
    Unknown = 255,
}

impl From<u8> for NetworkState {
    fn from(value: u8) -> Self {
        match value {
            0 => NetworkState::Active,
            1 => NetworkState::NextValidatorSetLocked,
            2 => NetworkState::ReadyForNextEpoch,
            3 => NetworkState::Unlocked,
            4 => NetworkState::Paused,
            5 => NetworkState::Restore,
            _ => NetworkState::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Ord, PartialOrd)]
pub struct SimplePeer {
    pub socket_address: String,
    pub peer_id: PeerId,
    pub staker_address: H160,
    pub key_hash: u64,
    pub kicked: bool,
    pub version: semver::Version,
    pub realm_id: ethers::types::U256,
}

// This is accurate for the current implementation
// the protocol index might change, but it would still represent the same peer.
// the key hash is a deterministic value of the address.
impl PartialEq for SimplePeer {
    fn eq(&self, other: &Self) -> bool {
        self.key_hash == other.key_hash
            && self.socket_address == other.socket_address
            && self.peer_id == other.peer_id
            && self.kicked == other.kicked
    }
}

impl From<&PeerValidator> for SimplePeer {
    fn from(validator: &PeerValidator) -> Self {
        let version =
            semver::Version::parse(&validator.version).unwrap_or(semver::Version::new(0, 0, 0));

        let peer_id = if validator.wallet_public_key.is_empty() {
            warn!(
                "Using validator index as peer_id because validator.wallet_public_key is not set"
            );
            PeerId::from_u16(validator.index.saturating_add(1))
        } else if validator.wallet_public_key.is_empty()
            || validator.wallet_public_key.iter().all(|b| *b == 0)
        {
            PeerId::NOT_ASSIGNED
        } else {
            PeerId::from_slice(&validator.wallet_public_key).expect("invalid wallet public key")
        };

        SimplePeer {
            socket_address: validator.socket_addr.clone(),
            peer_id,
            kicked: validator.is_kicked,
            staker_address: validator.staker_address,
            key_hash: validator.key_hash,
            version,
            realm_id: validator.realm_id,
        }
    }
}

impl SimplePeer {
    pub fn debug_address(&self) -> String {
        let peer_id = self.peer_id.to_string();
        let short_peer_id = if peer_id.len() > 4 {
            peer_id[..4].to_string()
        } else {
            peer_id
        };
        format!("{} [{}..] ", self.socket_address, short_peer_id)
    }

    pub fn unknown_peer() -> Self {
        SimplePeer {
            socket_address: "unknown".to_string(),
            peer_id: PeerId::NOT_ASSIGNED,
            staker_address: H160::zero(),
            key_hash: 0,
            kicked: false,
            version: semver::Version::new(0, 0, 0),
            realm_id: ethers::types::U256::zero(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct SimplePeerCollection(pub Vec<SimplePeer>);

impl From<Vec<SimplePeer>> for SimplePeerCollection {
    fn from(peers: Vec<SimplePeer>) -> Self {
        SimplePeerCollection(peers)
    }
}

impl From<SimplePeerCollection> for Vec<SimplePeer> {
    fn from(peers: SimplePeerCollection) -> Self {
        peers.0
    }
}

impl SimplePeerCollection {
    pub fn active_peers(&self) -> Self {
        Self(self.0.iter().filter(|p| !p.kicked).cloned().collect())
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.iter().for_each(|p| p.peer_id.hash(&mut hasher));
        hasher.finish()
    }

    pub fn realm_id(&self) -> Result<ethers::types::U256> {
        if self.0.is_empty() {
            return Err(unexpected_err("No peers yet... ", None));
        }

        let realm_id = match self.0.first() {
            Some(p) => p.realm_id,
            None => {
                return Err(unexpected_err("No peers yet... ", None));
            }
        };

        if realm_id == ethers::types::U256::zero() {
            return Err(unexpected_err(
                "Realm is 0... this should never happen.",
                None,
            ));
        }
        Ok(realm_id)
    }

    pub fn peer_id_by_address(&self, address: &str) -> Result<PeerId> {
        self.0
            .iter()
            .find(|p| p.socket_address == address)
            .map(|p| p.peer_id)
            .ok_or_else(|| {
                unexpected_err(
                    "Peer not found in peer list (peer_id)",
                    Some(format!(
                        "Peer {} not found in {}",
                        address,
                        self.debug_addresses()
                    )),
                )
            })
    }

    pub fn contains_address(&self, address: &str) -> bool {
        self.0.iter().any(|p| p.socket_address == address)
    }

    pub fn all_peers_except(&self, address: &str) -> Self {
        Self(
            self.0
                .iter()
                .filter(|p| p.socket_address != address)
                .cloned()
                .collect(),
        )
    }

    pub fn peer_keys(&self) -> Vec<u64> {
        self.0.iter().map(|p| p.key_hash).collect()
    }

    pub fn peer_group_id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.peer_keys().iter().for_each(|k| k.hash(&mut hasher));
        hasher.finish()
    }

    pub fn debug_addresses(&self) -> String {
        let mut addresses = String::new();
        for p in &self.0 {
            if !addresses.is_empty() {
                addresses.push_str(", ");
            }
            addresses.push_str(&p.debug_address());
        }
        addresses
    }

    pub fn peer_at_address(&self, address: &str) -> Result<SimplePeer> {
        for p in &self.0 {
            if p.socket_address == address {
                return Ok(p.clone());
            }
        }
        let msg = format!("Peer / Peers: {} / {}", address, self.debug_addresses());
        Err(unexpected_err(
            "Peer not found in peer list (peer_at_address)",
            Some(msg),
        ))
    }

    pub fn peer_by_id(&self, peer_id: &PeerId) -> Result<SimplePeer> {
        self.0
            .iter()
            .find(|p| p.peer_id == *peer_id)
            .cloned()
            .ok_or_else(|| {
                unexpected_err(
                    "Peer not found in peer list (peer_by_id)",
                    Some(format!("Peer: {}", peer_id)),
                )
            })
    }

    pub fn peers_by_id(&self, peer_ids: &[PeerId]) -> Self {
        let peers = self
            .0
            .iter()
            .filter(|p| peer_ids.contains(&p.peer_id))
            .cloned()
            .collect();
        SimplePeerCollection(peers)
    }

    pub fn peer_ids(&self) -> Vec<PeerId> {
        self.0.iter().map(|p| p.peer_id).collect()
    }

    pub fn threshold_for_set_testing_only(&self) -> usize {
        crate::utils::consensus::get_threshold_count(self.0.len())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn peers_for_nodeset(&self, nodeset: &[NodeSet]) -> Self {
        let mut peers = Vec::new();
        for node in nodeset {
            if let Some(peer) = self
                .0
                .iter()
                .find(|p| p.socket_address == node.socket_address)
            {
                peers.push(peer.clone());
            }
        }
        SimplePeerCollection(peers)
    }

    pub fn leader_for_active_peers(&self, hash_key: u64) -> Result<SimplePeer> {
        let leader_hash = SimplePeerCollection::generate_hash(hash_key);
        let group_size = match self.0.len() {
            0 => {
                return Err(unexpected_err("No peers found in leader_addr.", None));
            }
            size => size,
        };
        let leader_id = leader_hash % group_size as u64;
        Ok(self.0[leader_id as usize].clone())
    }

    pub fn address_is_leader(&self, hash_key: u64, addr: &String) -> bool {
        let leader = match self.leader_for_active_peers(hash_key) {
            Ok(leader) => leader,
            Err(e) => {
                tracing::error!("Error getting leader: {}", e);
                return false;
            }
        };

        addr == &leader.socket_address
    }

    fn generate_hash<T: Hash>(input: T) -> u64 {
        let mut s = DefaultHasher::new();
        input.hash(&mut s);
        s.finish()
    }
}
