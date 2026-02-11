use crate::testnet::actions::Actions;
use crate::validator::Validator;
use anyhow::Result;
use ethers::types::U256;
use lit_node_core::NodeSet;
use rand::Rng;
use std::net::Ipv4Addr;
use tracing::debug;

impl Actions {
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
        let realm_id = U256::from(realm);

        let kicked = self
            .contracts()
            .staking
            .get_kicked_validators(realm_id)
            .call()
            .await
            .unwrap_or_else(|_e| vec![]);

        let mut nodes_for_epoch = self
            .get_current_validator_structs(realm_id)
            .await
            .into_iter()
            .filter(|f| !kicked.contains(&f.node_address))
            .map(|v| format!("{}:{}", Ipv4Addr::from_bits(v.ip).to_string(), &v.port))
            .collect::<Vec<String>>();

        let nodes_for_epoch2 = nodes_for_epoch.clone();

        let all_nodes_count = kicked.len() + nodes_for_epoch.len();

        let threshold = self
            .contracts()
            .staking
            .current_validator_count_for_consensus(realm_id)
            .await
            .unwrap()
            .as_usize();

        let epoch = self.get_current_epoch(realm_id).await.as_u64();
        // this was using ports.len()
        // let threshold = std::cmp::min(nodes_for_epoch.len(), self.threshold(ports.len()));

        let mut node_set: Vec<NodeSet> = Vec::with_capacity(threshold);

        // if we are including validators, we need to add the validators to the node set and reduce the number of remaining nodes to add
        let validators_to_add = threshold - validators_to_include.len();

        // add the specific validators to the node set - this is generally used for fault tests, and remove from the list to choose the remaining nodes
        for validator in validators_to_include {
            node_set.push(NodeSet {
                socket_address: validator.socket_address(),
                value: 1,
            });

            nodes_for_epoch.retain(|node| node != &validator.socket_address());
        }

        let locked_rng = crate::rand::shared_rng();
        let mut locked_rng = locked_rng.lock().expect("Failed to lock rng");

        for _ in 0..validators_to_add {
            let random_node =
                nodes_for_epoch.remove(locked_rng.gen_range(0..nodes_for_epoch.len()));
            let random_node_set = NodeSet {
                socket_address: random_node,
                value: 1,
            };
            node_set.push(random_node_set);
        }

        debug!(
            "All nodes / online nodes (epoch {}): {} / {} and threshold: {}, and nodeset (l:{}): {:?}",
            epoch,
            all_nodes_count,
            nodes_for_epoch2.len(),
            threshold,
            node_set.len(),
            &node_set
        );

        node_set
    }

    pub async fn complete_node_set(&self) -> Result<Vec<NodeSet>> {
        let realm_id = U256::from(1);
        let nodes_for_epoch = self
            .get_current_validator_structs(realm_id)
            .await
            .into_iter()
            .map(|v| format!("{}:{}", Ipv4Addr::from_bits(v.ip).to_string(), &v.port))
            .collect::<Vec<String>>();

        Ok(nodes_for_epoch
            .iter()
            .map(|v| NodeSet {
                socket_address: v.clone(),
                value: 1,
            })
            .collect::<Vec<NodeSet>>())
    }
    pub async fn active_node_set(&self) -> Result<Vec<NodeSet>> {
        let realm_id = U256::from(1);
        let kicked = self
            .contracts()
            .staking
            .get_kicked_validators(realm_id)
            .call()
            .await
            .unwrap_or_else(|_e| vec![]);

        let nodes_for_epoch = self
            .get_current_validator_structs(realm_id)
            .await
            .into_iter()
            .filter(|f| !kicked.contains(&f.node_address))
            .map(|v| format!("{}:{}", Ipv4Addr::from_bits(v.ip).to_string(), &v.port))
            .collect::<Vec<String>>();

        Ok(nodes_for_epoch
            .iter()
            .map(|v| NodeSet {
                socket_address: v.clone(),
                value: 1,
            })
            .collect::<Vec<NodeSet>>())
    }
}
