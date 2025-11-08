use crate::peers::peer_reviewer::PeerComplaint;
use crate::peers::peer_state::models::SimplePeer;
use lit_node_core::PeerId;
use lit_observability::channels::TracedSender;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Debug, Formatter},
    time::SystemTime,
};
use xor_name::XorName;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeTransmissionEntry {
    /// Header that contains like a public key and the source and destination
    /// Unique identifier for the channel, epoch
    pub key: String,
    pub src_peer_id: PeerId,
    pub dest_peer_id: PeerId,
    pub value: Vec<u8>,
    pub timestamp: u128,
}

impl Debug for NodeTransmissionEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeTransmissionEntry")
            .field("key", &self.key)
            .field("src_peer_id", &self.src_peer_id)
            .field("dest_peer_id", &self.dest_peer_id)
            .field(
                "value",
                &String::from_utf8(self.value.clone()).expect("value is utf8"),
            )
            .field("timestamp", &self.timestamp)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct NodeTransmissionDetails {
    pub dest_peer: SimplePeer,
    pub round: String,
    pub node_transmission_entry: NodeTransmissionEntry,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeTransmissionBatchEntries {
    pub entries: Vec<NodeTransmissionEntry>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeShareSetRequest {
    pub sender_id: XorName,
    pub encrypted_entry: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeShareSetResponse {
    pub success: bool,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeShareBatchSetRequest {
    pub sender_id: XorName,
    pub encrypted_entries: Vec<u8>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NodeResponse {
    pub nodeindex: u16,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct RoundsShareSet {
    /// Header
    pub key: String,
    /// The value payload
    pub value: Vec<u8>,
    /// Channel ID
    pub channel_id: String,
    /// Index of the sender node
    pub from_peer_id: PeerId,
    /// Retry in ms
    pub retry: u16,
    /// Time the message was created on the sending node
    pub created: SystemTime,
}

#[derive(Debug, Clone)]
pub struct RoundRegistration {
    pub id: String,
    pub channels: Option<RoundCommsChannel>,
}

pub enum RoundCommand {
    IncomingData,
    AddChannel,
    RemoveChannel,
    Heartbeat,
}

pub struct RoundData {
    pub command: RoundCommand,
    pub round_registration: Option<RoundRegistration>,
    pub round_share_set: Option<RoundsShareSet>,
}

#[derive(Debug, Clone)]
pub struct RoundCommsChannel {
    pub tx: flume::Sender<RoundsShareSet>,
    pub rx: flume::Receiver<RoundsShareSet>,
}

#[derive(Debug, Clone)]
pub struct NodeWaitParams {
    pub txn_prefix: String,
    pub channels: Option<RoundCommsChannel>,
    pub tx_pr: TracedSender<PeerComplaint>,
    pub round: String,
    pub peer_id: PeerId,
    pub timeout: u64,
    pub poll: bool,
    pub exit_on_qty_recvd: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsSigshare {
    pub share: k256::Scalar,
    pub public_key: k256::AffinePoint,
    pub presignature_big_r: k256::AffinePoint,
    pub msg_hash: k256::Scalar,
}
