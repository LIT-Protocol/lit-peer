mod pkp;
use pkp::Pkp;

use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Middleware, Provider, ProviderError};
use ethers::signers::{LocalWallet, Signer, Wallet};
use ethers::types::{H160, I256, U256};
use k256::ecdsa::SigningKey;
use lit_blockchain::contracts::ledger::{Ledger, LedgerErrors};
use lit_blockchain::contracts::price_feed::{PriceFeed, PriceFeedErrors};
use lit_blockchain::util::decode_revert;
use lit_core::utils::binary::bytes_to_hex;
use tracing::{error, info, trace};

use crate::testnet::Testnet;
use crate::testnet::actions::Actions;
use rand_core::OsRng;
use std::sync::Arc;
use std::time::Duration;
const RETRY_WAIT_TIME_MS: u64 = 200;
const INITIAL_FUNDING_AMOUNT: &str = "100000000000000000000";
#[derive(Clone, Debug)]
pub struct EndUser {
    pub wallet: Wallet<SigningKey>,
    actions: Actions,
    pkps: Vec<Pkp>,
}

impl EndUser {
    pub fn new(testnet: &Testnet) -> Self {
        let new_wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
        info!("New wallet: {:?}", new_wallet.address());
        Self {
            wallet: new_wallet,
            actions: testnet.actions().clone(),
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

    pub async fn fetch_price_from_feed(&self, product_id: u64) -> Vec<U256> {
        let product_id = U256::from(product_id);
        let price_feed_contract_address: H160 = self.actions.contracts().price_feed.address();
        let provider: &Provider<Http> = &self.actions.deployer_provider();

        let client = SignerMiddleware::new(provider.clone(), self.wallet.clone());
        let price_feed_contract = PriceFeed::new(price_feed_contract_address, Arc::new(client));

        let product_prices = price_feed_contract.prices(product_id).await;
        if let Err(e) = product_prices {
            let error_with_type: Option<PriceFeedErrors> = e.decode_contract_revert();
            panic!("Couldn't get product prices: {:?}", error_with_type);
        }

        product_prices.unwrap().iter().map(|np| np.price).collect()
    }

    pub async fn first_node_price_from_feed(&self, product_id: u64) -> U256 {
        let prices = self.fetch_price_from_feed(product_id).await;
        info!("Current Prices: {:?}", prices);
        prices[0]
    }

    pub async fn get_first_pkp_ledger_balance(&self, notes: &str) -> I256 {
        self.get_ledger_balance(self.first_pkp().eth_address, notes)
            .await
    }

    pub async fn first_pkp_ledger_has_decreased_from(&self, amount: I256) -> bool {
        let mut current_balance = self
            .get_first_pkp_ledger_balance("Checking first PKP ledger balance.")
            .await;
        if current_balance >= amount {
            for i in 0..10 {
                info!(
                    "Waiting for first PKP ledger balance to decrease. Attempt: {}",
                    i
                );
                tokio::time::sleep(Duration::from_millis(RETRY_WAIT_TIME_MS)).await;
                current_balance = self
                    .get_first_pkp_ledger_balance("Checking first PKP ledger balance.")
                    .await;
                if current_balance < amount {
                    break;
                }
            }
        }

        let direction = if current_balance < amount {
            "decreased"
        } else {
            "increased"
        };
        info!(
            "First PKP ledger balance has {} from {:?} to {:?}.",
            direction, amount, current_balance
        );
        current_balance < amount
    }

    pub async fn get_wallet_ledger_balance(&self, notes: &str) -> I256 {
        self.get_ledger_balance(self.wallet.address(), notes).await
    }

    pub async fn wallet_ledger_has_decreased_from(&self, amount: I256) -> bool {
        let mut current_balance = self
            .get_wallet_ledger_balance("Checking wallet ledger balance.")
            .await;
        if current_balance >= amount {
            for i in 0..10 {
                info!(
                    "Waiting for wallet ledger balance to decrease. Attempt: {}",
                    i
                );
                tokio::time::sleep(Duration::from_millis(RETRY_WAIT_TIME_MS)).await;
                current_balance = self
                    .get_wallet_ledger_balance("Checking wallet ledger balance.")
                    .await;
                if current_balance < amount {
                    break;
                }
            }
        }

        let direction = if current_balance < amount {
            "decreased"
        } else {
            "increased"
        };
        info!(
            "Wallet ledger balance has {} from {:?} to {:?}.",
            direction, amount, current_balance
        );
        current_balance < amount
    }

    pub fn signing_provider(
        &self,
    ) -> Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>> {
        Arc::new(SignerMiddleware::new(
            self.actions.deployer_provider().clone(),
            self.wallet.clone(),
        ))
    }

    async fn get_ledger_balance(&self, address: H160, notes: &str) -> I256 {
        let ledger_contract_address: H160 = self.actions.contracts().ledger.address();
        let provider: &Provider<Http> = &self.actions.deployer_provider();

        let client = SignerMiddleware::new(provider.clone(), self.wallet.clone());
        let ledger_contract = Ledger::new(ledger_contract_address, Arc::new(client));

        let user_balance = ledger_contract.balance(address).await;
        if let Err(e) = user_balance {
            let error_with_type: Option<LedgerErrors> = e.decode_contract_revert();
            panic!("Couldn't get user balance: {:?}", error_with_type);
        }

        let user_balance = user_balance.unwrap();

        info!("{} : {:?} for address: {:?}", notes, user_balance, address);

        user_balance
    }

    pub async fn get_wallet_balance(&self) -> U256 {
        let provider = self.actions.deployer_provider();
        let balance = provider
            .get_balance(self.wallet.address(), None)
            .await
            .expect("Failed to get balance for wallet.");
        balance
    }

    pub async fn deposit_to_wallet_ledger_default(&self) {
        let provider = self.actions.deployer_provider();
        let balance = provider
            .get_balance(self.wallet.address(), None)
            .await
            .expect("Failed to get balance for wallet.");
        let deposit_amount = balance / 5;
        self.deposit_to_wallet_ledger(deposit_amount).await;
    }

    pub async fn deposit_to_first_pkp_ledger_default(&self) {
        let provider = self.actions.deployer_provider();
        let balance = provider
            .get_balance(self.wallet.address(), None)
            .await
            .expect("Failed to get balance for wallet.");
        let deposit_amount = balance / 5;
        self.deposit_to_first_pkp_ledger(deposit_amount).await;
    }

    pub async fn deposit_to_wallet_ledger(&self, deposit_amount: U256) {
        self.deposit_ledger_balance(deposit_amount, self.wallet.address())
            .await;
    }

    pub async fn deposit_to_first_pkp_ledger(&self, deposit_amount: U256) {
        if self.pkps.is_empty() {
            panic!("No PKPs found");
        }

        self.deposit_ledger_balance(deposit_amount, self.pkps[0].eth_address)
            .await;
    }

    pub async fn deposit_to_pkp_ledger(&self, pkp: &Pkp, deposit_amount: U256) {
        info!(
            "Depositing {:?} to PKP {:?}",
            deposit_amount, pkp.eth_address
        );
        self.deposit_ledger_balance(deposit_amount, pkp.eth_address)
            .await;
    }

    async fn deposit_ledger_balance(&self, deposit_amount: U256, user_address: H160) {
        let ledger_contract_address: H160 = self.actions.contracts().ledger.address();
        let provider: &Provider<Http> = &self.actions.deployer_provider();

        let client = SignerMiddleware::new(provider.clone(), self.wallet.clone());
        let ledger_contract = Ledger::new(ledger_contract_address, Arc::new(client));

        let tx = ledger_contract
            .deposit_for_user(user_address)
            .value(deposit_amount);
        let pending_tx = tx.send().await;
        if let Err(e) = pending_tx {
            error!("Problem depositing amount: {:?}", e);

            let error_with_type: Option<LedgerErrors> = e.decode_contract_revert();
            if let Some(error) = error_with_type {
                panic!("Couldn't deposit amount - contract revert: {:?}", error);
            }

            let decoded = decode_revert(&e, ledger_contract.abi());
            panic!("Couldn't deposit amount: {:?}", decoded);
        }

        let pending_tx = pending_tx.unwrap().interval(Duration::from_millis(100));
        let receipt = pending_tx.await.unwrap().expect("No receipt from txn");
        info!(
            "Ledger deposit receipt transasction hash: {:?}",
            receipt.transaction_hash
        );

        let log = &receipt.logs[0];
        trace!("log: {:?}", log);
    }

    pub async fn ledger_request_withdraw(&self, amount: I256, notes: &str) {
        let ledger_contract_address: H160 = self.actions.contracts().ledger.address();
        let provider: &Provider<Http> = &self.actions.deployer_provider();

        let client = SignerMiddleware::new(provider.clone(), self.wallet.clone());
        let ledger_contract = Ledger::new(ledger_contract_address, Arc::new(client));

        let tx = ledger_contract.request_withdraw(amount);
        let res = tx.send().await;

        if let Err(e) = res {
            let error_with_type: Option<LedgerErrors> = e.decode_contract_revert();
            panic!("Couldn't request to withdraw: {:?}", error_with_type);
        }

        info!("{}: {:?}", notes, amount);
    }

    pub async fn get_wallet_ledger_stable_balance(&self, notes: &str) -> I256 {
        let ledger_contract_address: H160 = self.actions.contracts().ledger.address();
        let provider: &Provider<Http> = &self.actions.deployer_provider();

        let client = SignerMiddleware::new(provider.clone(), self.wallet.clone());
        let ledger_contract = Ledger::new(ledger_contract_address, Arc::new(client));

        let user_stable_balance = ledger_contract.stable_balance(self.wallet.address()).await;
        if let Err(e) = user_stable_balance {
            let error_with_type: Option<LedgerErrors> = e.decode_contract_revert();
            panic!("Couldn't get user balance: {:?}", error_with_type);
        }

        let user_stable_balance = user_stable_balance.unwrap();

        info!("{} : {:?}", notes, user_stable_balance);

        user_stable_balance
    }

    pub async fn new_pkp(&mut self) -> Result<(String, U256, H160), anyhow::Error> {
        let pkp = Pkp::new(self, "naga-keyset1").await?;
        let pkp_info = (pkp.pubkey.clone(), pkp.token_id, pkp.eth_address.clone());
        self.pkps.push(pkp);
        Ok(pkp_info)
    }

    pub async fn new_pkp_with_key_set_id(
        &mut self,
        key_set_id: &str,
    ) -> Result<(String, U256, H160), anyhow::Error> {
        let pkp = Pkp::new(self, key_set_id).await?;
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
