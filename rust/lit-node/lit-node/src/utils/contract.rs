#![allow(dead_code)]

use crate::error::{Result, unexpected_err};
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use k256::ecdsa::SigningKey;
use lit_blockchain::contracts::backup_recovery::BackupRecovery;
use lit_blockchain::contracts::ledger::Ledger;
use lit_blockchain::contracts::pkp_permissions::PKPPermissions;
use lit_blockchain::contracts::pkpnft::PKPNFT;
use lit_blockchain::contracts::price_feed::PriceFeed;
use lit_blockchain::contracts::pubkey_router::PubkeyRouter;
use lit_blockchain::resolver::contract::ContractResolver;
use lit_blockchain::util::ether::middleware::EIP2771GasRelayerMiddleware;
use lit_core::config::LitConfig;
use std::sync::Arc;

pub async fn get_pkp_permissions_contract(
    config: Arc<LitConfig>,
) -> Result<PKPPermissions<Provider<Http>>> {
    // Get contract resolver.
    let resolver = ContractResolver::try_from(config.as_ref())
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    // Get PKP permissions contract.
    let pkp_permissions_contract = resolver
        .pkp_permissions_contract(config.as_ref())
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load PKP permissions contract".into())))?;
    trace!(
        "pkp_permissions_contract address: {}",
        pkp_permissions_contract.address()
    );

    Ok(pkp_permissions_contract)
}

pub async fn get_pkp_permissions_contract_with_gas_relay(
    config: &LitConfig,
    meta_signer_key: SigningKey,
) -> Result<
    PKPPermissions<
        EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    >,
> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let pkp_permissions_contract = resolver
        .pkp_permissions_contract_with_gas_relay(config, meta_signer_key)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load PKP permissions contract".into())))?;

    Ok(pkp_permissions_contract)
}

pub async fn get_backup_recovery_contract(
    config: &LitConfig,
) -> Result<BackupRecovery<Provider<Http>>> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let backup_recovery_contract = resolver
        .backup_recovery_contract(config)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load BackupRecovery contract".into())))?;

    Ok(backup_recovery_contract)
}

pub async fn get_backup_recovery_contract_with_signer(
    config: &LitConfig,
) -> Result<BackupRecovery<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let backup_recovery_contract = resolver
        .backup_recovery_contract_with_signer(config)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load BackupRecovery contract".into())))?;

    Ok(backup_recovery_contract)
}

pub async fn get_ledger_contract(config: &LitConfig) -> Result<Ledger<Provider<Http>>> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let ledger_contract = resolver
        .ledger_contract(config)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load Ledger contract".into())))?;

    Ok(ledger_contract)
}

#[deprecated(note = "Contract writes should be done using the gas relay instead.")]
pub async fn get_ledger_contract_with_signer(
    config: &LitConfig,
) -> Result<Ledger<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let ledger_contract = resolver
        .ledger_contract_with_signer(config)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load Ledger contract".into())))?;

    Ok(ledger_contract)
}

pub async fn get_ledger_contract_with_gas_relay(
    config: &LitConfig,
    meta_signer_key: SigningKey,
) -> Result<
    Ledger<EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>>,
> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let ledger_contract = resolver
        .ledger_contract_with_gas_relay(config, meta_signer_key)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load Ledger contract".into())))?;

    Ok(ledger_contract)
}

pub async fn get_price_feed_contract(config: &LitConfig) -> Result<PriceFeed<Provider<Http>>> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let price_feed_contract = resolver
        .price_feed_contract(config)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load PriceFeed contract".into())))?;

    Ok(price_feed_contract)
}

#[deprecated(note = "Contract writes should be done using the gas relay instead.")]
pub async fn get_price_feed_contract_with_signer(
    config: &LitConfig,
) -> Result<PriceFeed<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let price_feed_contract = resolver
        .price_feed_contract_with_signer(config)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load PriceFeed contract".into())))?;

    Ok(price_feed_contract)
}

pub async fn get_price_feed_contract_with_gas_relay(
    config: &LitConfig,
    meta_signer_key: SigningKey,
) -> Result<
    PriceFeed<
        EIP2771GasRelayerMiddleware<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    >,
> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let price_feed_contract = resolver
        .price_feed_contract_with_gas_relay(config, meta_signer_key)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load PriceFeed contract".into())))?;

    Ok(price_feed_contract)
}

pub async fn get_pkp_nft_contract(config: &LitConfig) -> Result<PKPNFT<Provider<Http>>> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let pkp_nft_contract = resolver
        .pkp_nft_contract(config)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load PKP NFT contract".into())))?;

    Ok(pkp_nft_contract)
}

pub async fn get_pub_key_router_contract(
    config: &LitConfig,
) -> Result<PubkeyRouter<Provider<Http>>> {
    let resolver = ContractResolver::try_from(config)
        .map_err(|e| unexpected_err(e, Some("failed to load ContractResolver".into())))?;

    let pub_key_router_contract = resolver
        .pub_key_router_contract(config)
        .await
        .map_err(|e| unexpected_err(e, Some("failed to load PKP NFT contract".into())))?;

    Ok(pub_key_router_contract)
}
