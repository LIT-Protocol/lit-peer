use std::borrow::BorrowMut;
use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;

use crate::testnet::actions::NetworkState;
use crate::testnet::contracts::ContractAddresses;

use super::{NodeAccount, SimpleTomlValue};
use command_group::CommandGroup;
use ethers::prelude::*;
use k256::ecdsa::SigningKey;
use lit_core::utils::binary::hex_to_bytes;
use lit_core::utils::toml::SimpleToml;
use lit_node_common::coms_keys::ComsKeys;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::Command;

use tracing::info;

pub const LITCONTRACTPATH: &str = "../../../blockchain/contracts";

// Required environment variables for the deployment scripts
const ENV_IPFS_API_KEY: &str = "IPFS_API_KEY";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAliasManifest {
    pub deployed_node_contracts_path: String,
    pub existing_staker_wallet_private_key: String,
    pub node_config_admin_address: String,
    pub node_config_ipfs_api_key: String,
    pub alias_ip: String,
    pub alias_port: usize,
    pub node_custom_runtime_config_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletManifestItem {
    pub idx: usize,
    pub node: WalletManifestNodeWallet,
    pub staker: WalletManifestStakerWallet,
}

impl WalletManifestItem {
    pub fn map_to_node_account(&self, provider: Arc<Provider<Http>>, chain_id: u64) -> NodeAccount {
        let staker_private_key = hex_to_bytes(self.staker.private_key.as_str())
            .expect("Couldn't parse the private key from hex into a vec");
        let staker_sk = SigningKey::from_bytes(k256::FieldBytes::from_slice(&staker_private_key))
            .expect("Couldn't parse the received key");
        let staker_wallet = LocalWallet::from(staker_sk.clone()).with_chain_id(chain_id);
        let staker_address_private_key = H256::from_slice(&staker_sk.to_bytes());

        let signing_provider = Arc::new(SignerMiddleware::new(provider, staker_wallet.clone()));

        let coms_keys_sender_priv_key =
            ComsKeys::parse_secret_key(&self.node.coms_keys_sender.private_key)
                .expect("Couldn't parse the coms keys sender private key");
        let coms_keys_receiver_priv_key =
            ComsKeys::parse_secret_key(&self.node.coms_keys_receiver.private_key)
                .expect("Couldn't parse the coms keys receiver private key");
        let coms_keys =
            ComsKeys::new_from_secret_keys(coms_keys_sender_priv_key, coms_keys_receiver_priv_key);

        let node_address = H160::from_slice(
            &hex_to_bytes(self.node.address.as_str())
                .expect("Could not convert node_address hex to bytes"),
        );

        let node_private_key = hex_to_bytes(self.node.private_key.as_str())
            .expect("Couldn't parse the private key from hex into a vec");
        let node_sk = SigningKey::from_bytes(k256::FieldBytes::from_slice(&node_private_key))
            .expect("Couldn't parse the received key");
        let node_address_private_key = H256::from_slice(&node_sk.to_bytes());

        NodeAccount {
            signing_provider,
            staker_address_private_key,
            staker_address: staker_wallet.address(),
            coms_keys,
            node_address,
            node_address_private_key,
        }
    }

    /// Asserts that the wallet manifest item is the same as the parameters in the corresponding
    /// node config file.
    pub fn assert_against_node_config(&self) {
        let config_file = &format!(
            "{}/node_configs/lit_config{}.toml",
            LITCONTRACTPATH, self.idx
        );
        let config_path = Path::new(config_file);
        let config = SimpleToml::try_from(config_path).expect("Couldn't read config file");

        // Assert staker address
        let staker_private_key = hex_to_bytes(self.staker.private_key.as_str())
            .expect("Couldn't parse the private key from hex into a vec");
        let staker_sk = SigningKey::from_bytes(k256::FieldBytes::from_slice(&staker_private_key))
            .expect("Couldn't parse the received key");
        let staker_wallet = LocalWallet::from(staker_sk);
        let staker_address = config
            .get_address("node", "staker_address")
            .expect("Couldn't retrieve the staking address");
        assert!(
            staker_address == staker_wallet.address(),
            "Staker address read from lit_configX.toml does not match one read from wallets.json"
        );

        // Assert node private key
        let node_private_key = hex_to_bytes(self.node.private_key.as_str())
            .expect("Couldn't parse the private key from hex into a vec");
        let node_sk_bytes = SigningKey::from_bytes(k256::FieldBytes::from_slice(&node_private_key))
            .expect("Couldn't parse the received key")
            .to_bytes();
        let config_node_private_key = SigningKey::from_bytes(k256::FieldBytes::from_slice(
            &config
                .get_signing_key()
                .expect("Couldn't retrieve the node wallet key"),
        ))
        .expect("Could not convert node wallet key to Signing Key")
        .to_bytes();
        assert_eq!(config_node_private_key, node_sk_bytes);
        let _node_address_private_key = H256::from_slice(
            &SigningKey::from_bytes(k256::FieldBytes::from_slice(
                &config
                    .get_signing_key()
                    .expect("Couldn't retrieve the node wallet key"),
            ))
            .expect("Could not convert node wallet key to Signing Key")
            .to_bytes(),
        );
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletManifestNodeWallet {
    pub address: String,
    pub private_key: String,
    pub public_key: String,
    pub coms_keys_sender: WalletManifestComsKeysItem,
    pub coms_keys_receiver: WalletManifestComsKeysItem,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletManifestComsKeysItem {
    pub public_key: String,
    pub private_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletManifestStakerWallet {
    pub address: String,
    pub private_key: String,
    pub public_key: String,
}

pub fn node_configs_path() -> String {
    format!("{}/node_configs", LITCONTRACTPATH)
}

pub fn alias_node_configs_path() -> String {
    format!("{}/alias_node_configs", LITCONTRACTPATH)
}

pub fn request_to_leave(staker_wallet_private_key: &str, staking_contract_address: &str) {
    // Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/requestToLeave.ts --staker-wallet-private-key <PRIVATE_KEY> --staking-address <STAKING_CONTRACT_ADDRESS>
    let args = [
        "ts-node",
        "--files",
        "scripts/requestToLeave.ts",
        "--staker-wallet-private-key",
        staker_wallet_private_key,
        "--staking-address",
        staking_contract_address,
    ];
    info!(
        "Running full command in {}: HARDHAT_NETWORK=localchain npx {}",
        LITCONTRACTPATH,
        args.join(" ")
    );
    let mut rv = Command::new("npx")
        .args(args)
        .env("HARDHAT_NETWORK", "localchain")
        .current_dir(fs::canonicalize(LITCONTRACTPATH).unwrap())
        // .stderr(Stdio::null()) // comment this out to see what's going on
        // .stdout(Stdio::null()) // comment this out to see what's going on
        .group_spawn()
        .expect("Failed to launch request to leave script");
    let exit_code = rv
        .wait()
        .expect("Failed to wait on request to leave script");
    if !exit_code.success() {
        panic!(
            "Request to leave script failed with exit code {:?}",
            exit_code
        );
    }
}

pub fn request_to_join<T>(
    staker_wallet_private_key: T,
    staking_contract_address: T,
    validator_ip: T,
    validator_port: T,
    validator_node_address: T,
    validator_comms_sender_pubkey: T,
    validator_comms_receiver_pubkey: T,
) where
    T: AsRef<str>,
{
    // Full command: HARDHAT_NETWORK=<NETWORK> npx ts-node --files scripts/requestToJoin.ts --staker-wallet-private-key <PRIVATE_KEY> --staking-address <STAKING_CONTRACT_ADDRESS> --validator-ip <VALIDATOR_IP> --validator-port <VALIDATOR_PORT> --validator-node-address <VALIDATOR_NODE_ADDRESS> --validator-comms-sender-pubkey <VALIDATOR_COMMS_SENDER_PUBKEY> --validator-comms-receiver-pubkey <VALIDATOR_COMMS_RECEIVER_PUBKEY>
    let args = [
        "ts-node",
        "--files",
        "scripts/requestToJoin.ts",
        "--staker-wallet-private-key",
        staker_wallet_private_key.as_ref(),
        "--staking-address",
        staking_contract_address.as_ref(),
        "--staking-balances-address",
        staking_contract_address.as_ref(),
        "--validator-ip",
        validator_ip.as_ref(),
        "--validator-port",
        validator_port.as_ref(),
        "--validator-node-address",
        validator_node_address.as_ref(),
        "--validator-comms-sender-pubkey",
        validator_comms_sender_pubkey.as_ref(),
        "--validator-comms-receiver-pubkey",
        validator_comms_receiver_pubkey.as_ref(),
    ];
    info!(
        "Running full command in {}: HARDHAT_NETWORK=localchain npx {}",
        LITCONTRACTPATH,
        args.join(" ")
    );

    let mut rv = Command::new("npx")
        .args(args)
        .env("HARDHAT_NETWORK", "localchain")
        .current_dir(fs::canonicalize(LITCONTRACTPATH).unwrap())
        // .stderr(Stdio::null()) // comment this out to see what's going on
        // .stdout(Stdio::null()) // comment this out to see what's going on
        .group_spawn()
        .expect("Failed to launch request to join script");
    let exit_code = rv.wait().expect("Failed to wait on request to join script");
    if !exit_code.success() {
        panic!(
            "Request to join script failed with exit code {:?}",
            exit_code
        );
    }
}

/// A wallet manifest is a JSON file that gets generated when the contract deployment tooling has
/// successfully made a deployment, that contains an array of wallets that were used during the deployment
/// and setup.
pub fn latest_wallet_manifest(is_alias_wallet_manifest: bool) -> Vec<WalletManifestItem> {
    // Fetch the latest manifest of the deployed wallets.
    let path = fs::canonicalize(LITCONTRACTPATH)
        .expect("Failed to get canonical path")
        .join("wallets");

    // Wallet manifests are named similar to this example: `wallets-1698822800413-localchain-3.json`
    let re = if is_alias_wallet_manifest {
        Regex::new(r"alias-wallets-(\d+)\.json").unwrap()
    } else {
        Regex::new(r"wallets-(\d+)-(.*)\.json").unwrap()
    };

    // First use regex to filter for matched files, then sort by descending order of the 1st
    // capture group (the timestamp), and then take the first one.
    let manifests: Vec<String> = fs::read_dir(path.clone())
        .expect("Failed to read directory")
        .filter(|entry| {
            let entry = entry.as_ref().unwrap();
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            re.is_match(filename)
        })
        .map(|entry| {
            let entry = entry.expect("Failed to get entry");
            let path = entry.path();
            path.file_name().unwrap().to_str().unwrap().to_string()
        })
        .collect();

    // Sort by descending order and take the first one.
    let latest_manifest = manifests
        .iter()
        .max_by_key(|filename| {
            let captures = re.captures(filename).unwrap();
            captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .expect("Failed to parse timestamp")
        })
        .unwrap()
        .to_string();
    info!("Fetched latest wallet manifest: {:?}", latest_manifest);

    // Parse the wallet manifest and select a random wallet that we will add an alias for.
    let manifest_path = path.join(latest_manifest);
    let manifest = fs::read_to_string(manifest_path).expect("Failed to read manifest");
    serde_json::from_str::<Vec<WalletManifestItem>>(&manifest).expect("Failed to parse JSON")
}

pub fn compile_contracts() {
    // First, check if the contracts are compiled, and if not recompile them by running npx anvil test.
    if !artifacts_exist() {
        info!("Compiling contracts");
        _compile_contracts();
    } else {
        info!("Contracts are already compiled");
    }
}

fn artifacts_exist() -> bool {
    let path = fs::canonicalize(LITCONTRACTPATH).unwrap();
    path.join("artifacts/contracts/lit-node/Staking.sol/Staking.json")
        .exists()
        && path
            .join("artifacts/contracts/lit-node/LITToken.sol/LITToken.json")
            .exists()
        && path
            .join("artifacts/@openzeppelin/contracts/token/ERC20/ERC20.sol/ERC20.json")
            .exists()
}

fn _compile_contracts() {
    info!("{:?}", fs::canonicalize("./").unwrap());
    let path = fs::canonicalize(LITCONTRACTPATH).unwrap();
    info!("Compiling in {:?}", path);
    let res = Command::new("npx")
        .current_dir(path)
        .arg("hardhat")
        .arg("compile")
        .output()
        //        .group_spawn()
        .expect("compile command failed");
    info!("{:?}", res);
}

pub fn default_staker_ip_addresses(base_port: usize, num_nodes: usize) -> Vec<String> {
    let mut ip_addresses = Vec::new();
    for i in 0..num_nodes {
        ip_addresses.push(format!("127.0.0.1:{}", base_port + i));
    }
    ip_addresses
}

pub async fn remote_deployment_and_config_creation(
    num_staked_and_joined_validators: usize,
    num_staked_only_validators: usize,
    generated_custom_node_runtime_config: bool,
) -> bool {
    // read and modify the config template
    let config_template_path = "./config/test/deploy-config-template.json";
    let file = fs::File::open(config_template_path).expect("File should open read only");
    let reader = BufReader::new(file);
    let mut config: serde_json::Map<String, Value> =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");
    if let Some(deploy_node_config) = config
        .get_mut("deployNodeConfig")
        .and_then(|v| v.as_object_mut())
    {
        deploy_node_config["numberOfStakedAndJoinedWallets"] =
            num_staked_and_joined_validators.into();
        deploy_node_config["numberOfStakedOnlyWallets"] = num_staked_only_validators.into();

        // Set the IP addresses
        let ip_addresses = default_staker_ip_addresses(
            7470,
            num_staked_and_joined_validators + num_staked_only_validators,
        );
        deploy_node_config["ipAddresses"] =
            Value::Array(ip_addresses.into_iter().map(Value::String).collect());

        // if custom node runtime configs were generated, then set customNodeRuntimeConfigPath to ../../rust/lit-node/lit-node/config/test/custom_node_runtime_config.toml
        if generated_custom_node_runtime_config {
            deploy_node_config.insert(
                "customNodeRuntimeConfigPath".into(),
                Value::String(
                    "../../rust/lit-node/lit-node/config/test/custom_node_runtime_config.toml"
                        .to_string(),
                ),
            );
        }
    } else {
        panic!("deployNodeConfig key is missing or is not an object");
    }

    // write the config
    let config_path = "./config/test/deploy-config.json";

    let output = serde_json::to_string_pretty(&config).expect("Failed to serialize config to JSON");
    fs::write(config_path, output).expect("Unable to write config to file");

    let config_path = fs::canonicalize(config_path).unwrap();

    let args = [
        "ts-node",
        "./scripts/deploy.ts",
        "--network",
        "localchain",
        "--deployConfig",
        config_path.to_str().unwrap(),
        // "../../rust/lit-node/lit-node/config/test/deploy-config.json",
    ];

    let chain_deploy_start = SystemTime::now();
    info!(
        "Running full command in {}: npx {}",
        LITCONTRACTPATH,
        args.join(" ")
    );

    let mut rv = populate_required_environment_variables(Command::new("npx").borrow_mut())
        .args(args)
        .current_dir(fs::canonicalize(LITCONTRACTPATH).unwrap())
        // .stderr(Stdio::null()) // comment this out to see what's going on with hardhat deploy
        // .stdout(Stdio::null()) // comment this out to see what's going on with hardhat deploy
        .group_spawn()
        .expect("Failed to launch contract deploy script");
    let exit_code = rv.wait().expect("Failed to wait on contract deploy script");
    if !exit_code.success() {
        panic!(
            "Contract deploy script failed with exit code {:?}",
            exit_code
        );
    }

    info!(
        "Chain deploy took {:?}",
        chain_deploy_start.elapsed().unwrap()
    );

    // Print the wallets that got created.
    let wallet_manifest = latest_wallet_manifest(false);
    for wallet in wallet_manifest {
        info!(
            "Created wallets: idx {:?} node {:?} staker {:?}",
            wallet.idx, wallet.node.address, wallet.staker.address
        );
    }

    true
}

fn populate_required_environment_variables(command: &mut Command) -> &mut Command {
    if let Ok(ipfs_api_key) = std::env::var(ENV_IPFS_API_KEY) {
        command.env(ENV_IPFS_API_KEY, ipfs_api_key)
    } else {
        command
    }
}

pub async fn contract_addresses_from_deployment() -> ContractAddresses {
    // extract the addresses from the deployment script output
    let deployed_core_contracts_path =
        &format!("{}/deployed-lit-core-contracts-temp.json", LITCONTRACTPATH);
    let deployed_node_contracts_path =
        &format!("{}/deployed-lit-node-contracts-temp.json", LITCONTRACTPATH);

    // Read and parse JSON from deployed_core_contracts_path
    let file = fs::File::open(deployed_core_contracts_path).expect("File should open read only");
    let reader = BufReader::new(file);
    let mut core_contracts: serde_json::Map<String, Value> =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");

    // Read and parse JSON from deployed_node_contracts_path
    let file = fs::File::open(deployed_node_contracts_path).expect("File should open read only");
    let reader = BufReader::new(file);
    let node_contracts: serde_json::Map<String, Value> =
        serde_json::from_reader(reader).expect("JSON was not well-formatted");

    // Merge node_contracts into core_contracts
    for (k, v) in node_contracts {
        core_contracts.insert(k, v);
    }

    // Fill and return the struct with values from core_contracts
    let contract_addresses = ContractAddresses {
        lit_token: H160::from_str(core_contracts["litTokenContractAddress"].as_str().unwrap())
            .unwrap(),
        backup_recovery: H160::from_str(
            core_contracts["backupRecoveryContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
        staking: H160::from_str(core_contracts["stakingContractAddress"].as_str().unwrap())
            .unwrap(),
        pkpnft: H160::from_str(core_contracts["pkpNftContractAddress"].as_str().unwrap()).unwrap(),
        pkp_helper: H160::from_str(core_contracts["pkpHelperContractAddress"].as_str().unwrap())
            .unwrap(),
        pubkey_router: H160::from_str(
            core_contracts["pubkeyRouterContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
        pkp_permissions: H160::from_str(
            core_contracts["pkpPermissionsContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
        contract_resolver: H160::from_str(core_contracts["contractResolver"].as_str().unwrap())
            .unwrap(),
        key_deriver: H160::from_str(
            core_contracts["hdKeyDeriverContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
        payment_delegation: H160::from_str(
            core_contracts["paymentDelegationContractAddress"]
                .as_str()
                .unwrap(),
        )
        .unwrap(),
        ledger: H160::from_str(core_contracts["ledgerContractAddress"].as_str().unwrap()).unwrap(),
        price_feed: H160::from_str(core_contracts["priceFeedContractAddress"].as_str().unwrap())
            .unwrap(),
    };

    contract_addresses
}
