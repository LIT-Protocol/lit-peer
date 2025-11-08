pub mod finder;
pub mod listener;
pub mod models;

use self::models::{PresignManager, PresignRequest, PresignRequestKey};
use crate::tss::common::tss_state::TssState;
use models::XorFilterWithThreshold;
use std::collections::HashMap;
use std::sync::Arc;

impl PresignRequestKey {
    pub fn from(request: PresignRequest) -> Self {
        PresignRequestKey {
            message_bytes: request.message_bytes,
            request_id: request.request_id,
        }
    }
}

impl PresignManager {
    pub fn new(
        rx: flume::Receiver<models::PresignMessage>,
        tx: flume::Sender<models::PresignMessage>,
        tss_state: Arc<TssState>,
    ) -> Self {
        info!("Creating new presign manager");

        let min_presigns = 0;
        let max_presigns = 10;
        let max_presign_concurrency = 2;
        let is_generating = false;
        let xor_filters: HashMap<models::PeerGroupId, XorFilterWithThreshold> = HashMap::new();

        PresignManager {
            min_presigns,
            max_presigns,
            max_presign_concurrency,
            rx,
            tx,
            tss_state,
            current_generation_count: [0u64; 3],
            generating_txn_ids: Vec::new(),
            last_generated: [std::time::Instant::now(); 3],
            xor_filters,
        }
    }
}
