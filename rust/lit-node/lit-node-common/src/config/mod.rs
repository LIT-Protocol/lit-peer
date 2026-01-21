use std::collections::HashMap;
use std::path::Path;

use async_std::path::PathBuf;
use ethers::types::H160;
use lit_api_core::config::{
    CFG_KEY_DOMAIN, CFG_KEY_ENABLED, CFG_KEY_PORT, CFG_KEY_REDIRECT_TO_HTTPS,
    CFG_KEY_TLS_AUTO_ENABLED,
};
use lit_api_core::config::{CFG_KEY_IDENT, LitApiConfig, http_section_key, https_section_key};
use lit_blockchain::config::{
    CFG_KEY_BLOCKCHAIN_CHAIN_ID, CFG_KEY_BLOCKCHAIN_CHAIN_NAME, LitBlockchainConfig,
};
use lit_blockchain::resolver::rpc::{ENDPOINT_MANAGER, RpcHealthcheckPoller};
use lit_core::config::{LitConfig, LitConfigBuilder, ReloadableLitConfig};
use lit_logging::config::LitLoggingConfig;
use url::Url;

use crate::error::{Result, parser_err, validation_err};

pub mod config_names;

pub const CFG_SECTION_KEY: &str = "node";

use config_names::{
    CFG_KEY_ACTIONS_SOCKET, CFG_KEY_ADMIN_ADDRESS, CFG_KEY_CHAIN_POLLING_INTERVAL_MS,
    CFG_KEY_CHATTER_CLIENT_TIMEOUT, CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY,
    CFG_KEY_COMS_KEYS_SENDER_PRIVKEY, CFG_KEY_ENABLE_ACTIONS_ALLOWLIST,
    CFG_KEY_ENABLE_EPOCH_TRANSITIONS, CFG_KEY_ENABLE_OBSERVABILITY_EXPORT, CFG_KEY_ENABLE_PAYMENT,
    CFG_KEY_ENABLE_PROXIED_CHATTER_CLIENT, CFG_KEY_ENABLE_SIWE_VALIDATION, CFG_KEY_GRPC_POOL_SIZE,
    CFG_KEY_GRPC_SERVER_CONC_LIMIT_PER_CONN, CFG_KEY_HEALTH_POLL_INTERVAL_MS,
    CFG_KEY_PAYMENT_INTERVAL_MS, CFG_KEY_RESTORE_LOG_INTERVAL_MS, CFG_KEY_RPC_URL,
    CFG_KEY_SIGNING_ROUND_TIMEOUT, CFG_KEY_STAKER_ADDRESS, CFG_KEY_WEB_CLIENT_TIMEOUT_SEC,
    CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS,
};
// NB: Before adding keys here ensure they don't conflict with LitApiConfig
// - port, address, ident e.t.c. are all reserved.
// pub static CFG_KEY_RPC_URL: &str = "rpc_url";
// pub static CFG_KEY_STAKER_ADDRESS: &str = "staker_address";
// pub static CFG_KEY_COMS_KEYS_SENDER_PRIVKEY: &str = "coms_keys_sender_privkey";
// pub static CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY: &str = "coms_keys_receiver_privkey";
// pub static CFG_KEY_ADMIN_ADDRESS: &str = "admin_address";
// pub static CFG_KEY_ENABLE_PROXIED_CHATTER_CLIENT: &str = "enable_proxied_chatter_client";
// pub static CFG_KEY_ENABLE_PAYMENT: &str = "enable_payment";
// pub static CFG_KEY_ENABLE_ACTIONS_ALLOWLIST: &str = "enable_actions_allowlist";
// pub static CFG_KEY_ENABLE_EPOCH_TRANSITIONS: &str = "enable_epoch_transitions";
// pub static CFG_KEY_ENABLE_OBSERVABILITY_EXPORT: &str = "enable_observability_export";
// pub static CFG_KEY_ECDSA_ROUND_TIMEOUT: &str = "ecdsa_round_timeout";
// pub static CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS: &str = "webauthn_allowed_origins";
// pub static CFG_KEY_CHAIN_POLLING_INTERVAL_MS: &str = "chain_polling_interval";
// pub static CFG_KEY_CHATTER_CLIENT_TIMEOUT: &str = "chatter_client_timeout";
// pub static CFG_KEY_ENABLE_SIWE_VALIDATION: &str = "enable_siwe_validation";
// pub static CFG_KEY_RESTORE_LOG_INTERVAL_MS: &str = "restore_log_interval";
// pub static CFG_KEY_ACTIONS_SOCKET: &str = "actions_socket";
// pub static CFG_KEY_HEALTH_POLL_INTERVAL_MS: &str = "health_poll_interval";
// pub static CFG_KEY_PAYMENT_INTERVAL_MS: &str = "payment_interval";

// Defaults
pub static CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT: i64 = 30000;
pub static CFG_KEY_SIGNING_ROUND_TIMEOUT_MS_DEFAULT: i64 = 30000;
pub static CFG_KEY_CHATTER_CLIENT_TIMEOUT_SECS_DEFAULT: i64 = 30;
pub static CFG_KEY_RESTORE_LOG_INTERVAL_MS_DEFAULT: i64 = 1000 * 60 * 10;
pub static CFG_KEY_ACTIONS_SOCKET_DEFAULT: &str = "/tmp/lit_actions.sock";
pub static CFG_KEY_PAYMENT_INTERVAL_MS_DEFAULT: i64 = 5000;
pub static CFG_KEY_WEB_CLIENT_TIMEOUT_SEC_DEFAULT: i64 = 30;

static REQUIRED_CFG_KEYS: [&str; 8] = [
    CFG_KEY_STAKER_ADDRESS,
    CFG_KEY_COMS_KEYS_SENDER_PRIVKEY,
    CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY,
    CFG_KEY_ADMIN_ADDRESS,
    CFG_KEY_ENABLE_PAYMENT,
    CFG_KEY_ENABLE_ACTIONS_ALLOWLIST,
    CFG_KEY_ENABLE_EPOCH_TRANSITIONS,
    CFG_KEY_DOMAIN,
];

static USER_EDITABLE_KEYS: [&str; 12] = [
    CFG_KEY_RPC_URL,
    CFG_KEY_ADMIN_ADDRESS,
    CFG_KEY_STAKER_ADDRESS,
    CFG_KEY_ENABLE_PROXIED_CHATTER_CLIENT,
    CFG_KEY_ENABLE_PAYMENT,
    CFG_KEY_ENABLE_ACTIONS_ALLOWLIST,
    CFG_KEY_ENABLE_EPOCH_TRANSITIONS,
    CFG_KEY_SIGNING_ROUND_TIMEOUT,
    CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS,
    CFG_KEY_CHAIN_POLLING_INTERVAL_MS,
    CFG_KEY_ENABLE_SIWE_VALIDATION,
    CFG_KEY_HEALTH_POLL_INTERVAL_MS,
];

static USER_EDITABLE_KEYS_IN_SECTIONS: [&str; 2] =
    [CFG_KEY_BLOCKCHAIN_CHAIN_ID, CFG_KEY_BLOCKCHAIN_CHAIN_NAME];

pub trait LitNodeConfig {
    fn try_new() -> Result<LitConfig>;
    fn from_builder(builder: LitConfigBuilder) -> Result<LitConfig>;
    fn verify(&self) -> Result<()>;
    fn export_user_editable(&self) -> Result<HashMap<String, String>>;
    fn verify_user_editable(&self, data: &HashMap<String, String>) -> Result<()>;

    // Accessors
    fn external_port(&self) -> Result<u16>;
    fn external_addr(&self) -> Result<String>;
    fn http_prefix_when_talking_to_other_nodes(&self) -> String;
    fn internal_port(&self) -> Result<u16>;
    fn rpc_url(&self) -> Result<String>;
    fn staker_address(&self) -> Result<String>;
    fn coms_keys_sender_privkey(&self) -> Result<String>;
    fn coms_keys_receiver_privkey(&self) -> Result<String>;
    fn admin_address(&self) -> Result<H160>;
    fn webauthn_allowed_origins(&self) -> Result<Vec<Url>>;
    fn chatter_client_timeout(&self) -> Result<u64>;
    fn grpc_server_concurrency_limit_per_connection(&self) -> Result<Option<u64>>;
    fn actions_socket(&self) -> Result<std::path::PathBuf>;

    // Feature flag bool accessors
    #[allow(dead_code)] // False positive
    fn enable_proxied_chatter_client(&self) -> Result<bool>;
    fn enable_payment(&self) -> Result<bool>;
    fn enable_actions_allowlist(&self) -> Result<bool>;
    fn enable_epoch_transitions(&self) -> Result<bool>;
    fn enable_siwe_validation(&self) -> Result<bool>;
    #[allow(dead_code)] // False positive
    fn enable_observability_export(&self) -> Result<bool>;

    // communications parameters for ECDSA rounds
    fn signing_round_timeout(&self) -> Result<i64>;
    fn chain_polling_interval_ms(&self) -> Result<i64>;

    // restore state parameters
    fn restore_log_interval(&self) -> Result<i64>;

    // endpoint polling and healthcheck
    fn rpc_health_poll_interval(&self) -> Result<i64>;

    fn grpc_pool_size(&self) -> Result<i64>;
    fn payment_interval_ms(&self) -> Result<i64>;
    fn web_client_timeout_s(&self) -> Result<i64>;
}

impl LitNodeConfig for LitConfig {
    fn try_new() -> Result<Self> {
        <LitConfig as LitNodeConfig>::from_builder(LitConfigBuilder::default())
    }

    fn from_builder(mut builder: LitConfigBuilder) -> Result<LitConfig> {
        // Set defaults
        builder = builder
            .set_key(Some(CFG_SECTION_KEY.into()))
            .set_section_default(CFG_KEY_IDENT, "Lit Node")
            // See LitApiConfig for other API options.
            .set_section_default(http_section_key(CFG_KEY_REDIRECT_TO_HTTPS), "true")
            .set_section_default(http_section_key(CFG_KEY_PORT), "8080")
            .set_section_default(http_section_key(CFG_KEY_ENABLED), "true")
            .set_section_default(https_section_key(CFG_KEY_PORT), "8443")
            .set_section_default(https_section_key(CFG_KEY_ENABLED), "false")
            .set_section_default(CFG_KEY_TLS_AUTO_ENABLED, "false")
            .set_section_default(
                CFG_KEY_SIGNING_ROUND_TIMEOUT,
                CFG_KEY_SIGNING_ROUND_TIMEOUT_MS_DEFAULT.to_string(),
            )
            .set_section_default(
                CFG_KEY_CHATTER_CLIENT_TIMEOUT,
                CFG_KEY_CHATTER_CLIENT_TIMEOUT_SECS_DEFAULT.to_string(),
            )
            .set_section_default(CFG_KEY_ENABLE_PROXIED_CHATTER_CLIENT, "false")
            .set_section_default(CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS, "http://*/,https://*/")
            .set_section_default(
                CFG_KEY_CHAIN_POLLING_INTERVAL_MS,
                CFG_KEY_CHAIN_POLLING_INTERVAL_MS_DEFAULT.to_string(),
            )
            .set_section_default(
                CFG_KEY_RESTORE_LOG_INTERVAL_MS,
                CFG_KEY_RESTORE_LOG_INTERVAL_MS_DEFAULT.to_string(),
            )
            .set_section_default(CFG_KEY_ENABLE_OBSERVABILITY_EXPORT, "false")
            .set_section_default(CFG_KEY_ENABLE_SIWE_VALIDATION, "true")
            .set_section_default(CFG_KEY_ACTIONS_SOCKET, CFG_KEY_ACTIONS_SOCKET_DEFAULT)
            .set_section_default(CFG_KEY_HEALTH_POLL_INTERVAL_MS, "60000");

        // Apply others
        builder = <LitConfig as LitBlockchainConfig>::apply_defaults(builder)?;
        builder = <LitConfig as LitLoggingConfig>::apply_defaults(builder)?;

        <LitConfig as LitApiConfig>::from_builder(builder)
    }

    fn verify(&self) -> Result<()> {
        for req in REQUIRED_CFG_KEYS {
            self.get_section_checked_string(req)?;
        }

        self.rpc_url()?;
        self.api_port(self.https_enabled())?;
        self.api_port_external(self.https_enabled())?;
        self.blockchain_wallet_private_key(None)?;
        self.blockchain_chain_id()?;
        self.blockchain_chain_name()?;

        Ok(())
    }

    /// Export user editable config
    fn export_user_editable(&self) -> Result<HashMap<String, String>> {
        let mut map: HashMap<String, String> = HashMap::new();

        for key in USER_EDITABLE_KEYS {
            map.insert(
                format!("{}.{}", CFG_SECTION_KEY, key),
                self.get_section_string(key).unwrap_or("".into()),
            );
        }

        for key in USER_EDITABLE_KEYS_IN_SECTIONS {
            let val = if CFG_KEY_RPC_URL.eq(key) {
                self.rpc_url().unwrap_or("".into())
            } else {
                self.get_string(key).unwrap_or("".into())
            };

            map.insert(key.to_owned(), val);
        }

        Ok(map)
    }

    /// Verify the user editable config map (ensure keys are valid)
    fn verify_user_editable(&self, data: &HashMap<String, String>) -> Result<()> {
        for (full_key, _) in data.iter() {
            if let Some((section_key, key)) = full_key.split_once('.') {
                if (section_key != CFG_SECTION_KEY || !USER_EDITABLE_KEYS.contains(&key))
                    && !USER_EDITABLE_KEYS_IN_SECTIONS.contains(&full_key.as_str())
                {
                    return Err(validation_err(
                        format!("user editing of config key '{}' not allowed", full_key),
                        None,
                    ));
                }
            } else {
                return Err(validation_err(
                    format!("user editing of config key '{}' not allowed", full_key),
                    None,
                ));
            }
        }

        Ok(())
    }

    // Accessors

    /// The external port used to access the node (rocket may bind to a different port but iptables will
    /// forward to it).
    fn external_port(&self) -> Result<u16> {
        Ok(self.api_port_external(self.https_enabled())? as u16)
    }

    fn external_addr(&self) -> Result<String> {
        Ok(format!("{}:{}", self.api_domain()?, self.external_port()?))
    }

    fn http_prefix_when_talking_to_other_nodes(&self) -> String {
        if self.https_enabled() {
            "https://".to_string()
        } else {
            "http://".to_string()
        }
    }

    fn internal_port(&self) -> Result<u16> {
        Ok(self.api_port(self.https_enabled())? as u16)
    }

    fn rpc_url(&self) -> Result<String> {
        if let Ok(url) = self.get_section_string(CFG_KEY_RPC_URL) {
            return Ok(url);
        }

        ENDPOINT_MANAGER.rpc_url(self.blockchain_chain_name()?)
    }

    fn staker_address(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_STAKER_ADDRESS)
    }

    fn coms_keys_sender_privkey(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_COMS_KEYS_SENDER_PRIVKEY)
    }

    fn coms_keys_receiver_privkey(&self) -> Result<String> {
        self.get_section_string(CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY)
    }

    fn admin_address(&self) -> Result<H160> {
        self.get_section_string(CFG_KEY_ADMIN_ADDRESS)?
            .parse::<H160>()
            .map_err(|e| {
                parser_err(
                    e,
                    Some("Could not convert admin_address to H160".to_string()),
                )
            })
    }

    fn webauthn_allowed_origins(&self) -> Result<Vec<Url>> {
        let origins: Vec<String> = self
            .get_section_string(CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS)
            .map(|s| s.split(',').map(|s| s.to_string()).collect())?;

        let origins: Vec<Url> = origins
            .iter()
            .map(|s| {
                Url::parse(s).map_err(|e| {
                    parser_err(
                        e,
                        Some(format!(
                            "Could not parse webauthn_allowed_origins url: {}",
                            s
                        )),
                    )
                })
            })
            .collect::<Result<Vec<Url>>>()?;

        Ok(origins)
    }

    fn chatter_client_timeout(&self) -> Result<u64> {
        self.get_section_int(CFG_KEY_CHATTER_CLIENT_TIMEOUT)
            .map(|i| i as u64)
    }

    fn grpc_server_concurrency_limit_per_connection(&self) -> Result<Option<u64>> {
        match self.get_section_int(CFG_KEY_GRPC_SERVER_CONC_LIMIT_PER_CONN) {
            Err(e) => {
                // Assume all Config kind errors are due to the config key not being found.
                if e.is_kind(lit_core::error::Kind::Config, true) {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
            Ok(limit) => Ok(Some(limit as u64)),
        }
    }

    // Feature flag bool accessors
    fn enable_proxied_chatter_client(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_PROXIED_CHATTER_CLIENT)
    }

    fn enable_payment(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_PAYMENT)
    }

    fn enable_actions_allowlist(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_ACTIONS_ALLOWLIST)
    }

    fn enable_epoch_transitions(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_EPOCH_TRANSITIONS)
    }

    fn enable_siwe_validation(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_SIWE_VALIDATION)
    }

    fn enable_observability_export(&self) -> Result<bool> {
        self.get_section_bool(CFG_KEY_ENABLE_OBSERVABILITY_EXPORT)
    }

    fn signing_round_timeout(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_SIGNING_ROUND_TIMEOUT)
    }

    fn chain_polling_interval_ms(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_CHAIN_POLLING_INTERVAL_MS)
    }

    fn restore_log_interval(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_RESTORE_LOG_INTERVAL_MS)
    }

    fn actions_socket(&self) -> Result<std::path::PathBuf> {
        self.get_section_string(CFG_KEY_ACTIONS_SOCKET)
            .map(Into::into)
    }

    fn rpc_health_poll_interval(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_HEALTH_POLL_INTERVAL_MS)
    }

    fn payment_interval_ms(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_PAYMENT_INTERVAL_MS)
    }

    fn web_client_timeout_s(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_WEB_CLIENT_TIMEOUT_SEC)
    }

    fn grpc_pool_size(&self) -> Result<i64> {
        self.get_section_int(CFG_KEY_GRPC_POOL_SIZE)
    }
}

pub fn key_path(staker_address: &str) -> PathBuf {
    let staker_address = match staker_address.starts_with("0x") {
        true => staker_address.to_string(),
        false => format!("0x{}", staker_address),
    };
    let path_root = format!("./node_keys/{}", staker_address.to_lowercase());
    PathBuf::from(&path_root)
}

pub fn encrypted_key_path(staker_address: &str) -> PathBuf {
    let mut path = key_path(staker_address);
    path.push("encrypted");
    path
}

pub fn presign_path(key_type: &str, staker_address: &str) -> PathBuf {
    let mut path = key_path(staker_address);
    path.push("presigns");
    path.push(key_type);
    path
}

pub fn typed_key_path(key_type: &str, staker_address: &str) -> PathBuf {
    let mut path = key_path(staker_address);
    path.push(key_type);
    path
}

pub fn key_commitment_path(staker_address: &str) -> PathBuf {
    let mut path = key_path(staker_address);
    path.push("key_share_commitment");
    path
}

pub fn attested_wallet_key_path(staker_address: &str) -> PathBuf {
    let mut path = key_path(staker_address);
    path.push("attested_wallet_key");
    path
}

pub fn comms_key_path(staker_address: &str) -> PathBuf {
    let mut path = key_path(staker_address);
    path.push("comms_keys");
    path
}

pub fn segmented_paths(
    path: impl AsRef<Path>,
    key: &str,
    levels: usize,
    from_end: bool,
) -> Result<PathBuf> {
    let mut keys: Vec<&str> = key.split("").collect();
    if keys.len() < levels {
        return Err(validation_err(
            "segmented_paths: provided key is not long enough for the levels required",
            None,
        ));
    }
    if from_end {
        keys.reverse();
    }

    let mut new = PathBuf::from(path.as_ref());
    for i in 0..levels {
        if let Some(k) = keys.get(i) {
            new.push(k);
        }
    }

    Ok(new)
}

pub fn load_cfg() -> Result<ReloadableLitConfig> {
    ReloadableLitConfig::new(|| {
        let cfg = <LitConfig as LitNodeConfig>::try_new()?;

        // Verify every load (will not replace running config unless it works)
        cfg.verify()?;

        Ok(cfg)
    })
}
