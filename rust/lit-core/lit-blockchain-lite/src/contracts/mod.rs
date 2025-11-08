// this file is used to load contracts for the node monitor - NOT suitable for use with the node.
use rand::thread_rng;
use std::error::Error;
use std::sync::Arc;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;

use allowlist::Allowlist;
use backup_recovery::BackupRecovery;
use contract_resolver::ContractResolver;
use lit_token::LITToken;
// use multisender::Multisender;
use payment_delegation::PaymentDelegation;
use pkp_helper::PKPHelper;
use pkp_permissions::PKPPermissions;
use pkpnft::PKPNFT;
use pkpnft_metadata::PKPNFTMetadata;
use pubkey_router::PubkeyRouter;
// use release_register::ReleaseRegister;
use staking::Staking;
// use domain_wallet_registry::DomainWalletRegistry;
use host_commands::HostCommands;
// use key_deriver::KeyDeriver;
use price_feed::PriceFeed;
// use wlit::WLIT;

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
// #[allow(clippy::all)]
// #[rustfmt::skip]
// pub mod multisender;
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

// #[allow(clippy::all)]
// #[rustfmt::skip]
// pub mod release_register;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod staking;
#[allow(clippy::all)]
#[rustfmt::skip]
pub mod payment_delegation;

#[allow(clippy::all)]
#[rustfmt::skip]
pub mod host_commands;


// #[allow(clippy::all)]
// #[rustfmt::skip]
// pub mod key_deriver;

// #[allow(clippy::all)]
// #[rustfmt::skip]
// pub mod domain_wallet_registry;

// #[allow(clippy::all)]
// #[rustfmt::skip]
// pub mod wlit;

// Special types
pub const STAKING_CONTRACT: &str = "STAKING";
pub const STAKING_BALANCES_CONTRACT: &str = "STAKING_BALANCES";
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

pub const HOST_COMMANDS_CONTRACT: &str = "HOST_COMMANDS";
pub const DOMAIN_WALLET_REGISTRY_CONTRACT: &str = "DOMAIN_WALLET_REGISTRY";
pub const KEY_DERIVER_CONTRACT: &str = "HD_KEY_DERIVER";

pub const WLIT_CONTRACT: &str = "WLIT";
pub const PRICE_FEED_CONTRACT: &str = "PRICE_FEED";

pub struct NodeMonitorLitConfig {
    pub blockchain_chain_id: u64,
    pub rpc_url: String,
    pub wallet_key: Option<String>,
}

// Staking

impl Staking<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<Staking<Provider<Http>>, Box<dyn Error>> {
        Ok(Staking::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl Staking<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<Staking<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>> {
        Ok(Staking::new(address, default_local_client(cfg, wallet_key)?))
    }
}

impl HostCommands<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<HostCommands<Provider<Http>>, Box<dyn Error>> {
        Ok(HostCommands::new(address, default_local_client_no_wallet(cfg)?))
    }
}

// impl DomainWalletRegistry<Provider<Http>> {
//     pub fn node_monitor_load(
//         cfg: &NodeMonitorLitConfig, address: H160,
//     ) -> Result<DomainWalletRegistry<Provider<Http>>, Box<dyn Error>> {
//         Ok(DomainWalletRegistry::new(address, default_local_client_no_wallet(cfg)?))
//     }
// }

// impl KeyDeriver<Provider<Http>> {
//     pub fn node_monitor_load(
//         cfg: &NodeMonitorLitConfig, address: H160,
//     ) -> Result<KeyDeriver<Provider<Http>>, Box<dyn Error>> {
//         Ok(KeyDeriver::new(address, default_local_client_no_wallet(cfg)?))
//     }
// }

// impl WLIT<Provider<Http>> {
//     pub fn node_monitor_load(
//         cfg: &NodeMonitorLitConfig, address: H160,
//     ) -> Result<WLIT<Provider<Http>>, Box<dyn Error>> {
//         Ok(WLIT::new(address, default_local_client_no_wallet(cfg)?))
//     }
// }

// BackupRecovery
impl BackupRecovery<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<BackupRecovery<Provider<Http>>, Box<dyn Error>> {
        Ok(BackupRecovery::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl BackupRecovery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<BackupRecovery<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>>
    {
        Ok(BackupRecovery::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// ContractResolver

impl ContractResolver<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<ContractResolver<Provider<Http>>, Box<dyn Error>> {
        Ok(ContractResolver::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl ContractResolver<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<
        ContractResolver<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        Box<dyn Error>,
    > {
        Ok(ContractResolver::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// ReleaseRegister

// impl ReleaseRegister<Provider<Http>> {
//     pub fn node_monitor_load(
//         cfg: &NodeMonitorLitConfig, address: H160,
//     ) -> Result<ReleaseRegister<Provider<Http>>, Box<dyn Error>> {
//         Ok(ReleaseRegister::new(address, default_local_client_no_wallet(cfg)?))
//     }
// }

// impl ReleaseRegister<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
//     pub fn node_monitor_load_with_signer(
//         cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
//     ) -> Result<ReleaseRegister<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>>
//     {
//         Ok(ReleaseRegister::new(address, default_local_client(cfg, wallet_key)?))
//     }
// }

// // Multisender

// impl Multisender<Provider<Http>> {
//     pub fn node_monitor_load(
//         cfg: &NodeMonitorLitConfig, address: H160,
//     ) -> Result<Multisender<Provider<Http>>, Box<dyn Error>> {
//         Ok(Multisender::new(address, default_local_client_no_wallet(cfg)?))
//     }
// }

// impl Multisender<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
//     pub fn node_monitor_load_with_signer(
//         cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
//     ) -> Result<Multisender<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>>
//     {
//         Ok(Multisender::new(address, default_local_client(cfg, wallet_key)?))
//     }
// }

// LITToken

impl LITToken<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<LITToken<Provider<Http>>, Box<dyn Error>> {
        Ok(LITToken::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl LITToken<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<LITToken<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>>
    {
        Ok(LITToken::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// PubkeyRouter

impl PubkeyRouter<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<PubkeyRouter<Provider<Http>>, Box<dyn Error>> {
        Ok(PubkeyRouter::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PubkeyRouter<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PubkeyRouter<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>>
    {
        Ok(PubkeyRouter::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// PKPNFT

impl PKPNFT<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<PKPNFT<Provider<Http>>, Box<dyn Error>> {
        Ok(PKPNFT::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PKPNFT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PKPNFT<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>> {
        Ok(PKPNFT::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// PKPHelper
impl PKPHelper<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<PKPHelper<Provider<Http>>, Box<dyn Error>> {
        Ok(PKPHelper::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PKPHelper<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PKPHelper<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>>
    {
        Ok(PKPHelper::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// PKPPermissions

impl PKPPermissions<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<PKPPermissions<Provider<Http>>, Box<dyn Error>> {
        Ok(PKPPermissions::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PKPPermissions<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PKPPermissions<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>>
    {
        Ok(PKPPermissions::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// PKPNFTMetadata

impl PKPNFTMetadata<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<PKPNFTMetadata<Provider<Http>>, Box<dyn Error>> {
        Ok(PKPNFTMetadata::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PKPNFTMetadata<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PKPNFTMetadata<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>>
    {
        Ok(PKPNFTMetadata::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// Allowlist

impl Allowlist<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<Allowlist<Provider<Http>>, Box<dyn Error>> {
        Ok(Allowlist::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl Allowlist<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<Allowlist<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>>
    {
        Ok(Allowlist::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// PaymentDelegation
impl PaymentDelegation<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<PaymentDelegation<Provider<Http>>, Box<dyn Error>> {
        Ok(PaymentDelegation::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PaymentDelegation<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<
        PaymentDelegation<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        Box<dyn Error>,
    > {
        Ok(PaymentDelegation::new(address, default_local_client(cfg, wallet_key)?))
    }
}

// PriceFeed
impl PriceFeed<Provider<Http>> {
    pub fn node_monitor_load(
        cfg: &NodeMonitorLitConfig, address: H160,
    ) -> Result<PriceFeed<Provider<Http>>, Box<dyn Error>> {
        Ok(PriceFeed::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl PriceFeed<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub fn node_monitor_load_with_signer(
        cfg: &NodeMonitorLitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<PriceFeed<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>>
    {
        Ok(PriceFeed::new(address, default_local_client(cfg, wallet_key)?))
    }
}

pub fn default_local_client(
    cfg: &NodeMonitorLitConfig, _wallet_key: Option<&str>,
) -> Result<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn Error>> {
    let url = cfg.rpc_url.clone();
    let provider = Provider::<Http>::try_from(url)?;
    let secret_key = SigningKey::random(&mut thread_rng());
    let wallet = LocalWallet::from(secret_key); //.with_chain_id(chain_id);

    Ok(Arc::new(SignerMiddleware::new(provider, wallet)))
}

pub fn default_local_client_no_wallet(
    cfg: &NodeMonitorLitConfig,
) -> Result<Arc<Provider<Http>>, Box<dyn Error>> {
    let url = cfg.rpc_url.clone();
    let provider = Provider::<Http>::try_from(url)?;
    Ok(Arc::new(provider))
}
