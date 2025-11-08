pub const KEYRING_KEY_NAME: &str = "recovery-key";
pub const KEYRING_DB_KEY_NAME: &str = "sqlcipher-key";

pub const ADMIN_CONTRACT_EMAIL: &str = "chris@litprotocol.com";

// Blockchain constants, currently configured for local dev. supply your own resolver contract address

pub const CONTRACT_CHRONICLE_RPC_URL: &str = "https://lit-protocol.calderachain.xyz/http";

pub const CONTRACT_CHRONICLE_CHAIN_ID: u64 = 175177;

/*
* Used in the contract resolver for determining the environment in which to load the contracts
* 0 - dev
* 1 - staging
* 2 - production
*/
pub const CONTRACT_RESOLVER_ENVIRONMENT: u8 = 2;

pub const LIT_NODE_DOWNLOAD_SHARE_ENDPOINT: &str = "/web/recovery/get_dec_share";
pub const LIT_NODE_DELETE_SHARE_ENDPOINT: &str = "/web/recovery/delete_dec_share";
pub const LIT_NODE_UPLOAD_DECRYPTION_SHARE_ENDPOINT: &str = "/web/recovery/set_dec_shares";

pub const BLS12381G1: &str = "BLS12381G1";
pub const SECP256K1: &str = "Secp256k1";
pub const NISTP256: &str = "P256";
pub const NISTP384: &str = "P384";
pub const ED25519: &str = "Ed25519";
pub const RISTRETTO25519: &str = "Ristretto25519";
pub const ED448: &str = "Ed448";
pub const JUBJUB: &str = "RedJubjub";
pub const DECAF377: &str = "RedDecaf377";
pub const BLS12381G1_SIGN: &str = "BLS12381G1Sign";

pub const CONFIG_STORAGE: [&str; 2] = [concat!(".", env!("CARGO_PKG_NAME")), "config.json"];

pub const LIT_BACKUP_NAME_PATTERN: &str = "*lit_backup_encrypted_keys*";
pub const LIT_BACKUP_SUFFIX: &str = ".tar.gz";
pub const BLS_ENCRYPTION_KEY_FN: &str = "bls_encryption_key";
pub const K256_ENCRYPTION_KEY_FN: &str = "k256_encryption_key";
pub const P256_ENCRYPTION_KEY_FN: &str = "p256_encryption_key";
pub const P384_ENCRYPTION_KEY_FN: &str = "p384_encryption_key";
pub const ED25519_ENCRYPTION_KEY_FN: &str = "ed25519_encryption_key";
pub const RISTRETTO25519_ENCRYPTION_KEY_FN: &str = "ristretto25519_encryption_key";
pub const ED448_ENCRYPTION_KEY_FN: &str = "ed448_encryption_key";
pub const JUBJUB_ENCRYPTION_KEY_FN: &str = "jubjub_encryption_key";
pub const DECAF377_ENCRYPTION_KEY_FN: &str = "decaf377_encryption_key";
pub const BLS12381G1_ENCRYPTION_KEY_FN: &str = "bls12381g1_encryption_key";
pub const SESSION_ID_FN: &str = "session_id";
