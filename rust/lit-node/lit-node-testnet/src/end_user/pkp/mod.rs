mod datil;
mod mainnet;

use super::Pkp;
use crate::end_user::EndUser;
use ethers::types::{Address, H160, U256};

impl Pkp {
    pub async fn new(end_user: &EndUser, key_set_id: &str) -> Result<Self, anyhow::Error> {
        // this check allows us to run this test on other systems / networks.
        if key_set_id.to_lowercase().contains("datil") {
            Pkp::new_datil(end_user, key_set_id).await
        } else {
            Pkp::new_mainnet(end_user, key_set_id).await
        }
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

    pub async fn new_pkp_with_auth_methods(
        end_user: &EndUser,
        key_set_id: &str,
    ) -> Result<Self, anyhow::Error> {
        if key_set_id.to_lowercase().contains("datil") {
            Pkp::new_pkp_with_auth_methods_datil(end_user, key_set_id).await
        } else {
            Pkp::new_pkp_with_auth_methods_mainnet(end_user, key_set_id).await
        }
    }

    pub async fn mint_grant_and_burn_next_pkp(
        end_user: &EndUser,
        ipfs_cid: &str,
        key_set_id: &str,
    ) -> Result<Self, anyhow::Error> {
        if key_set_id.to_lowercase().contains("datil") {
            Pkp::mint_grant_and_burn_next_pkp_datil(end_user, ipfs_cid, key_set_id).await
        } else {
            Pkp::mint_grant_and_burn_next_pkp_mainnet(end_user, ipfs_cid, key_set_id).await
        }
    }

    pub async fn burn_pkp(&self) -> Result<bool, anyhow::Error> {
        if self.is_datil {
            self.burn_pkp_datil().await
        } else {
            self.burn_pkp_mainnet().await
        }
    }
}
