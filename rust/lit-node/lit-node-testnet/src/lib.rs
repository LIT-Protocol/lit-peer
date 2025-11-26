pub mod end_user;
pub mod models;
pub mod node_collection;
pub mod rand;
pub mod testnet;
pub mod validator;

use self::testnet::Testnet;
use self::testnet::node_config::CustomNodeRuntimeConfig;
use self::validator::ValidatorCollection;
use crate::end_user::EndUser;
use crate::testnet::contracts::StakingContractRealmConfig;
use ethers::types::U256;
use lit_core::config::{ENV_LIT_CONFIG_FILE, LitConfigBuilder, ReloadableLitConfig};

use lit_observability::logging::simple_logging_subscriber;
use once_cell::sync::Lazy;
use std::{fs, path::Path, sync::Mutex};
use tracing::{debug, error};
use tracing_subscriber::util::SubscriberInitExt;

const DEFAULT_NUM_STAKED_AND_JOINED_VALIDATORS: usize = 5;

pub struct TestSetupBuilder {
    num_staked_and_joined_validators: usize,
    num_staked_only_validators: usize,
    is_fault_test: bool,
    force_deploy: bool,
    register_inactive_validators: bool,
    enable_payment: Option<String>,
    chain_polling_interval: Option<String>,
    epoch_length: Option<U256>,
    max_presign_count: Option<u64>,
    min_presign_count: Option<u64>,
    payment_interval_ms: Option<String>,
    wait_initial_epoch: bool,
    wait_for_root_keys: bool,
    fund_wallet: bool,
    fund_ledger_for_wallet: bool,
    custom_binary_path: Option<String>,
}

impl Default for TestSetupBuilder {
    fn default() -> Self {
        Self {
            num_staked_and_joined_validators: DEFAULT_NUM_STAKED_AND_JOINED_VALIDATORS,
            num_staked_only_validators: 0,
            is_fault_test: false,
            force_deploy: false,
            register_inactive_validators: false,
            enable_payment: Some("true".to_string()),
            chain_polling_interval: None,
            epoch_length: None,
            max_presign_count: Some(0),
            min_presign_count: Some(0),
            payment_interval_ms: None,
            wait_initial_epoch: true,
            wait_for_root_keys: true,
            fund_wallet: true,
            fund_ledger_for_wallet: true,
            custom_binary_path: None,
        }
    }
}

impl TestSetupBuilder {
    pub fn num_staked_and_joined_validators(
        mut self,
        num_staked_and_joined_validators: usize,
    ) -> Self {
        self.num_staked_and_joined_validators = num_staked_and_joined_validators;
        self
    }

    pub fn is_fault_test(mut self, is_fault_test: bool) -> Self {
        self.is_fault_test = is_fault_test;
        self
    }

    pub fn num_staked_only_validators(mut self, num_staked_only_validators: usize) -> Self {
        self.num_staked_only_validators = num_staked_only_validators;
        self
    }

    pub fn force_deploy(mut self, force_deploy: bool) -> Self {
        self.force_deploy = force_deploy;
        self
    }

    pub fn register_inactive_validators(mut self, register_inactive_validators: bool) -> Self {
        self.register_inactive_validators = register_inactive_validators;
        self
    }

    pub fn enable_payment(mut self, enable_payment: String) -> Self {
        self.enable_payment = Some(enable_payment);
        self
    }

    pub fn payment_interval_ms(mut self, payment_interval_ms: Option<String>) -> Self {
        self.payment_interval_ms = payment_interval_ms;
        self
    }

    pub fn chain_polling_interval(mut self, chain_polling_interval: String) -> Self {
        self.chain_polling_interval = Some(chain_polling_interval);
        self
    }

    pub fn epoch_length(mut self, epoch_length: usize) -> Self {
        self.epoch_length = Some(U256::from(epoch_length));
        self
    }

    pub fn max_presign_count(mut self, max_presign_count: u64) -> Self {
        self.max_presign_count = Some(max_presign_count);
        self
    }

    pub fn min_presign_count(mut self, min_presign_count: u64) -> Self {
        self.min_presign_count = Some(min_presign_count);
        self
    }

    pub fn wait_initial_epoch(mut self, wait_initial_epoch: bool) -> Self {
        self.wait_initial_epoch = wait_initial_epoch;
        self
    }

    pub fn wait_for_root_keys(mut self, wait_for_root_keys: bool) -> Self {
        self.wait_for_root_keys = wait_for_root_keys;
        self
    }

    pub fn fund_wallet(mut self, fund_wallet: bool) -> Self {
        self.fund_wallet = fund_wallet;
        self
    }

    pub fn fund_ledger_for_wallet(mut self, fund_ledger_for_wallet: bool) -> Self {
        self.fund_ledger_for_wallet = fund_ledger_for_wallet;
        self
    }

    pub fn custom_binary_path(mut self, custom_binary_path: Option<String>) -> Self {
        self.custom_binary_path = custom_binary_path;
        self
    }

    pub async fn build(self) -> (Testnet, ValidatorCollection, EndUser) {
        let node_keys_path = Path::new("./node_keys");
        if node_keys_path.exists() {
            fs::remove_dir_all(node_keys_path).unwrap();
        }
        fs::create_dir_all(node_keys_path).unwrap();

        let custom_node_runtime_config = CustomNodeRuntimeConfig::builder()
            .enable_payment(self.enable_payment)
            .payment_interval_ms(self.payment_interval_ms)
            .chain_polling_interval(self.chain_polling_interval)
            .build();

        let mut testnet = Testnet::builder()
            .num_staked_and_joined_validators(self.num_staked_and_joined_validators)
            .register_inactive_validators(self.register_inactive_validators)
            .num_staked_only_validators(self.num_staked_only_validators)
            .is_fault_test(self.is_fault_test)
            .custom_node_runtime_config(custom_node_runtime_config)
            .force_deploy(self.force_deploy)
            .build()
            .await;

        let staking_contract_realm_config = StakingContractRealmConfig::builder()
            .epoch_length(self.epoch_length)
            .max_presign_count_u64(self.max_presign_count)
            .min_presign_count_u64(self.min_presign_count)
            .build();

        let _testnet_contracts =
            Testnet::setup_contracts(&mut testnet, None, Some(staking_contract_realm_config))
                .await
                .expect("Failed to setup contracts");

        // if this is a cached testnet, we're not sure about timestamps, blocks, etc,... reset!
        if testnet.is_from_cache {
            debug!("Cached testnet detected, resetting timestamps.");
            // mine a block to get a new timestamp - it's likely that the latest block timestamp is from the past!
            testnet.actions().fast_forward_blocks(1).await;
            let epoch_length = self.epoch_length.unwrap_or(U256::from(160));
            debug!("Extending epoch end time by {}.", epoch_length);
            if let Err(e) = testnet
                .actions()
                .set_epoch_end_time_from_now(U256::from(1), epoch_length)
                .await
            {
                error!("Error extending epoch end time: {:?}", e);
            }
        }

        let num_staked_nodes =
            self.num_staked_and_joined_validators + self.num_staked_only_validators;

        let node_binary_feature_flags = if self.is_fault_test {
            "lit-actions,testing,proxy_chatter".to_string()
        } else {
            "lit-actions,testing".to_string()
        };

        let validator_collection = ValidatorCollection::builder()
            .num_staked_nodes(num_staked_nodes)
            .wait_initial_epoch(self.wait_initial_epoch)
            .wait_for_root_keys(self.wait_for_root_keys)
            .node_binary_feature_flags(node_binary_feature_flags)
            .custom_binary_path(self.custom_binary_path)
            .build(&testnet)
            .await
            .expect("Failed to build validator collection");

        let mut end_user = EndUser::new(&testnet);
        if self.fund_wallet {
            end_user.fund_wallet_default_amount().await;
            if self.fund_ledger_for_wallet {
                end_user.deposit_to_wallet_ledger_default().await;
            }
        }
        let r = end_user.new_pkp().await;
        if let Err(e) = r {
            panic!("Error minting PKP: {:?}", e);
        }

        (testnet, validator_collection, end_user)
    }
}

static LOGGING_SETUP: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
#[doc = "Setup configuration as node #0 and logging for tests"]
pub fn setup_logging(log_id: &str) {
    if let Ok(mut lock) = LOGGING_SETUP.lock() {
        if *lock {
            return;
        }
        *lock = true;

        println!("Setting up logging for tests");
        unsafe {
            std::env::set_var(ENV_LIT_CONFIG_FILE, "./tests/lit_logging_config.toml");
        }
        let cfg = load_cfg().expect("failed to load LitConfig");

        // special prefix for testing
        match simple_logging_subscriber(cfg.load().as_ref(), Some(format!("{} -", log_id))) {
            Ok(sub) => {
                sub.init();
            }
            Err(e) => {
                eprintln!("Failed to setup logging: {}", e);
            }
        }
    }
}

fn load_cfg() -> lit_core::error::Result<ReloadableLitConfig> {
    ReloadableLitConfig::new(|| {
        let cfg = LitConfigBuilder::default().build()?;
        // Verify every load (will not replace running config unless it works)
        Ok(cfg)
    })
}
