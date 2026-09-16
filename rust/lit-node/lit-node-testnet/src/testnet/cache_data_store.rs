use std::path::{Path, PathBuf};

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::testnet::anvil_cache::TEST_CACHE_ROOT;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheDataStore {
    pub anvil_is_running: bool,
    pub datil_state_is_loaded: bool,
}

impl CacheDataStore {
    pub fn new() -> Self {
        Self {
            anvil_is_running: false,
            datil_state_is_loaded: false,
        }
    }

    pub async fn from_file_or_new() -> Result<Self, Error> {
        match Self::read_from_file().await {
            Ok(cache_data_store) => Ok(cache_data_store),
            Err(e) => {
                error!("Failed to read cache data store from file: {}", e);
                Ok(Self::new())
            }
        }
    }

    pub fn set_anvil_is_running(&mut self, anvil_is_running: bool) {
        self.anvil_is_running = anvil_is_running;
    }

    pub fn set_datil_state_is_loaded(&mut self, datil_state_is_loaded: bool) {
        self.datil_state_is_loaded = datil_state_is_loaded;
    }

    pub async fn save(&self) -> Result<(), Error> {
        self.write_to_file().await?;
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<(), Error> {
        let cache_data_store = Self::read_from_file().await?;
        *self = cache_data_store;
        Ok(())
    }

    async fn write_to_file(&self) -> Result<(), Error> {
        let contents = serde_json::to_string(self)?;
        tokio::fs::write(Self::get_cache_data_store_path().await?, contents).await?;
        Ok(())
    }

    async fn read_from_file() -> Result<Self, Error> {
        let contents = tokio::fs::read_to_string(Self::get_cache_data_store_path().await?).await?;
        let cache_data_store: CacheDataStore = serde_json::from_str(&contents)?;
        Ok(cache_data_store)
    }

    async fn get_cache_data_store_path() -> Result<PathBuf, Error> {
        if !Path::new(TEST_CACHE_ROOT).exists() {
            tokio::fs::create_dir_all(TEST_CACHE_ROOT).await?;
        }
        Ok(Path::new(TEST_CACHE_ROOT).join("cache_data_store.json"))
    }
}
