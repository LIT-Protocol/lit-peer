pub mod actions;
pub mod anvil_cache;
pub mod cache_data_store;
pub mod chain;
pub mod contracts;
pub mod contracts_repo;
pub mod datil;
pub mod listener;
pub mod node_config;

use crate::testnet::contracts_repo::{
    contract_addresses_from_deployment, remote_deployment_and_config_creation,
};
use crate::testnet::datil::DatilTestnet;

use self::anvil_cache::check_and_load_test_state_cache;
use self::chain::ChainTrait;
use self::contracts::{ContractAddresses, Contracts, StakingContractGlobalConfig};
use self::node_config::{CustomNodeRuntimeConfig, generate_custom_node_runtime_config};
use command_group::GroupChild;

use crate::testnet::actions::NetworkState;
use contracts::StakingContractRealmConfig;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use ethers::types::Address;
use futures::future::BoxFuture;
use lit_blockchain::resolver::rpc::{ENDPOINT_MANAGER, RpcHealthcheckPoller};
use lit_core::utils::binary::hex_to_bytes;
use lit_core::utils::toml::SimpleToml;
use lit_node_common::coms_keys::ComsKeys;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, trace};

use actions::Actions;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerItem {
    pub addr: String,
    pub node_address: Address,
    pub sender_public_key: [u8; 32], // SenderPublicKey does not impl Deserialize
    pub receiver_public_key: [u8; 32], // ReceiverPublicKey does not impl Deserialize
    pub staker_address: Address,     // address of staking wallet
}

#[derive(PartialEq, Clone, Default, Debug)]
pub enum TestNetName {
    Naga,
    Hardhat,
    NoChain,
    #[default]
    Anvil,
}

#[derive(Clone, Debug)]
pub struct NodeAccount {
    pub signing_provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    pub node_address: Address,
    pub node_address_private_key: ethers::types::H256,
    pub staker_address: Address,
    pub staker_address_private_key: ethers::types::H256,
    pub coms_keys: ComsKeys,
}

impl PartialEq for NodeAccount {
    fn eq(&self, other: &Self) -> bool {
        self.staker_address_private_key == other.staker_address_private_key
            && self.node_address_private_key == other.node_address_private_key
    }
}

#[must_use]
pub struct TestnetBuilder {
    selected_network: TestNetName,
    num_staked_only_validators: usize,
    num_staked_and_joined_validators: usize,
    force_deploy: bool,
    staker_account_setup_mapper: Option<
        Box<dyn StakerAccountSetupMapper<Future = BoxFuture<'static, Result<(), anyhow::Error>>>>,
    >,
    realm_id: u8,

    // FIXME: these parameters need to be refactor since conceptually don't belong to Testnet struct.
    custom_node_runtime_config: Option<CustomNodeRuntimeConfig>,
    is_fault_test: bool,
    register_inactive_validators: bool,
    datil_testnet_state_cache_path: String,
    datil_testnet_contract_resolver_address: Address,
}

impl Default for TestnetBuilder {
    fn default() -> Self {
        Self {
            selected_network: TestNetName::default(),
            num_staked_only_validators: 0,
            num_staked_and_joined_validators: 10,
            force_deploy: false,
            staker_account_setup_mapper: None,
            realm_id: 1,
            custom_node_runtime_config: None,
            is_fault_test: false,
            register_inactive_validators: false,
            // these values are hardcoded since the datil chain comes from a fixed file in the test_data directory.
            datil_testnet_state_cache_path: "tests/test_data/datil_cache/datil-anvil-state.hex"
                .to_string(),
            datil_testnet_contract_resolver_address: Address::from_slice(
                &hex::decode("5fbdb2315678afecb367f032d93f642f64180aa3")
                    .expect("Failed to decode contract resolver address"),
            ),
        }
    }
}

impl TestnetBuilder {
    pub fn selected_testnet(self, selected_testnet: TestNetName) -> Self {
        Self {
            selected_network: selected_testnet,
            ..self
        }
    }

    pub fn num_staked_only_validators(self, num_staked_only_validators: usize) -> Self {
        Self {
            num_staked_only_validators,
            ..self
        }
    }

    pub fn num_staked_and_joined_validators(self, num_staked_and_joined_validators: usize) -> Self {
        Self {
            num_staked_and_joined_validators,
            ..self
        }
    }

    pub fn total_num_validators(&self) -> usize {
        self.num_staked_only_validators + self.num_staked_and_joined_validators
    }

    pub fn force_deploy(self, force_deploy: bool) -> Self {
        Self {
            force_deploy,
            ..self
        }
    }

    pub fn custom_node_runtime_config(
        self,
        custom_node_runtime_config: CustomNodeRuntimeConfig,
    ) -> Self {
        Self {
            custom_node_runtime_config: Some(custom_node_runtime_config),
            ..self
        }
    }

    pub fn is_fault_test(self, is_fault_test: bool) -> Self {
        Self {
            is_fault_test,
            ..self
        }
    }

    pub fn staker_account_setup_mapper(
        self,
        staker_account_setup_mapper: Option<
            Box<
                dyn StakerAccountSetupMapper<
                    Future = BoxFuture<'static, Result<(), anyhow::Error>>,
                >,
            >,
        >,
    ) -> Self {
        Self {
            staker_account_setup_mapper,
            ..self
        }
    }

    pub fn realm_id(self, realm_id: u8) -> Self {
        Self { realm_id, ..self }
    }

    pub fn register_inactive_validators(self, register_inactive_validators: bool) -> Self {
        Self {
            register_inactive_validators,
            ..self
        }
    }

    pub async fn build(self) -> Testnet {
        let chain = match self.selected_network {
            #[cfg(not(feature = "lit-peer-api-server"))]
            TestNetName::Hardhat => {
                Box::new(chain::hardhat::Hardhat::new(self.total_num_validators()))
                    as Box<dyn ChainTrait>
            }
            #[cfg(not(feature = "lit-peer-api-server"))]
            TestNetName::Anvil => {
                Box::new(chain::anvil::Anvil::new(self.total_num_validators(), false))
                    as Box<dyn ChainTrait>
            }
            #[cfg(not(feature = "lit-peer-api-server"))]
            TestNetName::NoChain => {
                Box::new(chain::no_chain::NoChain::new(self.total_num_validators()))
                    as Box<dyn ChainTrait>
            }
            TestNetName::Naga => {
                Box::new(chain::live_chain::Naga::new(self.total_num_validators()).await)
                    as Box<dyn ChainTrait>
            }
            _ => panic!(
                "Selected network {:?} is not supported",
                self.selected_network
            ),
        };

        let net_process = chain.start_chain().await;
        let mut provider = ENDPOINT_MANAGER
            .get_provider(chain.chain_name())
            .expect(&format!(
                "Error retrieving provider for chain {} - check name and/or rpc_config yaml.",
                chain.chain_name()
            ));

        let provider_mut = Arc::make_mut(&mut provider);
        let provider = Arc::new(provider_mut.set_interval(Duration::from_millis(10)).clone());
        let mut is_from_cache = false;
        let live_chain = self.selected_network == TestNetName::Naga;
        let datil_testnet = DatilTestnet::new(
            self.total_num_validators(),
            self.datil_testnet_state_cache_path,
            self.datil_testnet_contract_resolver_address,
            live_chain,
        )
        .await;
        // deploy the contracts via script first, so that we can read them when the testnet configuration is loaded.
        if self.selected_network == TestNetName::Anvil
            || self.selected_network == TestNetName::Hardhat
        {
            // First, determine whether we need to generate custom node runtime config.
            let need_custom_node_runtime_config =
                self.custom_node_runtime_config.is_some() || self.is_fault_test;

            let custom_node_runtime_config = self
                .custom_node_runtime_config
                .unwrap_or(Default::default());
            generate_custom_node_runtime_config(
                self.is_fault_test,
                &self.selected_network,
                &custom_node_runtime_config,
                None,
            );

            if !self.force_deploy {
                // Note:  We only try load the state cache if the network is active - this could change, but other types of loading are generally exception cases.
                is_from_cache = true;
                if !check_and_load_test_state_cache(
                    provider.clone(),
                    self.num_staked_and_joined_validators,
                    self.num_staked_only_validators,
                    &NetworkState::Active,
                    &custom_node_runtime_config,
                    self.is_fault_test,
                )
                .await
                {
                    remote_deployment_and_config_creation(
                        self.num_staked_and_joined_validators,
                        self.num_staked_only_validators,
                        need_custom_node_runtime_config,
                    )
                    .await;
                    info!("Deployed contracts via script.");
                    is_from_cache = false;
                }
            } else {
                // just deploy normally
                remote_deployment_and_config_creation(
                    self.num_staked_and_joined_validators,
                    self.num_staked_only_validators,
                    need_custom_node_runtime_config,
                )
                .await;
            }
        }

        let rpcurl = chain.rpc_url();
        let deploy_account = chain.deployer();
        let deploy_address = deploy_account.signing_provider.address();
        let existing_config_path = chain.reuse_config_path();

        Testnet {
            process: net_process,
            rpcurl,
            selected_network: self.selected_network,
            provider,
            deploy_address,
            chain_name: chain.chain_name().to_string(),
            chain_id: chain.chain_id(),
            realm_id: self.realm_id,
            node_accounts: chain.accounts(),
            deploy_account,
            existing_config_path,
            num_staked_only_validators: self.num_staked_only_validators,
            num_staked_and_joined_validators: self.num_staked_and_joined_validators,
            staker_account_setup_mapper: self.staker_account_setup_mapper,
            register_inactive_validators: self.register_inactive_validators,
            contracts: None,
            is_from_cache,
            datil_testnet,
        }
    }
}

pub struct TestnetContracts {
    contracts: Contracts,
    contract_addresses: ContractAddresses,
}

impl TestnetContracts {
    pub fn contracts(&self) -> &Contracts {
        &self.contracts
    }

    pub fn contract_addresses(&self) -> &ContractAddresses {
        &self.contract_addresses
    }
}

pub struct Testnet {
    process: Option<GroupChild>,
    pub datil_testnet: DatilTestnet,
    pub rpcurl: String, //http://localhost:8545
    pub chain_name: String,
    pub chain_id: u64,
    pub realm_id: u8,
    pub selected_network: TestNetName,
    pub provider: Arc<Provider<Http>>,
    pub deploy_address: Address,
    pub node_accounts: Arc<Vec<NodeAccount>>,
    pub deploy_account: NodeAccount,
    pub existing_config_path: Option<String>,
    /// Number of validators that have only staked but not joined, exclusive of those already accounted for in `num_staked_and_joined_validators`.
    pub num_staked_only_validators: usize,
    /// Number of validators that have staked and joined, exclusive of those already accounted for in `num_staked_only_validators`.
    pub num_staked_and_joined_validators: usize,

    staker_account_setup_mapper: Option<
        Box<dyn StakerAccountSetupMapper<Future = BoxFuture<'static, Result<(), anyhow::Error>>>>,
    >,
    pub register_inactive_validators: bool,
    contracts: Option<Contracts>,
    pub is_from_cache: bool,
}

impl Testnet {
    pub fn builder() -> TestnetBuilder {
        TestnetBuilder::default()
    }

    pub fn total_num_validators(&self) -> usize {
        self.num_staked_only_validators + self.num_staked_and_joined_validators
    }

    pub fn realm_id(&self) -> u8 {
        self.realm_id
    }

    // stop testnet and clean up
    fn stop(&mut self) {
        // return; // uncomment this if you want to keep anvil running
        match self.process.as_mut() {
            Some(process) => {
                process.kill().unwrap_or_else(|e| {
                    panic!("Testnet process {:?} couldn't be killed: {}", process, e)
                });
                //ps x -o  "%p %r %y %x %c "
                process.wait().unwrap_or_else(|e| {
                    panic!("Testnet process {:?} couldn't be waited on: {}", process, e)
                });
            }
            None => {
                info!(
                    "Testnet is an on chain testnet, was never started, or already exists, so there is no process to kill."
                );
            }
        }

        self.datil_testnet.shutdown();
        // if hardhat or node are spawning something and leaving it running after kill
        // Command::new("pkill").arg("node").spawn().unwrap();
    }

    pub fn actions(&self) -> Actions {
        let contracts = self.contracts.as_ref().unwrap();
        let datil_contracts = self.datil_testnet.contracts.clone();

        Actions::new(
            contracts.clone(),
            datil_contracts,
            self.deploy_account.signing_provider.clone(),
            self.selected_network.clone(),
            self.deploy_address,
        )
    }

    pub async fn setup_contracts(
        testnet: &mut Testnet,
        staking_contract_global_config: Option<StakingContractGlobalConfig>,
        staking_contract_realm_config: Option<StakingContractRealmConfig>,
    ) -> anyhow::Result<TestnetContracts> {
        let deployer_signing_provider = testnet.deploy_account.signing_provider.clone();

        let ca = match testnet.selected_network {
            TestNetName::Naga => {
                let n = chain::live_chain::Naga::new(0).await;
                Contracts::contract_addresses_from_resolver_address(
                    n.contract_resolver_address(),
                    deployer_signing_provider,
                )
                .await
            }
            _ => match testnet.existing_config_path.clone() {
                Some(path) => {
                    Contracts::contract_addresses_from_resolver_cfg(
                        path,
                        testnet.deploy_account.signing_provider.clone(),
                    )
                    .await
                }
                None => contract_addresses_from_deployment().await,
            },
        };

        let deployer_signing_provider = testnet.deploy_account.signing_provider.clone();

        trace!("contract addresses: {:?}", &ca);
        let contracts = Contracts::new(
            &ca,
            testnet,
            deployer_signing_provider.clone(),
            staking_contract_global_config,
            staking_contract_realm_config,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to deploy/resolve contracts: {}", e))?;

        // if we want to listen for events, we need to spawn a new task
        if true {
            let staking_clone = contracts.staking.clone();
            let pubkey_router_clone = contracts.pubkey_router.clone();
            tokio::spawn(async move {
                let quit_rx = tokio::sync::mpsc::channel(1).1;
                let r = listener::listen_for_events(
                    Arc::new(staking_clone),
                    Arc::new(pubkey_router_clone),
                    quit_rx,
                )
                .await;
                if let Err(e) = r {
                    tracing::error!("Error in event listener: {:?}", e);
                }
            });
        }

        testnet.contracts = Some(contracts.clone());

        Ok(TestnetContracts {
            contracts,
            contract_addresses: ca,
        })
    }
}

pub trait StakerAccountSetupMapper: Send + Sync {
    type Future: Future<Output = Result<(), anyhow::Error>>;

    fn run(&mut self, args: (usize, NodeAccount, Contracts)) -> Self::Future;
}

impl<T: Future<Output = Result<(), anyhow::Error>>, F: FnMut((usize, NodeAccount, Contracts)) -> T>
    StakerAccountSetupMapper for F
where
    F: Send + Sync,
{
    type Future = T;

    fn run(&mut self, args: (usize, NodeAccount, Contracts)) -> Self::Future {
        self(args)
    }
}

pub trait BeforeStartValidatorsFn: Send + Sync {
    type Future: Future<Output = Result<(), anyhow::Error>>;

    fn run(&mut self, actions: Actions) -> Self::Future;
}

impl<T: Future<Output = Result<(), anyhow::Error>>, F: FnMut(Actions) -> T> BeforeStartValidatorsFn
    for F
where
    F: Send + Sync,
{
    type Future = T;

    fn run(&mut self, actions: Actions) -> Self::Future {
        self(actions)
    }
}

// Implementing drop means we don't have to remember to clean up the testnet, and is more able to clean up even when there is a panic, since drop may still be called.
impl Drop for Testnet {
    fn drop(&mut self) {
        info!("Attempting to stop Testnet");
        self.stop();
    }
}

pub(crate) trait SimpleTomlValue {
    fn get_address(&self, section: &str, key: &str) -> Option<H160>;
    fn get_signing_key(&self) -> Option<Vec<u8>>;
}

impl SimpleTomlValue for SimpleToml {
    fn get_address(&self, section: &str, key: &str) -> Option<H160> {
        let section = self.data().get(section);
        section?;
        let value = section.unwrap().get(key);

        value?;
        let value = value.unwrap().as_str();
        let bytes = hex_to_bytes(value).expect("Could not parse hex");
        let address = H160::from_slice(&bytes);
        Some(address)
    }

    fn get_signing_key(&self) -> Option<Vec<u8>> {
        let section = self.data().get("blockchain.wallet.default");
        section?;
        let value = section.unwrap().get("private_key");

        value?;
        let value = value.unwrap().as_str();
        let bytes = hex_to_bytes(value).expect("Could not parse hex");

        Some(bytes)
    }
}
