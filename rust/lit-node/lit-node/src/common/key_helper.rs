use crate::error::{Result, unexpected_err};
use scc::HashMap;
use serde::de::DeserializeOwned;
use soteria_rs::*;
use std::{
    fmt::{self, Debug, Display, Formatter},
    sync::Arc,
};
use tokio::sync::Mutex;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyCacheType {
    Unprotected,
    Protected,
}

pub enum KeyCacheItemWrapper {
    Protected(Arc<Mutex<Protected<DEFAULT_BUF_SIZE>>>),
    Unprotected(Vec<u8>),
}

impl KeyCacheItemWrapper {
    pub fn protected<B: AsRef<[u8]>>(buffer: B) -> Self {
        let protected = Protected::new(buffer.as_ref());
        Self::Protected(Arc::new(Mutex::new(protected)))
    }

    pub fn unprotected<B: AsRef<[u8]>>(unprotected: B) -> Self {
        Self::Unprotected(unprotected.as_ref().to_vec())
    }

    pub async fn data<T, F>(&self, mapper: F) -> Result<T>
    where
        T: DeserializeOwned,
        F: FnOnce(&[u8]) -> Result<T>,
    {
        match self {
            Self::Protected(protected) => {
                let mut protected = protected.lock().await;
                let unprotected = protected.unprotect().ok_or(unexpected_err(
                    "",
                    Some("Failed to unprotect key, possible tampering".to_string()),
                ))?;
                mapper(unprotected.as_ref())
            }
            Self::Unprotected(data) => mapper(data.as_slice()),
        }
    }
}

pub struct KeyCache(Arc<HashMap<String, KeyCacheItemWrapper>>);

impl Clone for KeyCache {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Default for KeyCache {
    fn default() -> Self {
        Self(Arc::new(HashMap::new()))
    }
}

impl Debug for KeyCache {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut entries = Vec::with_capacity(self.0.len());
        self.0.scan(|key, value| {
            entries.push(key.clone());
        });
        write!(f, "KeyCache {{ {:#?} }}", entries)
    }
}

impl Display for KeyCache {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut entries = Vec::with_capacity(self.0.len());
        self.0.scan(|key, value| {
            entries.push(key.clone());
        });
        write!(f, "KeyCache {{ {} }}", entries.join(", "))
    }
}

impl AsRef<HashMap<String, KeyCacheItemWrapper>> for KeyCache {
    fn as_ref(&self) -> &HashMap<String, KeyCacheItemWrapper> {
        &self.0
    }
}

impl KeyCache {
    pub const DEFAULT_CAPACITY: usize = 64;
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(HashMap::with_capacity(capacity)))
    }
}
