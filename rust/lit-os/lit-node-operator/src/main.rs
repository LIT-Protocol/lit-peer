pub mod chronicle_replica;
pub mod error;

use crate::chronicle_replica::{
    ChronicleReplicaConfig, ChronicleReplicaManager, RealCommandRunner,
};
use crate::error::{Result, unexpected_err};

use lit_blockchain::resolver::contract::ContractResolver;
use lit_cli_os::config::LitCliOsConfig;
use lit_core::config::{LitConfig, LitConfigBuilder};
use lit_node_operator::get_instance_item;
use lit_node_operator::host_commands_listener::HostCommandsListener;
use nix::unistd::Uid;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::{MissedTickBehavior, interval};
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{EnvFilter, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging first
    init_journald_logging()?;
    debug!("Starting lit-node-operator daemon");

    // Ensure we're running as root (required for Docker and iptables)
    if !Uid::effective().is_root() {
        error!("lit-node-operator must be run as root, exiting");
        std::process::exit(1);
    }
    debug!("Running as root user.");

    // Load global configuration
    let cfg_builder = LitConfigBuilder::default();
    let cfg = <LitConfig as LitCliOsConfig>::from_builder(cfg_builder)
        .expect("failed to load lit config");
    debug!("LitConfig loaded successfully.");

    // Get the only instance on this node
    let item = get_instance_item(&cfg);
    debug!(
        instance_id = ?item.instance_env.instance_id,
        "Operating for instance"
    );

    // Determine if the local chronicle replica should be enabled from the main config
    let enable_replica = cfg.get_bool("node.enable_chronicle_replica").unwrap_or(false);
    debug!(
        enable_local_replica = enable_replica,
        config_path = "node.enable_chronicle_replica",
        "Chronicle Replica enabled status from LitConfig."
    );

    // Create Chronicle Replica Config with the enabled flag from config
    let mut chronicle_config = ChronicleReplicaConfig::default();
    chronicle_config.enable_local_replica = enable_replica;

    debug!(config = ?chronicle_config, "Using Chronicle Replica Config");

    // Spawn Chronicle Replica Manager task if enabled
    let _replica_manager_handle: Option<JoinHandle<()>> = if chronicle_config.enable_local_replica {
        debug!("Initializing and spawning Chronicle Replica monitoring task");

        // Clone the config for the worker task
        let replica_config_clone = chronicle_config.clone();
        let command_runner = RealCommandRunner::default();

        let handle = tokio::spawn(async move {
            let mut replica_manager =
                ChronicleReplicaManager::new(replica_config_clone, command_runner);

            let check_interval_secs = replica_manager.check_interval_seconds();
            let check_interval = Duration::from_secs(check_interval_secs);

            if check_interval.is_zero() {
                error!("Replica check interval is zero. Task cannot run and will exit.");
                return;
            }

            let mut interval_timer = interval(check_interval);
            interval_timer.set_missed_tick_behavior(MissedTickBehavior::Delay);

            // Start monitoring loop
            debug!("Replica Manager: Starting monitoring loop (interval: {:?})", check_interval);

            loop {
                trace!("Replica Manager: Loop Iteration Starts");
                debug!("Replica Manager: Starting check cycle");

                if let Err(e) = replica_manager.check_and_manage_replica().await {
                    error!(error = ?e, "Replica Manager: Error during check cycle.");
                } else {
                    debug!("Replica Manager: Check cycle finished.");
                }

                trace!("Replica Manager: Waiting for next tick...");
                interval_timer.tick().await;
                trace!("Replica Manager: Tick Received");
            }
        });

        debug!("Chronicle Replica monitoring task spawned successfully.");
        Some(handle)
    } else {
        info!("Chronicle Replica monitoring is disabled via configuration.");
        None
    };

    // Initialize listener
    let subnet = item.instance_env.subnet_id.clone().expect("node-type nodes must have a subnet");
    let env = item.build_env.env().expect("node-type nodes must have an env");

    debug!(
        subnet = ?subnet,
        env = ?env,
        "Instantiating HostCommands listener"
    );

    let resolver = ContractResolver::new(subnet, env, None);
    let host_commands_contract = resolver
        .host_commands_contract(&cfg)
        .await
        .expect("Datil and later must have a HostCommands contract");

    debug!("Starting HostCommandsListener");
    let listener = HostCommandsListener::new(host_commands_contract, item)?;

    listener.listen_for_events().await?;

    debug!("HostCommandsListener finished gracefully. Daemon exiting.");
    Ok(())
}

/// Initialize logging (to journald for production, a file for testing)
fn init_journald_logging() -> Result<()> {
    let journald_layer = tracing_journald::layer().map_err(|e| {
        eprintln!("Failed to initialize journald logging: {}", e);
        unexpected_err("Failed to initialize journald logging", Some(e.to_string()))
    })?;

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(journald_layer)
        .try_init()
        .map_err(|e| {
            eprintln!("Failed to initialize journald tracing subscriber: {}", e);
            unexpected_err("Failed to initialize journald tracing subscriber", Some(e.to_string()))
        })
}
