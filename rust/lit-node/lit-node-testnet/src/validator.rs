use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::io::BufReader;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::sync::Arc;

use anyhow::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::signers::Wallet;
use futures::future::join_all;
use lit_attestation::attestation::ENV_ATTESTATION_TYPE_OVERRIDE;
use lit_blockchain::contracts::staking::KeySetConfig;
use lit_blockchain::contracts::staking::Staking;
use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_core::error::Unexpected;
use lit_core::utils::binary::bytes_to_hex;
use lit_core::utils::toml::SimpleToml;
use lit_logging::config::ENV_LOGGING_TIMESTAMP;
use lit_node_core::NodeSet;
use url::Url;
// use lit_node::p2p_comms::web::chatter_server::chatter::chatter_service_client::ChatterServiceClient;
use rand::Rng;
use std::fs::File;
use std::path::Path;
use tracing::error;
use tracing::trace;
use tracing::{debug, info, warn};

use lit_node_core::response::SDKHandshakeResponseV0;

use super::testnet::NodeAccount;
use super::testnet::Testnet;
use super::testnet::actions::Actions;
use super::testnet::contracts::Contracts;
use super::testnet::contracts_repo::node_configs_path;

use lit_node_core::CurveType;
const DEFAULT_KEY_SET_NAME: &str = "naga-keyset1";
// this is a duplicated value
pub static INTERNAL_CHATTER_PORT_OFFSET: u16 = 19608;

// this is a duplicated function.
pub fn get_grpc_url_from_http_url(mut url: Url) -> Url {
    let new_port = url
        .port_or_known_default()
        .expect("Unable to parse http port")
        + INTERNAL_CHATTER_PORT_OFFSET;
    url.set_port(Some(new_port))
        .expect("Failed to set new port");
    url
}

// this is a duplicated function.
pub fn get_local_url_from_port(port: usize) -> Url {
    Url::parse(format!("http://127.0.0.1:{}", port).as_str()).expect("Failed to parse local url")
}

//this is a duplicated function
pub fn choose_random_indices(array_size: usize, num_random_indices: usize) -> HashSet<usize> {
    let mut indices = HashSet::new();
    for _ in 0..num_random_indices {
        let mut idx = rand::random::<usize>() % array_size;
        while indices.contains(&idx) {
            idx = rand::random::<usize>() % array_size;
        }
        indices.insert(idx);
    }
    indices
}

#[must_use]
pub struct ValidatorCollectionBuilder {
    node_config_folder_path: String,
    wait_initial_epoch: bool,
    wait_for_root_keys: bool,
    num_staked_nodes: usize,
    num_asleep_initially: usize,
    asleep_initially_override: Option<Vec<usize>>,
    custom_binary_path: Option<String>,
    pause_network_while_building: bool, // skips pausing the network while building validators.  Only used for backup&restore tests.
    node_binary_feature_flags: Option<String>,
    keyset_configs: Vec<KeySetConfig>,
}

impl Default for ValidatorCollectionBuilder {
    fn default() -> Self {
        Self {
            node_config_folder_path: node_configs_path(),
            wait_initial_epoch: true,
            wait_for_root_keys: true,
            num_staked_nodes: 10,
            num_asleep_initially: 0,
            asleep_initially_override: None,
            custom_binary_path: None,
            pause_network_while_building: true,
            node_binary_feature_flags: None,
            keyset_configs: vec![],
        }
    }
}

impl ValidatorCollectionBuilder {
    pub fn node_config_folder_path(mut self, node_config_folder_path: String) -> Self {
        self.node_config_folder_path = node_config_folder_path;
        self
    }

    pub fn wait_initial_epoch(mut self, wait_initial_epoch: bool) -> Self {
        self.wait_initial_epoch = wait_initial_epoch;
        self
    }

    pub fn wait_for_root_keys(mut self, wait_for_root_keys: bool) -> Self {
        self.wait_for_root_keys = wait_for_root_keys;
        self
    }

    pub fn num_staked_nodes(mut self, num_staked_nodes: usize) -> Self {
        self.num_staked_nodes = num_staked_nodes;
        self
    }

    pub fn num_asleep_initially(mut self, num_asleep_initially: usize) -> Self {
        self.num_asleep_initially = num_asleep_initially;
        self
    }

    pub fn asleep_initially_override(
        mut self,
        asleep_initially_override: Option<Vec<usize>>,
    ) -> Self {
        self.asleep_initially_override = asleep_initially_override;
        self
    }

    pub fn pause_network_while_building(mut self, pause_network_while_building: bool) -> Self {
        self.pause_network_while_building = pause_network_while_building;
        self
    }

    pub fn custom_binary_path(mut self, custom_binary_path: Option<String>) -> Self {
        self.custom_binary_path = custom_binary_path;
        self
    }

    pub fn keyset_configs(mut self, keyset_configs: Vec<KeySetConfig>) -> Self {
        self.keyset_configs = keyset_configs;
        self
    }

    pub fn node_binary_feature_flags(mut self, node_binary_feature_flags: String) -> Self {
        self.node_binary_feature_flags = Some(node_binary_feature_flags);
        self
    }

    pub async fn build(&mut self, testnet: &Testnet) -> Result<ValidatorCollection> {
        // make sure the old nodes are killed
        Command::new("pkill")
            .arg("-9")
            .arg("lit_node")
            .output()
            .map_err(|e| anyhow::anyhow!("failed to kill lit_node: {}", e))?;

        let actions = testnet.actions();
        let initial_state = actions.get_state(testnet.realm_id() as u64).await;
        if self.pause_network_while_building {
            info!(
                "Pausing network while building validators.  Initial state was : {:?}.",
                initial_state
            );

            actions.set_state_to_paused(testnet.realm_id() as u64).await;
        }

        // erase all the old node logs
        let logs_path = "./tests/test_logs";
        if std::path::Path::new(&logs_path).exists() {
            std::fs::remove_dir_all(logs_path)
                .map_err(|e| anyhow::anyhow!("failed to remove test_logs directory: {}", e))?;
        }
        std::fs::create_dir_all(logs_path)
            .map_err(|e| anyhow::anyhow!("failed to create test_logs directory: {}", e))?;

        // Choose random indices between 0 and num_nodes that will NOT be awake initially.
        let asleep_initially =
            choose_random_indices(self.num_staked_nodes, self.num_asleep_initially);

        if self.num_asleep_initially > 0 {
            info!(
                "TEST: Nodes at indices {:?} will be asleep initially",
                asleep_initially
            );
        }

        let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string()) == "1";

        // if not in CI remove the existing binary.
        if !in_github_ci {
            match std::fs::remove_file("./target/test-run/debug/lit_node") {
                Ok(_) => {}
                Err(e) => {
                    error!("Error removing the existing `lit_node` binary: {}", e);
                }
            }
        }

        let mut validators = vec![];
        for i in 0..self.num_staked_nodes {
            let node_account = &testnet.node_accounts[i];
            let node_config_file_path =
                format!("{}/lit_config{:?}.toml", self.node_config_folder_path, i);

            validators.push(
                ValidatorBuilder::default()
                    .build_mode(
                        self.custom_binary_path
                            .clone()
                            .map(BuildMode::UseCustomBuild),
                    )
                    .node_binary_feature_flags(
                        self.node_binary_feature_flags
                            .clone()
                            .unwrap_or("lit-actions,testing".to_string()),
                    )
                    .build(
                        node_config_file_path,
                        node_account,
                        testnet.deploy_account.signing_provider.clone(),
                    )
                    .await?,
            );
        }

        let mut validator_ports_to_check_awake = Vec::new();
        for (idx, validator) in validators.iter_mut().enumerate() {
            if let Some(asleep_initially) = &self.asleep_initially_override {
                info!("Using asleep_initially_override: {:?}", asleep_initially);
                if asleep_initially.contains(&idx) {
                    continue;
                } else {
                    validator.start_node(true, false).await?;
                    validator_ports_to_check_awake.push(validator.node.port);
                }
            } else {
                // only start the nodes meant to be awake.
                if !asleep_initially.contains(&idx) {
                    // to avoid breaking old tests, explicit check if the testnet is configured to register inactive validators.
                    // if testnet.register_inactive_validators {
                    //     validator.set_node_info_without_joining(&actions).await?;
                    // }
                    validator.start_node(true, false).await?;
                    validator_ports_to_check_awake.push(validator.node.port);
                }
            }
        }

        // wait for all nodes to be awake once all node processes have been started - this is faster than
        // starting a process and waiting for each node to be awake one by one.
        ValidatorCollection::ensure_all_nodes_awake(validator_ports_to_check_awake).await?;

        let realm_id = testnet.realm_id() as u64;

        if self.pause_network_while_building {
            info!(
                "Restoring state to {:?} for realm {}",
                initial_state, realm_id,
            );
            testnet.actions().set_state(realm_id, initial_state).await;
        }

        let actions = testnet.actions();

        let realm_id = U256::from(realm_id);

        // wait for active
        if self.wait_initial_epoch {
            actions.wait_for_active(realm_id).await;
        }

        // wait for the root keys to be registered
        if self.wait_for_root_keys {
            for keyset_config in &self.keyset_configs {
                actions
                    .wait_for_root_keys(realm_id, Some(keyset_config.identifier.clone()))
                    .await;
            }
        }

        if !testnet.is_from_cache {
            crate::testnet::contracts_repo::save_to_test_state_cache(
                testnet.provider.clone(),
                testnet.num_staked_and_joined_validators,
                testnet.num_staked_only_validators,
            )
            .await;
        }

        Ok(ValidatorCollection {
            validators,
            actions: actions.clone(),
            testnet_deployer_signing_provider: testnet.deploy_account.signing_provider.clone(),
            testnet_node_accounts: testnet.node_accounts.clone(),
            node_config_folder_path: self.node_config_folder_path.clone(),
            // node_binary_feature_flags: self
            //     .node_binary_feature_flags
            //     .clone()
            // .unwrap_or("lit-actions,testing".to_string()),
            keyset_configs: self.keyset_configs.clone(),
        })
    }
}

pub struct ValidatorCollection {
    validators: Vec<Validator>,
    actions: Actions,
    testnet_deployer_signing_provider:
        Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    testnet_node_accounts: Arc<Vec<NodeAccount>>,
    // node_binary_feature_flags: String,
    // build parameters
    node_config_folder_path: String,
    keyset_configs: Vec<KeySetConfig>,
}

impl ValidatorCollection {
    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }

    pub fn last_n_validators(&mut self, n: usize) -> Vec<&Validator> {
        self.validators.iter().rev().take(n).collect::<Vec<_>>()
    }

    pub fn validators(&self) -> &Vec<Validator> {
        &self.validators
    }

    pub fn first_n_validators(&mut self, n: usize) -> Vec<&Validator> {
        self.validators.iter().take(n).collect::<Vec<_>>()
    }

    pub fn asleep_nodes(&self) -> Vec<&Validator> {
        self.validators
            .iter()
            .filter(|v| v.node.is_offline())
            .collect::<Vec<_>>()
    }

    pub fn builder() -> ValidatorCollectionBuilder {
        ValidatorCollectionBuilder::default()
    }

    pub fn node_binary_feature_flags(
        &self,
        node_binary_feature_flags: String,
    ) -> ValidatorCollectionBuilder {
        ValidatorCollectionBuilder {
            node_binary_feature_flags: Some(node_binary_feature_flags),
            ..Default::default()
        }
    }

    pub fn config_files(&self) -> Vec<String> {
        self.validators
            .iter()
            .map(|v| v.node.config_file.to_string())
            .collect()
    }

    pub fn log_readers(&self) -> Vec<BufReader<File>> {
        self.validators.iter().map(|v| v.log_reader()).collect()
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub fn addresses(&self) -> Vec<String> {
        self.validators
            .iter()
            .map(|v| format!("http://{}:{}", v.node.ip, v.node.port))
            .collect()
    }

    pub fn keyset_configs(&self) -> &Vec<KeySetConfig> {
        &self.keyset_configs
    }

    pub fn ports(&self) -> Vec<usize> {
        self.validators.iter().map(|v| v.node.port).collect()
    }

    pub fn max_port(&self) -> usize {
        self.validators.iter().map(|v| v.node.port).max().unwrap()
    }

    pub fn size(&self) -> usize {
        self.validators.len()
    }

    pub fn inferred_threshold(&self) -> usize {
        std::cmp::max(3, (self.validators.len() * 2) / 3)
    }

    pub fn threshold(&self, port_count: usize) -> usize {
        std::cmp::max(3, (port_count * 2) / 3)
    }

    pub fn get_validator_by_idx(&self, idx: usize) -> &Validator {
        &self.validators[idx]
    }

    pub fn get_validator_by_idx_mut(&mut self, idx: usize) -> &mut Validator {
        &mut self.validators[idx]
    }

    pub fn get_validator_by_account(&self, account: &NodeAccount) -> Option<&Validator> {
        self.validators.iter().find(|v| v.account == *account)
    }

    pub fn get_validator_by_port(&self, port: usize) -> Option<&Validator> {
        self.validators.iter().find(|v| v.node.port == port)
    }

    pub async fn get_validator_epochs(&self) -> Result<Vec<(H160, u64)>> {
        let mut epochs = Vec::new();
        for validator in &self.validators {
            match validator.node.get_node_epoch().await {
                Ok(epoch) => epochs.push((validator.account.staker_address, epoch)),
                Err(e) => {
                    error!(
                        "Failed to get node epoch for validator {:?}: {}",
                        validator.account.staker_address, e
                    );
                }
            }
        }
        Ok(epochs)
    }

    /// Builds a new validator node, starts it if not specified to be asleep, and then waits for it to sync up to the chain. Defaults to realm id 1.
    pub async fn add_one(
        &mut self,
        is_asleep: bool,
        build_mode: Option<BuildMode>,
        realm_id: Option<U256>,
    ) -> Result<&Validator> {
        // get the next node account from the testnet
        let validator_idx = self.validators.len();
        let node_account = &self.testnet_node_accounts[validator_idx];
        let node_config_file_path = format!(
            "{}/lit_config{:?}.toml",
            self.node_config_folder_path, validator_idx
        );

        // build the validator
        let mut validator = ValidatorBuilder::default()
            .build_mode(build_mode)
            .build(
                node_config_file_path,
                node_account,
                self.testnet_deployer_signing_provider.clone(),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to build validator with error: {}", e))?;

        // start the validator if not specified to be asleep
        if !is_asleep {
            validator
                .start_node(true, true)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to start validator with error: {}", e))?;
        }

        // the validator requests to join the next validator set
        if let Some(realm_id) = realm_id {
            validator
                .request_to_join(&self.actions, realm_id)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to request to join with error: {}", e))?;
            // Check that all the nodes have synced up to chain.
            for keyset_config in &self.keyset_configs {
                self.actions
                    .wait_for_root_keys(realm_id, Some(keyset_config.identifier.clone()))
                    .await;
            }
        }

        self.validators.push(validator);

        Ok(&self.validators[validator_idx])
    }

    /// Builds a new validator node, starts it if not specified to be asleep, and then waits for it to sync up to the chain,
    /// using custom node config file path and node account.
    pub async fn add_one_custom(
        &mut self,
        is_asleep: bool,
        node_config_file_path: String,
        node_account: &NodeAccount,
        build_mode: Option<BuildMode>,
        realm_id: u64,
    ) -> Result<&Validator> {
        // build the validator
        let mut validator = ValidatorBuilder::default()
            .build_mode(build_mode)
            .build(
                node_config_file_path,
                node_account,
                self.testnet_deployer_signing_provider.clone(),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to build validator with error: {}", e))?;

        // start the validator if not specified to be asleep
        if !is_asleep {
            validator
                .start_node(true, true)
                .await
                .map_err(|e| anyhow::anyhow!("Failed to start validator with error: {}", e))?;
        }

        // the validator requests to join the next validator set
        validator
            .request_to_join(&self.actions, U256::from(realm_id))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to request to join with error: {}", e))?;

        // Check that all the nodes have synced up to chain.
        let realm_id = U256::from(realm_id);
        for keyset_config in &self.keyset_configs {
            self.actions
                .wait_for_root_keys(realm_id, Some(keyset_config.identifier.clone()))
                .await;
        }

        self.validators.push(validator);

        Ok(&self.validators[self.validators.len() - 1])
    }

    pub async fn ensure_all_nodes_started(&mut self) -> Result<()> {
        for validator in &mut self.validators {
            if validator.node.is_offline() {
                validator.start_node(true, true).await?;
            }
        }

        Ok(())
    }

    pub async fn ensure_all_nodes_awake(ports: Vec<usize>) -> Result<()> {
        info!("Waiting for ports to be awake: {:?}", ports);

        let mut futures = Vec::new();
        for port in ports {
            futures.push(tokio::spawn(Node::wait_for_node_awake(port)));
        }

        let _l = join_all(futures).await;

        info!("All nodes are awake");
        Ok(())
    }

    pub async fn start_node(&mut self, idx: usize) -> Result<()> {
        self.validators[idx].start_node(false, true).await
    }

    pub async fn start_node_from_clean_slate(&mut self, idx: usize) -> Result<()> {
        self.validators[idx].start_node(true, true).await
    }

    pub async fn start_node_by_port(&mut self, port: &str) -> Result<()> {
        let idx = self
            .validators
            .iter()
            .position(|v| v.node.port.to_string() == port)
            .expect_or_err("Failed to find validator with this port")?;
        self.validators[idx].start_node(false, true).await
    }

    pub fn stop_all_nodes(&mut self) -> Result<()> {
        for validator in &mut self.validators {
            validator.node.stop_node_process()?;
        }

        Ok(())
    }

    pub fn stop_nodes_with_accounts(&mut self, accounts: Vec<NodeAccount>) -> Result<()> {
        // first find the indices of the validators with these accounts
        let indices = accounts
            .iter()
            .map(|account| {
                self.validators
                    .iter()
                    .position(|v| v.account == *account)
                    .expect("Failed to find validator with this account")
            })
            .collect::<Vec<_>>();

        // then, stop the node process for each of these accounts
        for idx in indices {
            self.validators[idx].node.stop_node_process()?;
        }

        Ok(())
    }

    pub async fn stop_node(&mut self, idx: usize) -> Result<()> {
        info!("Stopping node at index: {}", idx);
        self.validators[idx].node.stop_node_process()
    }

    pub async fn stop_random_node(&mut self) -> Result<usize> {
        let mut rng = crate::rand::thread_rng();
        let random_node_idx_to_shutdown = rng.gen_range(0..self.size());
        info!("Stopping node at index: {}", random_node_idx_to_shutdown);
        self.validators[random_node_idx_to_shutdown]
            .node
            .stop_node_process()?;
        Ok(random_node_idx_to_shutdown)
    }

    pub async fn stop_node_by_port(&mut self, port: &str) -> Result<()> {
        let idx = self
            .validators
            .iter()
            .position(|v| v.node.port.to_string() == port)
            .expect_or_err("Failed to find validator with this port")?;
        info!("Stopping node at index: {}", idx);
        self.validators[idx].node.stop_node_process()
    }

    pub async fn get_active_validators(&self) -> Result<Vec<&Validator>> {
        self.get_active_validators_with_realm_id(1).await
    }

    pub async fn get_active_validators_with_realm_id(&self, realm: u64) -> Result<Vec<&Validator>> {
        let realm_id = U256::from(realm);
        let current_active_validators = self.actions.get_current_validators(realm_id).await;

        Ok(self
            .validators
            .iter()
            .filter(|v| current_active_validators.contains(&v.account.staker_address))
            .collect::<Vec<_>>())
    }

    pub async fn get_inactive_validators(&self) -> Result<Vec<&Validator>> {
        let realm_id = U256::from(1);
        let current_active_validators = self.actions.get_current_validators(realm_id).await;

        Ok(self
            .validators
            .iter()
            .filter(|v| !current_active_validators.contains(&v.account.staker_address))
            .collect::<Vec<_>>())
    }

    pub async fn random_validators_request_to_leave(
        &self,
        num_dealt_out: usize,
    ) -> Result<Vec<&Validator>> {
        // first get the current active validators
        let current_active_validators = self.get_active_validators().await?;

        // randomly choose validators from this list
        let random_validators_to_leave =
            choose_random_nums_in_range(num_dealt_out, 0, current_active_validators.len());
        let random_validators_to_leave = random_validators_to_leave
            .iter()
            .map(|i| current_active_validators[*i])
            .collect::<Vec<_>>();

        // for each of these validators, request to leave
        for validator in &random_validators_to_leave {
            validator.request_to_leave(&self.actions).await?;
        }

        Ok(random_validators_to_leave)
    }

    pub async fn random_validators_request_to_join(
        &mut self,
        num_dealt_in: usize,
        realm_id: u64,
    ) -> Result<Vec<&Validator>> {
        let realm_id = U256::from(realm_id);
        // scoped block so the immutable borrow is within it, and we can continue the mutable borrow outside of this block below it.
        let random_validators_to_join = {
            // first get the current inactive validators
            let current_inactive_validators = self.get_inactive_validators().await?;

            // randomly choose validators from this list
            let random_validators_to_join =
                choose_random_nums_in_range(num_dealt_in, 0, current_inactive_validators.len());
            // find which indices they belong to in the original list in self
            random_validators_to_join
                .iter()
                .map(|i| {
                    self.validators
                        .iter()
                        .position(|v| v.account == current_inactive_validators[*i].account)
                        .unwrap()
                })
                .collect::<Vec<_>>()
        };

        // for each of these validators, first spin up the node (remember, once the validator has joined in the contract,
        // they are assumed to already be online as its peers will be sending them messages)
        for idx in random_validators_to_join.clone() {
            let validator = self.validators[idx].borrow_mut();
            validator.start_node(false, true).await?;
        }

        // even after the nodes awake, we need to give the rest of the network time to recognize them.
        self.actions().sleep_millis(2000).await;
        // as soon as the chose to join, other nodes can kick them.

        // for each of these validators, request to join
        for idx in random_validators_to_join.clone() {
            let validator = &self.validators[idx];
            validator.request_to_join(&self.actions, realm_id).await?;
        }

        // likewise, we need to give the network time to recognize the new nodes.
        self.actions().sleep_millis(2000).await;

        Ok(self
            .validators
            .iter()
            .enumerate()
            .filter(|(i, _)| random_validators_to_join.contains(i))
            .map(|(_, v)| v)
            .collect::<Vec<_>>())
    }

    pub async fn random_threshold_nodeset(&self) -> Vec<NodeSet> {
        self.random_threshold_nodeset_with_realm_id(1, &vec![])
            .await
    }

    pub async fn partially_random_threshold_nodeset(
        &self,
        validators_to_include: &Vec<&Validator>,
    ) -> Vec<NodeSet> {
        self.random_threshold_nodeset_with_realm_id(1, validators_to_include)
            .await
    }

    pub async fn random_threshold_nodeset_with_realm_id(
        &self,
        realm: u64,
        validators_to_include: &Vec<&Validator>,
    ) -> Vec<NodeSet> {
        let mut rng = crate::rand::thread_rng();

        let realm_id = U256::from(realm);

        let kicked = self
            .actions()
            .contracts()
            .staking
            .get_kicked_validators(realm_id)
            .call()
            .await
            .unwrap_or_else(|_e| vec![]);

        let ports = self
            .actions
            .get_current_validator_structs(realm_id)
            .await
            .into_iter()
            .filter(|f| !kicked.contains(&f.node_address))
            .map(|v| v.port as usize)
            .collect::<Vec<usize>>();

        let mut nodes_for_epoch: Vec<String> = self
            .get_active_validators_with_realm_id(realm)
            .await
            .unwrap()
            .into_iter()
            .filter(|f| ports.contains(&f.node.port))
            .map(|v| v.node_address())
            .collect();

        let nodes_for_epoch2 = nodes_for_epoch.clone();

        let threshold = self
            .actions
            .contracts()
            .staking
            .current_validator_count_for_consensus(realm_id)
            .await
            .unwrap()
            .as_usize();

        let epoch = self.actions().get_current_epoch(realm_id).await.as_u64();
        // this was using ports.len()
        // let threshold = std::cmp::min(nodes_for_epoch.len(), self.threshold(ports.len()));

        let mut node_set: Vec<NodeSet> = Vec::with_capacity(threshold);

        // if we are including validators, we need to add the validators to the node set and reduce the number of remaining nodes to add
        let validators_to_add = threshold - validators_to_include.len();

        // add the specific validators to the node set - this is generally used for fault tests, and remove from the list to choose the remaining nodes
        for validator in validators_to_include {
            node_set.push(NodeSet {
                socket_address: validator.node_address(),
                value: 1,
            });

            nodes_for_epoch.retain(|node| node != &validator.node_address());
        }

        for _ in 0..validators_to_add {
            let random_node = nodes_for_epoch.remove(rng.gen_range(0..nodes_for_epoch.len()));
            let random_node_set = NodeSet {
                socket_address: random_node,
                value: 1,
            };
            node_set.push(random_node_set);
        }

        debug!(
            "All nodes / online nodes (epoch {}): {} / {} and threshold: {}, and nodeset (l:{}): {:?}",
            epoch,
            self.validators.len(),
            nodes_for_epoch2.len(),
            threshold,
            node_set.len(),
            &node_set
        );

        node_set
    }

    pub fn complete_node_set(&self) -> Vec<NodeSet> {
        self.validators
            .iter()
            .map(|v| NodeSet {
                socket_address: v.public_address(),
                value: 1,
            })
            .collect()
    }
}

impl Drop for ValidatorCollection {
    fn drop(&mut self) {
        info!("Stopping processes");
        self.stop_all_nodes().expect("Failed to stop nodes");
    }
}

pub struct ValidatorBuilder {
    node_binary_feature_flags: String,
    build_mode: Option<BuildMode>,
    realm_id: Option<U256>,
}

impl Default for ValidatorBuilder {
    fn default() -> Self {
        Self {
            node_binary_feature_flags: "lit-actions,testing".into(),
            build_mode: None,
            realm_id: None,
        }
    }
}

impl ValidatorBuilder {
    pub fn node_binary_feature_flags(mut self, node_binary_feature_flags: String) -> Self {
        self.node_binary_feature_flags = node_binary_feature_flags;
        self
    }

    pub fn build_mode(mut self, build_mode: Option<BuildMode>) -> Self {
        self.build_mode = build_mode;
        self
    }

    pub fn realm_id(mut self, realm_id: u64) -> Self {
        self.realm_id = Some(U256::from(realm_id));
        self
    }

    pub async fn build(
        self,
        node_config_file_path: String,
        node_account: &NodeAccount,
        deployer_signing_provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    ) -> Result<Validator> {
        // check and top up node funds if necessary
        ensure_node_account_funds(
            deployer_signing_provider,
            node_account.signing_provider.clone(),
        )
        .await?;

        let binary_path = match self.build_mode.unwrap_or(BuildMode::UseNewOrCachedBuild) {
            BuildMode::UseNewOrCachedBuild => Node::get_binary(
                self.node_binary_feature_flags.clone(),
                node_config_file_path.clone(),
            )?,
            BuildMode::UseCustomBuild(binary_path) => {
                info!("Using custom binary path: {}", binary_path);
                binary_path
            }
        };

        return Ok(Validator {
            node: NodeBuilder::new()
                .realm_id(self.realm_id)
                .binary_path(binary_path)
                .build(node_config_file_path)
                .await?,
            account: node_account.clone(),
        });
    }
}

#[derive(Debug)]
pub struct Validator {
    node: Node,
    account: NodeAccount,
}

impl Validator {
    fn log_reader(&self) -> BufReader<File> {
        let path = format!(
            "./tests/test_logs/0x{}",
            bytes_to_hex(self.account.staker_address.as_bytes())
        );
        debug!("Trying to open path: {}", path);
        let log_path = PathBuf::from(path);
        let file = File::open(log_path).expect("Failed to open log file");
        BufReader::new(file)
    }

    pub fn account(&self) -> &NodeAccount {
        &self.account
    }

    pub fn realm_id(&self) -> U256 {
        self.node.realm_id
    }

    pub fn public_address(&self) -> String {
        self.node.ip.to_string() + ":" + &self.node.port.to_string()
    }

    pub fn node_address(&self) -> String {
        self.node.ip.to_string() + ":" + &self.node.port.to_string()
    }

    pub async fn start_node(&mut self, clean_slate: bool, wait_for_node_awake: bool) -> Result<()> {
        if clean_slate {
            // remove the validator-specific files
            trace!(
                "Cleaning environment of any files for node: {}",
                self.account.staker_address
            );
            remove_files(format!(
                "0x{}",
                bytes_to_hex(self.account.staker_address.as_bytes())
            ));
        }

        // start the node
        self.node
            .start_node_process()
            .map_err(|e| anyhow::anyhow!("Failed to start node with error: {}", e))?;

        if wait_for_node_awake {
            // check the node is awake
            Node::wait_for_node_awake(self.node.port)
                .await
                .map_err(|e| {
                    anyhow::anyhow!("Failed to wait for node to wake up with error: {}", e)
                })?;
        }

        Ok(())
    }

    pub fn stop_node(&mut self) -> Result<()> {
        info!("Stopping node at port {}", self.node.port);
        self.node
            .stop_node_process()
            .map_err(|e| anyhow::anyhow!("Failed to stop node with error: {}", e))
    }

    pub async fn request_to_leave(&self, actions: &Actions) -> Result<()> {
        info!(
            "Node {} ({}) requesting to leave",
            self.node.port, self.account.staker_address
        );

        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            actions.contracts().staking.address(),
            self.account.signing_provider.clone(),
        );

        let cc = staking.request_to_leave();
        Contracts::process_contract_call(cc, "request to leave").await;

        Ok(())
    }

    pub async fn request_to_join(&self, actions: &Actions, realm_id: U256) -> Result<()> {
        info!(
            "Node {} (s:{} / n:{}) requesting to join realm {}",
            self.node.port, self.account.staker_address, self.account.node_address, realm_id
        );

        let staking = Staking::<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>::new(
            actions.contracts().staking.address(),
            self.account.signing_provider.clone(),
        );

        let cc = staking.request_to_join(realm_id);
        Contracts::process_contract_call_with_delay(cc, "request to join", 10).await;

        Ok(())
    }

    // pub async fn set_node_info_without_joining(&self, actions: &Actions) -> Result<()> {
    //     info!(
    //         "Node {} (s:{} / n:{}) is updating ip/port/details info.",
    //         self.node.port, self.account.staker_address, self.account.node_address,
    //     );
    //
    //     let staking = Staking::<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>::new(
    //         actions.contracts().staking.address(),
    //         self.account.signing_provider.clone(),
    //     );
    //
    //     let cc = staking.set_ip_port_node_address_and_communication_pub_keys(
    //         self.node.ip.into(),
    //         0,
    //         self.node.port as u32,
    //         self.account.node_address,
    //         U256::from(self.account.coms_keys.sender_public_key().to_bytes()),
    //         U256::from(self.account.coms_keys.receiver_public_key().to_bytes()),
    //     );
    //     Contracts::process_contract_call_with_delay(cc, "update node info without joining", 10)
    //         .await;
    //
    //     Ok(())
    // }

    pub fn is_node_offline(&mut self) -> bool {
        if let Some(child) = self.node.process.as_mut() {
            if child.try_wait().is_ok() {
                self.node.process = None;
                true
            } else {
                false
            }
        } else {
            true
        }
    }
}

#[must_use]
pub struct NodeBuilder {
    asleep_initially: bool,
    binary_path: Option<String>,
    log_mode: String,
    extra_env_vars: Vec<(String, String)>,
    realm_id: Option<U256>,
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeBuilder {
    pub fn new() -> Self {
        let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string()) == "1";
        let is_perf_test = std::env::var("LIT_PERFORMANCE_TEST").unwrap_or("0".to_string()) == "1";

        Self {
            asleep_initially: false,
            binary_path: None,
            log_mode: in_github_ci
                .then(|| "_=warn,lit_node=debug,lit_actions=debug".to_string())
                .unwrap_or(Self::get_log_mode()),
            extra_env_vars: is_perf_test
                .then(|| vec![("LIT_LOGGING_JAEGER".to_string(), "1".to_string())])
                .unwrap_or(vec![]),
            realm_id: None,
        }
    }

    fn get_log_mode() -> String {
        trace!("Getting logging levels for testing...");
        let default = "_=warn,lit_node=debug,lit_actions=debug".to_string();

        use std::fs;
        use std::path::Path;
        use toml_edit::DocumentMut;

        let mut rust_log_level = "_=warn".to_string(); // the default log level;

        let toml_path = Path::new("log_levels.toml");
        let toml_contents = fs::read_to_string(toml_path).unwrap_or_default();
        if toml_contents.is_empty() {
            info!("No log levels found in log_levels.toml, using 'debug' and 'warn' as defaults.");
            return default;
        }

        let toml_document = match toml_contents.parse::<DocumentMut>() {
            Ok(doc) => doc,
            Err(e) => {
                info!(
                    "Failed to parse log_levels.toml: {}. Using default log levels.",
                    e
                );
                return default;
            }
        };

        let modules: Vec<String> = toml_document
            .as_table()
            .iter()
            .map(|(module, _)| module.to_string())
            .collect();

        // info!("Found these logging levels in log_levels.toml: {:?}", modules);
        for module in modules {
            if let Some(ns_configs) = toml_document[&module].as_table() {
                for (ns, level) in ns_configs.iter() {
                    let log_level = level.to_string();
                    // remove the comment if it exists
                    let log_level = match log_level.find('#') {
                        Some(index) => log_level[..index].trim(),
                        None => log_level.trim(),
                    };
                    rust_log_level += &format!(",{}::{}={}", module, ns.trim(), log_level)
                        .replace("::_=", "=")
                        .replace("\"", "");
                }
            } else {
                info!("No log level set for {}", module);
            }
        }

        info!("Log levels: {}", rust_log_level);
        rust_log_level
    }

    pub fn asleep_initially(mut self, asleep_initially: bool) -> Self {
        self.asleep_initially = asleep_initially;
        self
    }

    pub fn binary_path(mut self, binary_path: String) -> Self {
        self.binary_path = Some(binary_path);
        self
    }

    pub fn log_mode(mut self, log_mode: String) -> Self {
        self.log_mode = log_mode;
        self
    }

    pub fn extra_env_vars(mut self, extra_env_vars: Vec<(String, String)>) -> Self {
        self.extra_env_vars = extra_env_vars;
        self
    }

    pub fn realm_id(mut self, realm_id: Option<U256>) -> Self {
        self.realm_id = realm_id;
        self
    }

    pub async fn build(self, config_file: String) -> Result<Node> {
        // if we're in CI, it's already built and in the root
        let path = self
            .binary_path
            .unwrap_or("./target/debug/lit_node".to_string());
        debug!("Config file: {}", config_file);
        debug!("Binary path: {}", path);
        // get the actual node port from the config file
        let node_config = SimpleToml::try_from(Path::new(&config_file))
            .map_err(|e| anyhow::anyhow!("Failed to load config file: {}", e))?;
        debug!("Node config: {:?}", node_config.data());
        let node_external_port = Node::get_node_port_from_config_file(&config_file)?;
        let node_ip = Node::get_node_ip_from_config_file(&config_file)?;

        trace!("Node port: {}", node_external_port);
        Ok(Node {
            realm_id: self.realm_id.unwrap_or_else(|| U256::from(1)),
            process: None,
            config_file,
            binary_path: path,
            log_mode: self.log_mode,
            extra_env_vars: self.extra_env_vars,

            port: node_external_port,
            ip: node_ip,
        })
    }
}

pub struct Node {
    process: Option<Child>,
    config_file: String,
    binary_path: String,
    log_mode: String,
    extra_env_vars: Vec<(String, String)>,

    // convenience things
    port: usize,
    ip: Ipv4Addr,
    realm_id: U256,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("config_file", &self.config_file)
            .field("binary_path", &self.binary_path)
            .field("log_mode", &self.log_mode)
            .field("extra_env_vars", &self.extra_env_vars)
            .field("port", &self.port)
            .field("ip", &self.ip)
            .field("realm_id", &self.realm_id)
            .finish()
    }
}

impl Node {
    pub fn realm_id(&self) -> U256 {
        self.realm_id
    }

    pub fn binary_path(&self) -> &str {
        &self.binary_path
    }

    pub fn ip(&self) -> Ipv4Addr {
        self.ip
    }

    pub fn port(&self) -> usize {
        self.port
    }

    pub fn is_offline(&self) -> bool {
        self.process.is_none()
    }

    fn start_node_process(&mut self) -> Result<()> {
        if self.process.is_some() {
            warn!("Node {} is already online", self.port);
            return Ok(());
        }

        info!(
            "Starting node at: {} - port: {}",
            self.binary_path, self.port
        );
        debug!(
            "Current working directory: {:?}",
            env::current_dir().unwrap()
        );
        let mut process = Command::new(&self.binary_path)
            .env(ENV_LIT_CONFIG_FILE, &self.config_file)
            .env("RUST_LOG", &self.log_mode)
            .env(ENV_ATTESTATION_TYPE_OVERRIDE, "ADMIN_SIGNED")
            .env(
                ENV_LOGGING_TIMESTAMP,
                std::env::var(ENV_LOGGING_TIMESTAMP).unwrap_or("0".to_string()),
            )
            .envs(self.extra_env_vars.clone())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to launch node with error: {}", e))?;

        // Check if the process has exited due to some failure using the specified binary, after waiting for 1000ms.
        std::thread::sleep(std::time::Duration::from_millis(1000));
        if let Some(exit_status) = process.try_wait()? {
            return Err(anyhow::anyhow!(
                "Node process could not be started, exit code: {:?}",
                exit_status
            ));
        }

        self.process = Some(process);
        Ok(())
    }

    fn stop_node_process(&mut self) -> Result<()> {
        let process = match self.process.as_mut() {
            Some(process) => process,
            None => {
                debug!("Node is already offline");
                return Ok(());
            }
        };

        let pid = process.id();
        info!("Killing PID: {}, PORT: {}", pid, self.port);

        // just kill the nodes - don't wait for them to stop
        process
            .kill()
            .map_err(|e| anyhow::anyhow!("Failed to kill node with error: {}", e))?;
        process
            .wait()
            .map_err(|e| anyhow::anyhow!("Failed to wait for child PID {pid}: {}", e))?;
        info!("process at port {} was killed", self.port);

        self.process = None;

        Ok(())
    }

    pub async fn wait_for_node_awake(port: usize) -> Result<()> {
        // loop until the node is awake
        let mut node_awake = false;
        while !node_awake {
            node_awake = Self::check_node_awake(port).await?;
            if !node_awake {
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            }
        }
        info!("Node {} is responding", port);

        Ok(())
    }

    pub async fn check_node_awake(port: usize) -> Result<bool> {
        let response = Self::handshake(port).await;

        if response.is_err() {
            debug!(
                "Checking for liveness; node {} is not responding. Err: {:?}",
                port,
                response.err().unwrap().source().unwrap().to_string()
            );
            return Ok(false);
        };

        let response = response?;

        if response.status() != 200 {
            info!(
                "Node {} is responding, but not ready. Status: {:?}",
                port,
                response.status()
            );
            return Ok(false);
        }

        let response_text = response.text().await?;

        warn!("Response from node {}: {}", port, response_text);

        Ok(true)
    }

    async fn handshake(port: usize) -> Result<reqwest::Response, reqwest::Error> {
        let request_id = &uuid::Uuid::new_v4().to_string();
        let cmd = "/web/handshake".to_string();
        let json_body = r#"{"clientPublicKey":"blah","challenge":"0x1234123412341234123412341234123412341234123412341234123412341234"}"#.to_string();
        let client = reqwest::Client::new();

        client
            .post(format!("http://127.0.0.1:{}/{}", port, cmd))
            .header("Content-Type", "application/json")
            .header("X-Request-Id", request_id)
            .body(json_body.clone())
            .send()
            .await
    }

    pub async fn get_node_epoch(&self) -> Result<u64> {
        let port = self.port;
        let response = Self::handshake(port).await?;
        let response_text = response.text().await?;

        let handshake_json = serde_json::from_str::<SDKHandshakeResponseV0>(&response_text)?;

        Ok(handshake_json.epoch)
    }

    fn get_node_config_from_file(config_file: &str) -> Result<SimpleToml> {
        SimpleToml::try_from(Path::new(config_file))
            .map_err(|e| anyhow::anyhow!("Failed to load config file: {}", e))
    }

    pub fn get_node_port_from_config_file(config_file: &str) -> Result<usize> {
        // get the actual node port from the config file
        let node_config = Self::get_node_config_from_file(config_file)?;
        node_config
            .get("node.http", "port")
            .ok_or_else(|| anyhow::anyhow!("Failed to get port from config file"))?
            .parse::<usize>()
            .map_err(|e| anyhow::anyhow!("Failed to parse port from config file: {}", e))
    }

    pub fn get_node_ip_from_config_file(config_file: &str) -> Result<Ipv4Addr> {
        // get the actual node port from the config file
        let node_config = Self::get_node_config_from_file(config_file)?;
        let node_ip = node_config
            .get("node", "domain")
            .ok_or_else(|| anyhow::anyhow!("Failed to get ip from config file"))?
            .parse::<String>()
            .map_err(|e| anyhow::anyhow!("Failed to parse ip from config file: {}", e))?;
        node_ip
            .parse::<Ipv4Addr>()
            .map_err(|e| anyhow::anyhow!("Failed to parse ip from config file: {}", e))
    }

    pub fn get_node_network_addr_from_config_file(config_file: &str) -> Result<String> {
        // get the actual node networking address from the config file
        let _node_config = Self::get_node_config_from_file(config_file)?;
        let node_port = Self::get_node_port_from_config_file(config_file)?;
        let node_ip = Self::get_node_ip_from_config_file(config_file)?;

        Ok(format!("http://{}:{}", node_ip, node_port))
    }

    pub fn build_binary(feature_flags: String) -> Result<bool> {
        // manifest path looks a bit funny, but it's correct in order to support Shiva building the binary ;-)
        let args = [
            "build",
            "--manifest-path",
            "../lit-node/Cargo.toml",
            "--features",
            &feature_flags,
        ];

        let start = std::time::Instant::now();
        info!(
            "Building node code with these args : {:?}  - please be patient.",
            &args
        );

        let test_run_dir = "./target/test-run/";
        // let test_run_dir = "./target/";
        if !std::path::Path::new(test_run_dir).exists() {
            std::fs::create_dir_all(test_run_dir).unwrap();
        }

        let build_command = Command::new("cargo")
            .env("CARGO_TARGET_DIR", test_run_dir)
            .args(args)
            .output()
            .expect("Failed to compile node with cargo build");
        assert!(
            build_command.status.success(),
            "We failed to build the node with error: {:?}",
            String::from_utf8(build_command.stderr).unwrap()
        );

        info!(
            "Building took {} ms.  Starting nodes.  Env: {:?}",
            start.elapsed().as_millis(),
            std::env::current_dir()
        );

        Ok(true)
    }

    pub fn get_binary(feature_flags: String, node_config_file_path: String) -> Result<String> {
        let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string()) == "1";

        // We copy the binary to a dedicated location. This is necessary because the existing binary cannot be overwritten,
        // especially when the node is running in a CI context.
        let node_external_port = Self::get_node_port_from_config_file(&node_config_file_path)?;
        let from_binary_path = match in_github_ci {
            true => "./target/debug/lit_node",
            false => "./target/test-run/debug/lit_node",
        };

        // when we run tests locally, we rebuild the binary ( < 1.5s if it's clean ), but in CI we use the pre-built binary.
        if !in_github_ci {
            if !std::path::Path::new(from_binary_path).exists() {
                info!("Binary not found at: {}, building...", from_binary_path);
                let _built = Self::build_binary(feature_flags)?;

                if !std::path::Path::new(from_binary_path).exists() {
                    return Err(anyhow::anyhow!("Failed to build binary"));
                }
            }
        }

        let to_binary_path = &format!("{}_{}", from_binary_path, node_external_port);

        debug!(
            "Loading pre-built binary from: {} and copying to: {}",
            from_binary_path, to_binary_path
        );

        // technically we could probably skip this, but it's possible that we hard-linked to a different binary in the past.
        if std::path::Path::new(to_binary_path).exists() {
            std::fs::remove_file(to_binary_path).unwrap();
        }

        // hard link the binary
        std::fs::hard_link(from_binary_path, to_binary_path)
            .map_err(|e| anyhow::anyhow!("Failed to hard link binary with error: {}", e))?;

        Ok(to_binary_path.to_string())
    }
}

#[derive(Debug)]
pub enum BuildMode {
    UseNewOrCachedBuild,
    UseCustomBuild(String), // path to the binary
}

pub fn remove_files(staker_address: String) {
    // logs may or may not exist, depending on the test & settings
    let path = format!("./tests/test_logs/{}", staker_address);
    match fs::remove_file(&path) {
        Ok(_) => {
            trace!("Removed file: {} for staker {}", path, staker_address);
        }
        Err(e) => {
            trace!(
                "Failed to remove file: {} for staker {} with error {}",
                path, staker_address, e
            );
        }
    }
}

pub fn remove_node_keys(staker_address: String) {
    let path = format!("./node_keys/{}", staker_address);
    match fs::remove_dir_all(&path) {
        Ok(_) => {
            info!("Removed file: {} for staker {}", path, staker_address);
        }
        Err(e) => {
            warn!(
                "Failed to remove file: {} for staker {} with error {}",
                path, staker_address, e
            );
        }
    }
}

async fn ensure_node_account_funds(
    deployer_signing_provider: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
    node_account: Arc<SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>>,
) -> Result<bool> {
    let deployer_balance = deployer_signing_provider
        .get_balance(deployer_signing_provider.address(), None)
        .await?;
    let node_balance = deployer_signing_provider
        .get_balance(node_account.address(), None)
        .await?;

    let min_balance: U256 = 1_000_000_000_000_000u128.into(); // .01 MATIC / ETH, whatever

    if deployer_balance < min_balance * 10 {
        return Err(anyhow::anyhow!(
            "Deployer balance {} is too low to top up node {}",
            deployer_balance,
            node_account.address()
        ));
    }

    if node_balance > min_balance {
        trace!(
            "Node {} balance: {} - acceptable!",
            node_account.address(),
            node_balance
        );
        return Ok(true);
    }

    info!(
        "Node {} balance {} is low.  Sending funds from deployer who has {}.",
        node_account.address(),
        node_balance,
        deployer_balance,
    );

    let tx = TransactionRequest::new()
        .to(node_account.address())
        .value(min_balance * 2)
        .from(deployer_signing_provider.address());

    let tx_hash = deployer_signing_provider.send_transaction(tx, None).await?;
    let _receipt = tx_hash.await?;

    let node_balance = deployer_signing_provider
        .get_balance(node_account.address(), None)
        .await?;

    info!(
        "Node {} topped up -  balance {}",
        node_account.address(),
        node_balance
    );
    Ok(true)
}

fn choose_random_nums_in_range(random_nums: usize, min: usize, max: usize) -> Vec<usize> {
    if random_nums > max - min {
        panic!(
            "Cannot choose {} random numbers in range {} to {}",
            random_nums, min, max
        );
    }

    debug!(
        "Choosing {} random numbers in range {} to {}",
        random_nums, min, max
    );
    let mut rng = crate::rand::thread_rng();
    let mut random_nums_in_range = vec![];
    while random_nums_in_range.len() < random_nums {
        let random_num = rng.gen_range(min..max);
        if !random_nums_in_range.contains(&random_num) {
            random_nums_in_range.push(random_num);
        }
    }

    random_nums_in_range
}

pub fn get_default_keyset_configs() -> Vec<KeySetConfig> {
    vec![default_keyset_config()]
}
pub fn default_keyset_config() -> KeySetConfig {
    KeySetConfig {
        identifier: DEFAULT_KEY_SET_NAME.to_string(),
        description: String::new(),
        minimum_threshold: 3,
        monetary_value: 0,
        complete_isolation: false,
        realms: vec![U256::from(1)],
        curves: CurveType::into_iter().map(|c| c.into()).collect(),
        counts: std::iter::once(U256::from(1))
            .chain(CurveType::into_iter().skip(1).map(|_| U256::from(2)))
            .collect(),
        recovery_session_id: Bytes::from_static(&[]),
    }
}
