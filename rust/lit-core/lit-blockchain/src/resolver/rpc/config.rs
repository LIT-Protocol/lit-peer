use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::{Result, config_err, conversion_err, io_err, serializer_err, validation_err};

const HTTPS_SCHEME: &str = "https";

pub const RPC_RESOLVER_CFG_LOCAL: &str = "./rpc-config.yaml";
pub const RPC_RESOLVER_CFG_SYSTEM: &str = "/etc/lit/rpc-config.yaml";

pub const RPC_RESOLVER_CFG_PATHS: [&str; 2] = [RPC_RESOLVER_CFG_LOCAL, RPC_RESOLVER_CFG_SYSTEM];

pub const RPC_RESOLVER_HTTPS_CHECK_EXCLUDES: [&str; 7] = [
    "hardhat", "ganache", "anvil", "localchain", "localchainArbitrum", "yellowstone", "litMainnet",
];

pub const RPC_CONFIG_PROTECTED_CHAINS: [&str; 2] = ["yellowstone", "litMainnet"];

pub const RPC_REQUIRED_TYPES: [(&str, RpcKind); 3] =
    [("ipfs_gateways", RpcKind::IPFS), ("solana", RpcKind::SOLANA), ("cosmos", RpcKind::COSMOS)];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RpcConfig {
    chains: BTreeMap<String, Vec<RpcEntry>>,
}

impl RpcConfig {
    pub fn load() -> Result<Self> {
        for path in RPC_RESOLVER_CFG_PATHS {
            let path = PathBuf::from(path);
            if path.exists() {
                return Self::try_from(path.as_path());
            }
        }

        Err(config_err(
            format!("failed to find RPC resolver config in: {RPC_RESOLVER_CFG_PATHS:?}"),
            None,
        ))
    }

    // Accessors
    pub fn chains(&self) -> &BTreeMap<String, Vec<RpcEntry>> {
        &self.chains
    }

    // Validator
    pub fn verify(&self) -> Result<()> {
        if self.chains.is_empty() {
            return Err(validation_err("invalid config: no chains defined", None));
        }
        for (chain_id, chain) in self.chains.iter() {
            for entry in chain {
                entry.verify(chain_id).map_err(|e| {
                    validation_err(e, Some(format!("invalid config: chain '{chain_id}' invalid")))
                })?;
            }
        }

        // Check if all entries have the required kind
        for (chain_id, kind) in RPC_REQUIRED_TYPES {
            if !self
                .chains
                .get(chain_id)
                .unwrap_or(&vec![])
                .iter()
                .filter(|e| e.kind != kind)
                .collect::<Vec<_>>()
                .is_empty()
            {
                return Err(validation_err(
                    format!("'{chain_id}' requires all entries to have the kind: '{:?}'", kind),
                    None,
                ));
            }
        }

        Ok(())
    }

    // Write
    pub fn write_file(&self, path: &Path) -> Result<()> {
        let contents = serde_yaml::to_string(&self)
            .map_err(|e| conversion_err(e, Some("failed to produce yaml from RpcConfig".into())))?;

        fs::write(path, contents)
            .map_err(|e| io_err(e, Some(format!("failed to write RpcConfig to: {path:?}"))))?;

        Ok(())
    }

    pub fn write_file_local(&self) -> Result<()> {
        self.write_file(Path::new(RPC_RESOLVER_CFG_LOCAL))
    }

    // Merge
    pub fn merge(&mut self, other: Self) -> Result<()> {
        for (chain_id, other_entries) in other.chains {
            if let Some(self_entries) = self.chains.get_mut(&chain_id) {
                // Prepend the other entries.
                let mut new_entries = other_entries;
                new_entries.append(self_entries);

                *self_entries = new_entries;
            } else {
                self.chains.insert(chain_id, other_entries);
            }
        }

        Ok(())
    }
}

impl TryFrom<&Path> for RpcConfig {
    type Error = crate::error::Error;

    fn try_from(value: &Path) -> StdResult<Self, Self::Error> {
        Self::try_from(fs::read(value).map_err(|e| io_err(e, None))?)
    }
}

impl TryFrom<Vec<u8>> for RpcConfig {
    type Error = crate::error::Error;

    fn try_from(bytes: Vec<u8>) -> StdResult<Self, Self::Error> {
        Self::try_from(&bytes[..])
    }
}

impl TryFrom<&[u8]> for RpcConfig {
    type Error = crate::error::Error;

    fn try_from(bytes: &[u8]) -> StdResult<Self, Self::Error> {
        serde_yaml::from_slice(bytes).map_err(|e| serializer_err(e, None))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
pub enum RpcKind {
    #[default]
    EVM,
    IPFS,
    SOLANA,
    COSMOS,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
#[serde(deny_unknown_fields)]
pub struct RpcEntry {
    #[serde(default)]
    kind: RpcKind,
    url: String,
    headers: Option<BTreeMap<String, String>>,
    apikey: Option<String>,
}

impl RpcEntry {
    pub fn new(
        kind: RpcKind, url: String, headers: Option<BTreeMap<String, String>>,
        apikey: Option<String>,
    ) -> Self {
        Self { kind, url, headers, apikey }
    }

    // Accessors
    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn headers(&self) -> &Option<BTreeMap<String, String>> {
        &self.headers
    }

    pub fn apikey(&self) -> &Option<String> {
        &self.apikey
    }

    pub fn kind(&self) -> &RpcKind {
        &self.kind
    }

    // Validator
    fn verify(&self, name: &str) -> Result<()> {
        if self.url.is_empty() {
            return Err(validation_err("no url defined", None));
        }

        let parsed = Url::parse(self.url.as_str())
            .map_err(|e| validation_err(e, Some("failed to parse url".into())))?;

        if !RPC_RESOLVER_HTTPS_CHECK_EXCLUDES.contains(&name)
            && !parsed.scheme().eq_ignore_ascii_case(HTTPS_SCHEME)
        {
            return Err(validation_err(
                format!("invalid scheme in url: {}", parsed.scheme()),
                None,
            ));
        }

        Ok(())
    }
}
