use super::{NodeAccount, SimpleTomlValue};
use crate::testnet::actions::NetworkState;
use crate::testnet::contracts_repo::{LITCONTRACTPATH, latest_wallet_manifest, node_configs_path};
use crate::testnet::node_config::{CustomNodeRuntimeConfig, generate_custom_node_runtime_config};
use anyhow::Result;
use ethers::providers::Http;
use ethers::providers::ProviderError;
use ethers::providers::{Middleware, Provider};
use lit_core::utils::toml::SimpleToml;
use std::fs; 
use std::path::Path;
use std::sync::Arc;
use std::time::SystemTime;
use tracing::{debug, error, info, trace};

pub const TEST_CACHE_ROOT: &str = "./tests/test_state_cache";

pub fn get_tar_name(num_staked: usize, num_nodes: usize, network_state: &NetworkState) -> String {
    let network_state = match network_state {
        NetworkState::Restore => "restore",
        _ => "active",
    };

    format!(
        "./{}/{}_{}_{}.tar.gz",
        TEST_CACHE_ROOT, num_staked, num_nodes, network_state
    )
}

pub fn get_dir_name(num_staked: usize, num_nodes: usize, network_state: &NetworkState) -> String {
    let network_state = match network_state {
        NetworkState::Restore => "restore",
        _ => "active",
    };
    format!(
        "{}/{}_{}_{}",
        TEST_CACHE_ROOT, num_staked, num_nodes, network_state
    )
}

pub async fn check_and_load_test_state_cache(
    provider: Arc<Provider<Http>>,
    num_staked: usize,
    num_nodes: usize,
    network_state: &NetworkState,
    custom_node_runtime_config: &CustomNodeRuntimeConfig,
    is_fault_test: bool,
) -> bool {
    let tar_name = get_tar_name(num_staked, num_nodes, network_state);

    if !Path::new(&tar_name).exists() {
        info!(
            "No test state cache found for this config (num_staked: {}, num_nodes: {}), will deploy contracts normally via script.",
            num_staked, num_nodes
        );
        return false;
    }

    let block_number = provider.get_block_number().await.unwrap();
    trace!("Block number before loading chain state: {}", block_number);

    lit_core::utils::tar::read_tar_gz_file(&tar_name, TEST_CACHE_ROOT)
        .expect(&format!("Failed to read tar.gz file: {}", tar_name));
    let dir_name = get_dir_name(num_staked, num_nodes, network_state);
    let dir = Path::new(&dir_name);

    info!("Loading test state from cache: {:?}", dir);

    let filename = "anvil_state.hex".to_string();
    let path = dir.join(&filename);

    if !path.exists() {
        error!("anvil_state.hex file does not exist in the cache");
        return false;
    };

    let contents = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(e) => {
            error!("Failed to read anvil_state.hex file: {}", e);
            return false;
        }
    };

    info!("Contents of anvil_state.hex length: {} ", contents.len());
    let params: Vec<String> = vec![contents];
    let res: Result<bool, ProviderError> =
        provider.request("anvil_loadState", params.clone()).await;

    if let Err(e) = res {
        panic!("Failed to load chain state into anvil: {}", e);
    };

    let block_number = provider.get_block_number().await.unwrap();
    trace!("Block number after loading chain state: {}", block_number);

    info!("Loading matching node configs for chain state...");

    // also copy back the node configs
    let node_configs_path = node_configs_path();
    fs::create_dir_all(&node_configs_path).unwrap();

    let node_configs_dir = &dir.join("node_configs");
    let dir_entries = fs::read_dir(node_configs_dir).unwrap();
    for entry in dir_entries {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            continue;
        }
        if entry.path().extension().unwrap() == "toml" {
            let dest_path = Path::new(&node_configs_path).join(entry.file_name());
            fs::copy(entry.path(), &dest_path).unwrap();
            generate_custom_node_runtime_config(
                is_fault_test,
                &crate::testnet::TestNetName::Anvil,
                custom_node_runtime_config,
                Some(dest_path.to_str().unwrap().to_string()),
            );
        }
    }

    // finally, put back the wallet
    let wallet_manifest_path = dir.join("wallet.json");
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let base_path = Path::new(LITCONTRACTPATH).join("wallets");
    fs::create_dir_all(&base_path).unwrap();
    let dest_path = base_path.join(format!(
        "wallets-{}-localchain-{}.json",
        timestamp, num_nodes
    ));
    fs::copy(wallet_manifest_path, dest_path).unwrap();
    info!("Chain state loaded from cache - not deploying contracts");

    let node_key_folder_source = dir.join("node_keys_cache");
    let node_key_folder_dest = "./node_keys"; // since we're including the "node_keys" folder itself.

    info!("Copying node keys to local directories...");
    // copy node keys to the test cache; folder name is changed to avoid .git_ignore issues.
    for entry in fs::read_dir(&node_key_folder_source).unwrap() {
        let entry = entry.unwrap();
        fs_extra::dir::copy(
            &entry.path(),
            &node_key_folder_dest,
            &fs_extra::dir::CopyOptions::new().overwrite(true),
        )
        .unwrap();
    }

    let deployed_core_contracts_dest =
        &format!("{}/deployed-lit-core-contracts-temp.json", LITCONTRACTPATH);
    let deployed_node_contracts_dest =
        &format!("{}/deployed-lit-node-contracts-temp.json", LITCONTRACTPATH);

    let deployed_core_contracts_src = &dir.join("deployed-lit-core-contracts-temp.json");
    let deployed_node_contracts_src = &dir.join("deployed-lit-node-contracts-temp.json");

    fs::copy(deployed_core_contracts_src, deployed_core_contracts_dest)
        .expect("Failed to copy deployed-lit-core-contracts-temp.json");
    fs::copy(deployed_node_contracts_src, deployed_node_contracts_dest)
        .expect("Failed to copy deployed-lit-node-contracts-temp.json");

    fs::remove_dir_all(&dir_name).expect("Failed to remove temp directory");
    debug!("Finished loading chain state from cache");

    true
}

pub async fn save_to_test_state_cache(
    provider: Arc<Provider<Http>>,
    num_staked_and_joined_validators: usize,
    num_staked_only_validators: usize,
    network_state: &NetworkState,
) {
    let temp_dir_name = get_dir_name(
        num_staked_and_joined_validators,
        num_staked_only_validators,
        network_state,
    );
    let tar_name = get_tar_name(
        num_staked_and_joined_validators,
        num_staked_only_validators,
        network_state,
    );

    let dir = Path::new(&temp_dir_name);
    if !dir.exists() {
        info!("Creating chain state cache directory: {:?}", dir);
        fs::create_dir_all(dir).unwrap();
    } else {
        info!("Chain state already saved for this config - not saving again.");
        return;
    }

    let block_number = provider.get_block_number().await.unwrap();
    info!(
        "Dumping chain state to file at block number {}",
        block_number
    );
    let params: Vec<String> = vec![];
    let res: String = provider.request("anvil_dumpState", params).await.unwrap();

    let filename = "anvil_state.hex".to_string();
    let path = dir.join(&filename);
    fs::write(&path, res).expect("Failed to write anvil_state.hex file");

    // also save the node configs
    info!("Getting node configs to cache...");
    let node_configs_path = node_configs_path();
    let node_configs = fs::read_dir(node_configs_path).unwrap();
    let node_configs_dir = &dir.join("node_configs");
    fs::create_dir_all(node_configs_dir).unwrap();
    for entry in node_configs {
        let entry = entry.unwrap();
        let dest_path = node_configs_dir.join(entry.file_name());
        fs::copy(entry.path(), dest_path).unwrap();
    }

    // also save the wallets
    info!("Getting wallets to cache...");
    let latest_wallet_manifest = latest_wallet_manifest(false);
    if latest_wallet_manifest.len()
        != (num_staked_and_joined_validators + num_staked_only_validators)
    {
        panic!(
            "When saving chain state cache, number of nodes in latest_wallet_manifest.json ({}) does not match num_nodes in chain config ({})",
            latest_wallet_manifest.len(),
            num_staked_and_joined_validators + num_staked_only_validators
        );
    }
    // output latest_wallet_manifest into dir/wallet.json as json
    let wallet_manifest_path = dir.join("wallet.json");
    let wallet_manifest_json = serde_json::to_string(&latest_wallet_manifest).unwrap();
    tokio::fs::write(wallet_manifest_path, wallet_manifest_json)
        .await
        .unwrap();

    // copy node keys to the test cache; folder name is changed to avoid .git_ignore issues.
    info!("Getting node key shares and wallet key to cache...");
    let node_key_folder_source = Path::new("./node_keys");
    let node_key_folder_dest = dir.join("node_keys_cache");
    fs::create_dir_all(&node_key_folder_dest).unwrap();

    for entry in fs::read_dir(&node_key_folder_source).unwrap() {
        let entry = entry.unwrap();
        fs_extra::dir::copy(
            &entry.path(),
            &node_key_folder_dest,
            &fs_extra::dir::CopyOptions::new(),
        )
        .unwrap();
    }

    info!("Getting deployed core contracts to cache...");
    let deployed_core_contracts_src =
        &format!("{}/deployed-lit-core-contracts-temp.json", LITCONTRACTPATH);
    let deployed_node_contracts_src =
        &format!("{}/deployed-lit-node-contracts-temp.json", LITCONTRACTPATH);

    let deployed_core_contracts_dest = dir.join("deployed-lit-core-contracts-temp.json");
    let deployed_node_contracts_dest = dir.join("deployed-lit-node-contracts-temp.json");

    fs::copy(deployed_core_contracts_src, deployed_core_contracts_dest).unwrap();
    fs::copy(deployed_node_contracts_src, deployed_node_contracts_dest).unwrap();

    info!("Writing tar file...");
    lit_core::utils::tar::write_tar_gz_file(&temp_dir_name, &tar_name)
        .expect("Failed to write tar.gz file");
    info!("Tar file created: {:?}", tar_name);

    info!("Removing temp directory '{}'...", temp_dir_name);
    fs::remove_dir_all(&temp_dir_name).expect("Failed to remove temp directory");
    info!("Finished saving chain state to cache: {:?}", tar_name);
}

/// Search within node_configs_path for the lit_configX.toml file that corresponds to the node_account parameters.
pub fn fetch_node_config_file_from_node_account(node_account: &NodeAccount) -> Result<String> {
    // List all files in node_configs_path
    let node_configs_path = node_configs_path();
    let dir_entries = fs::read_dir(node_configs_path)
        .map_err(|e| anyhow::anyhow!("Couldn't read directory: {}", e))?;

    // For each file, load the TOML and check for matching parameters
    for entry in dir_entries {
        let entry = entry.map_err(|e| anyhow::anyhow!("Couldn't read entry: {}", e))?;
        let path = entry.path();
        let config = SimpleToml::try_from(path.as_path())
            .map_err(|e| anyhow::anyhow!("Couldn't read config file: {}", e))?;

        // Check against node config
        let staker_address = config
            .get_address("node", "staker_address")
            .ok_or(anyhow::anyhow!("Couldn't retrieve the staking address"))?;
        let node_private_key = config
            .get_signing_key()
            .ok_or(anyhow::anyhow!("Couldn't retrieve the node wallet key"))?;

        if staker_address == node_account.staker_address
            && ethers::types::H256::from_slice(&node_private_key)
                == node_account.node_address_private_key
        {
            return path
                .to_str()
                .ok_or(anyhow::anyhow!("Couldn't convert path to string"))
                .map(|s| s.to_string());
        }
    }

    Err(anyhow::anyhow!("Couldn't find a matching node config file"))
}
