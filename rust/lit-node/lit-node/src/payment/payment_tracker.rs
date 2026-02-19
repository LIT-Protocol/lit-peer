use crate::payment::{batches::Batches, payed_endpoint::PayedEndpoint};
use crate::version::{DataVersionReader, DataVersionWriter};
use sdd::AtomicShared;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default, Copy, Clone)]
pub struct NodeCapacityConfig {
    pub pkp_sign_max_concurrency: u64,
    pub enc_sign_max_concurrency: u64,
    pub lit_action_max_concurrency: u64,
    pub sign_session_key_max_concurrency: u64,
    pub global_max_capacity: u64,
}

impl NodeCapacityConfig {
    pub fn new() -> Self {
        Self {
            pkp_sign_max_concurrency: 75,
            enc_sign_max_concurrency: 300,
            lit_action_max_concurrency: 50,
            sign_session_key_max_concurrency: 300,
            global_max_capacity: 300,
        }
    }

    pub fn get_op_capacity(&self, endpoint: &PayedEndpoint) -> u64 {
        match endpoint {
            PayedEndpoint::PkpSign => self.global_max_capacity / self.pkp_sign_max_concurrency,
            PayedEndpoint::EncryptionSign => {
                self.global_max_capacity / self.enc_sign_max_concurrency
            }
            PayedEndpoint::LitAction => self.global_max_capacity / self.lit_action_max_concurrency,
            PayedEndpoint::SignSessionKey => {
                self.global_max_capacity / self.sign_session_key_max_concurrency
            }
        }
    }
}

#[derive(Default)]
pub struct PaymentTracker {
    node_capacity_config: AtomicShared<NodeCapacityConfig>,
    used_capacity: AtomicU64,
    batches: Batches,
}

impl PaymentTracker {
    pub fn register_usage(&self, endpoint: &PayedEndpoint) {
        let capacity_req = self.get_capacity_requirement(endpoint);
        self.used_capacity.fetch_add(capacity_req, Ordering::SeqCst);
        debug!(
            "Payment Tracker: Registered usage for endpoint: {:?}, percentage: {:?}",
            endpoint,
            self.get_usage_percentage()
        );
    }

    pub fn deregister_usage(&self, endpoint: &PayedEndpoint) {
        let capacity_req = self.get_capacity_requirement(endpoint);
        self.used_capacity.fetch_sub(capacity_req, Ordering::SeqCst);
        debug!(
            "Payment Tracker: Deregistered usage for endpoint: {:?}, percentage: {:?}",
            endpoint,
            self.get_usage_percentage()
        );
    }

    pub fn get_usage_percentage(&self) -> u64 {
        let used_capacity = self.used_capacity.load(Ordering::SeqCst);
        trace!("Payment Tracker: Used capacity: {:?}", used_capacity);
        DataVersionReader::read_field_unchecked(
            &self.node_capacity_config,
            |node_capacity_config| (100 * used_capacity) / node_capacity_config.global_max_capacity,
        )
    }

    pub fn batches(&self) -> &Batches {
        &self.batches
    }

    fn get_capacity_requirement(&self, endpoint: &PayedEndpoint) -> u64 {
        DataVersionReader::read_field_unchecked(
            &self.node_capacity_config,
            |node_capacity_config| node_capacity_config.get_op_capacity(endpoint),
        )
    }

    pub fn update_node_capacity_config(&self, config: NodeCapacityConfig) {
        DataVersionWriter::store(&self.node_capacity_config, config);
    }
}
