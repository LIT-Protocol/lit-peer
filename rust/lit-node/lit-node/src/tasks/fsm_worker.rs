use crate::tss::common::traits::fsm_worker_metadata::FSMWorkerMetadata;
use arc_swap::ArcSwap;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::{Arc, atomic::AtomicU64};
use tracing::instrument;

#[derive(Debug)]
pub struct CounterBasedFSMWorkerMetadata {
    lifecycle_id: ArcSwap<u64>,
    greatest_peer_lifecycle_id: AtomicU64,

    shadow_lifecycle_id: ArcSwap<u64>,
    shadow_lifecyle_realm_id: AtomicU64,
    shadow_greatest_peer_lifecycle_id: AtomicU64,
}

impl Default for CounterBasedFSMWorkerMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl CounterBasedFSMWorkerMetadata {
    pub fn new() -> Self {
        Self {
            lifecycle_id: ArcSwap::new(Arc::new(0)),
            shadow_lifecycle_id: ArcSwap::new(Arc::new(0)),
            greatest_peer_lifecycle_id: 0.into(),
            shadow_greatest_peer_lifecycle_id: 0.into(),
            shadow_lifecyle_realm_id: 0.into(),
        }
    }
}

#[async_trait::async_trait]
impl FSMWorkerMetadata for CounterBasedFSMWorkerMetadata {
    type LifecycleId = u64;
    type ShadowLifecycleId = u64;

    fn set_shadow_realm_id(&self, realm_id: u64) {
        self.shadow_lifecyle_realm_id.store(realm_id, SeqCst);
    }

    fn get_lifecycle_id(&self, realm_id: u64) -> Self::LifecycleId {
        match self.is_shadow(realm_id) {
            true => *self.shadow_lifecycle_id.load_full(),
            false => *self.lifecycle_id.load_full(),
        }
    }

    #[instrument(level = "debug", skip_all)]
    fn update_lifecycle_id(&self, new_id: Option<Self::LifecycleId>, realm_id: u64) {
        match new_id {
            Some(id) => match self.is_shadow(realm_id) {
                true => self.shadow_lifecycle_id.store(Arc::new(id)),
                false => self.lifecycle_id.store(Arc::new(id)),
            },
            None => {
                // Increment the lifecycle id
                match self.is_shadow(realm_id) {
                    true => self
                        .shadow_lifecycle_id
                        .store(Arc::new(self.get_lifecycle_id(realm_id) + 1)),
                    false => self
                        .lifecycle_id
                        .store(Arc::new(self.get_lifecycle_id(realm_id) + 1)),
                }
            }
        }
    }

    #[instrument(level = "debug", skip_all)]
    fn compare_with_peer(&self, peer_lifecycle_id: Self::LifecycleId, realm_id: u64) {
        let temp_greatest_peer_lifecycle_id = match self.is_shadow(realm_id) {
            true => self.shadow_greatest_peer_lifecycle_id.load(SeqCst),
            false => self.greatest_peer_lifecycle_id.load(SeqCst),
        };

        if peer_lifecycle_id > temp_greatest_peer_lifecycle_id {
            match self.is_shadow(realm_id) {
                true => self
                    .shadow_greatest_peer_lifecycle_id
                    .store(peer_lifecycle_id, SeqCst),
                false => self
                    .greatest_peer_lifecycle_id
                    .store(peer_lifecycle_id, SeqCst),
            }
        }
    }

    #[instrument(level = "debug", skip_all)]
    async fn yield_until_update(&self, realm_id: u64) -> Self::LifecycleId {
        // Every 500ms, check if the greatest peer lifecycle ID is greater than the current lifecycle ID.
        loop {
            let my_lifecycle_id = match self.is_shadow(realm_id) {
                true => self.shadow_lifecycle_id.load_full(),
                false => self.lifecycle_id.load_full(),
            };

            let greatest_peer_lifecycle_id = match self.is_shadow(realm_id) {
                true => self.shadow_greatest_peer_lifecycle_id.load(SeqCst),
                false => self.greatest_peer_lifecycle_id.load(SeqCst),
            };

            if greatest_peer_lifecycle_id > *my_lifecycle_id {
                debug!(
                    "Peer lifecycle ID {} is greater than my lifecycle ID {}, updating metadata",
                    greatest_peer_lifecycle_id, my_lifecycle_id
                );
                return greatest_peer_lifecycle_id;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }

    fn is_shadow(&self, realm_id: u64) -> bool {
        realm_id == self.shadow_lifecyle_realm_id.load(SeqCst)
    }
}

#[derive(Debug)]
pub struct NoopFSMWorkerMetadata;

impl Default for NoopFSMWorkerMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl NoopFSMWorkerMetadata {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl FSMWorkerMetadata for NoopFSMWorkerMetadata {
    type LifecycleId = u64;
    type ShadowLifecycleId = u64;
    fn get_lifecycle_id(&self, _realm_id: u64) -> Self::LifecycleId {
        1
    }

    fn set_shadow_realm_id(&self, _realm_id: u64) {}

    fn update_lifecycle_id(&self, _new_id: Option<Self::LifecycleId>, _realm_id: u64) {}

    fn compare_with_peer(&self, _peer_lifecycle_id: Self::LifecycleId, _realm_id: u64) {}

    async fn yield_until_update(&self, _realm_id: u64) -> Self::LifecycleId {
        // Sleep for 100s so it never returns
        tokio::time::sleep(tokio::time::Duration::from_secs(100)).await;
        0
    }

    fn is_shadow(&self, _realm_id: u64) -> bool {
        false
    }
}
