use lit_core::error::Unexpected;
use lit_node_common::comms_payload::{ComsEncryptedPayload, ComsUnencryptedPayload};
use serde::{Serialize, de::DeserializeOwned};
use tracing::instrument;
use x25519_dalek::PublicKey;
use xor_name::XorName;

use crate::{
    error::{EC, Result},
    peers::PeerState,
};

#[instrument(level = "debug", skip_all)]
async fn get_receiver_pubkey(peer_state: &PeerState, peer_addr: &str) -> Result<PublicKey> {
    let peer_state_data = peer_state.connected_nodes().await;

    let peer_item = peer_state_data
        .get_peer_by_addr(peer_addr)
        .expect_or_err_code(
            EC::NodePeerNotFound,
            format!("Could not find peer with addr {}", peer_addr),
        )?;
    let rpk: PublicKey = PublicKey::from(peer_item.receiver_public_key);
    Ok(rpk)
}

#[instrument(level = "debug", skip_all)]
pub async fn encrypt_and_serialize(
    peer_state: &PeerState,
    peer_addr: &str,
    data: impl Serialize,
) -> Result<Vec<u8>> {
    // First get the receiver public key.
    let receiver_pubkey = get_receiver_pubkey(peer_state, peer_addr).await?;
    Ok(ComsUnencryptedPayload::from_object(data)?
        .encrypt(&peer_state.comskeys, &receiver_pubkey)
        .to_vec())
}

#[instrument(level = "debug", skip_all)]
async fn get_sender_pubkey(peer_state: &PeerState, sender_id: XorName) -> Result<PublicKey> {
    let peer_state_data = peer_state.connected_nodes().await;

    let peer_item = peer_state_data
        .get_peer_by_id(sender_id)
        .expect_or_err_code(
            EC::NodePeerNotFound,
            format!(
                "Could not find peer with id {}.  Peers: {:?}",
                sender_id,
                &peer_state_data
                    .peer_items()
                    .iter()
                    .map(|p| format!("{} : {}", p.id, p.addr))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        )?;
    let spk: PublicKey = PublicKey::from(peer_item.sender_public_key);
    Ok(spk)
}

#[instrument(level = "debug", skip_all)]
pub async fn deserialize_and_decrypt<T: DeserializeOwned>(
    peer_state: &PeerState,
    sender_id: XorName,
    data: &[u8],
) -> Result<T> {
    // First get the sender public key.
    let sender_pubkey = get_sender_pubkey(peer_state, sender_id).await?;

    ComsEncryptedPayload::from_bytes(data)
        .decrypt(&peer_state.comskeys, &sender_pubkey)?
        .into_object()
}
