use crate::error::{Result, unexpected_err};
use crate::peers::peer_state::models::SimplePeer;
use crate::peers::peer_state::models::SimplePeerCollection;
use crate::tss::common::models::NodeTransmissionEntry;
use lit_core::utils::binary::bytes_to_hex;
use sha2::{Digest, Sha256};
use std::time::SystemTime;
use tracing::error;

pub fn hash_message_to_hex_str(message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    bytes_to_hex(hasher.finalize())
}

pub fn hash_message_bytes_to_hex_str(message_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message_bytes);
    bytes_to_hex(hasher.finalize())
}

pub fn random_txn_id() -> Result<String> {
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| unexpected_err(e, Some("Could not get current time".into())))?
        .as_millis()
        .to_string())
}

pub fn get_body_descriptor_for_node_transmission_entry(message: &NodeTransmissionEntry) -> String {
    message.key.clone()
}

/// Validates and retrieves the peer information for our own address.
/// This ensures that the peer list entry for our address matches our actual staker address,
/// which is critical for detecting IP/port conflicts that could cause nodes to use incorrect peer IDs or keys.
///
/// # Arguments
/// * `peers` - The peer collection to look up the peer in
/// * `addr` - The socket address to look up (e.g., "127.0.0.1:7470")
/// * `own_staker_address` - The expected staker address (in hex format, e.g., "0x1e058cacb745417d47a88c0465029da3d11abf6e")
///
/// # Returns
/// * `Ok(SimplePeer)` - The validated peer information
/// * `Err(Error)` - If the peer cannot be found or if the staker address doesn't match (indicating peer list corruption)
///
/// # Errors
/// This function will return an error if:
/// - The address is not found in the peer collection
/// - The staker address in the peer list doesn't match the expected own_staker_address
pub fn validate_and_get_self_peer(
    peers: &SimplePeerCollection,
    addr: &str,
    own_staker_address: &str,
) -> Result<SimplePeer> {
    let self_peer = peers.peer_at_address(addr)?;
    let peer_staker_address_hex = bytes_to_hex(self_peer.staker_address.as_bytes());

    if own_staker_address != peer_staker_address_hex {
        error!(
            "Peer list has wrong staker_address for our address! addr: {}, own staker_address: {}, peer list staker_address: {}, peer_id from list: {}",
            addr, own_staker_address, peer_staker_address_hex, self_peer.peer_id
        );
        return Err(unexpected_err(
            format!(
                "Peer list corruption: address {} maps to wrong staker_address. Own: {}, Found: {}",
                addr, own_staker_address, peer_staker_address_hex
            ),
            None,
        ));
    }

    Ok(self_peer)
}
