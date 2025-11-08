pub mod chain;
// pub mod eip712;
pub mod networks;
use leptos::prelude::*;

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RpcApiType {
    BlockScout,
    OtterScan,
}

impl Display for RpcApiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<u32> for RpcApiType {
    fn into(self) -> u32 {
        match self {
            RpcApiType::BlockScout => 1,
            RpcApiType::OtterScan => 2,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NetworkConfig {
    pub network_name: String,
    pub environment: u8,
    pub subnet_id: String,
    pub branch_os: String,
    pub branch_assets: String,
    pub resolver_contract: String,
    pub rpc_api_type: RpcApiType,
    pub chain_url: String,
    pub chain_api_url: String,
    pub chain_name: String,
}

#[derive(Clone, Debug)]
pub struct GlobalState {
    pub page_header: RwSignal<String>,
    pub networks: RwSignal<Vec<NetworkConfig>>,
    pub current_network: RwSignal<String>,
    pub block: RwSignal<u64>,
    pub network_state: RwSignal<Vec<NetworkState>>,
    pub staker_names: RwSignal<HashMap<String, String>>,
    pub common_addresses: RwSignal<HashMap<String, String>>,
    pub proxy_url: String,
}

fn default_proxy_url() -> String {
    "https://salty-basin-51319-1df8d5992406.herokuapp.com/".to_string()
}

#[derive(Clone, Debug)]
pub struct NetworkState {
    pub realm_id: u64,
    pub network_state: RwSignal<String>,
    pub epoch: RwSignal<u64>,
}

#[derive(Clone, Debug)]
pub struct FixedGlobalState {
    pub page_header: String,
    pub active_network: NetworkConfig,
    pub network_state: Vec<FixedNetworkState>,
    pub block: u64,
}

#[derive(Clone, Debug)]
pub struct FixedNetworkState {
    pub network_state: String,
    pub epoch: u64,
}

impl NetworkState {
    pub fn new(realm_id: u64) -> Self {
        log::info!("Creating NetworkState for realm_id: {}", realm_id);
        Self {
            realm_id,
            network_state: RwSignal::new("[Pending Connection]".to_string()),
            epoch: RwSignal::new(0),
        }
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            page_header: RwSignal::new("Home".to_string()),
            current_network: RwSignal::new("localhost".to_string()),
            networks: RwSignal::new(GlobalState::default_networks()),
            network_state: RwSignal::new(vec![
                NetworkState::new(1),
                NetworkState::new(2),
                NetworkState::new(3),
                NetworkState::new(4),
            ]),
            block: RwSignal::new(0),
            proxy_url: default_proxy_url(),
            staker_names: RwSignal::new(HashMap::new()),
            common_addresses: RwSignal::new(HashMap::new()),
        }
    }

    pub fn index_for_realm_id(&mut self, realm_id: u64) -> usize {
        let index = self
            .network_state
            .get()
            .iter()
            .position(|n| n.realm_id == realm_id);
        if index.is_none() {
            self.network_state.write().push(NetworkState::new(realm_id));
            return self.network_state.get().len() - 1;
        }
        index.unwrap()
    }

    pub fn active_network(&self) -> NetworkConfig {
        let network_name = self.current_network.get_untracked();
        self.networks
            .get_untracked()
            .iter()
            .find(|n| n.network_name == network_name)
            .unwrap()
            .clone()
    }

    pub fn to_fixed(&self) -> FixedGlobalState {
        FixedGlobalState {
            page_header: self.page_header.get_untracked(),
            network_state: self
                .network_state
                .get()
                .iter()
                .map(|n| FixedNetworkState {
                    network_state: n.network_state.get_untracked(),
                    epoch: n.epoch.get_untracked(),
                })
                .collect(),
            active_network: self.active_network(),
            block: self.block.get_untracked(),
        }
    }

    pub fn network_write_signal(&self) -> WriteSignal<Vec<NetworkConfig>> {
        let (_, write_signal) = self.networks.clone().split();
        write_signal
    }

    pub fn staker_names_write_signal(&self) -> WriteSignal<HashMap<String, String>> {
        let (_, write_signal) = self.staker_names.clone().split();
        write_signal
    }

    pub fn common_addresses_write_signal(&self) -> WriteSignal<HashMap<String, String>> {
        let (_, write_signal) = self.common_addresses.clone().split();
        write_signal
    }
}
