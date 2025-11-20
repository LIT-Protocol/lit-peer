use ethers::types::{Address, H160, U256};
use lit_blockchain::contracts::price_feed;
use lit_blockchain::contracts::staking::{AddressMapping, Validator, Version, staking};
use lit_blockchain::resolver::contract::ContractResolver;
use lit_blockchain::util::decode_revert;
use lit_core::config::{LitConfig, ReloadableLitConfig};
use lit_core::error::Unexpected;
use lit_core::utils::binary::{bytes_to_hex, hex_to_bytes};
use moka::future::Cache;
use rocket::serde::{Deserialize, Serialize};
use sdd::AtomicShared;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{Instrument, debug_span, info, instrument, trace, warn};

use crate::error::{EC, Result, blockchain_err, conversion_err, io_err, unexpected_err_code};
use crate::models::PeerValidator;
use crate::payment::dynamic::{LitActionPriceConfig, NodePriceMeasurement};
use crate::payment::payed_endpoint::PayedEndpoint;
use crate::peers::peer_reviewer::MAX_COMPLAINT_REASON_VALUE;
use crate::tasks::peer_checker::PeerCheckerMessage;
use crate::tasks::utils::generate_hash;
use crate::utils::networking::get_web_addr_from_chain_info;
use crate::version::{DataVersionReader, DataVersionWriter};
use lit_node_common::config::{CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT, LitNodeConfig};
use lit_node_core::{CurveType, LitActionPriceComponent};

#[derive(PartialEq, Debug)]
pub enum PeerGroupEpoch {
    Current,
    Next,
}

impl std::fmt::Display for PeerGroupEpoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeerGroupEpoch::Current => write!(f, "current"),
            PeerGroupEpoch::Next => write!(f, "next"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenericConfig {
    pub token_reward_per_token_per_epoch: u64,
    pub key_types: Vec<CurveType>,
    pub minimum_validator_count: u64,
    pub max_presign_count: u64,
    pub min_presign_count: u64,
    pub peer_checking_interval_secs: u64,
    pub max_presign_concurrency: u64,
    pub rpc_healthcheck_enabled: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ActionsConfig {
    pub timeout_ms: u64,
    pub memory_limit_mb: u64,
    pub max_code_length: u64,
    pub max_response_length: u64,
    pub max_console_log_length: u64,
    pub max_fetch_count: u64,
    pub max_sign_count: u64,
    pub max_contract_call_count: u64,
    pub max_broadcast_and_collect_count: u64,
    pub max_call_depth: u64,
    pub max_retries: u64,
    pub async_actions_enabled: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct ComplaintConfig {
    pub tolerance: u64,
    pub interval_secs: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PeersForEpoch {
    pub validators: Vec<PeerValidator>,
    pub epoch_id: String,
    pub epoch_number: u64,
    pub epoch_length: u64,
    pub epoch_read_time: std::time::SystemTime,
}

#[derive(Debug, Clone)]
pub struct CachedRootKey {
    pub public_key: String,
    pub curve_type: CurveType,
}

#[derive(Debug)]
pub struct PeersByRealm {
    pub peers_for_prior_epoch: AtomicShared<PeersForEpoch>,
    pub peers_for_current_epoch: AtomicShared<PeersForEpoch>,
    pub peers_for_next_epoch: AtomicShared<PeersForEpoch>,
}

#[derive(Debug)]
pub struct ChainDataConfigManager {
    pub peers: PeersByRealm,
    pub shadow_peers: PeersByRealm,
    pub realm_id: AtomicShared<U256>,
    pub shadow_realm_id: AtomicShared<U256>,
    pub staker_address: AtomicShared<Address>,
    pub config: ReloadableLitConfig,
    pub root_keys: AtomicShared<Vec<CachedRootKey>>,
    pub generic_config: AtomicShared<GenericConfig>,
    pub actions_config: AtomicShared<ActionsConfig>,
    pub complaint_reason_to_config: Cache<U256, ComplaintConfig>,
    pub version_requirements: Cache<U256, Version>,
    pub dynamic_lit_action_price_configs: AtomicShared<Vec<LitActionPriceConfig>>,
    pub base_network_prices: AtomicShared<Vec<U256>>,
    peer_checker_tx: flume::Sender<PeerCheckerMessage>,
}

impl ChainDataConfigManager {
    pub async fn new(
        config: ReloadableLitConfig,
        peer_checker_tx: flume::Sender<PeerCheckerMessage>,
    ) -> Self {
        let peers_for_epoch = PeersForEpoch {
            validators: Vec::new(),
            epoch_id: "".to_string(),
            epoch_number: 0,
            epoch_read_time: std::time::SystemTime::now(),
            epoch_length: 0,
        };

        // Set up initial values for the complaint reason.
        let complaint_reason_to_config = Cache::builder().build();
        let default_complaint_config = ComplaintConfig {
            tolerance: 999,
            interval_secs: 60,
        };
        complaint_reason_to_config
            .insert(U256::from(1), default_complaint_config)
            .await;
        complaint_reason_to_config
            .insert(U256::from(2), default_complaint_config)
            .await;
        complaint_reason_to_config
            .insert(U256::from(3), default_complaint_config)
            .await;
        complaint_reason_to_config
            .insert(U256::from(4), default_complaint_config)
            .await;

        let peers = PeersByRealm {
            peers_for_current_epoch: AtomicShared::new(peers_for_epoch.clone()),
            peers_for_next_epoch: AtomicShared::new(peers_for_epoch.clone()),
            peers_for_prior_epoch: AtomicShared::new(peers_for_epoch.clone()),
        };

        let shadow_peers = PeersByRealm {
            peers_for_current_epoch: AtomicShared::new(peers_for_epoch.clone()),
            peers_for_next_epoch: AtomicShared::new(peers_for_epoch.clone()),
            peers_for_prior_epoch: AtomicShared::new(peers_for_epoch.clone()),
        };

        Self {
            peers,
            shadow_peers,
            realm_id: AtomicShared::new(U256::from(0)),
            staker_address: AtomicShared::new(Address::zero()),
            shadow_realm_id: AtomicShared::new(U256::from(0)),
            config,
            root_keys: AtomicShared::new(Vec::new()),
            generic_config: AtomicShared::new(GenericConfig {
                token_reward_per_token_per_epoch: 0,
                key_types: Vec::new(),
                minimum_validator_count: 2,
                max_presign_count: 0,
                min_presign_count: 0,
                peer_checking_interval_secs: 5,
                max_presign_concurrency: 2,
                rpc_healthcheck_enabled: false,
            }),
            actions_config: AtomicShared::new(ActionsConfig {
                timeout_ms: 30000,
                memory_limit_mb: 256,
                max_code_length: 16 * 1024 * 1024,
                max_response_length: 1024 * 100,
                max_console_log_length: 1024 * 100,
                max_fetch_count: 75,
                max_sign_count: 10,
                max_contract_call_count: 30,
                max_broadcast_and_collect_count: 30,
                max_call_depth: 5,
                max_retries: 3,
                async_actions_enabled: false,
            }),
            complaint_reason_to_config,
            version_requirements: Cache::builder().build(),
            dynamic_lit_action_price_configs: AtomicShared::new(Vec::new()),
            base_network_prices: AtomicShared::new(Vec::new()),
            peer_checker_tx,
        }
    }

    pub async fn watch_chain_data_config(&self, mut quit_rx: mpsc::Receiver<bool>) {
        let interval_delay =
            self.config
                .load()
                .chain_polling_interval_ms()
                .unwrap_or(CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT) as u64;

        let period = Duration::from_millis(interval_delay);
        let mut interval = tokio::time::interval(period);

        let mut ticks: u64 = 0;

        info!(
            "Starting: ChainDataConfigManager::watch_chain_data_config with period: {} ms",
            period.as_millis()
        );

        loop {
            tokio::select! {
                _ = quit_rx.recv() => {
                    break;
                }
                _ = interval.tick() => {
                    // Continue below.
                }
            }

            ticks += 1;

            async {
                if self.refresh_realm_id().await.is_ok() && self.get_realm_id().is_some() {
                    let res = self.set_peer_and_epoch_data_from_chain().await;
                    if let Err(e) = res {
                        warn!("Error setting peer and epoch config: {e:?}");
                    }

                    let res = self.set_root_keys_from_chain().await;
                    if let Err(e) = res {
                        warn!("Error setting root pubkeys from chain: {e:?}");
                    }

                    let res = self.set_all_config_from_chain().await;
                    if let Err(e) = res {
                        warn!("Error setting complaint config from chain: {e:?}");
                    }

                    let res = self.set_version_requirements(None).await;
                    if let Err(e) = res {
                        warn!("Error setting version requirements from chain: {e:?}");
                    }

                    let res = self.set_lit_action_price_config_from_chain().await;
                    if let Err(e) = res {
                        warn!("Error setting dynamic payment config from chain: {e:?}");
                    }

                    let res = self.set_base_network_prices_from_chain().await;
                    if let Err(e) = res {
                        warn!("Error setting base network prices from chain: {e:?}");
                    }
                }
            }
            .instrument(debug_span!("watch_chain_data_config"))
            .await;
        }

        info!("Stopped: ChainDataConfigManager::watch_chain_data_config");
    }

    fn get_config_with_resolver(&self) -> Result<(Arc<LitConfig>, ContractResolver)> {
        let config = self.config.load_full();
        let contract_resolver = ContractResolver::try_from(config.as_ref())
            .map_err(|e| unexpected_err_code(e, EC::NodeContractResolverConversionFailed, None))?;

        Ok((config, contract_resolver))
    }

    #[instrument(level = "debug", skip_all)]
    pub(crate) async fn set_root_keys_from_chain(&self) -> Result<()> {
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let staking_contract = contract_resolver.staking_contract(&config).await?;
        let staking_contract_address = staking_contract.address();
        let contract = contract_resolver.pub_key_router_contract(&config).await?;

        let root_keys: Vec<lit_blockchain::contracts::pubkey_router::RootKey> = contract
            .get_root_keys(
                staking_contract_address,
                crate::tss::util::DEFAULT_KEY_SET_NAME.to_string(),
            )
            .call()
            .await
            .map_err(|e| blockchain_err(e, Some("Unable to get root keys from contract".into())))?;

        let mut cache = Vec::with_capacity(root_keys.len());
        for k in root_keys.into_iter() {
            cache.push(CachedRootKey {
                public_key: bytes_to_hex(&k.pubkey),
                curve_type: CurveType::try_from(k.key_type).map_err(|e| io_err(e, None))?,
            });
        }
        DataVersionWriter::store(&self.root_keys, cache);
        Ok(())
    }

    pub fn get_realm_id(&self) -> Option<U256> {
        let realm_id = DataVersionReader::new(&self.realm_id).map(|r| *r);
        if realm_id == Some(U256::zero()) {
            return None;
        }
        realm_id
    }

    pub fn get_shadow_realm_id(&self) -> Option<U256> {
        let realm_id = DataVersionReader::new(&self.shadow_realm_id).map(|r| *r);
        if realm_id == Some(U256::zero()) {
            return None;
        }
        realm_id
    }

    pub fn root_keys(&self) -> Vec<CachedRootKey> {
        DataVersionReader::new_unchecked(&self.root_keys).clone()
    }

    pub fn get_actions_config(&self) -> ActionsConfig {
        *DataVersionReader::new_unchecked(&self.actions_config)
    }

    pub fn get_staker_address(&self) -> H160 {
        *DataVersionReader::new_unchecked(&self.staker_address)
    }

    pub fn get_dynamic_lit_action_price_configs(&self) -> Vec<LitActionPriceConfig> {
        DataVersionReader::new_unchecked(&self.dynamic_lit_action_price_configs).clone()
    }

    #[instrument(level = "debug", skip_all)]
    async fn refresh_realm_id(&self) -> Result<(U256, U256)> {
        trace!("refresh_realm_id");
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let staking = contract_resolver.staking_contract(&config).await?;
        let my_staker_address = config.staker_address()?;
        let my_staker_address = H160::from_slice(&hex_to_bytes(&my_staker_address)?);

        DataVersionWriter::store(&self.staker_address, my_staker_address);

        let realm_id = staking
            .get_realm_id_for_staker_address(my_staker_address)
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    decode_revert(&e, staking.abi()),
                    Some("Unable to contact chain to get realm id for node in the current/next epoch".into()),
                )
            });

        let realm_id = match realm_id {
            Ok(realm_id) => realm_id,
            Err(e) => {
                return Err(blockchain_err(
                    anyhow::Error::msg(format!(
                        "Unable to get realm id for node with staker {:?} in the current/next epoch",
                        my_staker_address
                    )),
                    None,
                ));
            }
        };

        if realm_id == U256::zero() {
            // return an error if the realm id is zero
            return Err(blockchain_err(
                anyhow::Error::msg(
                    "Unable to get realm id for node, who appears to be staked but not part of any realms",
                ),
                None,
            ));
        };

        DataVersionWriter::store(&self.realm_id, realm_id);

        let shadow_realm_id = staking
            .get_shadow_realm_id_for_staker_address(my_staker_address)
            .call()
            .await
            .map_err(|e| blockchain_err(e, Some("Unable to get shadow realm id for node".into())));

        let shadow_realm_id = shadow_realm_id.unwrap_or_else(|e| U256::from(0));

        if shadow_realm_id != U256::zero() {
            DataVersionWriter::store(&self.shadow_realm_id, shadow_realm_id);
        }

        trace!(
            "get_realm_id for staker {} for realm {} and shadow realm {}",
            my_staker_address, realm_id, shadow_realm_id
        );
        Ok((realm_id, shadow_realm_id))
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn set_peer_and_epoch_data_from_chain(&self) -> Result<()> {
        let Some(realm_id) = self.get_realm_id() else {
            error!("Unable to get realm id for node in the current/next epoch");
            return Ok(());
        };

        trace!("set_peer_and_epoch_data_from_chain");

        if realm_id != U256::from(0) {
            self.set_peers_and_epoch_data_from_chain_by_realm(realm_id, &self.peers)
                .await?;
        }

        let shadow_realm_id = self.get_shadow_realm_id();
        if let Some(shadow_realm_id) = shadow_realm_id {
            self.set_peers_and_epoch_data_from_chain_by_realm(shadow_realm_id, &self.shadow_peers)
                .await?;
        }

        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn set_lit_action_price_config_from_chain(&self) -> Result<()> {
        trace!("set_dynamic_payment_config_from_chain()");

        let configs = self.get_lit_action_price_configs().await?;

        let mut dynamic_payment_config =
            DataVersionWriter::new_unchecked(&self.dynamic_lit_action_price_configs);
        dynamic_payment_config.clear();
        dynamic_payment_config.extend(configs.into_iter());
        dynamic_payment_config.commit();
        Ok(())
    }

    pub async fn get_lit_action_price_configs(&self) -> Result<Vec<LitActionPriceConfig>> {
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let contract = contract_resolver.price_feed_contract(&config).await?;

        let price_feed_configs: Vec<price_feed::LitActionPriceConfig> = contract
            .get_lit_action_price_configs()
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get price feed config from chain.".into()),
                )
            })?;

        let configs: Vec<Result<LitActionPriceConfig>> = price_feed_configs
            .iter()
            .map(|c| {
                Ok(LitActionPriceConfig {
                    price_component: LitActionPriceComponent::try_from(c.price_component)
                        .map_err(|e| unexpected_err_code(e, EC::PaymentFailed, None))?,
                    price_measurement: NodePriceMeasurement::try_from(c.price_measurement)
                        .map_err(|e| unexpected_err_code(e, EC::PaymentFailed, None))?,
                    price: c.price.as_u64(),
                })
            })
            .collect();

        trace!("Dynamic payment configs: {:?}", configs);

        // note that this actually halts all payment processing, and all lit actions will fail.
        if configs.iter().any(|c| c.is_err()) {
            return Err(unexpected_err_code(
                "Invalid LitAction Price Configs",
                EC::PaymentFailed,
                None,
            ));
        }

        let configs = configs
            .iter()
            .map(|c| {
                c.clone()
                    .expect("Should never occur - we just checked for errors.")
            })
            .collect::<Vec<LitActionPriceConfig>>();

        Ok(configs)
    }

    async fn set_peers_and_epoch_data_from_chain_by_realm(
        &self,
        realm_id: U256,
        peers_by_realm: &PeersByRealm,
    ) -> Result<()> {
        trace!(
            "set_peers_and_epoch_data_from_chain_by_realm for realm {}",
            realm_id
        );
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let staking = contract_resolver.staking_contract(&config).await?;

        let epoch = staking
            .epoch(realm_id)
            .call()
            .await
            .map_err(|e| blockchain_err(e, Some("Unable to get epoch".into())))?;

        let mut current_validators = self
            .get_sorted_validators(PeerGroupEpoch::Current, realm_id)
            .await?;
        let mut next_validators = self
            .get_sorted_validators(PeerGroupEpoch::Next, realm_id)
            .await?;

        let mut all_validator_addresses_map =
            HashSet::with_capacity(current_validators.len() + next_validators.len());
        for v in current_validators.iter().chain(next_validators.iter()) {
            // dedup
            all_validator_addresses_map.insert(v.address);
        }
        let all_validator_addresses_list =
            all_validator_addresses_map.into_iter().collect::<Vec<_>>();
        let pubkey_mappings = staking
            .get_node_attested_pub_key_mappings(all_validator_addresses_list)
            .call()
            .await
            .map_err(|e| unexpected_err_code(e, EC::NodeBlockchainError, None))?;

        let pubkey_mappings = pubkey_mappings
            .into_iter()
            .map(|m| (m.node_address, m.pub_key))
            .collect::<HashMap<_, _>>();

        // Add in the attested wallet public keys
        for v in current_validators
            .iter_mut()
            .chain(next_validators.iter_mut())
        {
            let wallet_pub_key = pubkey_mappings.get(&v.address).ok_or(unexpected_err_code(
                "Missing attested wallet public key",
                EC::NodeBlockchainError,
                None,
            ))?;
            v.wallet_public_key.clear();
            v.wallet_public_key.extend_from_slice(&[4u8; 65]);
            wallet_pub_key
                .x
                .to_big_endian(&mut v.wallet_public_key[1..33]);
            wallet_pub_key
                .y
                .to_big_endian(&mut v.wallet_public_key[33..]);
        }

        let epoch_number = epoch.number;
        let epoch_retries = epoch.retries;
        let epoch_length = epoch.epoch_length;
        let mut hasher = Sha256::new();
        // we should be able to use validators.join(",") but it complains about
        // unsatisfied trait bounds so let's craft this manually
        let all_validator_addresses = current_validators
            .iter()
            .map(|v| {
                serde_json::to_string(&v.address)
                    .map_err(|e| conversion_err(e, Some("Unable to serialize validator".into())))
            })
            .collect::<Result<Vec<String>>>()?
            .join(",");

        let to_hash = format!(
            "{}-{}-{}",
            all_validator_addresses, epoch_number, epoch_retries
        );
        trace!(
            "{} Epoch id contents to be hashed: {}",
            config.internal_port()?,
            to_hash
        );
        hasher.update(to_hash.as_bytes());
        let epoch_id = bytes_to_hex(hasher.finalize());
        let epoch_number = epoch.number.as_u64();

        let mut peers_for_current_epoch =
            DataVersionWriter::new_unchecked(&peers_by_realm.peers_for_current_epoch);
        if peers_for_current_epoch.epoch_number < epoch_number {
            DataVersionWriter::store(
                &peers_by_realm.peers_for_prior_epoch,
                peers_for_current_epoch.clone_value(),
            );
            peers_for_current_epoch.epoch_read_time = std::time::SystemTime::now();
        };
        peers_for_current_epoch.validators = current_validators;
        peers_for_current_epoch.epoch_id = epoch_id.clone();
        peers_for_current_epoch.epoch_number = epoch_number;
        peers_for_current_epoch.commit();

        let mut peers_for_next_epoch =
            DataVersionWriter::new_unchecked(&peers_by_realm.peers_for_next_epoch);
        peers_for_next_epoch.validators = next_validators;
        peers_for_next_epoch.epoch_id = format!("{}-next", epoch_id);

        if peers_for_next_epoch.epoch_number < epoch_number + 1 {
            // this isn't super meaningful at this point.
            peers_for_next_epoch.epoch_read_time = std::time::SystemTime::now();
        };
        peers_for_next_epoch.epoch_number = epoch_number + 1;
        peers_for_next_epoch.commit();

        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn set_all_config_from_chain(&self) -> Result<()> {
        trace!("set_all_config_from_chain()");
        let Some(realm_id) = self.get_realm_id() else {
            trace!("Unable to get realm id for node in the current/next epoch");
            return Ok(());
        };

        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let contract = contract_resolver.staking_contract(&config).await?;
        let staking_contract_config = contract.global_config().call().await.map_err(|e| {
            blockchain_err(
                e,
                Some("Unable to get staking contract config from chain.".into()),
            )
        })?;
        let token_reward_per_token_per_epoch = staking_contract_config
            .token_reward_per_token_per_epoch
            .as_u64();

        let key_types = staking_contract_config
            .key_types
            .iter()
            .map(|k| CurveType::try_from(*k).expect("Key Types in Staking Config should be valid."))
            .collect::<Vec<CurveType>>();
        let minimum_validator_count = staking_contract_config.minimum_validator_count.as_u64();

        let realm_config = contract.realm_config(realm_id).call().await.map_err(|e| {
            blockchain_err(e, Some("Unable to get realm config from chain.".into()))
        })?;

        let max_presign_count = realm_config.max_presign_count.as_u64();
        let min_presign_count = realm_config.min_presign_count.as_u64();
        let peer_checking_interval_secs = realm_config.peer_checking_interval_secs.as_u64();
        let max_presign_concurrency = realm_config.max_presign_concurrency.as_u64();
        let rpc_healthcheck_enabled = realm_config.rpc_healthcheck_enabled;

        let mut generic_config = DataVersionWriter::new_unchecked(&self.generic_config);
        generic_config.key_types = key_types;
        generic_config.minimum_validator_count = minimum_validator_count;
        generic_config.max_presign_count = max_presign_count;
        generic_config.min_presign_count = min_presign_count;
        generic_config.peer_checking_interval_secs = peer_checking_interval_secs;
        generic_config.max_presign_concurrency = max_presign_concurrency;
        generic_config.rpc_healthcheck_enabled = rpc_healthcheck_enabled;

        let lit_actions_config =
            contract
                .lit_actions_config(realm_id)
                .call()
                .await
                .map_err(|e| {
                    blockchain_err(e, Some("Unable to get actions config from chain.".into()))
                })?;

        let mut actions_config = DataVersionWriter::new_unchecked(&self.actions_config);
        actions_config.timeout_ms = lit_actions_config.timeout_ms.as_u64();
        actions_config.memory_limit_mb = lit_actions_config.memory_limit_mb.as_u64();
        actions_config.max_code_length = lit_actions_config.max_code_length.as_u64();
        actions_config.max_response_length = lit_actions_config.max_response_length.as_u64();
        actions_config.max_console_log_length = lit_actions_config.max_console_log_length.as_u64();
        actions_config.max_fetch_count = lit_actions_config.max_fetch_count.as_u64();
        actions_config.max_sign_count = lit_actions_config.max_sign_count.as_u64();
        actions_config.max_contract_call_count =
            lit_actions_config.max_contract_call_count.as_u64();
        actions_config.max_broadcast_and_collect_count =
            lit_actions_config.max_broadcast_and_collect_count.as_u64();
        actions_config.max_call_depth = lit_actions_config.max_call_depth.as_u64();
        actions_config.max_retries = lit_actions_config.max_retries.as_u64();
        actions_config.async_actions_enabled = lit_actions_config.async_actions_enabled;

        // Set complaint configs from chain - we want to fetch for reasons from 1 to MAX_COMPLAINT_REASON_VALUE.
        for i in 1..=MAX_COMPLAINT_REASON_VALUE {
            let complaint_config: staking::ComplaintConfig = contract
                .complaint_config(U256::from(i))
                .call()
                .await
                .map_err(|e| {
                    blockchain_err(
                        e,
                        Some(format!("Unable to get complaint config for reason {}", i)),
                    )
                })?;

            // If the config for this reason has not been set, we want to preserve the current values.
            if complaint_config.tolerance == U256::zero()
                && complaint_config.interval_secs == U256::zero()
            {
                continue;
            }

            self.complaint_reason_to_config
                .insert(
                    U256::from(i),
                    ComplaintConfig {
                        tolerance: complaint_config.tolerance.as_u64(),
                        interval_secs: complaint_config.interval_secs.as_u64(),
                    },
                )
                .await;
        }
        generic_config.commit();
        actions_config.commit();
        Ok(())
    }

    pub async fn set_base_network_prices_from_chain(&self) -> Result<()> {
        trace!("set_base_network_prices_from_chain()");
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let contract = contract_resolver.price_feed_contract(&config).await?;
        let product_ids = PayedEndpoint::get_all_product_ids();
        let chain_base_network_prices = contract
            .base_network_prices(product_ids)
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get base network prices from chain.".into()),
                )
            })?;

        DataVersionWriter::store(&self.base_network_prices, chain_base_network_prices);
        Ok(())
    }

    /// This function updates the version requirement data that is cached against chain data.
    ///
    /// If `contract_event_data` is `None`, then all version requirements are fetched from the chain.
    /// If `contract_event_data` is `Some`, then only the specific version requirement is fetched from the chain.
    pub async fn set_version_requirements(
        &self,
        contract_event_data: Option<(U256, Version)>,
    ) -> Result<()> {
        trace!("set_version_requirements_from_chain()");
        let Some(realm_id) = self.get_realm_id() else {
            trace!("Unable to get realm id for node in the current/next epoch");
            return Ok(());
        };

        // If we have detected a version requirement change from a contract event, then we only need to update that version requirement.
        if let Some((version, version_requirement)) = contract_event_data {
            self.version_requirements
                .insert(version, version_requirement)
                .await;
            return Ok(());
        }

        // Otherwise, we need to update all version requirements.
        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let contract = contract_resolver.staking_contract(&config).await?;

        let minimum_supported_version: Version = contract
            .get_min_version(realm_id)
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get min version from staking contract".into()),
                )
            })?;

        let maximum_supported_version: Version = contract
            .get_max_version(realm_id)
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get max version from staking contract".into()),
                )
            })?;

        self.version_requirements
            .insert(U256::from(0), minimum_supported_version)
            .await;
        self.version_requirements
            .insert(U256::from(1), maximum_supported_version)
            .await;

        Ok(())
    }

    pub async fn get_min_version_requirement(&self) -> Result<Version> {
        self.version_requirements
            .get(&U256::from(0))
            .await
            .expect_or_err("Minimum version requirement not found")
    }

    pub async fn get_max_version_requirement(&self) -> Result<Version> {
        self.version_requirements
            .get(&U256::from(1))
            .await
            .expect_or_err("Maximum version requirement not found")
    }

    async fn get_sorted_validators(
        &self,
        current_or_next: PeerGroupEpoch,
        realm_id: U256,
    ) -> Result<Vec<PeerValidator>> {
        if realm_id == U256::from(0) {
            return Ok(vec![]);
        }

        let (config, contract_resolver) = self.get_config_with_resolver()?;
        let staking = contract_resolver.staking_contract(&config).await?;

        let validators = match current_or_next {
            PeerGroupEpoch::Current => staking
                .get_validators_structs_in_current_epoch(realm_id)
                .call()
                .await
                .map_err(|e| {
                    blockchain_err(e, Some("Unable to get validators in current epoch".into()))
                })?,
            PeerGroupEpoch::Next => staking
                .get_validators_structs_in_next_epoch(realm_id)
                .call()
                .await
                .map_err(|e| {
                    blockchain_err(e, Some("Unable to get validators in next epoch".into()))
                })?,
        };

        let node_addresses = validators
            .iter()
            .map(|v| v.node_address)
            .collect::<Vec<_>>();
        let address_mapping: Vec<AddressMapping> = staking
            .get_node_staker_address_mappings(node_addresses)
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get node and staker addresses in next epoch".into()),
                )
            })?;

        let kicked_validators = staking
            .get_kicked_validators(realm_id)
            .call()
            .await
            .map_err(|e| {
                blockchain_err(
                    e,
                    Some("Unable to get kicked validators in next epoch".into()),
                )
            })?;

        let mut peer_validators = Self::sort_and_filter_validators(
            validators,
            kicked_validators,
            address_mapping,
            realm_id,
        );
        self.update_validator_versions(&mut peer_validators).await;

        Ok(peer_validators)
    }

    pub fn sort_and_filter_validators(
        validators: Vec<Validator>,
        kicked_validators: Vec<H160>,
        address_mapping: Vec<AddressMapping>,
        realm_id: U256,
    ) -> Vec<PeerValidator> {
        let mut peer_validators: Vec<PeerValidator> = validators
            .into_iter()
            .map(|validator| PeerValidator {
                ip: validator.ip,
                ipv6: validator.ipv_6,
                port: validator.port,
                address: validator.node_address,
                reward: validator.reward,
                coms_sender_pub_key: validator.sender_pub_key,
                coms_receiver_pub_key: validator.receiver_pub_key,
                index: 0,
                key_hash: 0,
                socket_addr: get_web_addr_from_chain_info(validator.ip, validator.port),
                staker_address: address_mapping
                    .iter()
                    .find(|am| am.node_address == validator.node_address)
                    .map(|am| am.staker_address)
                    .unwrap_or(H160::zero()),
                is_kicked: kicked_validators.contains(
                    &address_mapping
                        .iter()
                        .find(|am| am.node_address == validator.node_address)
                        .map(|am| am.staker_address)
                        .unwrap_or(H160::zero()),
                ),
                version: "0.0.0".to_string(),
                wallet_public_key: vec![],
                realm_id,
            })
            .collect();

        peer_validators.sort_by(|a, b| a.staker_address.cmp(&b.staker_address));

        // set the index on each validator
        peer_validators.iter_mut().enumerate().for_each(|(i, pv)| {
            pv.index = i as u16;
            pv.key_hash = generate_hash(pv.staker_address);
        });

        peer_validators
    }

    async fn update_validator_versions(&self, peer_validators: &mut Vec<PeerValidator>) {
        let (tx, rx) = flume::bounded(1);

        match self
            .peer_checker_tx
            .send_async(PeerCheckerMessage::GetPeers(tx))
            .await
        {
            Ok(_) => {
                let peer_data = match rx.recv_async().await {
                    Ok(peer_data) => peer_data,
                    Err(e) => {
                        warn!("Error receiving peer data: {:?}", e);
                        return;
                    }
                };
                for peer_item in peer_data.peer_items() {
                    if let Some(validator) = peer_validators
                        .iter_mut()
                        .find(|v| v.staker_address == peer_item.staker_address)
                    {
                        validator.version = peer_item.version.clone();
                    }
                }
            }
            Err(e) => {
                warn!(
                    "Error sending peer checker message, versions will be 0.0.0 for all validators: {:?}",
                    e
                );
            }
        }
    }
}
