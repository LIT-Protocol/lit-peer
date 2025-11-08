#![allow(dead_code)]

use crate::error::{EC, unexpected_err_code};
use crate::p2p_comms::comms::push::{
    is_operation_epoch_change, parse_epoch_change_operation_id, parse_node_share_key,
};
use crate::tss::common::models::{NodeTransmissionEntry, RoundCommand, RoundData, RoundsShareSet};
use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;

use lit_core::error::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;
use tracing::instrument;

#[instrument(level = "debug", name = "handle_node_share_set", skip_all, fields(txn_prefix = entry.key))]
pub async fn handle_node_share_set(
    tx_round_sender: &Arc<flume::Sender<RoundData>>,
    fsm_worker_metadata: &Arc<dyn FSMWorkerMetadata<LifecycleId = u64, ShadowLifecycleId = u64>>,
    entry: NodeTransmissionEntry,
    remote_addr: SocketAddr,
) -> Result<()> {
    let parsed = parse_node_share_key(&entry.key)?;

    let operation_type_and_id = parsed.operation_type_and_id.clone();

    let round_number = parsed.round;

    let channel_id = format!("{}-{}", operation_type_and_id, round_number);
    let created = SystemTime::now();
    // to be deleted when network reaches v0.2.15 -> this translates the incoming codes.
    let key = entry.key;

    let round_share_set = RoundsShareSet {
        key,
        value: entry.value,
        channel_id,
        from_peer_id: entry.src_peer_id,
        retry: 0,
        created,
    };

    // If the dkg_id is from an earlier lifecycle ID, then we need to update the metadata and abort
    // and restart any current epoch changes.
    if is_operation_epoch_change(&operation_type_and_id) {
        let parsed_txn_prefix = parse_epoch_change_operation_id(&parsed.operation_type_and_id)?;
        fsm_worker_metadata
            .compare_with_peer(parsed_txn_prefix.lifecycle_id, parsed_txn_prefix.realm_id);
    }

    let round_data = RoundData {
        command: RoundCommand::IncomingData,
        round_registration: None,
        round_share_set: Some(round_share_set),
    };

    let tx_round_sender = tx_round_sender.clone();

    tokio::spawn(async move {
        let r = tx_round_sender.send_async(round_data).await.map_err(|e| {
            unexpected_err_code(
                e,
                EC::NodeUnknownError,
                Some("Error pushing to bg message queue".into()),
            )
        });
        if let Err(e) = r {
            error!("Error sending message in handle_node_share_set: {:?}", e);
        }
    });

    Ok(())
}
