use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use lit_blockchain_lite::contracts::{
    contract_resolver::ContractResolver, pkp_helper::pkp_helper::PKPHelper,
    pkp_permissions::PKPPermissions, pkpnft::PKPNFT, pubkey_router::PubkeyRouter, staking::Staking,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct DatilContracts {
    pub deployer_provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub staking: Staking<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub pkpnft: PKPNFT<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub pubkey_router: PubkeyRouter<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub pkp_permissions: PKPPermissions<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub pkp_helper: PKPHelper<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub contract_resolver:
        ContractResolver<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
}

impl DatilContracts {
    pub async fn new(
        deployer_signing_provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
        contract_resolver_address: Address,
    ) -> Self {
        let env = 0;
        let contract_resolver =
            ContractResolver::new(contract_resolver_address, deployer_signing_provider.clone());

        let staking_address = contract_resolver
            .get_contract(
                contract_resolver.staking_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let staking = Staking::new(staking_address, deployer_signing_provider.clone());

        let pkpnft_address = contract_resolver
            .get_contract(
                contract_resolver.pkp_nft_contract().call().await.unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let pkpnft = PKPNFT::new(pkpnft_address, deployer_signing_provider.clone());

        let pubkey_router_address = contract_resolver
            .get_contract(
                contract_resolver
                    .pub_key_router_contract()
                    .call()
                    .await
                    .unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let pubkey_router =
            PubkeyRouter::new(pubkey_router_address, deployer_signing_provider.clone());

        let pkp_permissions_address = contract_resolver
            .get_contract(
                contract_resolver
                    .pkp_permissions_contract()
                    .call()
                    .await
                    .unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let pkp_permissions =
            PKPPermissions::new(pkp_permissions_address, deployer_signing_provider.clone());

        let pkp_helper_address = contract_resolver
            .get_contract(
                contract_resolver
                    .pkp_helper_contract()
                    .call()
                    .await
                    .unwrap(),
                env,
            )
            .call()
            .await
            .unwrap();
        let pkp_helper = PKPHelper::new(pkp_helper_address, deployer_signing_provider.clone());

        Self {
            deployer_provider: deployer_signing_provider.clone(),
            staking,
            pkpnft,
            pubkey_router,
            pkp_permissions,
            pkp_helper,
            contract_resolver,
        }
    }
}
