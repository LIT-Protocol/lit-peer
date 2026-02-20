// To use this, you need to install Foundry using this command: curl -L https://foundry.paradigm.xyz | bash
use super::ChainTrait;
use crate::testnet::NodeAccount;
use crate::testnet::cache_data_store::CacheDataStore;
use crate::testnet::chain::known_accounts::first_anvil_account;
#[cfg(not(feature = "lit-peer-api-server"))]
use crate::testnet::contracts_repo::compile_contracts;
use command_group::{CommandGroup, GroupChild}; // node/anvil launches many processes to manage the testnet, so we need to use a group interface to manage them, as killing only the process we know about will leave zombies.
use ethers::prelude::*;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::net::{TcpStream, ToSocketAddrs};
use tracing::{debug, info};

pub struct Anvil {
    num_nodes: usize,
    port: u16,
    is_datil_testnet: bool,
    // num_staked: usize,
}

impl Anvil {
    // pub fn new(num_nodes: usize, num_staked: usize) -> impl ChainTrait {
    pub fn new(num_nodes: usize, is_datil_testnet: bool) -> impl ChainTrait {
        let port = if is_datil_testnet { 8549 } else { 8545 };

        Anvil {
            num_nodes,
            // num_staked,
            port,
            is_datil_testnet,
        }
    }
}

use async_trait::async_trait;
// impl chain for Anvil
#[async_trait]
impl ChainTrait for Anvil {
    fn chain_id(&self) -> u64 {
        31337
    }

    fn num_nodes(&self) -> usize {
        self.num_nodes
    }

    fn rpc_url(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }

    fn chain_name(&self) -> &'static str {
        if self.is_datil_testnet {
            "anvilDatil"
        } else {
            "anvil"
        }
    }

    async fn start_chain(&self) -> Option<GroupChild> {
        #[cfg(not(feature = "lit-peer-api-server"))]
        compile_contracts();

        let mut cache_data_store = CacheDataStore::from_file_or_new()
            .await
            .unwrap_or(CacheDataStore::new());
        // when running in CI, anvil is already running in a docker container, so no need to start it.
        // we run echo 'hi' as a dummy process instead.
        let in_github_ci = std::env::var("IN_GITHUB_CI").unwrap_or("0".to_string());
        if in_github_ci == "1" {
            info!("Not starting chain in CI.");
            if is_anvil_running(&self.rpc_url()).await {
                info!("Anvil is running in CI at {}. ", self.rpc_url());
                cache_data_store.set_anvil_is_running(true);
                let _ = cache_data_store.save().await; // if it fails we reset.
            } else {
                panic!(
                    "Anvil is not running in CI at {}.  It should have been loaded by the docker container.",
                    self.rpc_url()
                );
            }

            return None;
        }

        if is_anvil_running(&self.rpc_url()).await {
            if self.port == 8549 {
                info!("Datil Anvil is already running.  Skipping kill.");
                cache_data_store.set_anvil_is_running(true);
                let _ = cache_data_store.save().await; // if it fails we reset.
                return None;
            } else {
                info!("anvil is already running.  Attempting to kill");
                Command::new("pkill")
                    .arg("anvil")
                    .output()
                    .expect("failed to kill anvil");

                tokio::time::sleep(Duration::from_millis(500)).await;
                if is_anvil_running(&self.rpc_url()).await {
                    panic!("anvil running and couldn't be killed");
                }
            }
        }

        // We use group_spawn because node launches several subprocesses,
        // and we need to kill them using group api to stop the testnet
        let command_path;
        if std::env::var("IN_GITHUB_CI").is_ok() {
            let home_dir = std::env::var("HOME").expect("Could not get home dir");
            command_path = format!("{}/.cargo/bin/anvil", home_dir);
            let path = std::path::PathBuf::from(command_path.clone());
            if !path.is_file() {
                panic!("can't find anvil. Aborting test.");
            }
        } else {
            let home_dir = std::env::var("HOME").expect("Could not get home dir");
            command_path = format!("{}/.foundry/bin/anvil", home_dir);
        }
        debug!("found path for anvil: {}", &command_path);

        let mut command = Command::new(command_path);
        command.arg("--port").arg(self.port.to_string());

        let rv = command
            // .env("RUST_LOG", "trace") // if you need to debug anvil you can uncomment this.
            // .env("RUST_LOG", "info") // if you just need to see console.log from the contract uncomment this instead
            .env("ETHERNAL_API_TOKEN", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJmaXJlYmFzZVVzZXJJZCI6IlQ5Sk1xZjgwMUVoUk9XSTNaTVRTM2dQRTRrdjIiLCJhcGlLZXkiOiJBRFlSRUVOLVhSRE1DVEgtSjNXTUdIWC1IQ1haSE0yXHUwMDAxIiwiaWF0IjoxNjkxMDk0NDczfQ.Rpc_oExqnwCl-iRKLQbQCN7P7nUIuucJtoiE46xVn3g") // localhost
            .stderr(Stdio::null()) // comment this out to see what's going on with anvil
            .stdout(Stdio::null()) // comment this out to see what's going on with anvil
            .group_spawn()
            .expect("Failed to launch Anvil testnet.  Are you sure Foundry is installed?");
        if !has_anvil_started(&self.rpc_url(), Duration::new(10, 0)).await {
            panic!(
                "anvil has not come up.  Aborting test.  You may comment out the stdout/stderr lines above to see what's going on."
            );
        }
        info!("Anvil has started on port {}", self.port);
        if self.port == 8549 {
            cache_data_store.set_anvil_is_running(true);
            cache_data_store.set_datil_state_is_loaded(false);
            let _ = cache_data_store.save().await; // if it fails we reset.
        }

        Some(rv)
    }

    // for hardhat and no_chain, this trait function should be overriden.
    fn deployer(&self) -> NodeAccount {
        first_anvil_account(self.chain_id(), self.chain_name())
    }
}

pub async fn is_anvil_running<A: ToSocketAddrs + ?Sized>(host: &A) -> bool {
    match TcpStream::connect(host).await {
        Ok(..) => true,
        Err(..) => false,
    }
}

async fn has_anvil_started<A: ToSocketAddrs + ?Sized>(host: &A, waitfor: Duration) -> bool {
    async fn waitfor_anvil_to_start<A: ToSocketAddrs + ?Sized>(host: &A) {
        loop {
            if is_anvil_running(host).await {
                return;
            }
            info!("Waiting for anvil to come up...");
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    match tokio::time::timeout(waitfor, waitfor_anvil_to_start(host)).await {
        Err(..) => false,
        Ok(..) => true,
    }
}
