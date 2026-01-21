pub mod comms;
pub mod web;

use self::comms::channels::{deregister_comms_channel, register_comms_channel};
use self::comms::push::node_share_push_direct;
use self::comms::wait::node_share_await;
use crate::error::unexpected_err;
use flume::Sender;
use lit_node_common::config::LitNodeConfig;
use lit_node_core::NodeSet;
use lit_observability::channels::TracedSender;
use std::sync::Arc;
use std::time::Duration;

use crate::{
    peers::peer_state::models::{SimplePeer, SimplePeerCollection},
    tss::common::{
        models::{NodeTransmissionDetails, NodeWaitParams, RoundData},
        tss_state::TssState,
    },
};
use lit_core::error::Result;
use lit_node_core::PeerId;

pub struct CommsManager {
    tx_batch_manager: TracedSender<NodeTransmissionDetails>,
    tx_round_manager: Arc<Sender<RoundData>>,
    peers: SimplePeerCollection,
    self_peer: SimplePeer,
    txn_prefix: String,
    round: String,
    wait_params: NodeWaitParams,
}

impl Drop for CommsManager {
    fn drop(&mut self) {
        let tx_round_manager = self.tx_round_manager.clone();
        let txn_prefix = self.txn_prefix.clone();
        let round = self.round.clone();
        tokio::spawn(async move {
            deregister_comms_channel(tx_round_manager, &txn_prefix, &round).await;
        });
    }
}
impl CommsManager {
    pub async fn new(
        state: &Arc<TssState>,
        peer_id_offset: u16,
        txn_prefix: &str,
        round: &str,
        node_set: &[NodeSet],
    ) -> Result<Self> {
        let peers = state.peer_state.peers();
        let peers = peers.active_peers();
        let peers = peers.peers_for_nodeset(node_set);
        Self::new_with_peers(state, txn_prefix, &peers, round).await
    }

    pub async fn new_with_peers(
        state: &Arc<TssState>,
        txn_prefix: &str,
        peers: &SimplePeerCollection,
        round: &str,
    ) -> Result<Self> {
        // channel setup -> we're going to set up a channel for each round.  We could do this for each DKG as well, but this is a bit easier to debug :-)
        let tx_batch_manager = state.tx_batch_manager.clone(); // channel to send batched messages
        let tx_round_manager = state.tx_round_manager.clone(); // channel to send round messages

        let timeout = state
            .peer_state
            .lit_config
            .load_full()
            .signing_round_timeout()
            .unwrap_or(10000) as u64;

        let channels = register_comms_channel(tx_round_manager.clone(), txn_prefix, round).await?;

        let addr = &state.addr;

        let self_peer = peers.peer_at_address(addr)?;

        let wait_params = NodeWaitParams {
            channels: Some(channels.clone()),
            tx_pr: state.peer_state.complaint_channel.clone(),
            txn_prefix: txn_prefix.to_string(),
            round: round.to_string(),
            timeout,
            peer_id: self_peer.peer_id,
            exit_on_qty_recvd: None,
            poll: false,
        };

        let comms_manager = CommsManager {
            tx_batch_manager,
            tx_round_manager,
            peers: peers.clone(),
            wait_params,
            self_peer,
            txn_prefix: txn_prefix.to_string(),
            round: round.to_string(),
        };

        Ok(comms_manager)
    }

    pub async fn broadcast<B>(&self, data: B) -> Result<bool>
    where
        B: serde::Serialize,
    {
        let data = serde_json::to_string(&data)
            .map_err(|e| unexpected_err(e, Some("Error while serializing data".into())))?;
        let data = data.as_bytes().to_vec();
        self.broadcast_bytes(data).await
    }

    pub async fn broadcast_and_collect<B, C>(&self, data: B) -> Result<Vec<(PeerId, C)>>
    where
        B: serde::Serialize,
        C: serde::de::DeserializeOwned,
    {
        let expected_peers = self.peers.all_peers_except(&self.self_peer.socket_address);
        self.broadcast_and_collect_from::<B, C>(data, &expected_peers)
            .await
    }

    pub async fn broadcast_and_collect_from<B, C>(
        &self,
        data: B,
        expected_peers: &SimplePeerCollection,
    ) -> Result<Vec<(PeerId, C)>>
    where
        B: serde::Serialize,
        C: serde::de::DeserializeOwned,
    {
        let data = serde_json::to_string(&data)
            .map_err(|e| unexpected_err(e, Some("Error while serializing data".into())))?;
        let data = data.as_bytes().to_vec();
        let data = self.broadcast_bytes(data).await?;
        self.collect_from::<C>(expected_peers).await
    }

    pub async fn send_direct<B>(&self, dest_peer: &SimplePeer, data: B) -> Result<bool>
    where
        B: serde::Serialize,
    {
        let data = serde_json::to_string(&data)
            .map_err(|e| unexpected_err(e, Some("Error while serializing data".into())))?;
        let data = data.as_bytes().to_vec();
        self.send_bytes_direct(dest_peer, data).await
    }

    pub async fn collect<C>(&self) -> Result<Vec<(PeerId, C)>>
    where
        C: serde::de::DeserializeOwned,
    {
        let expected_peers = self.peers.all_peers_except(&self.self_peer.socket_address);
        self.collect_from::<C>(&expected_peers).await
    }

    pub async fn collect_from<C>(
        &self,
        expected_peers: &SimplePeerCollection,
    ) -> Result<Vec<(PeerId, C)>>
    where
        C: serde::de::DeserializeOwned,
    {
        let data = self.await_bytes_from(expected_peers, None).await?;
        let data = data
            .into_iter()
            .map(|(index, data)| {
                let data = std::str::from_utf8(data.as_slice()).unwrap_or("ERR");
                let data: C = serde_json::from_str(data).map_err(|e| {
                    unexpected_err(e, Some("Error while deserializing data".into()))
                })?;
                Ok((index, data))
            })
            .collect::<Result<Vec<(PeerId, C)>>>()?;

        Ok(data)
    }

    // Collects from the earliest participants messages received.
    pub async fn collect_from_earliest<C>(
        &self,
        expected_peers: &SimplePeerCollection,
        messages_to_collect: usize,
    ) -> Result<Vec<(PeerId, C)>>
    where
        C: serde::de::DeserializeOwned,
    {
        self.collect_or_poll_from_earliest(expected_peers, messages_to_collect, false)
            .await
    }

    // Polls for messages from the earliest participants.
    // Unlike standard collection, this function doesn't return an error if the messages are not received.
    pub async fn poll_from_earliest<C>(
        &self,
        expected_peers: &SimplePeerCollection,
        messages_to_collect: usize,
    ) -> Result<Vec<(PeerId, C)>>
    where
        C: serde::de::DeserializeOwned,
    {
        self.collect_or_poll_from_earliest(expected_peers, messages_to_collect, true)
            .await
    }

    async fn collect_or_poll_from_earliest<C>(
        &self,
        expected_peers: &SimplePeerCollection,
        messages_to_collect: usize,
        poll: bool,
    ) -> Result<Vec<(PeerId, C)>>
    where
        C: serde::de::DeserializeOwned,
    {
        let mut wait_params = self.wait_params.clone();
        wait_params.exit_on_qty_recvd = Some(messages_to_collect);
        wait_params.poll = poll;

        let data = match self
            .await_bytes_from(expected_peers, Some(wait_params))
            .await
        {
            Ok(data) => data,
            Err(e) => {
                if poll {
                    return Ok(vec![]);
                }
                return Err(e);
            }
        };

        let data = data
            .iter()
            .map(|(index, data)| {
                let data = std::str::from_utf8(data).unwrap_or("ERR");
                let data: C = serde_json::from_str(data).map_err(|e| {
                    unexpected_err(e, Some("Error while deserializing data".into()))
                })?;
                Ok((*index, data))
            })
            .collect::<Result<Vec<(PeerId, C)>>>()?;

        Ok(data)
    }

    pub async fn broadcast_bytes(&self, data: Vec<u8>) -> Result<bool> {
        // this isn't in a task because it's being placed on a channel.

        let broadcast_peers = self.peers.all_peers_except(&self.self_peer.socket_address);

        for dest_peer in &broadcast_peers.0 {
            // only broadcast to participants that are part of this protocol run - ie, signing & real-time presign use a subset.
            // if dest_peer.protocol_index.is_some() {
            let _ = node_share_push_direct(
                &self.txn_prefix,
                &self.tx_batch_manager,
                &self.self_peer,
                dest_peer,
                &self.round,
                data.clone(),
            )
            .await?;
            // }
        }

        Ok(true)
    }

    pub async fn send_bytes_direct(&self, dest_peer: &SimplePeer, data: Vec<u8>) -> Result<bool> {
        node_share_push_direct(
            &self.txn_prefix,
            &self.tx_batch_manager,
            &self.self_peer,
            dest_peer,
            &self.round,
            data,
        )
        .await
    }

    pub async fn await_bytes_from(
        &self,
        expected_peers: &SimplePeerCollection,
        wait_params: Option<NodeWaitParams>,
    ) -> Result<Vec<(PeerId, Vec<u8>)>> {
        let wait_params = match wait_params {
            Some(params) => params,
            None => self.wait_params.clone(),
        };

        node_share_await(
            wait_params,
            self.tx_round_manager.clone(),
            &self.peers,
            expected_peers,
        )
        .await
        .map_err(|e| unexpected_err(e, Some("Error while waiting for incoming data".into())))
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.wait_params.timeout = timeout.as_millis() as u64;
    }
}
