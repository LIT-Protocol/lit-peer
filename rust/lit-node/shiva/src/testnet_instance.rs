use std::process::{Child, Command};

use anyhow::anyhow;
use ethers::types::U256;
use lit_node_testnet::validator::ValidatorCollection;
use tracing::info;

use crate::models::{ContractAbis, ContractAddresses, TestNetCreateParams};

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
}

impl TestnetInstance {
    pub async fn init(params: TestNetCreateParams) -> Result<Self, anyhow::Error> {
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
            .custom_binary_path(params.custom_build_path)
            .build(&testnet)
            .await
            .map_err(|e| anyhow::anyhow!("Error while spawning validators: {}", e))?;

        let actions = testnet.actions();

        Ok(TestnetInstance {
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
        })
    }

    pub async fn stop_random_node(&mut self) -> Result<(), anyhow::Error> {
        info!("Stopping random node");
        let realm_id = U256::from(1); // instance.realm_id.clone();
        let current_epoch = self.validators.actions().get_current_epoch(realm_id).await;
        let res = self.validators.stop_random_node().await;
        if res.is_ok() {
            info!("Stopped random node");
            self.validators
                .actions()
                .wait_for_epoch(realm_id, current_epoch + 1)
                .await;
            return Ok(());
        }

        Err(anyhow::anyhow!("error while kicking random validator"))
    }

    pub async fn wait_for_epoch(&mut self) {
        let realm_id = U256::from(1); // instance.realm_id.clone();
        info!("Stopping random node");
        let current_epoch = self.validators.actions().get_current_epoch(realm_id).await;
        let _res = self
            .validators
            .actions()
            .wait_for_epoch(realm_id, current_epoch + 1)
            .await;
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
    #[ignore]
    async fn create_testnet_runtime() {
        setup_logging();

        fn test_exists_path(path: &str) -> bool {
            let file_exists = std::path::Path::new(path).exists();
            tracing::info!("File exists: {}", file_exists);
            file_exists
        }

        let realm_id = U256::from(1); // instance.realm_id.clone();

        test_exists_path("../target/debug/lit_node");
        test_exists_path("../lit_node");
        test_exists_path("../lit_node/target/debug");
        test_exists_path("../../lit_node");
        test_exists_path("./target/debug/lit_node");

        let params: TestNetCreateParams = TestNetCreateParams {
            uuid: "test".to_string(),
            node_count: 3,
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
                network.validators.size() == 3,
                "Validator set size should match config"
            );
            assert!(
                network.actions.get_current_epoch(realm_id).await == U256::from(2),
                "Should have an epoch of 2 after future resolves"
            );
        }
    }
}
