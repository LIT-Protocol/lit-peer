use crate::{
    consts::{
        CONTRACT_CHRONICLE_CHAIN_ID, CONTRACT_CHRONICLE_RPC_URL, CONTRACT_RESOLVER_ENVIRONMENT,
    },
    error::RecoveryResult,
};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecoveryConfig {
    pub resolver_address: Option<String>,
    pub rpc_url: Option<String>,
    pub chain_id: Option<u64>,
    // 0 - develop
    // 1 - staging
    // 2 - production
    pub environment: Option<u8>,
}

impl TryFrom<String> for RecoveryConfig {
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let conf =
            serde_json::from_str(value.as_str()).map_err(crate::error::Error::InvalidJsonFormat)?;
        Ok(conf)
    }

    type Error = crate::error::Error;
}

impl RecoveryConfig {
    #[allow(dead_code)]
    fn from_slice(v: &[u8]) -> RecoveryResult<Self> {
        let conf: RecoveryConfig =
            serde_json::from_slice(v).map_err(crate::error::Error::InvalidJsonFormat)?;
        Ok(conf)
    }

    pub fn load(config_path: Option<PathBuf>) -> RecoveryResult<Self> {
        let mut config_path =
            config_path.unwrap_or(dirs::home_dir().expect("Failed to fetch user's home directory"));
        config_path.extend(crate::consts::CONFIG_STORAGE);

        // create .lit_recovery folder if it doesn't exist
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        if config_path.exists() {
            let conf = std::fs::read(config_path)?;
            if conf.is_empty() {
                Err(crate::error::Error::General("Could not find config file on disk".to_string()))
            } else {
                let conf: RecoveryConfig = serde_json::from_slice(&conf)?;
                Ok(conf)
            }
        } else {
            let config = Self {
                resolver_address: None,
                rpc_url: Some(crate::consts::CONTRACT_CHRONICLE_RPC_URL.into()),
                chain_id: Some(crate::consts::CONTRACT_CHRONICLE_CHAIN_ID),
                environment: Some(2),
            };
            let conf = serde_json::to_vec(&config)?;
            let mut fd = std::fs::File::create(config_path.clone())?;
            fd.write_all(&conf)?;
            Ok(config)
        }
    }

    pub fn save(&self, config_path: Option<PathBuf>) -> RecoveryResult<()> {
        let mut config_path =
            config_path.unwrap_or(dirs::home_dir().expect("Failed to fetch user's home directory"));
        config_path.extend(crate::consts::CONFIG_STORAGE);

        // create .lit_recovery folder if it doesn't exist
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                match std::fs::create_dir_all(parent) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Failed to create config directory: {}", e);
                        println!(
                            "Current directory: {}",
                            std::env::current_dir().unwrap().display()
                        );
                        return Err(e.into());
                    }
                }
            }
        }

        let mut fd = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_path)?;

        let conf = serde_json::to_vec(self)?;

        match fd.write_all(&conf) {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to write config to file: {}", e);
                return Err(e.into());
            }
        }
        Ok(())
    }

    pub fn get_rpc_url_or_default(&self) -> String {
        match self.rpc_url.clone() {
            Some(url) => url,
            None => CONTRACT_CHRONICLE_RPC_URL.into(),
        }
    }

    pub fn get_chain_id_or_default(&self) -> u64 {
        match self.chain_id {
            Some(id) => id,
            None => CONTRACT_CHRONICLE_CHAIN_ID,
        }
    }

    pub fn get_env_or_default(&self) -> u8 {
        match self.environment {
            Some(env) => env,
            None => CONTRACT_RESOLVER_ENVIRONMENT,
        }
    }
}
