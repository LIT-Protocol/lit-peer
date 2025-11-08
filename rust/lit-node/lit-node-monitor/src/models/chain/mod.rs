pub mod blockscout;
pub mod otter;

use serde::{Deserialize, Serialize};
use std::time::UNIX_EPOCH;

#[derive(Serialize, Deserialize)]
pub struct SimpleTx {
    pub block_hash: String,
    pub block_number: String,
    pub from: String,
    pub to: String,
    pub gas: String,
    pub gas_price: String,
    pub hash: String,
    pub input: String,
    pub is_error: String,
    pub nonce: String,
    pub time_stamp: String,
    pub transaction_index: String,
}

impl SimpleTx {
    pub fn chain_time_stamp(&self) -> String {
        self.time_stamp
            .parse::<u64>()
            .ok()
            .map(|ts| {
                let system_time = UNIX_EPOCH + std::time::Duration::from_secs(ts);
                chrono::DateTime::<chrono::Utc>::from(system_time)
            })
            .unwrap()
            .format("%Y%m%d %H:%M:%S")
            .to_string()
    }
}
