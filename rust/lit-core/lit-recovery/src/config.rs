use crate::{
    consts::{
        CONTRACT_CHRONICLE_CHAIN_ID, CONTRACT_CHRONICLE_RPC_URL, CONTRACT_RESOLVER_ENVIRONMENT,
    },
    error::{Error, RecoveryResult},
};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::io::Write;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum ChainEnvironment {
    #[default]
    Develop = 0,
    Staging = 1,
    Production = 2,
}

impl Display for ChainEnvironment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ChainEnvironment::Develop => "develop",
                ChainEnvironment::Staging => "staging",
                ChainEnvironment::Production => "production",
            }
        )
    }
}

impl std::str::FromStr for ChainEnvironment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "develop" => Ok(ChainEnvironment::Develop),
            "staging" => Ok(ChainEnvironment::Staging),
            "production" => Ok(ChainEnvironment::Production),
            _ => Err(Error::General(format!("invalid chain environment: {}", s))),
        }
    }
}

impl TryFrom<u8> for ChainEnvironment {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ChainEnvironment::Develop),
            1 => Ok(ChainEnvironment::Staging),
            2 => Ok(ChainEnvironment::Production),
            _ => Err(Error::General(format!("Invalid chain environment: {}", value))),
        }
    }
}

impl serde::Serialize for ChainEnvironment {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_u8(*self as u8)
    }
}

impl<'de> serde::Deserialize<'de> for ChainEnvironment {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let c = u8::deserialize(d)?.try_into().map_err(serde::de::Error::custom)?;
        Ok(c)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecoveryConfig {
    pub resolver_address: Option<String>,
    pub rpc_url: Option<String>,
    pub chain_id: Option<u64>,
    // 0 - develop
    // 1 - staging
    // 2 - production
    pub environment: Option<ChainEnvironment>,
}

impl TryFrom<String> for RecoveryConfig {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let conf = serde_json::from_str(value.as_str()).map_err(Error::InvalidJsonFormat)?;
        Ok(conf)
    }
}

impl RecoveryConfig {
    #[allow(dead_code)]
    fn from_slice(v: &[u8]) -> RecoveryResult<Self> {
        let conf: RecoveryConfig = serde_json::from_slice(v).map_err(Error::InvalidJsonFormat)?;
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
                Err(Error::General("Could not find config file on disk".to_string()))
            } else {
                let conf: RecoveryConfig = serde_json::from_slice(&conf)?;
                Ok(conf)
            }
        } else {
            let config = Self {
                resolver_address: None,
                rpc_url: Some(CONTRACT_CHRONICLE_RPC_URL.into()),
                chain_id: Some(CONTRACT_CHRONICLE_CHAIN_ID),
                environment: Some(ChainEnvironment::Production),
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
                            std::env::current_dir()
                                .expect("to know the current directory")
                                .display()
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
        self.rpc_url.clone().unwrap_or_else(|| CONTRACT_CHRONICLE_RPC_URL.into())
    }

    pub fn get_chain_id_or_default(&self) -> u64 {
        self.chain_id.unwrap_or_else(|| CONTRACT_CHRONICLE_CHAIN_ID)
    }

    pub fn get_env_or_default(&self) -> ChainEnvironment {
        self.environment.unwrap_or_else(|| {
            CONTRACT_RESOLVER_ENVIRONMENT.try_into().expect("invalid environment")
        })
    }
}
