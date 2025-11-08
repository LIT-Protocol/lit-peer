use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum Error {
    #[error("Contract Error: {0}")]
    Contract(String),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Format error: {0}")]
    Fmt(#[from] std::fmt::Error),

    #[error("Utf8Error error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("ReadlineError error: {0}")]
    Readline(#[from] rustyline::error::ReadlineError),

    #[error("FromHex error: {0}")]
    FromHex(#[from] hex::FromHexError),

    #[error("Keyring error: {0}")]
    KeyRing(#[from] cryptex::error::KeyRingError),

    #[error("Keyring item not found: {0}")]
    NotFoundInKeyRing(String),

    #[error("SqlCipher error: {0}")]
    SqlCipher(#[from] rusqlite::Error),

    #[error("Not a directory")]
    #[allow(dead_code)]
    NotADir(std::path::PathBuf),

    #[error("Not a file")]
    #[allow(dead_code)]
    NotAFile(std::path::PathBuf),

    #[error("You're not a registered Backup Party member in the contract")]
    NotABackupParty(String),

    #[error("Invalid yaml format: {0}")]
    InvalidYamlFormat(#[from] serde_yaml::Error),

    #[error("{0}")]
    InvalidRequest(String),

    #[error("Invalid json format: {0}")]
    InvalidJsonFormat(#[from] serde_json::Error),

    #[error("Invalid cbor format: {0}")]
    InvalidCborFormat(String),

    #[error("Parsing error: {0}")]
    InvalidEthereumAddress(String),

    #[error("{0}")]
    General(String),
}

pub type RecoveryResult<T> = Result<T, Error>;
