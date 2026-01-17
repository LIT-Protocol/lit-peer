use crate::testnet::NodeAccount;
use crate::testnet::chain::anvil::first_anvil_account;

use super::super::ChainTrait;
use command_group::{CommandGroup, GroupChild}; 
use std::process::Command;

use ethers::prelude::*;
use ethers::signers::LocalWallet;
use lit_node_common::coms_keys::ComsKeys;
use std::sync::Arc;
pub struct NagaTest {
    num_nodes: usize,
    // num_staked: usize,
    wallet: LocalWallet,
}

impl NagaTest {
    // pub fn new(num_nodes: usize, num_staked: usize) -> impl ChainTrait {
    pub fn new(num_nodes: usize) -> impl ChainTrait {
        NagaTest {
            num_nodes,
            wallet: LocalWallet::new(&mut rand::thread_rng()),
        }
    }
}

use async_trait::async_trait;
// impl chain for NagaTest
#[async_trait]
impl ChainTrait for NagaTest {
    fn chain_id(&self) -> u64 {
        175188
    }

    fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    fn rpc_url(&self) -> String {
        "https://yellowstone-rpc.litprotocol.com".to_string()
    }

    fn chain_name(&self) -> &'static str {
        "yellowstone"
    }

    // This is where we'll load default values from GitHub.
    async fn start_chain(&self) -> GroupChild {        

        return Command::new("/bin/bash")
            .args(["-c", "echo '*** NagaTest is already running ***'"])
            .group_spawn()
            .expect("Could not spawn echo process");
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