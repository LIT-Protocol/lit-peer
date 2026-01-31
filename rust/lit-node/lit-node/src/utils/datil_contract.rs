use crate::config::chain::ChainDataConfigManager;
use crate::error::{Result, unexpected_err};
use crate::version::DataVersionReader;
use ethers::prelude::*;
use lit_blockchain::resolver::rpc::{ENDPOINT_MANAGER, RpcHealthcheckPoller};
use lit_blockchain_lite::contracts::contract_resolver::ContractResolver;
use lit_blockchain_lite::contracts::pkp_permissions::PKPPermissions;
use lit_blockchain_lite::contracts::pkpnft::PKPNFT;
use lit_blockchain_lite::contracts::pubkey_router::PubkeyRouter;

pub struct DatilContracts {
    pub pkp_permissions: PKPPermissions<Provider<Http>>,
    pub pkp_nft: PKPNFT<Provider<Http>>,
    pub pubkey_router: PubkeyRouter<Provider<Http>>,
}

impl DatilContracts {
    pub async fn new(cdm: &ChainDataConfigManager, key_set_id: &str) -> Result<Self> {
        let key_set_config = DataVersionReader::read_field_unchecked(&cdm.key_sets, |key_sets| {
            key_sets.get(key_set_id).cloned().ok_or_else(|| {
                unexpected_err(
                    format!("Key set with identifier {key_set_id} not found"),
                    None,
                )
            })
        })?;

        let key_set_description_parts =
            key_set_config.description.split("|").collect::<Vec<&str>>();
        let chain_name = key_set_description_parts[0];
        let hex_contract_resolver_address = key_set_description_parts[1];

        let provider = ENDPOINT_MANAGER.get_provider(chain_name).unwrap_or_else(|_| panic!("Error retrieving provider for chain {chain_name} - check name and/or rpc_config yaml."));

        let contract_resolver_address = Address::from_slice(
            &hex::decode(hex_contract_resolver_address)
                .expect("Failed to decode contract resolver address"),
        );
        let env = 0;
        let contract_resolver = ContractResolver::new(contract_resolver_address, provider.clone());
        let pkp_permissions_address = contract_resolver
            .get_contract(
                contract_resolver
                    .pkp_permissions_contract()
                    .call()
                    .await
                    .map_err(|e| {
                        unexpected_err(e, Some("failed to load PKP permissions contract".into()))
                    })?,
                env,
            )
            .call()
            .await
            .map_err(|e| {
                unexpected_err(e, Some("failed to load PKP permissions contract".into()))
            })?;

        let pkp_permissions_contract =
            PKPPermissions::new(pkp_permissions_address, provider.clone());

        let pkp_nft_address = contract_resolver
            .get_contract(
                contract_resolver
                    .pkp_nft_contract()
                    .call()
                    .await
                    .map_err(|e| {
                        unexpected_err(e, Some("failed to load PKP NFT contract".into()))
                    })?,
                env,
            )
            .call()
            .await
            .map_err(|e| unexpected_err(e, Some("failed to load PKP NFT contract".into())))?;

        let pkp_nft_contract = PKPNFT::new(pkp_nft_address, provider.clone());

        let pubkey_router_address = contract_resolver
            .get_contract(
                contract_resolver
                    .pub_key_router_contract()
                    .call()
                    .await
                    .map_err(|e| {
                        unexpected_err(e, Some("failed to load Pubkey Router contract".into()))
                    })?,
                env,
            )
            .call()
            .await
            .map_err(|e| unexpected_err(e, Some("failed to load Pubkey Router contract".into())))?;

        let pubkey_router_contract = PubkeyRouter::new(pubkey_router_address, provider.clone());

        Ok(Self {
            pkp_permissions: pkp_permissions_contract,
            pkp_nft: pkp_nft_contract,
            pubkey_router: pubkey_router_contract,
        })
    }
}

pub fn is_datil_key_set_id(key_set_id: &str) -> bool {
    key_set_id.to_lowercase().contains("datil")
}
