use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use lit_blockchain::contracts::pkp_helper::pkp_helper::PKPHelper;
use lit_blockchain::contracts::pkp_permissions::PKPPermissions;
use lit_blockchain::contracts::pkpnft::PKPNFT;
use lit_blockchain::util::decode_revert;

const RPC_URL: &str = "https://yellowstone-rpc.litprotocol.com/";
const PKP_PERMISSIONS_ADDRESS: &str = "0x9C48C70DD379FCe946f889f0072b1017e2eCF94C";
const PKP_NFT_ADDRESS: &str = "0x71F58526F898773Eb6ca168f0f3673f5365718d5";
const PKP_HELPER_ADDRESS: &str = "0xD44B4732eA9bcfac666cd6c4B6920e9f29d6042D";

#[derive(Debug)]
struct PkpPermissionsRouterCheck {
    pkp_permissions_address: Address,
    pkp_permissions_router_address: Address,
    pkp_nft_address: Address,
    pkp_nft_router_address: Address,
    pkp_helper_address: Address,
    pkp_helper_resolver: Address,
    total_supply: U256,
    sample_token_id: Option<U256>,
    sample_token_eth_address: Option<Address>,
    sample_token_eth_address_error: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let provider = Arc::new(provider);

    let pkp_permissions_address = Address::from_str(PKP_PERMISSIONS_ADDRESS)?;
    let pkp_nft_address = Address::from_str(PKP_NFT_ADDRESS)?;
    let pkp_helper_address = Address::from_str(PKP_HELPER_ADDRESS)?;

    let pkp_permissions = PKPPermissions::new(pkp_permissions_address, provider.clone());
    let pkp_nft = PKPNFT::new(pkp_nft_address, provider.clone());
    let pkp_helper = PKPHelper::new(pkp_helper_address, provider);

    let pkp_permissions_router_address = pkp_permissions.get_router_address().call().await?;
    let pkp_nft_router_address = pkp_nft.get_router_address().call().await?;
    let pkp_helper_resolver = pkp_helper.contract_resolver().call().await?;
    let total_supply = pkp_nft.total_supply().call().await?;

    let mut sample_token_id = None;
    let mut sample_token_eth_address = None;
    let mut sample_token_eth_address_error = None;

    if !total_supply.is_zero() {
        let token_id = pkp_nft.token_by_index(U256::zero()).call().await?;
        sample_token_id = Some(token_id);
        match pkp_permissions.get_eth_address(token_id).call().await {
            Ok(eth_address) => {
                sample_token_eth_address = Some(eth_address);
            }
            Err(e) => {
                let revert_msg = decode_revert(&e, pkp_permissions.abi());
                let err_msg = if revert_msg.is_empty() {
                    format!("{:?}", e)
                } else {
                    revert_msg
                };
                sample_token_eth_address_error = Some(err_msg);
            }
        }
    }

    let check = PkpPermissionsRouterCheck {
        pkp_permissions_address,
        pkp_permissions_router_address,
        pkp_nft_address,
        pkp_nft_router_address,
        pkp_helper_address,
        pkp_helper_resolver,
        total_supply,
        sample_token_id,
        sample_token_eth_address,
        sample_token_eth_address_error,
    };
    println!("{:#?}", check);

    Ok(())
}
