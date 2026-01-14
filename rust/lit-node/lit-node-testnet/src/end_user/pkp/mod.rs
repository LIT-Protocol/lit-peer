mod datil;
mod mainnet;

use crate::end_user::EndUser;
use ethers::abi::AbiEncode;
use ethers::middleware::SignerMiddleware;
use ethers::types::{Address, Bytes, H160, U256};
use lit_blockchain::contracts::pkpnft::PKPNFT;
use lit_blockchain::util::decode_revert;
use lit_core::utils::binary::bytes_to_hex;
use std::sync::Arc;
use tracing::{error, info};

use super::Pkp;

impl Pkp {
    pub async fn new(end_user: &EndUser, key_set_id: &str) -> Result<Self, anyhow::Error> {
        let key_type: U256 = U256::from(2); // 2 is ECDSA key type

        let pkpnft_address = end_user.actions().contracts().pkpnft.address();

        let client = Arc::new(SignerMiddleware::new(
            end_user.signing_provider().clone(),
            end_user.wallet.clone(),
        ));

        let pkpnft = PKPNFT::new(pkpnft_address, client);

        info!("Minting a new PKP from the test harness.");
        let mint_cost = pkpnft.mint_cost().call().await?;
        info!("Mint cost: {:}", mint_cost);

        let mint_tx = pkpnft
            .mint_next(key_type, key_set_id.to_string())
            .value(mint_cost);

        let receipt = mint_tx
            .send()
            .await
            .map_err(|e| {
                let revert_msg = format!(
                    "Failed to send PKP mint transaction: {}",
                    decode_revert(&e, end_user.actions().contracts().pkpnft.abi())
                );
                error!(revert_msg);
                anyhow::anyhow!(revert_msg)
            })?
            .await
            .map_err(|e| {
                let revert_msg = format!("Failed while waiting for PKP mint confirmation: {}", e);
                error!(revert_msg);
                anyhow::anyhow!(revert_msg)
            })?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed - no receipt generated"))?;

        if receipt.logs.is_empty() {
            return Err(anyhow::anyhow!("Transaction receipt contains no logs"));
        }
        let token_id = receipt.logs[0].topics[1];
        let token_id = U256::from(token_id.as_bytes());

        let r = end_user
            .actions()
            .contracts()
            .pubkey_router
            .get_pubkey(token_id)
            .call()
            .await?;
        let pubkey = bytes_to_hex(r);

        let eth_address = pkpnft.get_eth_address(token_id).call().await?;

        info!(
            "Minted PKP with token id: {} / pubkey : {} / eth address: {:?}",
            token_id.encode_hex(), &pubkey, eth_address
        );

        Ok(Pkp {
            signing_provider: end_user.signing_provider().clone(),
            actions: Arc::new(end_user.actions().clone()),
            pubkey: pubkey.clone(),
            token_id,
            eth_address,
            key_set_id: key_set_id.to_string(),
            is_datil: false,
        })
    }

    pub fn info(&self) -> (String, U256, H160, String) {
        (
            self.pubkey.clone(),
            self.token_id,
            self.eth_address,
            self.key_set_id.clone(),
        )
    }

    #[doc = "Grant an address permission to use a PKP"]
    pub async fn add_permitted_address_to_pkp(
        &self,
        addr_to_add: H160,
        scopes: &[U256],
    ) -> Result<bool, anyhow::Error> {
        if self.is_datil {
            self.add_permitted_address_to_pkp_datil(addr_to_add, scopes)
                .await
        } else {
            self.add_permitted_address_to_pkp_mainnet(addr_to_add, scopes)
                .await
        }
    }

    #[doc = "Transfer a PKP"]
    pub async fn transfer_pkp_with_wallet(
        &self,
        to_address: Address,
    ) -> Result<bool, anyhow::Error> {
        if self.is_datil {
            self.transfer_pkp_with_wallet_datil(to_address).await
        } else {
            self.transfer_pkp_with_wallet_mainnet(to_address).await
        }
    }

    #[doc = "Grant an action permission to use a PKP"]
    pub async fn add_permitted_action_to_pkp(
        &self,
        ipfs_cid: &str,
        scopes: &[U256],
    ) -> Result<bool, anyhow::Error> {
        if self.is_datil {
            self.add_permitted_action_to_pkp_datil(ipfs_cid, scopes)
                .await
        } else {
            self.add_permitted_action_to_pkp_mainnet(ipfs_cid, scopes)
                .await
        }
    }

    pub async fn is_permitted_action(&self, ipfs_cid: &str) -> Result<bool, anyhow::Error> {
        if self.is_datil {
            self.is_permitted_action_datil(ipfs_cid).await
        } else {
            self.is_permitted_action_mainnet(ipfs_cid).await
        }
    }

    #[doc = "Grant a Address Authmethod permission to use a PKP"]
    pub async fn add_permitted_address_auth_method_to_pkp(
        &self,
        address_token: Vec<u8>,
        scopes: &[U256],
    ) -> Result<bool, anyhow::Error> {
        if self.is_datil {
            self.add_permitted_address_auth_method_to_pkp_datil(address_token, scopes)
                .await
        } else {
            self.add_permitted_address_auth_method_to_pkp_mainnet(address_token, scopes)
                .await
        }
    }

    pub async fn mint_grant_and_burn_next_pkp(
        end_user: &EndUser,
        ipfs_cid: &str,
        key_set_id: &str,
    ) -> Result<Self, anyhow::Error> {
        // Use the deployer account by default
        let client = end_user.signing_provider().clone();

        let key_type: U256 = U256::from(2);

        let pkpnft_address = end_user.actions().contracts().pkpnft.address();
        let pkpnft = PKPNFT::new(pkpnft_address, Arc::new(client));

        info!("Minting, granting and burning a new PKP from the test harness.");
        let mint_cost = pkpnft.mint_cost().call().await?;

        // Convert ipfs_cid to Bytes
        let ipfs_bytes = Bytes::from(bs58::decode(ipfs_cid).into_vec()?);

        let mgb_tx = pkpnft
            .mint_grant_and_burn_next(key_type, key_set_id.to_string(), ipfs_bytes)
            .value(mint_cost);

        let receipt = mgb_tx
            .send()
            .await
            .map_err(|e| {
                let revert_msg = format!(
                    "Failed to send PKP mint transaction: {}",
                    decode_revert(&e, end_user.actions().contracts().pkpnft.abi())
                );
                error!(revert_msg);
                anyhow::anyhow!(revert_msg)
            })?
            .await
            .map_err(|e| {
                let revert_msg = format!("Failed while waiting for PKP mint confirmation: {}", e);
                error!(revert_msg);
                anyhow::anyhow!(revert_msg)
            })?
            .ok_or_else(|| anyhow::anyhow!("Transaction failed - no receipt generated"))?;

        let token_id = receipt.logs[0].topics[1];
        let token_id = U256::from(token_id.as_bytes());

        let r = end_user
            .actions()
            .contracts()
            .pubkey_router
            .get_pubkey(token_id)
            .call()
            .await?;

        let pubkey = bytes_to_hex(r);
        let eth_address = pkpnft.get_eth_address(token_id).call().await?;

        info!(
            "Minted PKP with token id: {} / pubkey : {} / eth address: {:?}",
            token_id, &pubkey, eth_address
        );

        Ok(Pkp {
            signing_provider: end_user.signing_provider().clone(),
            actions: Arc::new(end_user.actions().clone()),
            pubkey,
            token_id,
            key_set_id: key_set_id.to_string(),
            eth_address: eth_address.into(),
            is_datil: false,
        })
    }

    pub async fn burn_pkp(&self) -> Result<bool, anyhow::Error> {
        if self.is_datil {
            self.burn_pkp_datil().await
        } else {
            self.burn_pkp_mainnet().await
        }
    }
}
