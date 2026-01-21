use ethers::types::U256;
use tokio::sync::RwLock;

use crate::{
    models::{
        ContractAbis, ContractAddresses, TestNetCreateParams, TestNetCreateRequest, TestNetInfo,
        TestNetState,
    },
    testnet_instance::TestnetInstance,
};

pub struct ShivaClient {
    pub testnet_instances: RwLock<Vec<TestnetInstance>>,
}

impl Default for ShivaClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ShivaClient {
    pub fn new() -> Self {
        Self {
            testnet_instances: RwLock::new(Vec::new()),
        }
    }

    pub async fn get_testnet_ids(&self) -> Result<Vec<String>, anyhow::Error> {
        let testnet_instances = self.testnet_instances.read().await;

        let ids = testnet_instances
            .as_slice()
            .iter()
            .map(|instance| instance.id.clone())
            .collect();
        Ok(ids)
    }

    pub async fn get_testnet_info(&self, id: String) -> Result<TestNetInfo, anyhow::Error> {
        let testnet_instances = self.testnet_instances.read().await;
        let testnet_instance = testnet_instances.iter().find(|instance| instance.id == id);
        if let Some(instance) = testnet_instance {
            Ok(TestNetInfo {
                contract_addresses: ContractAddresses::new(instance.contracts.contract_addresses()),
                contract_abis: ContractAbis::new(&instance.contracts)?,
                validator_addresses: instance.validators.addresses().clone(),
                epoch_length: instance.epoch_length,
                contract_resolver_abi: instance.resolver_abi()?,
                rpc_url: instance.test_net.rpcurl.clone(),
            })
        } else {
            Err(anyhow::anyhow!("Testnet instance {} not found", id))
        }
    }
    pub async fn create_testnets(
        &self,
        id: String,
        create_request: TestNetCreateRequest,
    ) -> Result<TestNetState, anyhow::Error> {
        let params = TestNetCreateParams {
            uuid: id,
            node_count: create_request.node_count,
            polling_interval: create_request.polling_interval,
            epoch_length: create_request.epoch_length,
            existing_config_path: create_request.existing_config_path,
            which: create_request.which,
            ecdsa_round_timeout: create_request.ecdsa_round_timeout,
            enable_payment: create_request.enable_payment,
            custom_build_path: create_request.custom_build_path,
            lit_action_server_custom_build_path: create_request.lit_action_server_custom_build_path,
        };
        let testnet_instance = TestnetInstance::init(params).await?;
        let mut testnet_instances = self.testnet_instances.write().await;
        let state = testnet_instance.state.clone();
        testnet_instances.push(testnet_instance);
        Ok(state)
    }

    pub async fn delete_testnet(&self, id: String) -> Result<TestNetState, anyhow::Error> {
        let mut testnet_instances = self.testnet_instances.write().await;
        if let Some(pos) = testnet_instances
            .iter()
            .position(|instance| instance.id == id)
        {
            let mut instance = testnet_instances.remove(pos);
            instance.set_state(TestNetState::Shutdown);
            let final_state = instance.state.clone();
            drop(instance);
            Ok(final_state)
        } else {
            Err(anyhow::anyhow!("Testnet instance {} not found", id))
        }
    }

    pub async fn poll_testnet_state(&self, id: String) -> Result<TestNetState, anyhow::Error> {
        self.get_testnet_state(id).await
    }
    pub async fn get_testnet_state(&self, id: String) -> Result<TestNetState, anyhow::Error> {
        let testnet_instances = self.testnet_instances.read().await;
        let testnet_instance = testnet_instances.iter().find(|instance| instance.id == id);
        if let Some(instance) = testnet_instance {
            Ok(instance.state.clone())
        } else {
            Err(anyhow::anyhow!("Testnet instance {} not found", id))
        }
    }
    pub async fn stop_random_node(&self, id: String) -> Result<usize, anyhow::Error> {
        let mut testnet_instances = self.testnet_instances.write().await;
        let testnet_instance = testnet_instances
            .iter_mut()
            .find(|instance| instance.id == id);

        match testnet_instance {
            Some(instance) => {
                instance.stop_random_node().await?;
                Ok(instance.validators.size())
            }
            None => Err(anyhow::anyhow!("Testnet instance {} not found", id)),
        }
    }
    pub async fn stop_random_node_wait(&self, id: String) -> Result<usize, anyhow::Error> {
        let mut testnet_instances = self.testnet_instances.write().await;
        let testnet_instance = testnet_instances
            .iter_mut()
            .find(|instance| instance.id == id);

        match testnet_instance {
            Some(instance) => {
                instance.stop_random_node_and_wait().await?;
                Ok(instance.validators.size())
            }
            None => Err(anyhow::anyhow!("Testnet instance {} not found", id)),
        }
    }
    pub async fn transition_epoch_wait(&self, id: String) -> Result<bool, anyhow::Error> {
        let mut testnet_instances = self.testnet_instances.write().await;
        let testnet_instance = testnet_instances
            .iter_mut()
            .find(|instance| instance.id == id);

        match testnet_instance {
            Some(instance) => {
                instance.set_state(TestNetState::Mutating);
                let realm_id = U256::from(1);
                let current_epoch = instance
                    .validators
                    .actions()
                    .get_current_epoch(realm_id)
                    .await;
                let _ = instance
                    .validators
                    .actions()
                    .wait_for_epoch(realm_id, current_epoch + 1)
                    .await;
                instance.set_state(TestNetState::Active);
                Ok(true)
            }
            None => Err(anyhow::anyhow!("Testnet instance {} not found", id)),
        }
    }
}
