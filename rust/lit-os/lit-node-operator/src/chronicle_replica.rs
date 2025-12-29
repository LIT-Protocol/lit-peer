//! Manages the Yellowstone Chronicle replica container's lifecycle and network access control.
//! Uses a CommandRunner trait for testability. Assumes external script handles iptables setup.

use crate::error::{EC, Result, unexpected_err_code};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::path::Path;
use std::process::{Output, Stdio};
use std::time::Duration;
use tokio::process::Command;
use tokio::time::Instant;
use tracing::{debug, error, info, warn};

// --- Static Configuration ---
const DOCKER_CONTAINER_NAME: &str = "yellowstone";
const START_SCRIPT_PATH: &str = "/var/chronicle/start_yellowstone_replica.sh";
const IPTABLES_CHAIN: &str = "YELLOWSTONE_ACCESS";

// --- Default Runtime Configuration Values ---
pub const DEFAULT_CHECK_INTERVAL_SECS: u64 = 20;
pub const DEFAULT_DOCKER_STARTUP_TIMEOUT_SECS: u64 = 900;
pub const DEFAULT_UNHEALTHY_RECREATION_TIMEOUT_SECS: u64 = 86400; // 24 hours
pub const DEFAULT_MAX_SYNCING_WHILE_UNHEALTHY_TIMEOUT_SECS: u64 = 3600; // 1 hour

/// Represents the observed health status of the replica container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplicaHealthStatus {
    /// Container is running but health check is pending or indicates 'starting'
    Starting,
    /// Container is running and health check reports 'healthy'
    Healthy,
    /// Container is running but health check reports 'unhealthy' (e.g., sync diff)
    Unhealthy,
    /// Container is running but health status couldn't be determined
    NoResponse,
    /// Container process does not exist or is stopped
    NotFound,
}

/// Holds the runtime state managed by the ChronicleReplicaManager.
#[derive(Debug)]
pub struct ReplicaState {
    /// Current perceived health status from Docker
    health_status: ReplicaHealthStatus,
    /// Tracks if the firewall is currently configured to ACCEPT (true) or REJECT (false)
    firewall_accepting: bool,
    /// When the last check cycle began
    last_check_time: Instant,
    /// When the container first entered an unhealthy state in the current sequence
    unhealthy_since: Option<Instant>,
    /// When the container first entered the starting state or was last recreated
    starting_since: Option<Instant>,
    /// When the container started being unhealthy AND reporting eth_syncing=true
    syncing_while_unhealthy_since: Option<Instant>,
}

/// Runtime configuration for the Chronicle Replica Manager.
#[derive(Clone, Debug)]
pub struct ChronicleReplicaConfig {
    /// Master switch to enable/disable management of the local replica
    pub enable_local_replica: bool,
    /// How often (in seconds) to check the replica's health status
    pub check_interval_seconds: u64,
    /// How long (in seconds) to wait for a container in 'Starting' state before recreating it
    pub docker_startup_timeout_seconds: u64,
    /// How long (in seconds) to tolerate an unhealthy container before recreating it
    pub unhealthy_recreation_timeout_seconds: u64,
    /// How long (in seconds) to tolerate an unhealthy container reporting eth_syncing=true before recreating it
    pub max_syncing_while_unhealthy_timeout_seconds: u64,
}

// Default implementations for State and Config
impl Default for ReplicaState {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            health_status: ReplicaHealthStatus::NotFound,
            firewall_accepting: false,
            last_check_time: now,
            unhealthy_since: None,
            starting_since: Some(now),
            syncing_while_unhealthy_since: None,
        }
    }
}

impl Default for ChronicleReplicaConfig {
    fn default() -> Self {
        Self {
            enable_local_replica: true,
            check_interval_seconds: DEFAULT_CHECK_INTERVAL_SECS,
            docker_startup_timeout_seconds: DEFAULT_DOCKER_STARTUP_TIMEOUT_SECS,
            unhealthy_recreation_timeout_seconds: DEFAULT_UNHEALTHY_RECREATION_TIMEOUT_SECS,
            max_syncing_while_unhealthy_timeout_seconds:
                DEFAULT_MAX_SYNCING_WHILE_UNHEALTHY_TIMEOUT_SECS,
        }
    }
}

/// Defines the interface for executing external commands needed by the manager.
/// Allows swapping implementations for testing.
#[async_trait]
pub trait CommandRunner: Send + Sync {
    /// Runs 'docker ps' check.
    async fn run_docker_ps(&self) -> Result<Output>;
    /// Runs 'docker inspect' check for health status.
    async fn run_docker_inspect_health(&self) -> Result<Output>;
    /// Runs 'docker inspect' to get container start time.
    async fn get_container_start_time(&self) -> Result<Option<DateTime<Utc>>>;
    /// Runs 'docker exec' to check eth_syncing status inside the container.
    async fn check_eth_syncing_inside_container(&self) -> Result<bool>;
    /// Runs the recreation script.
    async fn run_recreation_script(&self) -> Result<Output>;
    /// Runs an iptables command.
    async fn run_iptables_cmd(&self, args: &[&str]) -> Result<()>;
}

/// Executes real commands using tokio::process::Command.
#[derive(Debug, Clone, Default)]
pub struct RealCommandRunner;

#[async_trait]
impl CommandRunner for RealCommandRunner {
    async fn run_docker_ps(&self) -> Result<Output> {
        Command::new("docker")
            .args(["ps", "-q", "--filter", &format!("name=^{}$", DOCKER_CONTAINER_NAME)])
            .output()
            .await
            .map_err(|e| {
                unexpected_err_code(
                    e,
                    EC::ReplicaIoError,
                    Some("IO error executing 'docker ps'".to_string()),
                )
            })
    }

    async fn run_docker_inspect_health(&self) -> Result<Output> {
        Command::new("docker")
            .args([
                "inspect",
                "--format",
                "{{if .State.Health}}{{.State.Health.Status}}{{else}}no-healthcheck{{end}}",
                DOCKER_CONTAINER_NAME,
            ])
            .output()
            .await
            .map_err(|e| {
                unexpected_err_code(
                    e,
                    EC::ReplicaIoError,
                    Some("IO error executing 'docker inspect' for health".to_string()),
                )
            })
    }

    async fn get_container_start_time(&self) -> Result<Option<DateTime<Utc>>> {
        let output = Command::new("docker")
            .args(["inspect", "--format", "{{.State.StartedAt}}", DOCKER_CONTAINER_NAME])
            .output()
            .await
            .map_err(|e| {
                unexpected_err_code(
                    e,
                    EC::ReplicaIoError,
                    Some("IO error executing 'docker inspect' for start time".to_string()),
                )
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            warn!(
                "Failed to get container start time via docker inspect: {}. Assuming no start time available.",
                stderr
            );
            return Ok(None);
        }

        let started_at_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if started_at_str.is_empty() || started_at_str == "<no value>" {
            warn!("Docker inspect returned no start time value.");
            return Ok(None);
        }

        match DateTime::parse_from_rfc3339(&started_at_str) {
            Ok(dt) => Ok(Some(dt.with_timezone(&Utc))),
            Err(e) => {
                error!("Failed to parse container start time '{}': {}", started_at_str, e);
                Err(unexpected_err_code(
                    e,
                    EC::ReplicaIoError,
                    Some("Failed to parse docker start time".to_string()),
                ))
            }
        }
    }

    async fn check_eth_syncing_inside_container(&self) -> Result<bool> {
        let cmd_args = [
            "exec",
            DOCKER_CONTAINER_NAME,
            "curl",
            "-s",
            "-X",
            "POST",
            "--header",
            "Content-Type: application/json",
            "--data",
            r#"{"jsonrpc":"2.0","method":"eth_syncing","params":[],"id":1}"#,
            "http://localhost:8547",
        ];

        let output = Command::new("docker").args(&cmd_args).output().await.map_err(|e| {
            unexpected_err_code(
                e,
                EC::ReplicaIoError,
                Some("IO error running docker exec curl for eth_syncing".to_string()),
            )
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            error!("'docker exec curl eth_syncing' command failed: {}", stderr);
            return Ok(false);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        match serde_json::from_str::<Value>(&stdout) {
            Ok(json) => {
                let is_syncing = json.get("result").map_or(false, |res| res.is_object());
                debug!("eth_syncing check inside container result: syncing = {}", is_syncing);
                Ok(is_syncing)
            }
            Err(e) => {
                error!(
                    "Failed to parse JSON response from eth_syncing curl: {}. Response body: {}",
                    e, stdout
                );
                Ok(false)
            }
        }
    }

    async fn run_recreation_script(&self) -> Result<Output> {
        if !Path::new(START_SCRIPT_PATH).exists() {
            return Err(unexpected_err_code(
                format!("Recreate script not found: {}", START_SCRIPT_PATH),
                EC::ExternalCommandFailed,
                None,
            ));
        }

        Command::new("bash")
            .arg(START_SCRIPT_PATH)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .output()
            .await
            .map_err(|e| {
                unexpected_err_code(
                    e,
                    EC::ReplicaIoError,
                    Some("IO error executing recreate script".to_string()),
                )
            })
    }

    async fn run_iptables_cmd(&self, args: &[&str]) -> Result<()> {
        let cmd_str = format!("iptables {}", args.join(" "));
        debug!("Executing: {}", cmd_str);

        let status = Command::new("iptables").args(args).status().await.map_err(|e| {
            unexpected_err_code(
                e,
                EC::ReplicaIoError,
                Some("IO error running iptables".to_string()),
            )
        })?;

        if !status.success() {
            error!("iptables command failed: '{}' (status: {:?})", cmd_str, status.code());

            Err(unexpected_err_code(
                format!(
                    "iptables command execution failed: Command: {}, Exit code: {:?}",
                    cmd_str,
                    status.code()
                ),
                EC::ExternalCommandFailed,
                None,
            ))
        } else {
            Ok(())
        }
    }
}

/// Orchestrates monitoring, firewall control, and recreation of the Chronicle replica container.
/// Generic over R: CommandRunner to allow injecting real or mock runners for testing.
pub struct ChronicleReplicaManager<R: CommandRunner> {
    config: ChronicleReplicaConfig,
    state: ReplicaState,
    runner: R,
}

impl<R: CommandRunner + 'static> ChronicleReplicaManager<R> {
    /// Creates a new manager instance with the provided configuration and command runner.
    pub fn new(config: ChronicleReplicaConfig, runner: R) -> Self {
        debug!("Initializing ChronicleReplicaManager with config: {:?}", config);

        ChronicleReplicaManager { config, state: ReplicaState::default(), runner }
    }

    /// Returns the current observed health status.
    pub fn health_status(&self) -> &ReplicaHealthStatus {
        &self.state.health_status
    }

    /// Returns true if the firewall is currently configured to allow traffic.
    pub fn is_firewall_accepting(&self) -> bool {
        self.state.firewall_accepting
    }

    /// Returns the instant the container entered the unhealthy state, if currently unhealthy.
    pub fn unhealthy_since(&self) -> Option<Instant> {
        self.state.unhealthy_since
    }

    /// Returns the instant the container entered the starting state, if currently starting.
    pub fn starting_since(&self) -> Option<Instant> {
        self.state.starting_since
    }

    /// Returns the instant the container started syncing while unhealthy.
    pub fn syncing_while_unhealthy_since(&self) -> Option<Instant> {
        self.state.syncing_while_unhealthy_since
    }

    /// Returns the configured check interval in seconds.
    pub fn check_interval_seconds(&self) -> u64 {
        self.config.check_interval_seconds
    }

    /// Main entry point called periodically to check and manage the replica's state and firewall.
    pub async fn check_and_manage_replica(&mut self) -> Result<()> {
        if !self.config.enable_local_replica {
            info!(
                "Monitoring disabled via config. Ensuring firewall is definitively set to REJECT state."
            );
            self.update_firewall(false).await.map_err(|e| {
                error!(error=?e, "Failed trying to set firewall to REJECT because monitoring is disabled.");
                unexpected_err_code(
                    e,
                    EC::ExternalCommandFailed,
                    Some("Failed setting firewall reject (monitor disabled)".to_string()),
                )
            })?;

            self.state.firewall_accepting = false;
            debug!("Monitoring disabled check complete. Firewall set/verified REJECT.");
            return Ok(());
        }

        debug!("Running Chronicle replica health check cycle...");
        self.state.last_check_time = Instant::now();

        let previous_status = self.state.health_status.clone();
        let current_health = self.determine_replica_health_status().await?;
        self.log_state_transition(&previous_status, &current_health);
        self.state.health_status = current_health;

        match self.state.health_status {
            ReplicaHealthStatus::NotFound => self.handle_not_found_state().await?,
            ReplicaHealthStatus::Starting => self.handle_starting_state().await?,
            ReplicaHealthStatus::Healthy => self.handle_healthy_state().await?,
            ReplicaHealthStatus::Unhealthy | ReplicaHealthStatus::NoResponse => {
                self.handle_unhealthy_state(&previous_status).await?
            }
        }

        debug!("Replica health check cycle completed.");
        Ok(())
    }

    /// Handles NotFound state: ensures firewall blocks, attempts recreation.
    async fn handle_not_found_state(&mut self) -> Result<()> {
        if self.state.firewall_accepting {
            info!("Container not found, ensuring firewall is blocking.");
            self.update_firewall(false).await?;
            self.state.firewall_accepting = false;
        } else {
            debug!("Container not found, firewall already blocked.");
        }

        warn!("Container not found state triggered recreation via script.");
        self.recreate_container_via_script().await?;

        Ok(())
    }

    /// Handles Starting state: ensures firewall blocks, monitors startup duration using actual start time.
    async fn handle_starting_state(&mut self) -> Result<()> {
        let now_instant = Instant::now();
        let now_dt = Utc::now();

        self.state.unhealthy_since = None;
        self.state.syncing_while_unhealthy_since = None;

        if self.state.starting_since.is_none() {
            debug!("Entering Starting state, setting internal starting_since timer.");
            self.state.starting_since = Some(now_instant);
        }

        if self.state.firewall_accepting {
            warn!("Container is 'starting', ensuring firewall rules are blocking.");
            self.update_firewall(false).await?;
            self.state.firewall_accepting = false;
        } else {
            debug!("Container 'starting', firewall correctly blocking.");
        }

        debug!("Checking container startup duration timeout...");
        match self.runner.get_container_start_time().await {
            Ok(Some(start_time_dt)) => {
                debug!(start_time = %start_time_dt, "Successfully retrieved container actual start time.");
                let startup_duration = now_dt.signed_duration_since(start_time_dt);
                let startup_duration_std = match startup_duration.to_std() {
                    Ok(dur) => dur,
                    Err(_) => {
                        warn!(
                            "Calculated negative startup duration (clock skew?). Treating as zero."
                        );
                        Duration::ZERO
                    }
                };

                let timeout = Duration::from_secs(self.config.docker_startup_timeout_seconds);

                debug!(
                    start_time = %start_time_dt,
                    current_time = %now_dt,
                    duration_secs = startup_duration_std.as_secs(),
                    timeout_secs = timeout.as_secs(),
                    "Calculated startup duration vs timeout."
                );

                if startup_duration_std > timeout {
                    warn!(
                        start_time = %start_time_dt,
                        duration_secs = startup_duration_std.as_secs(),
                        timeout_secs = timeout.as_secs(),
                        "Container stuck in 'starting' state based on actual start time. Startup timeout exceeded, triggering recreation."
                    );
                    self.recreate_container_via_script().await?;
                } else {
                    debug!("Container startup duration within timeout.");
                }
            }
            Ok(None) => {
                warn!(
                    "Could not determine container actual start time while in Starting state. Timeout check skipped this cycle."
                );
            }
            Err(e) => {
                error!(error = ?e, "Error getting container start time. Timeout check skipped this cycle.");
            }
        }

        Ok(())
    }

    /// Handles Healthy state: ensures firewall accepts, resets state timers.
    async fn handle_healthy_state(&mut self) -> Result<()> {
        if self.state.unhealthy_since.is_some() || self.state.starting_since.is_some() {
            debug!("Container healthy, resetting unhealthy/starting timers.");
            self.state.unhealthy_since = None;
            self.state.starting_since = None;
            self.state.syncing_while_unhealthy_since = None;
        }

        if !self.state.firewall_accepting {
            info!("Container healthy, setting firewall rules to ACCEPT traffic.");
            self.update_firewall(true).await?;
            self.state.firewall_accepting = true;
        } else {
            debug!("Container healthy and firewall already accepting.");
        }

        Ok(())
    }

    /// Handles Unhealthy/NoResponse states: blocks traffic, tracks duration, checks eth_syncing, recreates if needed.
    async fn handle_unhealthy_state(
        &mut self, previous_status: &ReplicaHealthStatus,
    ) -> Result<()> {
        let now = Instant::now();

        // Set internal unhealthy_since timer if entering the state
        self.state.starting_since = None;
        if self.state.unhealthy_since.is_none() {
            warn!(
                status = ?self.state.health_status,
                "Container entered unhealthy state, starting unhealthy timer."
            );
            self.state.unhealthy_since = Some(now);
            self.state.syncing_while_unhealthy_since = None;
        }

        // Ensure firewall is blocked for unhealthy/non-responsive states
        info!(
            status = ?self.state.health_status,
            "Ensuring firewall is definitively set to REJECT state."
        );

        self.update_firewall(false).await.map_err(|e| {
            error!(error = ?e, "CRITICAL: Failed trying to set firewall to REJECT for unhealthy state.");
            e
        })?;

        self.state.firewall_accepting = false;

        if *previous_status == ReplicaHealthStatus::Healthy {
            warn!(
                from = ?previous_status,
                to = ?self.state.health_status,
                "Replica transitioned from Healthy state. Sync issue or check failure suspected."
            );
            warn!("Replica Out of Sync!!!");
        }

        // Check for unhealthy timeout
        if let Some(unhealthy_time) = self.state.unhealthy_since {
            let unhealthy_duration = now.duration_since(unhealthy_time);
            let timeout = Duration::from_secs(self.config.unhealthy_recreation_timeout_seconds);

            debug!(
                "Checking unhealthy duration: {:.1?} / {:.1?} seconds",
                unhealthy_duration.as_secs_f32(),
                timeout.as_secs_f32()
            );

            if unhealthy_duration > timeout {
                warn!(
                    status = ?self.state.health_status,
                    duration_secs = unhealthy_duration.as_secs(),
                    timeout_secs = timeout.as_secs(),
                    "Container has been in unhealthy state longer than timeout."
                );

                // Check eth_syncing before potentially recreating
                info!("Unhealthy timeout exceeded. Checking internal eth_syncing status...");
                match self.runner.check_eth_syncing_inside_container().await {
                    Ok(true) => {
                        if self.state.syncing_while_unhealthy_since.is_none() {
                            warn!(
                                "eth_syncing is TRUE inside container after unhealthy timeout. \
                                Starting max_syncing_while_unhealthy timer and deferring recreation."
                            );
                            self.state.syncing_while_unhealthy_since = Some(now);
                        } else {
                            let syncing_duration = now
                                .duration_since(self.state.syncing_while_unhealthy_since.unwrap());
                            let max_syncing_timeout = Duration::from_secs(
                                self.config.max_syncing_while_unhealthy_timeout_seconds,
                            );

                            info!(
                                "Checking max syncing duration: {:.1?} / {:.1?} seconds",
                                syncing_duration.as_secs_f32(),
                                max_syncing_timeout.as_secs_f32()
                            );

                            if syncing_duration > max_syncing_timeout {
                                warn!(
                                    status = ?self.state.health_status,
                                    unhealthy_duration_secs = unhealthy_duration.as_secs(),
                                    syncing_duration_secs = syncing_duration.as_secs(),
                                    max_sync_timeout_secs = max_syncing_timeout.as_secs(),
                                    "Container has been syncing while being unhealthy longer than max timeout. \
                                    Proceeding with recreation despite eth_syncing=true."
                                );
                                self.state.syncing_while_unhealthy_since = None;
                                self.recreate_container_via_script().await?;
                            } else {
                                warn!(
                                    "eth_syncing is TRUE inside container. Max sync duration {:.1?}s \
                                    is within timeout {:.1?}s. Deferring recreation.",
                                    syncing_duration.as_secs_f32(),
                                    max_syncing_timeout.as_secs_f32()
                                );
                            }
                        }
                    }
                    Ok(false) => {
                        warn!(
                            "eth_syncing is FALSE inside container after unhealthy timeout. Proceeding with recreation."
                        );
                        self.state.syncing_while_unhealthy_since = None;
                        self.recreate_container_via_script().await?;
                    }
                    Err(e) => {
                        // Error checking syncing status, log and recreate as a fallback
                        error!(error = ?e, "Error checking eth_syncing status. Proceeding with recreation as a fallback.");
                        self.state.syncing_while_unhealthy_since = None;
                        self.recreate_container_via_script().await?;
                    }
                }
            } else {
                if self.state.syncing_while_unhealthy_since.is_some() {
                    debug!(
                        "Resetting max_syncing_while_unhealthy timer as container unhealthy_since is None."
                    );
                    self.state.syncing_while_unhealthy_since = None;
                }
            }
        }

        Ok(())
    }

    /// Logs state transitions for important status changes.
    fn log_state_transition(&self, previous: &ReplicaHealthStatus, current: &ReplicaHealthStatus) {
        if previous != current {
            info!("Replica state transition: {:?} -> {:?}", previous, current);
        }
    }

    /// Determines the current health status of the container using Docker commands.
    async fn determine_replica_health_status(&self) -> Result<ReplicaHealthStatus> {
        let ps_output = self.runner.run_docker_ps().await?;

        if !ps_output.status.success() {
            let stderr = String::from_utf8_lossy(&ps_output.stderr).to_string();
            error!("'docker ps' command failed: {}", stderr);
            return Err(unexpected_err_code(
                format!("docker ps command failed: {}", stderr),
                EC::ExternalCommandFailed,
                None,
            ));
        }

        if ps_output.stdout.is_empty() {
            debug!("Container '{}' process not found.", DOCKER_CONTAINER_NAME);
            return Ok(ReplicaHealthStatus::NotFound);
        }

        let inspect_output = self.runner.run_docker_inspect_health().await?;

        if !inspect_output.status.success() {
            let stderr = String::from_utf8_lossy(&inspect_output.stderr).to_string();
            warn!("'docker inspect' command failed: {}. Assuming NoResponse state.", stderr);
            error!("'docker inspect' failed detail: {}", stderr);
            return Ok(ReplicaHealthStatus::NoResponse);
        }

        let health_status_str =
            String::from_utf8_lossy(&inspect_output.stdout).trim().to_lowercase();

        debug!("Docker health status reported: '{}'", health_status_str);

        match health_status_str.as_str() {
            "healthy" => Ok(ReplicaHealthStatus::Healthy),
            "starting" => Ok(ReplicaHealthStatus::Starting),
            "unhealthy" => Ok(ReplicaHealthStatus::Unhealthy),
            "no-healthcheck" | "" => {
                warn!("Container health status is '{}'. Treating as Starting.", health_status_str);
                Ok(ReplicaHealthStatus::Starting)
            }
            other => {
                warn!("Unexpected health status '{}'. Treating as Unhealthy.", other);
                Ok(ReplicaHealthStatus::Unhealthy)
            }
        }
    }

    /// Recreates the container using the external start script.
    async fn recreate_container_via_script(&mut self) -> Result<()> {
        info!("Executing container recreation script...");

        let output = self.runner.run_recreation_script().await?;

        debug!(status = ?output.status.code(), "Script execution finished (via runner)");

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            error!(
                script = START_SCRIPT_PATH,
                status = ?output.status.code(),
                "Script execution failed"
            );
            error!("Script stdout:\n{}", stdout);
            error!("Script stderr:\n{}", stderr);

            return Err(unexpected_err_code(
                format!("Recreation script failed: Exit code: {:?}", output.status.code()),
                EC::ExternalCommandFailed,
                None,
            ));
        }

        info!("Container recreation script executed successfully.");
        debug!("Script stdout:\n{}", String::from_utf8_lossy(&output.stdout));

        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            debug!("Script stderr:\n{}", stderr);
        }

        info!("Resetting internal state after successful container recreation.");
        self.state = ReplicaState::default();
        self.state.health_status = ReplicaHealthStatus::Starting;
        self.state.firewall_accepting = false;
        self.state.starting_since = Some(Instant::now());

        Ok(())
    }

    /// Updates iptables firewall rules (flushes chain, adds ACCEPT or REJECT).
    async fn update_firewall(&self, enable_accept: bool) -> Result<()> {
        let target_rule_args = if enable_accept {
            vec!["-A", IPTABLES_CHAIN, "-j", "ACCEPT"]
        } else {
            vec!["-A", IPTABLES_CHAIN, "-j", "REJECT", "--reject-with", "icmp-port-unreachable"]
        };

        let action_desc = if enable_accept { "ALLOW" } else { "REJECT" };
        info!(
            "Updating firewall: Setting {} rule in iptables chain '{}'",
            action_desc, IPTABLES_CHAIN
        );

        self.runner.run_iptables_cmd(&["-F", IPTABLES_CHAIN]).await.map_err(|e| {
            error!(
                "CRITICAL: Failed to flush iptables chain '{}'. Firewall state unknown.",
                IPTABLES_CHAIN
            );
            unexpected_err_code(
                e,
                EC::ExternalCommandFailed,
                Some("Failed to flush iptables chain".to_string()),
            )
        })?;

        self.runner.run_iptables_cmd(&target_rule_args).await.map_err(|e| {
            error!(
                "CRITICAL: Failed to add '{}' rule to iptables chain '{}'. Firewall state incorrect.",
                target_rule_args.join(" "),
                IPTABLES_CHAIN
            );
            unexpected_err_code(
                e,
                EC::ExternalCommandFailed,
                Some(format!("Failed to add iptables {} rule", action_desc)),
            )
        })?;

        info!(
            "Successfully updated firewall chain '{}' to {} (via runner)",
            IPTABLES_CHAIN, action_desc
        );

        Ok(())
    }
}
