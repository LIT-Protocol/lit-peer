use std::sync::Arc;
use std::time::Duration;

use crate::testnet::SimpleTomlValue;
use anyhow::Result;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::builders::ContractCall;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain_datil::contracts::{
    backup_recovery::BackupRecovery, contract_resolver::*, lit_token::lit_token::LITToken,
    payment_delegation::PaymentDelegation, pkp_helper::pkp_helper::PKPHelper,
    pkp_permissions::PKPPermissions, pkpnft::PKPNFT, pubkey_router::PubkeyRouter, staking::Staking,
};
use lit_core::utils::toml::SimpleToml;

use serde::Deserialize;
use serde::Serialize;
use tracing::info;

#[derive(Debug, Clone)]
pub struct Contracts {
    pub lit_token: LITToken<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
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

impl Contracts {
    /// Loads contracts from contract addresses without applying any global or realm configs.
    pub async fn new_contracts(
        ca: &ContractAddresses,
        provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    ) -> Contracts {
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

        Contracts {
            lit_token,
            backup_recovery,
            staking,
            pkpnft,
            pubkey_router,
            pkp_permissions,
            pkp_helper,
            contract_resolver,
            payment_delegation,
        }
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
        contract_resolver: Address,
        provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    ) -> ContractAddresses {
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
        }
    }

    pub async fn contract_addresses_from_resolver_cfg(
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
        Self::contract_addresses_from_resolver(contract_resolver, provider).await
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

        let payment_delegation = PaymentDelegation::<
            SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
        >::new(address, client.clone());

        Ok(Contracts {
            lit_token,
            backup_recovery,
            staking,
            pkpnft,
            pubkey_router,
            pkp_permissions,
            pkp_helper,
            contract_resolver,
            payment_delegation,
        })
    }
}
