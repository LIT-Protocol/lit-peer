use std::collections::HashMap;
use std::error::Error;

use crate::utils::get_address_with_network;
use crate::utils::get_lit_config_with_network;

use super::GlobalState;
use super::NetworkConfig;
use super::RpcApiType;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Write;
use leptos::prelude::WriteSignal;
use serde::{Deserialize, Serialize};
use yaml_rust2::YamlLoader;
#[derive(Clone, Debug, Serialize, Deserialize)]
struct AnsibleData {
    pub subnet_id: String,
    pub branch_os: String,
    pub branch_assets: String,
    pub environment: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ChainDetails {
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "resolverContractAddress")]
    pub resolver_contract: String,
    #[serde(default = "default_rpc_api_type")]
    pub rpc_api_type: RpcApiType,
    #[serde(rename = "rpcUrl")]
    pub chain_url: String, // this calls the rpc api ->  uses blockscount as an HTTPS call.
    #[serde(default = "default_chain_api_url")]
    pub chain_api_url: String, //  this calls the rpc chain url -> ethers.rs calls.
    #[serde(rename = "chainName")]
    pub chain_name: String,
    pub facets: Facets,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Facets {
    #[serde(rename = "Staking")]
    pub staking: Option<Vec<FacetDetails>>,
    #[serde(rename = "PKPNFT")]
    pub pkp_nft: Option<Vec<FacetDetails>>,
    #[serde(rename = "PubkeyRouter")]
    pub pubkey_router: Option<Vec<FacetDetails>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FacetDetails {
    #[serde(rename = "facetName")]
    pub facet_name: String,
    #[serde(rename = "facetAddress")]
    pub facet_address: String,
}

fn default_rpc_api_type() -> RpcApiType {
    RpcApiType::BlockScout
}

fn default_chain_api_url() -> String {
    "".to_string()
}

impl GlobalState {
    pub fn default_networks() -> Vec<NetworkConfig> {
        vec![Self::local_network()]
    }

    pub async fn get_refreshed_networks(
        new_networks: WriteSignal<Vec<NetworkConfig>>,
        active_network: RwSignal<String>,
        new_staker_names: WriteSignal<HashMap<String, String>>,
        new_common_addresses: WriteSignal<HashMap<String, String>>,
        write_status_signal: WriteSignal<String>,
    ) -> bool {
        // must load node operators details first
        write_status_signal.set("Loading node machine/operator details ...".to_string());
        let staker_names = get_staker_names().await.unwrap();
        staker_names.iter().for_each(|(key, value)| {
            new_staker_names.write().insert(key.clone(), value.clone());
            // log::info!("Staker details: {:?} / {:?}", key.clone(), value.clone());
        });

        for i in 0..=20 {
            new_staker_names
                .write()
                .insert(format!("127.0.0.1:{}", 7470 + i), format!("Local-{}", i));
        }
        Self::populate_common_addresses(&Self::local_network(), new_common_addresses).await;

        #[cfg(any(feature = "naga-dev", feature = "naga-all"))]
        {
            let _ = Self::refresh_single_network(
                "naga-dev",
                new_networks,
                new_common_addresses,
                write_status_signal,
            )
            .await;
            active_network.set("naga-dev".to_string());
        }

        #[cfg(any(feature = "naga-test", feature = "naga-all"))]
        {
            let _ = Self::refresh_single_network(
                "naga-test",
                new_networks,
                new_common_addresses,
                write_status_signal,
            )
            .await;
            active_network.set("naga-test".to_string());
        }

        #[cfg(any(feature = "naga-staging", feature = "naga-all"))]
        {
            let _ = Self::refresh_single_network(
                "naga-staging",
                new_networks,
                new_common_addresses,
                write_status_signal,
            )
            .await;
            active_network.set("naga-staging".to_string());
        }

        #[cfg(any(feature = "naga-prod", feature = "naga-all"))]
        {
            let _r = Self::refresh_single_network(
                "naga-prod",
                new_networks,
                new_common_addresses,
                write_status_signal,
            )
            .await;
            active_network.set("naga-prod".to_string());
        }

        #[cfg(any(feature = "naga-proto", feature = "naga-all"))]
        {
            let _r = Self::refresh_single_network(
                "naga-proto",
                new_networks,
                new_common_addresses,
                write_status_signal,
            )
            .await;
            active_network.set("naga-proto".to_string());
        }

        #[cfg(any(feature = "internalDev", feature = "naga-all"))]
        {
            let _r = Self::refresh_single_network(
                "internalDev",
                new_networks,
                new_common_addresses,
                write_status_signal,
            )
            .await;
            active_network.set("internalDev".to_string());
        }

        true
    }

    async fn refresh_single_network(
        network_name: &str,
        new_networks: WriteSignal<Vec<NetworkConfig>>,
        new_common_addresses: WriteSignal<HashMap<String, String>>,
        write_status_signal: WriteSignal<String>,
    ) -> bool {
        write_status_signal.set(format!("Loading network data for {} ...", network_name));

        let (network, extra_facets) = get_network_config(network_name).await.unwrap();
        new_networks.write().push(network.clone());

        if let Some(staking) = extra_facets.staking {
            add_facet_details(staking, new_common_addresses);
        };
        if let Some(pkp_nft) = extra_facets.pkp_nft {
            add_facet_details(pkp_nft, new_common_addresses);
        };
        if let Some(pubkey_router) = extra_facets.pubkey_router {
            add_facet_details(pubkey_router, new_common_addresses);
        };

        Self::populate_common_addresses(&network, new_common_addresses).await;
        true
    }

    async fn populate_common_addresses(
        network: &NetworkConfig,
        new_common_addresses: WriteSignal<HashMap<String, String>>,
    ) -> bool {
        let common_addresses = get_common_addresses(&network).await.unwrap();
        common_addresses.iter().for_each(|(key, value)| {
            new_common_addresses
                .write()
                .insert(key.clone(), value.clone());
        });

        true
    }

    fn local_network() -> NetworkConfig {
        NetworkConfig {
            network_name: "localhost".to_string(),
            environment: 0,
            subnet_id: "n/a".to_string(),
            branch_os: "n/a".to_string(),
            branch_assets: "n/a".to_string(),
            resolver_contract: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
            rpc_api_type: RpcApiType::OtterScan,
            chain_url: "http://127.0.0.1:8545".to_string(),
            chain_api_url: "http://127.0.0.1:8545".to_string(),
            chain_name: "http://127.0.0.1:8545".to_string(),
        }
    }
}

fn add_facet_details(
    facets: Vec<FacetDetails>,
    new_common_addresses: WriteSignal<HashMap<String, String>>,
) {
    facets.iter().for_each(|facet| {
        new_common_addresses
            .write()
            .insert(facet.facet_address.to_lowercase(), facet.facet_name.clone());
    });
}

async fn get_network_config(network_name: &str) -> Result<(NetworkConfig, Facets), String> {
    let (ansible_src_url, network_src_url) = match network_name {
        "naga-test" => ("20-test-decentralized/20-naga_test.yml", "naga-test"),
        "naga-prod" => ("10-prod/10-naga.yml", "naga-prod"),
        "naga-dev" => ("30-test-centralized/30-naga_dev.yml", "naga-dev"),
        "naga-staging" => ("15-staging/15-naga_staging.yml", "naga-staging"),
        "naga-proto" => ("10-prod/10-naga_proto.yml", "naga-proto"),
        "internalDev" => ("20-test-decentralized/20-internaldev.yml", "internal-dev"),
        _ => return Err("Network Not Found.".to_string()),
    };

    let ansible_data = get_ansible_data(network_name, ansible_src_url)
        .await
        .unwrap();
    let mut network_data = get_network_data(network_src_url).await.unwrap();

    let chain_api_url = network_data.chain_url.clone();

    network_data.chain_api_url = format!(
        "{}{}",
        chain_api_url.replace("yellowstone-rpc", "yellowstone-explorer"),
        "/api"
    );

    Ok((
        NetworkConfig {
            network_name: network_name.to_string(),
            environment: ansible_data.environment,
            subnet_id: ansible_data.subnet_id,
            branch_os: ansible_data.branch_os,
            branch_assets: ansible_data.branch_assets,
            resolver_contract: network_data.resolver_contract,
            rpc_api_type: network_data.rpc_api_type,
            chain_url: network_data.chain_url,
            chain_api_url: network_data.chain_api_url,
            chain_name: network_data.chain_name,
        },
        network_data.facets,
    ))
}

async fn get_ansible_data(
    network_name: &str,
    ansible_src_url: &str,
) -> Result<AnsibleData, String> {
    let src_url = format!(
        "https://lit-protocol.github.io/lit-ansible/networks/{}",
        ansible_src_url
    );

    let response = reqwest::get(src_url).await;
    if response.is_err() {
        return Err(response.err().unwrap().to_string());
    }

    let body = response.unwrap().text().await;
    if body.is_err() {
        return Err(body.err().unwrap().to_string());
    }

    let body = body.unwrap();
    let docs = YamlLoader::load_from_str(&body).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    let network_name = network_name
        .to_lowercase()
        .replace("-", "_")
        .replace("_prod", "");
    let network_name = network_name.as_str();
    let vars = doc[network_name]["vars"].clone();
    log::info!("env: {:?}", vars["env"]);
    let results = AnsibleData {
        subnet_id: match vars["subnet_id"].as_str() {
            None => "n/a".to_string(),
            Some(subnet_id) => subnet_id.to_string(),
        },
        branch_os: match vars["branch_os"].as_str() {
            None => "n/a".to_string(),
            Some(branch_os) => branch_os.to_string(),
        },
        branch_assets: match vars["branch_assets"].as_str() {
            None => "n/a".to_string(),
            Some(branch_assets) => branch_assets.to_string(),
        },
        environment: match vars["env"].as_str() {
            None => 0,
            Some(environment) => match environment {
                "dev" => 0,
                "test" => 1,
                "staging" => 1,
                "prod" => 2,
                _ => 0,
            },
        },
    };

    Ok(results)
}

async fn get_network_data(network_src_url: &str) -> Result<ChainDetails, String> {
    let src_url = format!(
        "https://raw.githubusercontent.com/LIT-Protocol/networks/refs/heads/main/{}/deployed-lit-node-contracts-temp.json",
        network_src_url
    );
    let response = reqwest::get(src_url).await;
    if response.is_err() {
        return Err(response.err().unwrap().to_string());
    }

    let body = response.unwrap().text().await;
    if body.is_err() {
        return Err(body.err().unwrap().to_string());
    }

    let mut results: ChainDetails = serde_json::from_str(&body.unwrap()).unwrap();
    results.rpc_api_type = RpcApiType::BlockScout;

    log::info!("results: {:?}", results);
    Ok(results)
}

pub async fn get_staker_names() -> Result<HashMap<String, String>, String> {
    let mut stakers = HashMap::new();

    let _ = staker_names_from("02-nodes-external/02-ovh.yml", &mut stakers).await;
    let _ = staker_names_from("02-nodes-external/02-leaseweb.yml", &mut stakers).await;
    let _ = staker_names_from("02-nodes-external/02-selfhosted.yml", &mut stakers).await;
    let _ = staker_names_from("01-nodes-internal/01-ovh.yml", &mut stakers).await;
    let _ = staker_names_from("01-nodes-internal/01-dedicated.yml", &mut stakers).await;
    let _ = staker_names_from("01-nodes-internal/01-leaseweb.yml", &mut stakers).await;
    let _ = staker_names_from("01-nodes-internal/01-cherryserver.yml", &mut stakers).await;

    Ok(stakers)
}

pub async fn staker_names_from(
    src_url: &str,
    staker_names: &mut HashMap<String, String>,
) -> Result<bool, String> {
    let src_url = format!(
        "https://lit-protocol.github.io/lit-ansible/machines/{}",
        src_url
    );
    let response = reqwest::get(src_url).await;
    if response.is_err() {
        return Err(response.err().unwrap().to_string());
    }

    let body = response.unwrap().text().await;
    if body.is_err() {
        return Err(body.err().unwrap().to_string());
    }

    let body = body.unwrap();
    let docs = YamlLoader::load_from_str(&body).unwrap();
    let doc = &docs[0];
    let hosts = doc["all"]["hosts"].clone();

    let hosts = hosts.into_hash().unwrap();

    for (key, value) in hosts.iter() {
        // log::info!("Key: {:?}, Value: {:?}", &key, &value); // remove to see other info available
        let host_name = key.as_str().unwrap().to_string();
        let guest_ip = match value["guest_ip"].as_str() {
            Some(guest_ip) => guest_ip.to_string(),
            None => value["host_ip"].as_str().unwrap_or("na").to_string(),
        };
        let guest_ip = match guest_ip.contains("/") {
            true => guest_ip.split("/").nth(0).unwrap().to_string(),
            false => guest_ip,
        };
        staker_names.insert(guest_ip, host_name);
    }

    // log::info!("Staker names: {:?}", &staker_names);
    Ok(true)
}

pub async fn get_common_addresses(
    network_config: &NetworkConfig,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut common_addresses = HashMap::new();

    // basic contracts
    let staking_contract_address =
        get_address_with_network(network_config, crate::contracts::STAKING_CONTRACT).await?;
    common_addresses.insert(
        format!("0x{}", hex::encode(staking_contract_address.0)),
        crate::contracts::STAKING_CONTRACT.to_string(),
    );
    let pubkey_router_contract_address =
        get_address_with_network(network_config, crate::contracts::PUB_KEY_ROUTER_CONTRACT).await?;
    common_addresses.insert(
        format!("0x{}", hex::encode(pubkey_router_contract_address.0)),
        crate::contracts::PUB_KEY_ROUTER_CONTRACT.to_string(),
    );
    let release_register_contract_address =
        get_address_with_network(network_config, crate::contracts::RELEASE_REGISTER_CONTRACT)
            .await?;
    common_addresses.insert(
        format!("0x{}", hex::encode(release_register_contract_address.0)),
        crate::contracts::RELEASE_REGISTER_CONTRACT.to_string(),
    );
    let pkp_nft_contract_address =
        get_address_with_network(network_config, crate::contracts::PKP_NFT_CONTRACT).await?;
    common_addresses.insert(
        format!("0x{}", hex::encode(pkp_nft_contract_address.0)),
        crate::contracts::PKP_NFT_CONTRACT.to_string(),
    );

    // nodes
    let staking_contract_address =
        match get_address_with_network(network_config, crate::contracts::STAKING_CONTRACT).await {
            Ok(staking_contract_address) => staking_contract_address,
            Err(e) => {
                log::error!("Error getting staking contract address: {:?}", e);
                return Err("Error getting staking contract address".into());
            }
        };

    let cfg = &get_lit_config_with_network(network_config);
    let staking =
        crate::contracts::staking::Staking::node_monitor_load(cfg, staking_contract_address)
            .unwrap();

    let validators = crate::pages::validators::get_validators(&staking, true, 1).await;

    for validator in validators {
        common_addresses.insert(validator.wallet_address, validator.host_name);
    }
    Ok(common_addresses)
}
