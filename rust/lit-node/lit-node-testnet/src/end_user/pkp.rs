use crate::{end_user::EndUser, testnet::actions::Actions};
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use ethers::types::{Address, Bytes, H160, U256};
use k256::ecdsa::SigningKey;
use lit_blockchain::contracts::pkp_permissions::{AuthMethod, PKPPermissions};
use lit_blockchain::contracts::pkpnft::PKPNFT;
use lit_blockchain::util::decode_revert;
use lit_core::utils::binary::bytes_to_hex;
use std::sync::Arc;
use tracing::{debug, error, info};

#[derive(Debug, Clone)]
pub struct Pkp {
    signing_provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>, // sign transactions for this PKP as the owner of the PKP
    actions: Arc<Actions>, // handy reference to the various contracts
    pub pubkey: String,
    pub token_id: U256,
    pub eth_address: H160,
}

impl Pkp {
    pub async fn new(end_user: &EndUser) -> Result<Self, anyhow::Error> {
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
            .mint_next(key_type, "naga-keyset1".to_string())
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
            token_id, &pubkey, eth_address
        );

        Ok(Pkp {
            signing_provider: end_user.signing_provider().clone(),
            actions: Arc::new(end_user.actions().clone()),
            pubkey: pubkey.clone(),
            token_id,
            eth_address,
        })
    }

    pub fn info(&self) -> (String, U256, H160) {
        (self.pubkey.clone(), self.token_id, self.eth_address)
    }

    #[doc = "Grant an address permission to use a PKP"]
    pub async fn add_permitted_address_to_pkp(
        &self,
        addr_to_add: H160,
        scopes: &[U256],
    ) -> Result<bool, anyhow::Error> {
        let client = self.signing_provider.clone();

        let token_id = self.token_id;

        info!(
            "add_permitted_address_to_pkp - adding address: {} to pkp: {}",
            addr_to_add, self.pubkey
        );

        let pkp_permissions_address = self.actions.contracts().pkp_permissions.address();
        let pkp_permissions = PKPPermissions::new(pkp_permissions_address, client.clone());
        let pacc = pkp_permissions.add_permitted_address(token_id, addr_to_add, scopes.to_vec());

        let tx = pacc.send().await;
        if tx.is_err() {
            error!("Error adding address to pkp: {:?}", tx.unwrap_err());
            return Err(anyhow::anyhow!(
                "Error adding address to PKP - couldn't send tx"
            ));
        }
        let tx = tx.unwrap();

        let tr = tx.await;
        if tr.is_err() {
            error!("Error adding address to pkp: {:?}", tr.unwrap_err());
            return Err(anyhow::anyhow!(
                "Error adding address to PKP - waiting for tx"
            ));
        }
        let tr = tr.unwrap();
        if tr.is_none() {
            error!("Error adding address to pkp: No transaction receipt?");
            return Err(anyhow::anyhow!(
                "Error adding address to PKP - no tx receipt"
            ));
        }

        let pa = pkp_permissions
            .get_permitted_addresses(token_id)
            .call()
            .await?;
        info!("permitted addresses: {:?}", pa);
        if !pa.contains(&addr_to_add) {
            return Err(anyhow::anyhow!("Address not added to permitted list"));
        }

        Ok(true)
    }

    #[doc = "Transfer a PKP"]
    pub async fn transfer_pkp_with_wallet(
        &self,
        to_address: Address,
    ) -> Result<bool, anyhow::Error> {
        info!(
            "Transferring PKP with token id: {} to address: {}",
            self.token_id, to_address
        );

        let pkpnft_address = self.actions.contracts().pkpnft.address();
        let pkpnft = PKPNFT::new(pkpnft_address, self.signing_provider.clone());

        let cc = pkpnft.transfer_from(
            self.signing_provider.clone().address(),
            to_address,
            self.token_id,
        );
        let tx = cc.send().await;
        if tx.is_err() {
            let e = tx.unwrap_err();
            info!(
                "Decoded error: {}",
                decode_revert(&e, self.actions.contracts().pkpnft.abi()).to_string()
            );
            error!("Error sending transfer PKP: {:?}", e);
            return Err(anyhow::anyhow!("Error transferring PKP - sending tx"));
        }
        let tx = tx.unwrap();

        let tr = tx.await;
        if tr.is_err() {
            error!("Error waiting for transfer PKP: {:?}", tr.unwrap_err());
            return Err(anyhow::anyhow!("Error transferring PKP - waiting for tx"));
        }
        let tr = tr.unwrap();
        if tr.is_none() {
            error!("Error transferring PKP: No transaction receipt?");
            return Err(anyhow::anyhow!("Error transferring PKP - no tx receipt"));
        }

        Ok(true)
    }

    #[doc = "Grant an action permission to use a PKP"]
    pub async fn add_permitted_action_to_pkp(
        &self,
        ipfs_cid: &str,
        scopes: &[U256],
    ) -> Result<bool, anyhow::Error> {
        info!(
            "ipfs_cid to permit for token id: {} is: {}",
            self.token_id, ipfs_cid
        );

        let pkp_permissions_address = self.actions.contracts().pkp_permissions.address();
        let pkp_permissions =
            PKPPermissions::new(pkp_permissions_address, self.signing_provider.clone());
        let pacc = pkp_permissions.add_permitted_action(
            self.token_id,
            Bytes::from(bs58::decode(ipfs_cid).into_vec().unwrap()),
            scopes.to_vec(),
        );

        let tx = pacc.send().await;
        if tx.is_err() {
            error!("Error adding action to pkp: {:?}", tx.unwrap_err());
            return Err(anyhow::anyhow!("Error minting PKP"));
        }
        let tx = tx.unwrap();

        let tr = tx.await;
        if tr.is_err() {
            error!("Error adding action to pkp: {:?}", tr.unwrap_err());
            return Err(anyhow::anyhow!("Error minting PKP"));
        }
        let tr = tr.unwrap();
        if tr.is_none() {
            error!("Error adding action to pkp: No transaction receipt?");
            return Err(anyhow::anyhow!("Error minting PKP"));
        }

        Ok(true)
    }

    #[doc = "Grant a Address Authmethod permission to use a PKP"]
    pub async fn add_permitted_address_auth_method_to_pkp(
        &self,
        address_token: Vec<u8>,
        scopes: &[U256],
    ) -> Result<bool, anyhow::Error> {
        let address_auth_method = AuthMethod {
            auth_method_type: U256::from(1),
            id: Bytes::from(address_token),
            user_pubkey: Bytes::from(vec![]),
        };
        debug!("Address Auth method to permit: {:?}", address_auth_method);

        let pkp_permissions_address = self.actions.contracts().pkp_permissions.address();
        let pkp_permissions =
            PKPPermissions::new(pkp_permissions_address, self.signing_provider.clone());
        let paam = pkp_permissions.add_permitted_auth_method(
            self.token_id,
            address_auth_method,
            scopes.to_vec(),
        );

        let tx = paam.send().await;
        if tx.is_err() {
            error!(
                "Error adding address authMethod to pkp: {:?}",
                tx.unwrap_err()
            );
            return Err(anyhow::anyhow!("Error minting PKP"));
        }
        let tx = tx.unwrap();

        let tr = tx.await;
        if tr.is_err() {
            error!(
                "Error adding address authMethod to pkp: {:?}",
                tr.unwrap_err()
            );
            return Err(anyhow::anyhow!("Error minting PKP"));
        }
        let tr = tr.unwrap();
        if tr.is_none() {
            error!("Error adding address authMethod to pkp: No transaction receipt?");
            return Err(anyhow::anyhow!("Error minting PKP"));
        }

        Ok(true)
    }

    pub async fn mint_grant_and_burn_next_pkp(
        end_user: &EndUser,
        ipfs_cid: &str,
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
            .mint_grant_and_burn_next(key_type, "naga-keyset1".to_string(), ipfs_bytes)
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
            eth_address: eth_address.into(),
        })
    }
}
