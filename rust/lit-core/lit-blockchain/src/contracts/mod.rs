use std::convert::Into;
use std::sync::Arc;

use ethers::core::k256::SecretKey;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;

use lit_core::config::LitConfig;

use crate::config::LitBlockchainConfig;
use crate::contracts::allowlist::Allowlist;
use crate::contracts::backup_recovery::BackupRecovery;
use crate::contracts::contract_resolver::ContractResolver;
use crate::contracts::forwarder::Forwarder;
use crate::contracts::host_commands::HostCommands;
use crate::contracts::ledger::Ledger;
use crate::contracts::lit_token::LITToken;
use crate::contracts::multisender::Multisender;
use crate::contracts::payment_delegation::PaymentDelegation;
use crate::contracts::pkp_helper::PKPHelper;
use crate::contracts::pkp_permissions::PKPPermissions;
use crate::contracts::pkpnft::PKPNFT;
use crate::contracts::pkpnft_metadata::PKPNFTMetadata;
use crate::contracts::price_feed::PriceFeed;
use crate::contracts::pubkey_router::PubkeyRouter;
use crate::contracts::release_register::ReleaseRegister;
use crate::contracts::staking::Staking;
use crate::resolver::rpc::ENDPOINT_MANAGER;
use crate::resolver::rpc::RpcHealthcheckPoller;

use crate::error::{Result, conversion_err};
use crate::util::ether::middleware::EIP2771GasRelayerMiddleware;

// Internal Contracts:
#[allow(clippy::module_inception)]
pub mod erc20;


#[allow(clippy::all)]
#[rustfmt::skip]
pub mod allowlist;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod backup_recovery;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod contract_resolver;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod ledger;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod lit_token;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod multisender;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod pkp_helper;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod pkp_permissions;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod pkpnft;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod pkpnft_metadata;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod price_feed;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod pubkey_router;
#[allow(clippy::all)]
pub mod release;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod release_register;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod staking;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod payment_delegation;

#[allow(clippy::all)]
#[rustfmt::skip]
pub mod host_commands;

#[allow(clippy::all)]
#[rustfmt::skip]
pub mod forwarder;

// Special types
pub const STAKING_CONTRACT: &str = "STAKING";
pub const CONTRACT_RESOLVER_CONTRACT: &str = "CONTRACT_RESOLVER";

// Found in resolver
pub const RELEASE_REGISTER_CONTRACT: &str = "RELEASE_REGISTER";
pub const MULTI_SENDER_CONTRACT: &str = "MULTI_SENDER";
pub const LIT_TOKEN_CONTRACT: &str = "LIT_TOKEN";
pub const PUB_KEY_ROUTER_CONTRACT: &str = "PUB_KEY_ROUTER";
pub const PKP_NFT_CONTRACT: &str = "PKP_NFT";
pub const RATE_LIMIT_NFT_CONTRACT: &str = "RATE_LIMIT_NFT";
pub const PKP_HELPER_CONTRACT: &str = "PKP_HELPER";
pub const PKP_PERMISSIONS_CONTRACT: &str = "PKP_PERMISSIONS";
pub const PKP_NFT_METADATA_CONTRACT: &str = "PKP_NFT_METADATA";
pub const ALLOWLIST_CONTRACT: &str = "ALLOWLIST";
pub const BACKUP_RECOVERY_CONTRACT: &str = "BACKUP_RECOVERY";
pub const PAYMENT_DELEGATION_CONTRACT: &str = "PAYMENT_DELEGATION";
pub const LEDGER_CONTRACT: &str = "LEDGER";
pub const PRICE_FEED_CONTRACT: &str = "PRICE_FEED";
pub const HOST_COMMANDS_CONTRACT: &str = "HOST_COMMANDS";
pub const FORWARDER_CONTRACT: &str = "FORWARDER";

// Staking

impl Staking<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<Staking<Provider<Http>>> {
        Ok(Staking::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl Staking<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<Staking<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(Staking::new(address, default_local_client(cfg, wallet_key)?))
    }
}

impl
    Staking<EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>>
{
    pub(crate) fn load_with_gas_relay(
        cfg: &LitConfig, address: H160, forwarder_address: H160,
        gas_relayer_wallet_key: Option<&str>, meta_signer_key: impl Into<SigningKey>,
    ) -> Result<
        Staking<
            EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        >,
    > {
        let chain = cfg.blockchain_chain_name()?;
        let provider = ENDPOINT_MANAGER.get_provider(chain.as_str())?;
        let meta_signer = meta_signer_key.into();
        let meta_wallet = LocalWallet::from(meta_signer.clone());
        let meta_client = SignerMiddleware::new(provider, meta_wallet);

        let forwarder_with_gas_signer =
            Forwarder::new(forwarder_address, default_local_client(cfg, gas_relayer_wallet_key)?);

        Ok(Staking::new(
            address,
            Arc::new(EIP2771GasRelayerMiddleware::new(
                meta_client, meta_signer, forwarder_with_gas_signer,
            )),
        ))
    }
}

// BackupRecovery

impl BackupRecovery<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<BackupRecovery<Provider<Http>>> {
        Ok(BackupRecovery::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl BackupRecovery<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<BackupRecovery<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(BackupRecovery::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// Ledger

impl Ledger<Provider<Http>> {
    #[allow(dead_code)]
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<Ledger<Provider<Http>>> {
        Ok(Ledger::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl Ledger<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<Ledger<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(Ledger::new(address, default_local_client(cfg, wallet_key)?))
    }
}

impl
    Ledger<EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>>
{
    pub(crate) fn load_with_gas_relay(
        cfg: &LitConfig, address: H160, forwarder_address: H160,
        gas_relayer_wallet_key: Option<&str>, meta_signer_key: impl Into<SigningKey>,
    ) -> Result<
        Ledger<
            EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        >,
    > {
        let chain = cfg.blockchain_chain_name()?;
        let provider = ENDPOINT_MANAGER.get_provider(chain.as_str())?;
        let meta_signer = meta_signer_key.into();
        let meta_wallet = LocalWallet::from(meta_signer.clone());
        let meta_client = SignerMiddleware::new(provider, meta_wallet);

        let forwarder_with_gas_signer =
            Forwarder::new(forwarder_address, default_local_client(cfg, gas_relayer_wallet_key)?);

        Ok(Ledger::new(
            address,
            Arc::new(EIP2771GasRelayerMiddleware::new(
                meta_client, meta_signer, forwarder_with_gas_signer,
            )),
        ))
    }
}

// ContractResolver

impl ContractResolver<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<ContractResolver<Provider<Http>>> {
        Ok(ContractResolver::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl ContractResolver<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<ContractResolver<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(ContractResolver::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// ReleaseRegister

impl ReleaseRegister<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<ReleaseRegister<Provider<Http>>> {
        Ok(ReleaseRegister::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl ReleaseRegister<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<ReleaseRegister<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(ReleaseRegister::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// Multisender

impl Multisender<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<Multisender<Provider<Http>>> {
        Ok(Multisender::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl Multisender<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<Multisender<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(Multisender::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// LITToken

impl LITToken<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<LITToken<Provider<Http>>> {
        Ok(LITToken::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl LITToken<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<LITToken<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(LITToken::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// PubkeyRouter

impl PubkeyRouter<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<PubkeyRouter<Provider<Http>>> {
        Ok(PubkeyRouter::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PubkeyRouter<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PubkeyRouter<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(PubkeyRouter::new(address, default_local_client(cfg, wallet_key)?))
    }
}

impl
    PubkeyRouter<
        EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    >
{
    pub(crate) fn load_with_gas_relay(
        cfg: &LitConfig, address: H160, forwarder_address: H160,
        gas_relayer_wallet_key: Option<&str>, meta_signer_key: impl Into<SigningKey>,
    ) -> Result<
        PubkeyRouter<
            EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        >,
    > {
        let chain = cfg.blockchain_chain_name()?;
        let provider = ENDPOINT_MANAGER.get_provider(chain.as_str())?;
        let meta_signer = meta_signer_key.into();
        let meta_wallet = LocalWallet::from(meta_signer.clone());
        let meta_client = SignerMiddleware::new(provider, meta_wallet);

        let forwarder_with_gas_signer =
            Forwarder::new(forwarder_address, default_local_client(cfg, gas_relayer_wallet_key)?);

        Ok(PubkeyRouter::new(
            address,
            Arc::new(EIP2771GasRelayerMiddleware::new(
                meta_client, meta_signer, forwarder_with_gas_signer,
            )),
        ))
    }
}

// PKPNFT

impl PKPNFT<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<PKPNFT<Provider<Http>>> {
        Ok(PKPNFT::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PKPNFT<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PKPNFT<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(PKPNFT::new(address, default_local_client(cfg, wallet_key)?))
    }
}

impl
    PKPNFT<EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>>
{
    pub(crate) fn load_with_gas_relay(
        cfg: &LitConfig, address: H160, forwarder_address: H160,
        gas_relayer_wallet_key: Option<&str>, meta_signer_key: impl Into<SigningKey>,
    ) -> Result<
        PKPNFT<
            EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        >,
    > {
        let chain = cfg.blockchain_chain_name()?;
        let provider = ENDPOINT_MANAGER.get_provider(chain.as_str())?;
        let meta_signer = meta_signer_key.into();
        let meta_wallet = LocalWallet::from(meta_signer.clone());
        let meta_client = SignerMiddleware::new(provider, meta_wallet);

        let forwarder_with_gas_signer =
            Forwarder::new(forwarder_address, default_local_client(cfg, gas_relayer_wallet_key)?);

        Ok(PKPNFT::new(
            address,
            Arc::new(EIP2771GasRelayerMiddleware::new(
                meta_client, meta_signer, forwarder_with_gas_signer,
            )),
        ))
    }
}

// RateLimitNFT

// PKPHelper

impl PKPHelper<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<PKPHelper<Provider<Http>>> {
        Ok(PKPHelper::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PKPHelper<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PKPHelper<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(PKPHelper::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// PKPPermissions

impl PKPPermissions<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<PKPPermissions<Provider<Http>>> {
        Ok(PKPPermissions::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PKPPermissions<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PKPPermissions<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(PKPPermissions::new(address, default_local_client(cfg, wallet_key)?))
    }
}

impl
    PKPPermissions<
        EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    >
{
    pub(crate) fn load_with_gas_relay(
        cfg: &LitConfig, address: H160, forwarder_address: H160,
        gas_relayer_wallet_key: Option<&str>, meta_signer_key: impl Into<SigningKey>,
    ) -> Result<
        PKPPermissions<
            EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        >,
    > {
        let chain = cfg.blockchain_chain_name()?;
        let provider = ENDPOINT_MANAGER.get_provider(chain.as_str())?;
        let meta_signer = meta_signer_key.into();
        let meta_wallet = LocalWallet::from(meta_signer.clone());
        let meta_client = SignerMiddleware::new(provider, meta_wallet);

        let forwarder_with_gas_signer =
            Forwarder::new(forwarder_address, default_local_client(cfg, gas_relayer_wallet_key)?);

        Ok(PKPPermissions::new(
            address,
            Arc::new(EIP2771GasRelayerMiddleware::new(
                meta_client, meta_signer, forwarder_with_gas_signer,
            )),
        ))
    }
}

// PKPNFTMetadata

impl PKPNFTMetadata<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<PKPNFTMetadata<Provider<Http>>> {
        Ok(PKPNFTMetadata::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PKPNFTMetadata<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PKPNFTMetadata<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(PKPNFTMetadata::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// Allowlist

impl Allowlist<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<Allowlist<Provider<Http>>> {
        Ok(Allowlist::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl Allowlist<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<Allowlist<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(Allowlist::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// PaymentDelegation

impl PaymentDelegation<Provider<Http>> {
    pub(crate) fn load(
        cfg: &LitConfig, address: H160,
    ) -> Result<PaymentDelegation<Provider<Http>>> {
        Ok(PaymentDelegation::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PaymentDelegation<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PaymentDelegation<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(PaymentDelegation::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// PriceFeed

impl PriceFeed<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<PriceFeed<Provider<Http>>> {
        Ok(PriceFeed::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PriceFeed<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PriceFeed<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
        Ok(PriceFeed::new(address, default_local_client(cfg, wallet_key)?))
    }
}

impl
    PriceFeed<
        EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    >
{
    pub(crate) fn load_with_gas_relay(
        cfg: &LitConfig, address: H160, forwarder_address: H160,
        gas_relayer_wallet_key: Option<&str>, meta_signer_key: impl Into<SigningKey>,
    ) -> Result<
        PriceFeed<
            EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        >,
    > {
        let chain = cfg.blockchain_chain_name()?;
        let provider = ENDPOINT_MANAGER.get_provider(chain.as_str())?;
        let meta_signer = meta_signer_key.into();
        let meta_wallet = LocalWallet::from(meta_signer.clone());
        let meta_client = SignerMiddleware::new(provider, meta_wallet);

        let forwarder_with_gas_signer =
            Forwarder::new(forwarder_address, default_local_client(cfg, gas_relayer_wallet_key)?);

        Ok(PriceFeed::new(
            address,
            Arc::new(EIP2771GasRelayerMiddleware::new(
                meta_client, meta_signer, forwarder_with_gas_signer,
            )),
        ))
    }
}

// HostCommands

impl HostCommands<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<HostCommands<Provider<Http>>> {
        Ok(HostCommands::new(address, default_local_client_no_wallet(cfg)?))
    }
}

// Util

pub fn load_wallet(cfg: &LitConfig, wallet_key: Option<&str>) -> Result<Wallet<SigningKey>> {
    let private_key_bytes = cfg.blockchain_wallet_private_key_bytes(wallet_key)?;

    let secret_key = SecretKey::from_bytes(k256::FieldBytes::from_slice(&private_key_bytes))
        .map_err(|e| {
            conversion_err(
                e,
                Some("Could not convert 'blockchain.wallet.{..}.private_key to SecretKey".into()),
            )
        })?;

    Ok(LocalWallet::from(secret_key).with_chain_id(cfg.blockchain_chain_id()?)) // if you don't use this with_chain_id() function, you will get an error when you try to sign a txn.
}

pub fn default_local_client(
    cfg: &LitConfig, wallet_key: Option<&str>,
) -> Result<Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
    let chain = cfg.blockchain_chain_name()?;
    let wallet = load_wallet(cfg, wallet_key)?;
    let provider = ENDPOINT_MANAGER.get_provider(chain.as_str())?;

    Ok(Arc::new(SignerMiddleware::new(provider, wallet)))
}

pub fn default_local_client_no_wallet(cfg: &LitConfig) -> Result<Arc<Provider<Http>>> {
    let chain = cfg.blockchain_chain_name()?;
    let provider = ENDPOINT_MANAGER.get_provider(chain.as_str())?;

    Ok(provider)
}
