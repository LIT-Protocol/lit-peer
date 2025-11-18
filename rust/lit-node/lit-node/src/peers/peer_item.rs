use super::peer_state::models::PeerValidatorStatus;
use crate::error::Result;
use crate::utils::key_share_proof::KeyShareProofs;
use core::fmt;
use ethers::types::Address;
use libsecp256k1::PublicKey;
use lit_attestation::Attestation;
use lit_core::utils::binary::bytes_to_hex;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use xor_name::XorName;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerItem {
    pub id: XorName,
    pub addr: String,
    pub public_key: PublicKey,
    pub node_address: Address,
    pub sender_public_key: [u8; 32], // SenderPublicKey does not impl Deserialize
    pub receiver_public_key: [u8; 32], // ReceiverPublicKey does not impl Deserialize
    pub staker_address: Address,     // address of staking wallet
    pub status: PeerValidatorStatus,
    pub attestation: Option<Attestation>,
    pub version: String,
}

impl PeerItem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        addr: &str,
        public_key: PublicKey,
        node_address: Address,
        sender_public_key: [u8; 32],
        receiver_public_key: [u8; 32],
        staker_address: Address,
        status: PeerValidatorStatus,
        attestation: Option<Attestation>,
        version: String,
        key_share_proofs: KeyShareProofs,
    ) -> Self {
        PeerItem {
            id: XorName::from_content(addr.as_bytes()),
            addr: addr.into(),
            public_key,
            node_address,
            sender_public_key,
            receiver_public_key,
            staker_address,
            status,
            attestation,
            version,
        }
    }
}

impl fmt::Display for PeerItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PeerItem: id: {}, addr: {}, public_key: {}, node_address: {}, sender_public_key: {}, receiver_public_key: {}, staker_address: {}, status: {:?}, attestation: {:?}, version: {}",
            self.id,
            self.addr,
            bytes_to_hex(self.public_key.serialize()),
            self.node_address,
            bytes_to_hex(self.sender_public_key),
            bytes_to_hex(self.receiver_public_key),
            self.staker_address,
            self.status,
            self.attestation,
            self.version,
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct PeerData(HashMap<String, PeerItem>);

impl PeerData {
    pub fn get_peer_by_addr(&self, addr: &str) -> Option<PeerItem> {
        self.0.get(addr).cloned()
    }

    pub fn peer_items(&self) -> Vec<PeerItem> {
        self.0.values().cloned().collect()
    }

    pub fn get_peer_by_id(&self, id: XorName) -> Option<PeerItem> {
        self.0.iter().find(|p| p.1.id == id).map(|p| p.1.clone())
    }

    pub fn get_peer_by_staker_addr(&self, staker_address: Address) -> Option<PeerItem> {
        self.0
            .iter()
            .find(|p| p.1.staker_address == staker_address)
            .map(|p| p.1.clone())
    }

    // Upsert
    pub fn insert(&mut self, peeritem: PeerItem) -> Result<()> {
        self.0.insert(peeritem.addr.clone(), peeritem);
        Ok(())
    }

    pub fn remove_by_addr(&mut self, addr: &str) -> Result<()> {
        self.0.remove(addr);
        Ok(())
    }
}
