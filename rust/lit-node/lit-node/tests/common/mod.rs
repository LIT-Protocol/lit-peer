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

use ethers::types::U256;
use lit_blockchain::contracts::staking::KeySetConfig;
use lit_core::config::ENV_LIT_CONFIG_FILE;
use lit_node::tss::util::DEFAULT_KEY_SET_NAME;
use lit_node_common::config::load_cfg;
use lit_node_core::CurveType;
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

pub fn get_default_keyset_configs() -> Vec<KeySetConfig> {
    vec![default_keyset_config()]
}
pub fn default_keyset_config() -> KeySetConfig {
    KeySetConfig {
        identifier: DEFAULT_KEY_SET_NAME.to_string(),
        description: String::new(),
        minimum_threshold: 3,
        monetary_value: 0,
        complete_isolation: false,
        realms: vec![U256::from(1)],
        curves: CurveType::into_iter().map(|c| c.into()).collect(),
        counts: std::iter::once(U256::from(1))
            .chain(CurveType::into_iter().skip(1).map(|_| U256::from(2)))
            .collect(),
        recovery_party_members: Vec::new(),
    }
}
