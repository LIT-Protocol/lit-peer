// NB: Before adding keys here ensure they don't conflict with LitApiConfig
// - port, address, ident e.t.c. are all reserved.
pub const CFG_KEY_RPC_URL: &str = "rpc_url";
pub const CFG_KEY_STAKER_ADDRESS: &str = "staker_address";
pub const CFG_KEY_COMS_KEYS_SENDER_PRIVKEY: &str = "coms_keys_sender_privkey";
pub const CFG_KEY_COMS_KEYS_RECEIVER_PRIVKEY: &str = "coms_keys_receiver_privkey";
pub const CFG_KEY_ADMIN_ADDRESS: &str = "admin_address";
pub const CFG_KEY_ENABLE_PROXIED_CHATTER_CLIENT: &str = "enable_proxied_chatter_client";
pub const CFG_KEY_ENABLE_PAYMENT: &str = "enable_payment";
pub const CFG_KEY_ENABLE_ACTIONS_ALLOWLIST: &str = "enable_actions_allowlist";
pub const CFG_KEY_ENABLE_EPOCH_TRANSITIONS: &str = "enable_epoch_transitions";
pub const CFG_KEY_ENABLE_OBSERVABILITY_EXPORT: &str = "enable_observability_export";
pub const CFG_KEY_SIGNING_ROUND_TIMEOUT: &str = "signing_round_timeout";
pub const CFG_KEY_WEBAUTHN_ALLOWED_ORIGINS: &str = "webauthn_allowed_origins";
pub const CFG_KEY_CHAIN_POLLING_INTERVAL_MS: &str = "chain_polling_interval";
pub const CFG_KEY_CHATTER_CLIENT_TIMEOUT: &str = "chatter_client_timeout";
pub const CFG_KEY_ENABLE_SIWE_VALIDATION: &str = "enable_siwe_validation";
pub const CFG_KEY_RESTORE_LOG_INTERVAL_MS: &str = "restore_log_interval";
pub const CFG_KEY_ACTIONS_SOCKET: &str = "actions_socket";
pub const CFG_KEY_HEALTH_POLL_INTERVAL_MS: &str = "health_poll_interval";
pub const CFG_KEY_PAYMENT_INTERVAL_MS: &str = "payment_interval";
pub const CFG_KEY_WEB_CLIENT_TIMEOUT_SEC: &str = "web_client_timeout";
pub const CFG_KEY_GRPC_SERVER_CONC_LIMIT_PER_CONN: &str = "grpc_server_conc_limit_per_conn";
pub const CFG_KEY_GRPC_POOL_SIZE: &str = "grpc_client_pool_size";
