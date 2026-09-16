extern crate dotenv;

pub mod assertions;
pub mod auth_sig;
pub mod ecdsa;
pub mod faults;
pub mod interpolation;
pub mod lit_actions;
pub mod networking;
pub mod peers;
pub mod pkp;
pub mod recovery_party;
pub mod session_sigs;
pub mod version;
pub mod web_user_tests;

use lit_blockchain::resolver::contract::ContractResolver;
use lit_core::config::LitConfig;

use std::sync::Arc;

use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_node_common::config::load_cfg;
use lit_observability::logging::simple_logging_subscriber;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tracing_subscriber::util::SubscriberInitExt;

static LOGGING_SETUP: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
#[doc = "Setup configuration as node #0 and logging for tests"]
pub fn setup_logging() {
    if let Ok(mut lock) = LOGGING_SETUP.lock() {
        if *lock {
            return;
        }
        *lock = true;

        debug!("Setting up logging for tests");
        unsafe {
            std::env::set_var(ENV_LIT_CONFIG_FILE, "./tests/lit_logging_config.toml");
        }
        let cfg = load_cfg().expect("failed to load LitConfig");

        // special prefix for testing
        match simple_logging_subscriber(cfg.load().as_ref(), Some("TEST -".to_string())) {
            Ok(sub) => {
                sub.init();
            }
            Err(e) => {
                error!("Failed to setup logging: {}", e);
            }
        }
    }
}

pub fn load_config() -> (Arc<LitConfig>, Arc<ContractResolver>) {
    // Load config
    let cfg = load_cfg().expect("failed to load LitConfig");
    let loaded_config = cfg.load_full();

    let resolver = Arc::new(
        ContractResolver::try_from(cfg.load().as_ref()).expect("failed to load ContractResolver"),
    );

    (loaded_config, resolver)
}
