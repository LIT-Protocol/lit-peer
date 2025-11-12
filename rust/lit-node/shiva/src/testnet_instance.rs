use std::{
    env,
    path::{Path, PathBuf},
    process::{Child, Command},
};

use anyhow::anyhow;
use ethers::types::U256;
use lit_node_testnet::validator::ValidatorCollection;
use tracing::{info, warn};

use crate::models::{ContractAbis, ContractAddresses, TestNetCreateParams, TestNetState};

use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::testnet::contracts::StakingContractRealmConfig;
use lit_node_testnet::testnet::{TestnetContracts, actions};

// Custom impl to avoid `From<T>` trait as it requires borrowing which we do not want as we cannot brrow from the runtime context
impl ContractAbis {
    pub fn new(contracts: &TestnetContracts) -> Result<Self, anyhow::Error> {
        let lit_token = serde_json::to_string(contracts.contracts().lit_token.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let erc20 = serde_json::to_string(contracts.contracts().erc20.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let backup_recovery =
            serde_json::to_string(contracts.contracts().backup_recovery.abi()).unwrap();
        let staking = serde_json::to_string(contracts.contracts().staking.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let pkpnft = serde_json::to_string(contracts.contracts().pkpnft.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let pubkey_router = serde_json::to_string(contracts.contracts().pubkey_router.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let pkp_helper = serde_json::to_string(contracts.contracts().pkp_helper.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let pkp_permissions = serde_json::to_string(contracts.contracts().pkp_permissions.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;

        let contract_resolver =
            serde_json::to_string(contracts.contracts().contract_resolver.abi())
                .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        let payment_delegation =
            serde_json::to_string(contracts.contracts().payment_delegation.abi())
                .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;

        Ok(Self {
            lit_token,
            erc20,
            backup_recovery,
            staking,
            pkpnft,
            pkp_helper,
            pkp_permissions,
            pubkey_router,
            contract_resolver,
            payment_delegation,
        })
    }
}

impl ContractAddresses {
    pub fn new(addresses: &lit_node_testnet::testnet::contracts::ContractAddresses) -> Self {
        Self {
            lit_token: format!("{:#x}", addresses.lit_token),
            backup_recovery: format!("{:#x}", addresses.backup_recovery),
            staking: format!("{:#x}", addresses.staking),
            pkpnft: format!("{:#x}", addresses.pkpnft),
            pubkey_router: format!("{:#x}", addresses.pubkey_router),
            pkp_permissions: format!("{:#x}", addresses.pkp_permissions),
            pkp_helper: format!("{:#x}", addresses.pkp_helper),
            contract_resolver: format!("{:#x}", addresses.contract_resolver),
            key_deriver: format!("{:#x}", addresses.key_deriver),
            payment_delegation: format!("{:#x}", addresses.payment_delegation),
        }
    }
}

pub struct TestnetInstance {
    pub id: String,
    pub node_count: usize,
    pub polling_interval: String,
    pub epoch_length: i32,
    pub exisiting_config_path: Option<String>,
    pub ecdsa_round_timeout: Option<String>,
    pub enable_payment: Option<String>,
    pub action_server: Option<Child>,
    pub actions: actions::Actions,
    pub test_net: Testnet,
    pub contracts: lit_node_testnet::testnet::TestnetContracts,
    pub validators: lit_node_testnet::validator::ValidatorCollection,
    pub state: TestNetState,
}

impl TestnetInstance {
    pub async fn init(params: TestNetCreateParams) -> Result<Self, anyhow::Error> {
        let _dir_guard = WorkingDirGuard::enter_workspace()?;
        let mut lit_action_process: Option<Child> = None;
        if params.custom_build_path.is_some()
            && params.lit_action_server_custom_build_path.is_some()
        {
            let lit_action_path = match params.clone().lit_action_server_custom_build_path {
                Some(path) => path,
                None => {
                    // TODO add way of signaling to parent we have experienced a startup error and we should not continue with setup.
                    return Err(anyhow::anyhow!(
                        "Can not specify one prebuilt binary, must provide both lit action and lit node"
                    ));
                }
            };

            info!("Spawning Lit Action server at path: {}", lit_action_path);

            let lit_action_server = Command::new(lit_action_path)
                .spawn()
                .map_err(|e| anyhow::anyhow!("Error while spawning lit action server: {}", e))?;
            lit_action_process = Some(lit_action_server);
        }

        let mut testnet = Testnet::builder()
            .num_staked_and_joined_validators(params.node_count)
            .build()
            .await;
        let testnet_contracts = Testnet::setup_contracts(
            &mut testnet,
            None,
            Some(
                StakingContractRealmConfig::builder()
                    .epoch_length(Some(U256::from(params.epoch_length)))
                    .build(),
            ),
        )
        .await
        .map_err(|e| anyhow::anyhow!("Error while spawning testnet contracts: {}", e))?;

        let validator_collection = ValidatorCollection::builder()
            .num_staked_nodes(params.node_count)
            // .custom_binary_path(params.custom_build_path)
            .build(&testnet)
            .await
            .map_err(|e| anyhow::anyhow!("Error while spawning validators: {}", e))?;

        let actions = testnet.actions();

        let mut instance = TestnetInstance {
            id: params.uuid.clone(),
            node_count: params.node_count,
            polling_interval: params.polling_interval.clone(),
            epoch_length: params.epoch_length,
            exisiting_config_path: params.existing_config_path.clone(),
            ecdsa_round_timeout: params.ecdsa_round_timeout.clone(),
            enable_payment: params.enable_payment.clone(),
            action_server: lit_action_process,
            test_net: testnet,
            actions,
            contracts: testnet_contracts,
            validators: validator_collection,
            state: TestNetState::Busy,
        };

        instance.set_state(TestNetState::Active);
        Ok(instance)
    }

    pub async fn stop_random_node(&mut self) -> Result<(), anyhow::Error> {
        self.stop_random_node_internal(false).await
    }

    pub async fn stop_random_node_and_wait(&mut self) -> Result<(), anyhow::Error> {
        self.stop_random_node_internal(true).await
    }

    pub fn resolver_abi(&self) -> Result<String, anyhow::Error> {
        let abi_string = serde_json::to_string(self.contracts.contracts().contract_resolver.abi())
            .map_err(|e| anyhow!("Could not serialize contract data {}", e))?;
        Ok(abi_string)
    }

    pub async fn stop_node(&mut self, address: String) -> Result<(), anyhow::Error> {
        let mut idx: usize = 999;
        for i in 0..self.validators.size() {
            if self
                .validators
                .get_validator_by_idx(i)
                .account()
                .node_address
                .to_string()
                == address
            {
                idx = i;
            }
        }

        if idx != 999 {
            let _res = self.validators.stop_node(idx).await;
            return Ok(());
        }

        Err(anyhow::anyhow!("error while kicking random validator"))
    }

    pub fn set_state(&mut self, state: TestNetState) {
        self.state = state;
    }

    async fn stop_random_node_internal(
        &mut self,
        wait_for_epoch: bool,
    ) -> Result<(), anyhow::Error> {
        info!("Stopping random node");
        self.set_state(TestNetState::Mutating);
        let realm_id = U256::from(1); // instance.realm_id.clone();
        let current_epoch = self.validators.actions().get_current_epoch(realm_id).await;
        match self.validators.stop_random_node().await {
            Ok(_) => {
                info!("Stopped random node");
                if wait_for_epoch {
                    self.validators
                        .actions()
                        .wait_for_epoch(realm_id, current_epoch + 1)
                        .await;
                }
                self.set_state(TestNetState::Active);
                Ok(())
            }
            Err(e) => {
                self.set_state(TestNetState::UNKNOWN);
                Err(anyhow::anyhow!(
                    "error while kicking random validator {}",
                    e
                ))
            }
        }
    }
}

struct WorkingDirGuard {
    previous: PathBuf,
}

impl WorkingDirGuard {
    fn enter_workspace() -> Result<Option<Self>, anyhow::Error> {
        let previous = env::current_dir()?;
        let target = determine_workspace_dir(&previous);

        if let Some(dir) = target {
            if dir != previous {
                env::set_current_dir(&dir)?;
                info!("Set working directory to {:?}", dir);
                return Ok(Some(Self { previous }));
            }
        }

        Ok(Some(Self { previous }))
    }
}

impl Drop for WorkingDirGuard {
    fn drop(&mut self) {
        if let Err(e) = env::set_current_dir(&self.previous) {
            warn!(
                "Failed to restore working directory {:?}: {}",
                self.previous, e
            );
        }
    }
}

fn determine_workspace_dir(previous: &Path) -> Option<PathBuf> {
    if let Ok(root) = env::var("LIT_ASSETS_ROOT") {
        let candidate = PathBuf::from(root).join("rust/lit-node/lit-node");
        if candidate.exists() {
            return Some(candidate);
        }
    }

    let mut current = previous.to_path_buf();
    loop {
        let candidate = current.join("rust/lit-node/lit-node");
        if candidate.exists() {
            return Some(candidate);
        }
        if !current.pop() {
            break;
        }
    }

    if let Ok(exe_path) = env::current_exe() {
        for ancestor in exe_path.ancestors() {
            let candidate = ancestor.join("rust/lit-node/lit-node");
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }

    None
}

impl Drop for TestnetInstance {
    fn drop(&mut self) {
        info!("Stopping processes");
        if let Some(la) = self.action_server.as_mut() {
            info!("Killing lit action server with pid: {}", la.id());
            let _ = la.kill();
        }
    }
}

#[cfg(test)]
mod tests {
    use ethers::types::U256;
    use lit_node_testnet::setup_logging;

    use crate::models::TestNetCreateParams;
    use crate::testnet_instance;

    #[tokio::test]
    async fn create_testnet_runtime() {
        setup_logging("SHIV");

        const NODE_COUNT: usize = 5;
        fn test_exists_path(path: &str) -> bool {
            let file_exists = std::path::Path::new(path).exists();
            tracing::info!("File exists: {}", file_exists);
            println!("File exists: {}", file_exists);
            file_exists
        }

        let realm_id = U256::from(1); // instance.realm_id.clone();

        test_exists_path("../target/debug/lit_node");
        test_exists_path("../lit_node");
        test_exists_path("../lit_node/target/debug");
        test_exists_path("../../lit_node");
        test_exists_path("./target/debug/lit_node");

        println!(
            "current dir: {}",
            std::env::current_dir().unwrap().to_str().unwrap()
        );

        let params: TestNetCreateParams = TestNetCreateParams {
            uuid: "test".to_string(),
            node_count: NODE_COUNT,
            polling_interval: "30".to_string(),
            epoch_length: 200,
            existing_config_path: None,
            which: None,
            ecdsa_round_timeout: Some("1000".to_string()),
            enable_payment: Some("false".to_string()),
            custom_build_path: Some("../lit-node/target/debug/lit_node".to_string()),
            lit_action_server_custom_build_path: None,
        };

        let res = testnet_instance::TestnetInstance::init(params.clone()).await;
        if let Err(e) = res {
            panic!("Could not standup testnet error: {}", e);
        }

        if let Ok(network) = res {
            assert!(
                network.action_server.is_none(),
                "Action server should not be defined with this config"
            );
            assert!(
                network.ecdsa_round_timeout.is_some(),
                "ECDSA round timeout should be defined"
            );
            assert!(
                network.node_count == params.node_count,
                "Node count should equal param value"
            );
            assert!(
                network.validators.asleep_nodes().is_empty(),
                "Asleep nodes should be 0"
            );
            assert!(
                network.validators.size() == NODE_COUNT,
                "Validator set size should match config"
            );
            assert!(
                network.actions.get_current_epoch(realm_id).await == U256::from(2),
                "Should have an epoch of 2 after future resolves"
            );
        }
    }
}
