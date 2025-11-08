#![cfg(test)]

use lit_node_operator::chronicle_replica::{
    ChronicleReplicaConfig, ChronicleReplicaManager, CommandRunner, ReplicaHealthStatus,
};
use lit_node_operator::error::Result;

use async_trait::async_trait;
use std::os::unix::process::ExitStatusExt;
use std::process::{ExitStatus, Output};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::{advance, pause};
use tracing::Level;

/// Shared state between tests and the mock command runner.
#[derive(Debug, Clone, Default)]
pub struct MockCommandState {
    pub ps_should_panic: bool,      // Simulate IO/Cmd error for ps by panicking
    pub ps_exists: bool,            // Simulate if container is found by ps
    pub inspect_should_panic: bool, // Simulate IO/Cmd error for inspect by panicking
    pub inspect_status: String,     // Health status string ("healthy", etc.)
    pub script_should_panic: bool,  // Simulate IO error for script execution by panicking
    pub script_exit_code: Option<i32>, // Simulate script non-zero exit code (returns Ok(Output))
    pub iptables_should_panic: bool, // Simulate IO/Cmd error for iptables by panicking

    // --- Verification fields ---
    pub ps_called_count: u32,
    pub inspect_called_count: u32,
    pub script_called_count: u32,
    pub firewall_actions: Vec<bool>, // true=ACCEPT, false=REJECT
    pub iptables_flush_calls: u32,
    pub iptables_add_rule_calls: u32,
}

pub type SharedMockState = Arc<Mutex<MockCommandState>>;

/// Mock implementation of CommandRunner for testing
#[derive(Clone)]
pub struct MockCommandRunner {
    state: SharedMockState,
}

impl MockCommandRunner {
    fn new(state: SharedMockState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl CommandRunner for MockCommandRunner {
    /// Simulates `docker ps`.
    async fn run_docker_ps(&self) -> Result<Output> {
        let mut guard = self.state.lock().expect("Mutex poisoned");
        guard.ps_called_count += 1;
        tracing::debug!(?guard, "MOCK: run_docker_ps called");

        if guard.ps_should_panic {
            panic!("Mock docker ps forced panic");
        } else {
            Ok(Output {
                status: ExitStatus::from_raw(0),
                stdout: if guard.ps_exists { b"mock_id".to_vec() } else { b"".to_vec() },
                stderr: vec![],
            })
        }
    }

    /// Simulates `docker inspect`.
    async fn run_docker_inspect(&self) -> Result<Output> {
        let mut guard = self.state.lock().expect("Mutex poisoned");
        guard.inspect_called_count += 1;
        tracing::debug!(?guard, "MOCK: run_docker_inspect called");

        if guard.inspect_should_panic {
            panic!("Mock docker inspect forced panic");
        } else {
            Ok(Output {
                status: ExitStatus::from_raw(0),
                stdout: guard.inspect_status.as_bytes().to_vec(),
                stderr: vec![],
            })
        }
    }

    /// Simulates running the script. Returns Ok(Output{non-zero}) for script failure,
    /// Panics for IO failure.
    async fn run_recreation_script(&self) -> Result<Output> {
        let mut guard = self.state.lock().expect("Mutex poisoned");
        guard.script_called_count += 1;
        tracing::debug!(?guard, "MOCK: run_recreation_script called");

        if guard.script_should_panic {
            // Simulate IO error case
            panic!("Mock script execution forced panic");
        } else if let Some(exit_code) = guard.script_exit_code {
            // Simulate script non-zero exit
            Ok(Output {
                status: ExitStatus::from_raw(exit_code << 8),
                stdout: b"Mock script failed!".to_vec(),
                stderr: b"Simulated script error".to_vec(),
            })
        } else {
            Ok(Output {
                status: ExitStatus::from_raw(0),
                stdout: b"Mock script executed successfully".to_vec(),
                stderr: vec![],
            })
        }
    }

    /// Simulates running iptables.
    async fn run_iptables_cmd(&self, args: &[&str]) -> Result<()> {
        let mut guard = self.state.lock().expect("Mutex poisoned");
        tracing::debug!(?args, ?guard, "MOCK: run_iptables_cmd called");

        // Record actions for verification
        if args.contains(&"-F") {
            guard.iptables_flush_calls += 1;
        }

        if args.contains(&"-A") {
            guard.iptables_add_rule_calls += 1;
            if args.contains(&"ACCEPT") {
                guard.firewall_actions.push(true);
            } else if args.contains(&"DROP") || args.contains(&"REJECT") {
                guard.firewall_actions.push(false);
            }
        }

        if guard.iptables_should_panic {
            panic!("Mock iptables command forced panic");
        } else {
            Ok(())
        }
    }
}

// --- Test Utilities ---

/// Initialize tracing for tests
fn init_test_logging() {
    let _ = tracing_subscriber::fmt().with_max_level(Level::DEBUG).with_test_writer().try_init();
}

/// Create a manager and state for testing
fn setup_test_manager(
    config: ChronicleReplicaConfig,
) -> (ChronicleReplicaManager<MockCommandRunner>, SharedMockState) {
    let mock_state = Arc::new(Mutex::new(MockCommandState::default()));
    let mock_runner = MockCommandRunner::new(mock_state.clone());
    let manager = ChronicleReplicaManager::new(config, mock_runner);
    (manager, mock_state)
}

/// Create a test configuration with specified timeouts
fn test_config(startup_timeout_secs: u64, unhealthy_timeout_secs: u64) -> ChronicleReplicaConfig {
    ChronicleReplicaConfig {
        enable_local_replica: true,
        check_interval_seconds: 1,
        docker_startup_timeout_seconds: startup_timeout_secs,
        unhealthy_recreation_timeout_seconds: unhealthy_timeout_secs,
    }
}

// --- Test Cases ---

#[tokio::test]
async fn initial_state_is_not_found_and_firewall_blocked() {
    init_test_logging();
    let config = test_config(10, 20);
    let (manager, _) = setup_test_manager(config);

    assert_eq!(*manager.health_status(), ReplicaHealthStatus::NotFound);
    assert!(!manager.is_firewall_accepting());
    assert!(manager.unhealthy_since().is_none());
    assert!(manager.starting_since().is_some());
}

#[tokio::test]
async fn not_found_triggers_recreation() {
    init_test_logging();
    let (mut manager, mock_state) = setup_test_manager(test_config(10, 20));
    // Manager starts with firewall_accepting = false

    // Setup Mock: Docker reports container not found
    {
        mock_state.lock().unwrap().ps_exists = false;
    }

    // Action
    manager.check_and_manage_replica().await.expect("Check cycle failed");

    // Assertions
    {
        let state = mock_state.lock().unwrap();
        assert_eq!(state.ps_called_count, 1);
        assert_eq!(state.inspect_called_count, 0);
        assert_eq!(state.script_called_count, 1); // Recreate script called
        // FIX: Assert that no firewall action was needed because it was already blocking
        assert!(state.firewall_actions.is_empty());
    }
    // Check manager state via getters
    assert_eq!(*manager.health_status(), ReplicaHealthStatus::Starting); // State after recreate
    assert!(!manager.is_firewall_accepting()); // Firewall state unchanged (still false)
    assert!(manager.starting_since().is_some());
}

#[tokio::test]
async fn starting_within_timeout_no_recreation() {
    init_test_logging();
    pause(); // Freeze time for testing timeouts

    let startup_timeout = 10;
    let (mut manager, mock_state) = setup_test_manager(test_config(startup_timeout, 20));

    // Setup initial state (container not found -> triggers recreation)
    {
        mock_state.lock().unwrap().ps_exists = false;
    }
    manager.check_and_manage_replica().await.expect("Setup 1");
    let initial_start_time = manager.starting_since();

    // Reset counters for the actual test
    {
        let mut s = mock_state.lock().unwrap();
        s.script_called_count = 0;
        s.firewall_actions.clear();
        s.ps_called_count = 0;
    }

    // Setup state for test: container exists but is starting
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "starting".to_string();
    }

    // Advance time but not past timeout
    advance(Duration::from_secs(startup_timeout - 1)).await;
    manager.check_and_manage_replica().await.expect("Check cycle failed");

    // Assert no recreation happened
    {
        let s = mock_state.lock().unwrap();
        assert_eq!(s.script_called_count, 0);
        assert!(s.firewall_actions.is_empty());
    }
    assert_eq!(*manager.health_status(), ReplicaHealthStatus::Starting);
    assert!(!manager.is_firewall_accepting());
    assert_eq!(manager.starting_since(), initial_start_time);
}

#[tokio::test]
async fn starting_exceeds_timeout_triggers_recreation() {
    init_test_logging();
    pause(); // Freeze time for testing timeouts

    let startup_timeout = 10;
    let (mut manager, mock_state) = setup_test_manager(test_config(startup_timeout, 20));

    // Setup initial state (container not found -> triggers recreation)
    {
        mock_state.lock().unwrap().ps_exists = false;
    }
    manager.check_and_manage_replica().await.expect("Setup 1");
    let first_start_time = manager.starting_since();

    // Reset counters for the actual test
    {
        let mut s = mock_state.lock().unwrap();
        s.script_called_count = 0;
        s.firewall_actions.clear();
        s.ps_called_count = 0;
        s.inspect_called_count = 0;
    }

    // Setup state for test: container exists but is starting
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "starting".to_string();
    }

    // Advance time past timeout
    advance(Duration::from_secs(startup_timeout + 1)).await;
    manager.check_and_manage_replica().await.expect("Check cycle failed");

    // Assert recreation happened
    {
        assert_eq!(mock_state.lock().unwrap().script_called_count, 1);
    }
    assert_eq!(*manager.health_status(), ReplicaHealthStatus::Starting);
    assert!(!manager.is_firewall_accepting());
    assert!(manager.starting_since().is_some());
    assert_ne!(manager.starting_since(), first_start_time);
}

#[tokio::test]
async fn healthy_state_allows_firewall() {
    init_test_logging();
    let (mut manager, mock_state) = setup_test_manager(test_config(10, 20));

    // Setup initial state (container not found -> triggers recreation)
    {
        mock_state.lock().unwrap().ps_exists = false;
    }
    manager.check_and_manage_replica().await.expect("Setup 1");

    // Reset counters for the actual test
    {
        let mut s = mock_state.lock().unwrap();
        s.script_called_count = 0;
        s.firewall_actions.clear();
        s.ps_called_count = 0;
        s.inspect_called_count = 0;
    }

    // Setup state for test: container is healthy
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "healthy".to_string();
    }

    // Action
    manager.check_and_manage_replica().await.expect("Check cycle to Healthy");

    // Assert firewall is now accepting
    {
        assert_eq!(mock_state.lock().unwrap().firewall_actions.last(), Some(&true));
    }
    assert_eq!(*manager.health_status(), ReplicaHealthStatus::Healthy);
    assert!(manager.is_firewall_accepting());
    assert!(manager.starting_since().is_none());
    assert!(manager.unhealthy_since().is_none());

    // Next cycle with already-healthy state shouldn't change firewall
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "healthy".to_string();
        s.firewall_actions.clear();
    }
    manager.check_and_manage_replica().await.expect("Subsequent Healthy check");

    // Assert no new firewall actions
    {
        assert!(mock_state.lock().unwrap().firewall_actions.is_empty());
    }
    assert_eq!(*manager.health_status(), ReplicaHealthStatus::Healthy);
    assert!(manager.is_firewall_accepting());
}

#[tokio::test]
async fn unhealthy_state_blocks_firewall_and_starts_timer() {
    init_test_logging();
    let (mut manager, mock_state) = setup_test_manager(test_config(10, 20));

    // First setup not found -> recreation
    {
        mock_state.lock().unwrap().ps_exists = false;
    }
    manager.check_and_manage_replica().await.expect("Setup 1");

    // Then setup healthy state
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "healthy".to_string();
        s.script_called_count = 0;
        s.firewall_actions.clear();
        s.ps_called_count = 0;
        s.inspect_called_count = 0;
    }
    manager.check_and_manage_replica().await.expect("Setup 2");
    assert!(manager.is_firewall_accepting());

    // Reset for test
    {
        mock_state.lock().unwrap().firewall_actions.clear();
    }

    // Setup unhealthy state for test
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "unhealthy".to_string();
    }

    // Action
    manager.check_and_manage_replica().await.expect("Check cycle to Unhealthy");

    // Assert firewall is now blocking
    {
        assert_eq!(mock_state.lock().unwrap().firewall_actions.last(), Some(&false));
    }
    assert_eq!(*manager.health_status(), ReplicaHealthStatus::Unhealthy);
    assert!(!manager.is_firewall_accepting());
    assert!(manager.unhealthy_since().is_some());
}

#[tokio::test]
#[should_panic(expected = "Mock script execution forced panic")] // Expect panic if script IO fails
async fn unhealthy_timeout_triggers_recreation_panic_on_script_io_error() {
    init_test_logging();
    pause(); // Freeze time for testing timeouts

    let unhealthy_timeout = 10;
    let (mut manager, mock_state) = setup_test_manager(test_config(5, unhealthy_timeout));

    // Setup initial state (not found -> recreation)
    {
        mock_state.lock().unwrap().ps_exists = false;
    }
    manager.check_and_manage_replica().await.expect("Setup 1");

    // Setup unhealthy state
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "unhealthy".to_string();
        s.script_called_count = 0;
        s.firewall_actions.clear();
        s.ps_called_count = 0;
        s.inspect_called_count = 0;
    }
    manager.check_and_manage_replica().await.expect("Setup 2");

    // Setup mock to panic when script is called
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "unhealthy".to_string();
        s.script_should_panic = true;
    }

    // Advance past timeout
    advance(Duration::from_secs(unhealthy_timeout + 1)).await;

    // This call should panic inside script execution
    manager.check_and_manage_replica().await.expect("Check cycle should panic");
}

#[tokio::test]
async fn unhealthy_timeout_triggers_recreation_script_exit_error() {
    init_test_logging();
    pause(); // Freeze time for testing timeouts

    let unhealthy_timeout = 10;
    let (mut manager, mock_state) = setup_test_manager(test_config(5, unhealthy_timeout));

    // Setup initial state (not found -> recreation)
    {
        mock_state.lock().unwrap().ps_exists = false;
    }
    manager.check_and_manage_replica().await.expect("Setup 1");

    // Setup unhealthy state
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "unhealthy".to_string();
        s.script_called_count = 0;
        s.firewall_actions.clear();
        s.ps_called_count = 0;
        s.inspect_called_count = 0;
    }
    manager.check_and_manage_replica().await.expect("Setup 2");

    // Setup script to return error code
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "unhealthy".to_string();
        s.script_exit_code = Some(1);
    }

    // Advance past timeout
    advance(Duration::from_secs(unhealthy_timeout + 1)).await;

    // Check that the function returns an error when script fails
    let result = manager.check_and_manage_replica().await;
    assert!(result.is_err());

    // Assert state didn't reset because recreation failed
    assert_eq!(*manager.health_status(), ReplicaHealthStatus::Unhealthy);
    assert!(manager.unhealthy_since().is_some()); // Timer still running

    // Assert script was called
    {
        assert_eq!(mock_state.lock().unwrap().script_called_count, 1);
    }
}

#[tokio::test]
async fn recovery_unhealthy_to_healthy() {
    init_test_logging();
    let (mut manager, mock_state) = setup_test_manager(test_config(10, 20));

    // Setup initial state (not found -> recreation)
    {
        mock_state.lock().unwrap().ps_exists = false;
    }
    manager.check_and_manage_replica().await.expect("Setup 1");

    // Setup unhealthy state
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "unhealthy".to_string();
        s.script_called_count = 0;
        s.firewall_actions.clear();
        s.ps_called_count = 0;
        s.inspect_called_count = 0;
    }
    manager.check_and_manage_replica().await.expect("Setup 2");
    assert!(!manager.is_firewall_accepting());
    assert!(manager.unhealthy_since().is_some());

    // Reset for test
    {
        mock_state.lock().unwrap().firewall_actions.clear();
    }

    // Setup healthy state for test
    {
        let mut s = mock_state.lock().unwrap();
        s.ps_exists = true;
        s.inspect_status = "healthy".to_string();
    }

    // Action - transition to healthy
    manager.check_and_manage_replica().await.expect("Check cycle to Healthy");

    // Assert now healthy and accepting
    {
        assert_eq!(mock_state.lock().unwrap().firewall_actions.last(), Some(&true));
    }
    assert_eq!(*manager.health_status(), ReplicaHealthStatus::Healthy);
    assert!(manager.is_firewall_accepting());
    assert!(manager.unhealthy_since().is_none());
}

#[tokio::test]
async fn monitoring_disabled_forces_firewall_block() {
    init_test_logging();

    // Setup with monitoring disabled
    let (mut manager, mock_state) = setup_test_manager(ChronicleReplicaConfig {
        enable_local_replica: false,
        ..test_config(10, 20)
    });

    // Action - run check with monitoring disabled
    manager.check_and_manage_replica().await.expect("Check cycle (disabled) failed");

    // Assert proper behavior
    {
        let state = mock_state.lock().unwrap();
        assert!(state.firewall_actions.last().map_or(true, |&a| !a)); // Ensure blocking (or empty)
        assert_eq!(state.ps_called_count, 0); // No Docker checks performed
        assert_eq!(state.inspect_called_count, 0);
        assert_eq!(state.script_called_count, 0);
    }
    assert!(!manager.is_firewall_accepting());

    // Second check should do nothing since already blocked
    {
        let mut s = mock_state.lock().unwrap();
        s.firewall_actions.clear();
        s.ps_called_count = 0;
    }
    manager.check_and_manage_replica().await.expect("Check cycle (disabled, 2nd) failed");

    // Assert no new firewall actions
    {
        assert!(mock_state.lock().unwrap().firewall_actions.is_empty());
    }
    assert!(!manager.is_firewall_accepting());
}
