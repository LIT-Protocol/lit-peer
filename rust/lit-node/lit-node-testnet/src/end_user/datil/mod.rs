mod pkp;
use pkp::Pkp;

use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider, ProviderError};
use ethers::signers::{LocalWallet, Signer, Wallet};
use ethers::types::{H160, U256};
use k256::ecdsa::SigningKey;
use lit_core::utils::binary::bytes_to_hex;
use tracing::info;

use crate::testnet::ImportedDatilTestnet;
use crate::testnet::datil::actions::Actions;
use rand_core::OsRng;
use std::sync::Arc;
const INITIAL_FUNDING_AMOUNT: &str = "100000000000000000000";

#[derive(Clone, Debug)]
pub struct EndUser {
    pub wallet: Wallet<SigningKey>,
    actions: Actions,
    pkps: Vec<Pkp>,
}

impl EndUser {
    pub fn new(imported_datil_testnet: &ImportedDatilTestnet) -> Self {
        let new_wallet =
            LocalWallet::new(&mut OsRng).with_chain_id(imported_datil_testnet.chain_id());
        info!("New wallet: {:?}", new_wallet.address());
        Self {
            wallet: new_wallet,
            actions: imported_datil_testnet.actions().clone(),
            pkps: vec![],
        }
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub fn first_pkp(&self) -> &Pkp {
        if self.pkps.is_empty() {
            panic!("No PKPs found");
        }

        &self.pkps[0]
    }

    pub fn pkp_by_token_id(&self, token_id: U256) -> &Pkp {
        self.pkps
            .iter()
            .find(|pkp| pkp.token_id == token_id)
            .expect("PKP not found by token id")
    }

    pub fn pkp_by_pubkey(&self, pubkey: String) -> &Pkp {
        self.pkps
            .iter()
            .find(|pkp| pkp.pubkey == pubkey)
            .expect("PKP not found by pubkey")
    }

    pub async fn fund_wallet_default_amount(&self) {
        self.set_wallet_balance(INITIAL_FUNDING_AMOUNT).await;
    }

    pub async fn set_wallet_balance(&self, amount: &str) {
        let provider = self.actions.deployer_provider();

        let res: Result<(), ProviderError> = provider
            .request(
                "anvil_setBalance",
                [
                    format!("0x{}", bytes_to_hex(self.wallet.address())),
                    amount.to_string(),
                ],
            )
            .await;

        if let Err(e) = res {
            panic!("Couldn't set balance: {:?}", e);
        }
    }

    pub fn signing_provider(
        &self,
    ) -> Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
        Arc::new(SignerMiddleware::new(
            self.actions.deployer_provider().clone(),
            self.wallet.clone(),
        ))
    }

    pub async fn get_wallet_balance(&self) -> U256 {
        let provider = self.actions.deployer_provider();
        let balance = provider
            .get_balance(self.wallet.address(), None)
            .await
            .expect("Failed to get balance for wallet.");
        balance
    }

    pub async fn new_pkp(&mut self) -> Result<(String, U256, H160), anyhow::Error> {
        let pkp = Pkp::new(self).await?;
        let pkp_info = (pkp.pubkey.clone(), pkp.token_id, pkp.eth_address.clone());
        self.pkps.push(pkp);
        Ok(pkp_info)
    }

    pub async fn new_pkp_with_permitted_address(
        &mut self,
        addr: H160,
    ) -> Result<(String, U256, H160), anyhow::Error> {
        let (pubkey, token_id, eth_address) = self.new_pkp().await?;

        let pkp = self.pkp_by_pubkey(pubkey.clone());
        pkp.add_permitted_address_to_pkp(addr, &[U256::from(1)])
            .await?;

        Ok((pubkey, token_id, eth_address))
    }

    pub async fn mint_grant_and_burn_next_pkp(&self, ipfs_cid: &str) -> Result<Pkp, anyhow::Error> {
        Pkp::mint_grant_and_burn_next_pkp(self, ipfs_cid).await
    }
}
