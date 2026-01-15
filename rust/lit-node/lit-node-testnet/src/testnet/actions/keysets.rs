use anyhow::Result;
use ethers::prelude::*;
use ethers::utils::keccak256;
use lit_blockchain::contracts::pubkey_router::{PubkeyRoutingData, RootKey};
use lit_blockchain::contracts::staking::staking;
use lit_core::utils::binary::{bytes_to_hex, hex_to_bytes};
use lit_node_core::CurveType;
use tracing::info;

use super::Actions;

pub struct RootKeyConfig {
    pub curve_type: CurveType,
    pub count: usize,
}

impl Actions {
    pub async fn get_root_keys(&self, curve_type: u8, keyset_id: &str) -> Option<Vec<String>> {
        let all_root_keys = self.get_all_root_keys(keyset_id).await;

        all_root_keys.as_ref()?;
        let all_root_keys: Vec<RootKey> = all_root_keys.unwrap();

        let root_keys: Vec<String> = all_root_keys
            .iter()
            .filter(|k| k.key_type == U256::from(curve_type))
            .map(|k| bytes_to_hex(k.pubkey.clone()))
            .collect::<Vec<String>>();

        Some(root_keys)
    }

    pub async fn get_all_root_keys(&self, keyset_id: &str) -> Option<Vec<RootKey>> {
        let staking_address = self.contracts.staking.address();
        let root_keys = self
            .contracts
            .pubkey_router
            .get_root_keys(staking_address, keyset_id.to_string())
            .call()
            .await
            .unwrap();

        if !root_keys.is_empty() {
            tracing::trace!("Root keys: {:?}", root_keys);
            return Some(root_keys);
        } else {
            info!("No root keys yet for contract {:?}", staking_address);
        }

        None
    }

    pub async fn add_default_keyset(
        &self,
        realm_id: U256,
        identifier: String,
        description: String,
    ) -> Result<()> {
        let root_key_configs = vec![
            RootKeyConfig {
                curve_type: CurveType::BLS,
                count: 1,
            },
            RootKeyConfig {
                curve_type: CurveType::K256,
                count: 2,
            },
            RootKeyConfig {
                curve_type: CurveType::P256,
                count: 2,
            },
            RootKeyConfig {
                curve_type: CurveType::P384,
                count: 2,
            },
            RootKeyConfig {
                curve_type: CurveType::Ed25519,
                count: 2,
            },
            RootKeyConfig {
                curve_type: CurveType::Ed448,
                count: 2,
            },
            RootKeyConfig {
                curve_type: CurveType::Ristretto25519,
                count: 2,
            },
            RootKeyConfig {
                curve_type: CurveType::RedJubjub,
                count: 2,
            },
            RootKeyConfig {
                curve_type: CurveType::RedDecaf377,
                count: 2,
            },
            RootKeyConfig {
                curve_type: CurveType::BLS12381G1,
                count: 2,
            },
        ];
        self.add_keyset(realm_id, identifier, description, root_key_configs)
            .await
    }

    pub async fn add_keyset(
        &self,
        realm_id: U256,
        identifier: String,
        description: String,
        root_key_configs: Vec<RootKeyConfig>,
    ) -> Result<()> {
        let curves = root_key_configs
            .iter()
            .map(|rkc| rkc.curve_type.into())
            .collect();
        let counts = root_key_configs
            .iter()
            .map(|rkc| U256::from(rkc.count))
            .collect();
        info!("Curves/Counts: {:?}/{:?}", curves, counts);
        let key_set_config = staking::KeySetConfig {
            minimum_threshold: 3,
            monetary_value: 0,
            complete_isolation: false,
            identifier: identifier.clone(),
            description: description,
            realms: vec![realm_id],
            curves: curves,
            counts: counts,
            recovery_session_id: Bytes::from_static(&[]),
        };
        self.add_keyset_config(key_set_config).await
    }

    pub async fn add_keyset_config(&self, key_set_config: staking::KeySetConfig) -> Result<()> {
        let realm_id = key_set_config.realms[0];
        let identifier = key_set_config.identifier.clone();
        let cc = self.contracts.staking.set_key_set(key_set_config);
        let result = cc
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Error sending tx to add second keyset! {:?}", e))?;
        let _result = result
            .log_msg("add_second_keyset")
            .await
            .map_err(|e| anyhow::anyhow!("Error waiting for successful add keyset tx! {:?}", e))?;
        info!(
            "Added keyset {} with identifier `{}` successfully",
            realm_id, identifier
        );
        Ok(())
    }

    pub async fn get_all_keyset_configs(&self) -> Result<Vec<staking::KeySetConfig>> {
        let key_set_configs = self
            .contracts
            .staking
            .key_sets()
            .call()
            .await?
            .into_iter()
            .map(|ks| staking::KeySetConfig::try_from(ks).unwrap())
            .collect();
        Ok(key_set_configs)
    }

    pub async fn get_keyset_config(&self, identifier: String) -> Result<staking::KeySetConfig> {
        let key_set_config = self
            .contracts
            .staking
            .get_key_set(identifier)
            .call()
            .await?;
        Ok(key_set_config)
    }

    pub async fn get_keyset_id_for_root_key(&self, root_key: &str) -> Result<String> {
        let key_set_configs = self.get_all_keyset_configs().await?;
        let root_key_bytes = hex_to_bytes(root_key.to_string())?;

        for key_set_config in key_set_configs {
            let keyset_id = key_set_config.identifier.clone();
            let root_keys = self.get_all_root_keys(&keyset_id).await;
            if root_keys.is_none() {
                continue;
            }
            let root_keys = root_keys.unwrap();
            for root_key in root_keys {
                if root_key.pubkey == root_key_bytes {
                    return Ok(keyset_id);
                }
            }
        }
        Err(anyhow::anyhow!("Could not find root key in any keyset."))
    }

    pub async fn get_keyset_id_for_pkp(&self, pubkey: &str) -> Result<String> {
        let pubkey_bytes = hex_to_bytes(pubkey.to_string())?;
        let hashed_pubkey = keccak256(pubkey_bytes);
        let token_id = U256::from_big_endian(hashed_pubkey.as_slice());

        let pubkey_routing_data: Result<PubkeyRoutingData> = self
            .contracts
            .pubkey_router
            .pubkeys(token_id)
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("Error getting pubkey routing data: {:?}", e));

        if pubkey_routing_data.is_ok() {
            let pubkey_routing_data = pubkey_routing_data.unwrap();
            if !pubkey_routing_data.key_set_identifier.is_empty() {
                return Ok(pubkey_routing_data.key_set_identifier);
            }
        }

        if self.datil_contracts.is_none() {
            info!(
                "No datil contracts exist, and no pubkey routing data found in mainnet routing contract for pubkey: {}",
                pubkey
            );
            return Err(anyhow::anyhow!(
                "Could not find token id in pubkey routing contract, and no datil contracts exist."
            ));
        }
        let datil_contracts = self.datil_contracts.as_ref().unwrap();
        let pubkey_routing_data: Result<
            lit_blockchain_lite::contracts::pubkey_router::PubkeyRoutingData,
        > = datil_contracts
            .pubkey_router
            .pubkeys(token_id)
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("Error getting datil pubkey routing data: {:?}", e));

        if pubkey_routing_data.is_ok() {
            if pubkey_routing_data.unwrap().key_type == U256::zero() {
                return Err(anyhow::anyhow!(
                    "Could not find token id in datil pubkey routing contract."
                ));
            }

            let keyset_configs = self.get_all_keyset_configs().await.unwrap();
            let key_set_config = keyset_configs
                .iter()
                .find(|ks| ks.identifier.to_lowercase().contains("datil"));

            if let Some(keyset_config) = key_set_config {
                return Ok(keyset_config.identifier.clone());
            }
        }

        return Err(anyhow::anyhow!(
            "Could not find token id in any pubkey routing contract."
        ));
    }

    pub async fn set_default_keyset_id(&self, realm_id: u64, keyset_id: &str) -> Result<()> {
        let realm_id = U256::from(realm_id);
        let mut realm_config = self.contracts.staking.realm_config(realm_id).call().await?;
        realm_config.default_key_set = keyset_id.to_string();
        self.contracts
            .staking
            .set_realm_config(realm_id, realm_config)
            .send()
            .await?;
        Ok(())
    }
}
