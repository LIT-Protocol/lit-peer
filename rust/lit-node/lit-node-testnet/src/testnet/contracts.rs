use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use super::SimpleTomlValue;
use super::Testnet;
use super::WhichTestnet;
use anyhow::Result;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::builders::ContractCall;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain::contracts::erc20::ERC20;
use lit_blockchain::contracts::staking::GlobalConfig;
use lit_blockchain::contracts::staking::RealmConfig;
use lit_blockchain::contracts::{
    backup_recovery::BackupRecovery, contract_resolver::*, ledger::Ledger,
    lit_token::lit_token::LITToken, payment_delegation::PaymentDelegation,
    pkp_helper::pkp_helper::PKPHelper, pkp_permissions::PKPPermissions, pkpnft::PKPNFT,
    price_feed::PriceFeed, pubkey_router::PubkeyRouter, staking::Staking,
};
use lit_core::utils::toml::SimpleToml;

use serde::Deserialize;
use serde::Serialize;
use tracing::info;

#[derive(Debug, Clone)]
pub struct Contracts {
    pub lit_token: LITToken<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub erc20: ERC20<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub backup_recovery: BackupRecovery<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub staking: Staking<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub pkpnft: PKPNFT<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub pubkey_router: PubkeyRouter<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub pkp_permissions: PKPPermissions<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub pkp_helper: PKPHelper<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub contract_resolver:
        ContractResolver<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub payment_delegation:
        PaymentDelegation<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub ledger: Ledger<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub price_feed: PriceFeed<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractAddresses {
    pub lit_token: Address,
    pub backup_recovery: Address,
    pub staking: Address,
    pub pkpnft: Address,
    pub pubkey_router: Address,
    pub pkp_permissions: Address,
    pub pkp_helper: Address,
    pub contract_resolver: Address,
    pub key_deriver: Address,
    pub payment_delegation: Address,
    pub ledger: Address,
    pub price_feed: Address,
}

#[derive(Default)]
#[must_use]
pub struct StakingContractGlobalConfigBuilder {
    token_reward_per_token_per_epoch: Option<U256>,
    key_types: Option<Vec<U256>>,
    minimum_validator_count: Option<U256>,
}

#[derive(Default)]
#[must_use]
pub struct StakingContractRealmConfigBuilder {
    realm_id: Option<U256>,
    epoch_length: Option<U256>,
    max_concurrent_requests: Option<U256>,
    max_presign_count: Option<U256>,
    min_presign_count: Option<U256>,
    peer_checking_interval_secs: Option<U256>,
    max_presign_concurrency: Option<U256>,
    complaint_reason_to_config: Option<HashMap<U256, ComplaintConfig>>,
    default_key_set: Option<String>,
}

impl StakingContractRealmConfigBuilder {
    pub fn max_concurrent_requests(mut self, value: U256) -> Self {
        self.max_concurrent_requests = Some(value);
        self
    }

    pub fn max_presign_count(mut self, value: U256) -> Self {
        self.max_presign_count = Some(value);
        self
    }

    pub fn max_presign_count_u64(mut self, value: Option<u64>) -> Self {
        if let Some(value) = value {
            self.max_presign_count = Some(U256::from(value));
        }
        self
    }

    pub fn min_presign_count(mut self, value: U256) -> Self {
        self.min_presign_count = Some(value);
        self
    }

    pub fn min_presign_count_u64(mut self, value: Option<u64>) -> Self {
        if let Some(value) = value {
            self.min_presign_count = Some(U256::from(value));
        }
        self
    }

    pub fn peer_checking_interval_secs(mut self, value: U256) -> Self {
        self.peer_checking_interval_secs = Some(value);
        self
    }

    pub fn max_presign_concurrency(mut self, value: U256) -> Self {
        self.max_presign_concurrency = Some(value);
        self
    }

    pub fn complaint_reason_to_config(mut self, value: HashMap<U256, ComplaintConfig>) -> Self {
        self.complaint_reason_to_config = Some(value);
        self
    }

    pub fn epoch_length(mut self, value: Option<U256>) -> Self {
        self.epoch_length = value;
        self
    }

    pub fn realm_id(mut self, value: U256) -> Self {
        self.realm_id = Some(value);
        self
    }

    pub fn default_key_set(mut self, value: Option<String>) -> Self {
        self.default_key_set = value;
        self
    }

    pub fn build(self) -> StakingContractRealmConfig {
        StakingContractRealmConfig {
            realm_id: self.realm_id.unwrap_or(U256::from(1)),
            epoch_length: self.epoch_length,
            max_concurrent_requests: self.max_concurrent_requests,
            max_presign_count: self.max_presign_count,
            min_presign_count: self.min_presign_count,
            peer_checking_interval_secs: self.peer_checking_interval_secs,
            max_presign_concurrency: self.max_presign_concurrency,
            complaint_reason_to_config: self.complaint_reason_to_config,
            default_key_set: self.default_key_set,
        }
    }
}

impl StakingContractGlobalConfigBuilder {
    pub fn token_reward_per_token_per_epoch(mut self, value: U256) -> Self {
        self.token_reward_per_token_per_epoch = Some(value);
        self
    }

    pub fn key_types(mut self, value: Vec<U256>) -> Self {
        self.key_types = Some(value);
        self
    }

    pub fn minimum_validator_count(mut self, value: U256) -> Self {
        self.minimum_validator_count = Some(value);
        self
    }

    pub fn build(self) -> StakingContractGlobalConfig {
        StakingContractGlobalConfig {
            token_reward_per_token_per_epoch: self.token_reward_per_token_per_epoch,
            key_types: self.key_types,
            minimum_validator_count: self.minimum_validator_count,
            reward_epoch_duration: Some(U256::from(86400)), // 1 day
            max_time_lock: Some(U256::from(31536000)),      // 1 year
            min_time_lock: Some(U256::from(86400 * 100)),   // 100 days
            bmin: Some(U256::from(1)),
            bmax: Some(U256::from(1000)),
            k: Some(U256::from(1)),
            p: Some(U256::from(1)),
            enable_stake_autolock: Some(true),
            token_price: Some(U256::from(1)), // 1 ether
            profit_multiplier: Some(U256::from(1)),
            usd_cost_per_month: Some(U256::from(1)),
            max_emission_rate: Some(U256::from(1)),
            min_stake_amount: Some(U256::from(1)),
            max_stake_amount: Some(U256::from(100_000_000)),
            min_self_stake: Some(U256::from(2)),
            min_self_stake_timelock: Some(U256::from(86400 * 15)), // 90 days
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct StakingContractGlobalConfig {
    token_reward_per_token_per_epoch: Option<U256>,
    key_types: Option<Vec<U256>>,
    reward_epoch_duration: Option<U256>,
    max_time_lock: Option<U256>,
    min_time_lock: Option<U256>,
    bmin: Option<U256>,
    bmax: Option<U256>,
    k: Option<U256>,
    p: Option<U256>,
    enable_stake_autolock: Option<bool>,
    token_price: Option<U256>,
    profit_multiplier: Option<U256>,
    usd_cost_per_month: Option<U256>,
    max_emission_rate: Option<U256>,
    min_stake_amount: Option<U256>,
    max_stake_amount: Option<U256>,
    min_self_stake: Option<U256>,
    min_self_stake_timelock: Option<U256>,
    minimum_validator_count: Option<U256>,
}

#[derive(Debug)]
#[allow(unused)]
pub struct StakingContractRealmConfig {
    realm_id: U256,
    epoch_length: Option<U256>,
    max_concurrent_requests: Option<U256>,
    max_presign_count: Option<U256>,
    min_presign_count: Option<U256>,
    peer_checking_interval_secs: Option<U256>,
    max_presign_concurrency: Option<U256>,
    complaint_reason_to_config: Option<HashMap<U256, ComplaintConfig>>,
    default_key_set: Option<String>,
}

#[derive(Default)]
#[must_use]
pub struct ComplaintConfigBuilder {
    tolerance: Option<U256>,
    interval_secs: Option<U256>,
    kick_penalty_percent: Option<U256>,
    kick_penalty_demerits: Option<U256>,
}

impl ComplaintConfigBuilder {
    pub fn tolerance(mut self, value: U256) -> Self {
        self.tolerance = Some(value);
        self
    }

    pub fn interval_secs(mut self, value: U256) -> Self {
        self.interval_secs = Some(value);
        self
    }

    pub fn kick_penalty_percent(mut self, value: U256) -> Self {
        self.kick_penalty_percent = Some(value);
        self
    }

    pub fn kick_penalty_demerits(mut self, value: U256) -> Self {
        self.kick_penalty_demerits = Some(value);
        self
    }

    pub fn build(self) -> ComplaintConfig {
        ComplaintConfig {
            tolerance: self.tolerance,
            interval_secs: self.interval_secs,
            kick_penalty_percent: self.kick_penalty_percent,
            kick_penalty_demerits: self.kick_penalty_demerits,
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct ComplaintConfig {
    tolerance: Option<U256>,
    interval_secs: Option<U256>,
    kick_penalty_percent: Option<U256>,
    kick_penalty_demerits: Option<U256>,
}

impl ComplaintConfig {
    pub fn builder() -> ComplaintConfigBuilder {
        ComplaintConfigBuilder::default()
    }
}

impl StakingContractGlobalConfig {
    pub fn builder() -> StakingContractGlobalConfigBuilder {
        StakingContractGlobalConfigBuilder::default()
    }
}

impl StakingContractRealmConfig {
    pub fn builder() -> StakingContractRealmConfigBuilder {
        StakingContractRealmConfigBuilder::default()
    }
}

impl Contracts {
    pub async fn new(
        ca: &ContractAddresses,
        testnet: &mut Testnet,
        provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        staking_contract_global_config: Option<StakingContractGlobalConfig>,
        staking_contract_realm_config: Option<StakingContractRealmConfig>,
    ) -> Result<Contracts> {
        let lit_token = LITToken::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            ca.lit_token,
            provider.clone(),
        );

        let contract_resolver = ContractResolver::<
            SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
        >::new(ca.contract_resolver, provider.clone());

        let backup_recovery = BackupRecovery::<
            SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
        >::new(ca.backup_recovery, provider.clone());

        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            ca.staking,
            provider.clone(),
        );
        let erc20 = ERC20::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            ca.lit_token,
            provider.clone(),
        );
        let pkpnft = PKPNFT::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            ca.pkpnft,
            provider.clone(),
        );

        let pubkey_router =
            PubkeyRouter::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
                ca.pubkey_router,
                provider.clone(),
            );
        let pkp_permissions = PKPPermissions::<
            SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
        >::new(ca.pkp_permissions, provider.clone());

        let pkp_helper =
            PKPHelper::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
                ca.pkp_helper,
                provider.clone(),
            );

        let payment_delegation = PaymentDelegation::<
            SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
        >::new(ca.payment_delegation, provider.clone());

        let ledger = Ledger::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            ca.ledger,
            provider.clone(),
        );
        let price_feed =
            PriceFeed::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
                ca.price_feed,
                provider.clone(),
            );

        if testnet.which != WhichTestnet::NoChain {
            if let Some(staking_contract_global_config) = staking_contract_global_config {
                Self::update_staking_global_config(staking.clone(), staking_contract_global_config)
                    .await?;
            }

            if let Some(staking_contract_realm_config) = staking_contract_realm_config {
                Self::update_staking_realm_config(staking.clone(), staking_contract_realm_config)
                    .await?;
            }
        }

        info!(
            "Resolver contract in staking contract {:?}",
            staking.contract_resolver().await.unwrap()
        );

        let contracts = Contracts {
            lit_token,
            erc20,
            backup_recovery,
            staking,
            pkpnft,
            pubkey_router,
            pkp_permissions,
            pkp_helper,
            contract_resolver,
            payment_delegation,
            ledger,
            price_feed,
        };

        // Loop through each staker account to execute each of their setup.
        #[cfg(feature = "testing")]
        if let Some(staker_account_setup_mapper) = testnet.staker_account_setup_mapper.as_mut() {
            for (idx, node_account) in testnet.node_accounts.iter().enumerate() {
                info!(
                    "Running custom setup function for account {:?}",
                    node_account
                );

                if let Err(e) = staker_account_setup_mapper
                    .run((idx, node_account.to_owned(), contracts.clone()))
                    .await
                {
                    panic!("Error running staker account setup: {:?}", e);
                }
            }
        }

        Ok(contracts)
    }

    pub async fn process_contract_call(
        cc: ContractCall<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>, ()>,
        desc: &str,
    ) -> bool {
        Self::process_contract_call_with_delay(cc, desc, 10).await
    }

    pub async fn process_contract_call_with_delay(
        cc: ContractCall<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>, ()>,
        desc: &str,
        delay_ms: u64,
    ) -> bool {
        let tx = cc.send().await;

        match tx {
            Ok(tx) => {
                let r = tx
                    .interval(Duration::from_millis(delay_ms))
                    .log_msg(desc)
                    .await;
                match r {
                    Ok(_) => {
                        info!("Success {}.", desc);
                        // info!("Success {}: {:?}", desc, r);
                        true
                    }
                    Err(e) => {
                        info!("Error {}: {:?}", desc, e);
                        false
                    }
                }
            }
            Err(e) => {
                info!("Error {}: {:?}", desc, e);
                false
            }
        }
    }

    pub async fn contract_addresses_from_resolver(
        config_path: String,
        provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    ) -> ContractAddresses {
        let config_path = format!("./{}/lit_config0.toml", config_path); // fix me
        let path = std::path::Path::new(&config_path);
        let cfg = SimpleToml::try_from(path).unwrap();

        info!(
            "Reusing earlier deployment.  Loading contract addresses from '{:?}'",
            config_path
        );

        // get the staking contract address from the config file - it's the subnetid
        let staking = cfg
            .get_address("subnet", "id")
            .expect("couldn't load staking address");

        // get the resolver contract address from the staking contract
        let staking_contract = Staking::new(staking, provider.clone());
        let contract_resolver = staking_contract.contract_resolver().call().await.unwrap();
        let resolver = ContractResolver::new(contract_resolver, provider.clone());

        let env: u8 = 0;

        // get contract addresses from resolver contract
        let lit_token = resolver
            .get_contract(resolver.lit_token_contract().call().await.unwrap(), env)
            .call()
            .await
            .unwrap();
        let pkpnft = resolver
            .get_contract(resolver.pkp_nft_contract().call().await.unwrap(), env)
            .call()
            .await
            .unwrap();

        let pkp_helper = resolver
            .get_contract(resolver.pkp_helper_contract().call().await.unwrap(), env)
            .call()
            .await
            .unwrap();

        let pubkey_router = resolver
            .get_contract(
                resolver.pub_key_router_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let pkp_permissions = resolver
            .get_contract(
                resolver.pkp_permissions_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let backup_recovery = resolver
            .get_contract(
                resolver.backup_recovery_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let staking = resolver
            .get_contract(resolver.staking_contract().call().await.unwrap(), env)
            .call()
            .await
            .unwrap();

        let key_deriver = resolver
            .get_contract(
                resolver.hd_key_deriver_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();

        let payment_delegation = resolver
            .get_contract(
                resolver.payment_delegation_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();

        let ledger = resolver
            .get_contract(resolver.ledger_contract().call().await.unwrap(), env)
            .call()
            .await
            .unwrap();
        let price_feed = resolver
            .get_contract(resolver.price_feed_contract().call().await.unwrap(), env)
            .call()
            .await
            .unwrap();

        ContractAddresses {
            lit_token,
            backup_recovery,
            staking,
            pkpnft,
            pkp_helper,
            pubkey_router,
            pkp_permissions,
            contract_resolver,
            key_deriver,
            payment_delegation,
            ledger,
            price_feed,
        }
    }

    pub async fn new_blank(
        client: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    ) -> Result<Contracts> {
        let address = Address::zero();
        let lit_token = LITToken::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            address,
            client.clone(),
        );

        let contract_resolver = ContractResolver::<
            SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
        >::new(address, client.clone());

        let backup_recovery = BackupRecovery::<
            SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
        >::new(address, client.clone());
        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            address,
            client.clone(),
        );
        let erc20 = ERC20::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            address,
            client.clone(),
        );
        let pkpnft = PKPNFT::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            address,
            client.clone(),
        );

        let pubkey_router =
            PubkeyRouter::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
                address,
                client.clone(),
            );
        let pkp_permissions = PKPPermissions::<
            SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
        >::new(address, client.clone());

        let pkp_helper =
            PKPHelper::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
                address,
                client.clone(),
            );

        let ledger = Ledger::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            address,
            client.clone(),
        );
        let price_feed =
            PriceFeed::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
                address,
                client.clone(),
            );

        let payment_delegation = PaymentDelegation::<
            SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
        >::new(address, client.clone());

        Ok(Contracts {
            lit_token,
            erc20,
            backup_recovery,
            staking,
            pkpnft,
            pubkey_router,
            pkp_permissions,
            pkp_helper,
            contract_resolver,
            payment_delegation,
            ledger,
            price_feed,
        })
    }

    #[allow(unused)]
    pub async fn update_staking_realm_config(
        staking: Staking<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        realm_config: StakingContractRealmConfig,
    ) -> Result<()> {
        info!("Updating staking contract realm config: {:?}", realm_config);

        if let Some(complaint_reason_to_config) = realm_config.complaint_reason_to_config {
            info!("Updating staking contract complaint reason configs");

            for (reason, new_config) in complaint_reason_to_config {
                // First, get current chain config for this reason.
                let current_config: lit_blockchain::contracts::staking::ComplaintConfig = staking
                    .complaint_config(reason)
                    .call()
                    .await
                    .map_err(|e| anyhow::anyhow!("unable to get complaint config: {:?}", e))?;

                // Then, set the config with any new values.
                let cc = staking.set_complaint_config(
                    reason,
                    lit_blockchain::contracts::staking::ComplaintConfig {
                        tolerance: new_config.tolerance.unwrap_or(current_config.tolerance),
                        interval_secs: new_config
                            .interval_secs
                            .unwrap_or(current_config.interval_secs),
                        kick_penalty_percent: new_config
                            .kick_penalty_percent
                            .unwrap_or(current_config.kick_penalty_percent),
                        kick_penalty_demerits: new_config
                            .kick_penalty_demerits
                            .unwrap_or(current_config.kick_penalty_demerits),
                    },
                );
                Self::process_contract_call(
                    cc,
                    format!("updating staking complaint config for reason {:?}", reason).as_str(),
                )
                .await;
            }
        }

        if let Some(custom_epoch_length) = realm_config.epoch_length {
            info!(
                "Updating staking contract epoch length to {}",
                custom_epoch_length
            );

            let cc = staking.set_epoch_length(realm_config.realm_id, custom_epoch_length);
            Self::process_contract_call(cc, "updating staking epoch length").await;
        }

        if let Some(max_presign_count) = realm_config.max_presign_count {
            let realm_id = realm_config.realm_id;
            info!(
                "Updating staking contract max presign count to {}",
                max_presign_count
            );
            let mut new_config: RealmConfig = staking
                .realm_config(realm_id)
                .call()
                .await
                .map_err(|e| anyhow::anyhow!("unable to get realm config: {:?}", e))?;
            new_config.max_presign_count = max_presign_count;
            if let Some(min_presign_count) = realm_config.min_presign_count {
                new_config.min_presign_count = min_presign_count
            }
            let cc = staking.set_realm_config(realm_id, new_config);
            Self::process_contract_call(cc, "updating staking max presign count").await;
        }

        Ok(())
    }

    #[allow(unused)]
    pub async fn update_staking_global_config(
        staking: Staking<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        global_config: StakingContractGlobalConfig,
    ) -> Result<()> {
        info!("Updating staking contract global config");

        // Update the config using the defaults where the user didn't specify a value.
        let global_config = staking
            .global_config()
            .call()
            .await
            .map_err(|e| anyhow::anyhow!("unable to get staking config: {:?}", e))?;

        let cc = staking.set_config(GlobalConfig {
            token_reward_per_token_per_epoch: global_config.token_reward_per_token_per_epoch,
            reward_epoch_duration: U256::from(86400), // 1 day
            max_time_lock: U256::from(31536000),      // 1 year
            min_time_lock: U256::from(86400 * 100),   // 100 days
            bmin: U256::from(1),
            bmax: U256::from(1000),
            k: U256::from(1),
            p: U256::from(1),
            enable_stake_autolock: true,
            token_price: U256::from(1),
            profit_multiplier: U256::from(1),
            usd_cost_per_month: U256::from(1),
            max_emission_rate: U256::from(1),
            min_stake_amount: U256::from(1),
            max_stake_amount: U256::from(100_000_000),
            min_self_stake: U256::from(2),
            min_self_stake_timelock: U256::from(86400 * 15), // 15 days
            minimum_validator_count: global_config.minimum_validator_count,
            min_validator_count_to_clamp_minimum_threshold: U256::from(4),
            min_threshold_to_clamp_at: U256::from(3),
            vote_to_advance_time_out: U256::from(60), // 15 days
        });
        if !Self::process_contract_call(cc, "updating staking config").await {
            return Err(anyhow::anyhow!("Error updating staking config"));
        }

        Ok(())
    }
}
