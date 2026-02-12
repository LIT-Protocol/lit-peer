use crate::testnet::NodeAccount;
use crate::testnet::chain::known_accounts::first_anvil_account;

use super::super::ChainTrait;
use command_group::GroupChild;
use ethers::prelude::*;
use ethers::signers::LocalWallet;
use lit_node_common::coms_keys::ComsKeys;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use toml_edit::DocumentMut;
use tracing::info;

#[derive(Debug)]
pub struct Naga {
    name: String,
    chain_id: u64,
    chain_name: String,
    rpc_url: String,
    pub contract_resolver_address: Address,
    num_nodes: usize,
    wallet: LocalWallet,
}

impl Naga {
    pub async fn new(num_nodes: usize) -> impl ChainTrait {
        let std_warning = "Please ensure the contents of this file are valid, or remove it entirely to run tests against a selected local network.";
        println!(
            "Found live configuration file live_testnet.toml. Getting configuration values for testing..."
        );
        let toml_path = Path::new("live_testnet.toml");
        let toml_contents = fs::read_to_string(toml_path).unwrap_or_default();
        if toml_contents.is_empty() {
            panic!("No configuration found in live_testnet.toml.  {std_warning}");
        }

        let toml_document = match toml_contents.parse::<DocumentMut>() {
            Ok(doc) => doc,
            Err(e) => panic!("Failed to parse live_testnet.toml: {}. {std_warning}", e),
        };

        // technically there should only be one table, but we'll loop through them all just in case.
        let networks: Vec<String> = toml_document
            .as_table()
            .iter()
            .map(|(module, _)| module.to_string())
            .collect();

        info!("networks: {networks:?}");

        let mut name = String::new();
        let mut chain_id = 0;
        let mut chain_name = String::new();
        let mut rpc_url = String::new();
        let mut contract_resolver_address = Address::zero();

        for network in networks {
            if let Some(config) = toml_document[&network].as_table() {
                name = config
                    .get("name")
                    .expect(
                        format!("name is required for network {network}. {}", std_warning).as_str(),
                    )
                    .to_string();
                chain_id = config
                    .get("chain_id")
                    .expect(
                        format!(
                            "chain_id is required for network {network}. {}",
                            std_warning
                        )
                        .as_str(),
                    )
                    .as_str()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                chain_name = config
                    .get("chain_name")
                    .expect(
                        format!(
                            "chain_name is required for network {network}. {}",
                            std_warning
                        )
                        .as_str(),
                    )
                    .to_string();
                rpc_url = config
                    .get("rpc_url")
                    .expect(
                        format!("rpc_url is required for network {network}. {}", std_warning)
                            .as_str(),
                    )
                    .to_string();
                contract_resolver_address = config
                    .get("contract_resolver_address")
                    .expect(
                        format!(
                            "contract_resolver_address is required for network {network}. {}",
                            std_warning
                        )
                        .as_str(),
                    )
                    .as_str()
                    .unwrap()
                    .parse::<Address>()
                    .unwrap();
            } else {
                panic!("No configuration found for network {network}. {std_warning}");
            };
        }

        if name.is_empty()
            || chain_id == 0
            || chain_name.is_empty()
            || rpc_url.is_empty()
            || contract_resolver_address == Address::zero()
        {
            panic!("Invalid configuration found in live_testnet.toml. {std_warning}");
        }

        let config = Naga {
            name: name.replace("\"", "").trim().to_string(),
            chain_id,
            chain_name: chain_name.replace("\"", "").trim().to_string(),
            rpc_url: rpc_url.replace("\"", "").trim().to_string(),
            contract_resolver_address,
            num_nodes,
            wallet: LocalWallet::new(&mut rand::thread_rng()),
        };
        info!("Naga configuration: {config:?}");
        config
    }

    pub fn get_contract_resolver_address(&self) -> Address {
        self.contract_resolver_address
    }
}

use async_trait::async_trait;
// impl chain for NagaTest
#[async_trait]
impl ChainTrait for Naga {
    fn chain_id(&self) -> u64 {
        self.chain_id
    }

    fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    fn rpc_url(&self) -> String {
        self.rpc_url.clone()
    }
    fn chain_name(&self) -> &'static str {
        Box::leak(self.chain_name.clone().into_boxed_str())
    }

    fn contract_resolver_address(&self) -> Address {
        self.contract_resolver_address
    }

    // This is where we'll load default values from GitHub.
    async fn start_chain(&self) -> Option<GroupChild> {
        info!(
            "Network {} on chain {} should already exist at {}. ",
            self.name,
            self.chain_name(),
            self.rpc_url()
        );
        None
    }

    // Yes, this is the same first anvil account.  It'll be used as a fake deployer account for the naga testnet.
    // This means it'll need to be funded by the testnet faucet found at faucet.litprotocol.com.
    // The wallet address for this key is 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266.
    fn deployer(&self) -> NodeAccount {
        first_anvil_account(self.chain_id(), self.chain_name())
    }

    fn accounts(&self) -> Arc<Vec<NodeAccount>> {
        let mut accounts: Vec<NodeAccount> = Vec::new();
        for _i in 0..self.num_nodes {
            let wallet = self.wallet.clone();
            let provider = self.rpc_provider().clone();
            accounts.push(NodeAccount {
                node_address: Address::zero(),
                signing_provider: Arc::new(SignerMiddleware::new(provider, wallet)),
                node_address_private_key: H256::zero(),
                staker_address_private_key: H256::zero(),
                staker_address: Address::zero(),
                coms_keys: ComsKeys::new(),
            });
        }
        Arc::new(accounts)
    }
}
