use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    #[arg(short, long)]
    pub password: Option<String>,
    #[arg(short, long)]
    pub shares_db: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    #[command(name = "register", about = "Register this wallet key to the recovery address")]
    RegisterToRecoverContract,
    #[command(
        name = "download",
        about = "Download a recovery share from the node assigned to your wallet address"
    )]
    DownloadShare,
    #[command(
        name = "upload-pub-keys",
        about = "Upload public key information from recovery shares"
    )]
    UploadPublicKey,
    #[command(name = "list", about = "List all recovery shares")]
    ListShareDetails {
        #[arg(long)]
        session_id: Option<String>,
        #[arg(long)]
        encryption_key: Option<String>,
        #[arg(long)]
        curve: Option<String>,
        #[arg(long)]
        subnet_id: Option<String>,
        #[arg(long)]
        url: Option<String>,
    },
    #[command(name = "delete", about = "Delete a recovery share")]
    DeleteShare {
        #[arg(long)]
        session_id: Option<String>,
        #[arg(long)]
        encryption_key: Option<String>,
        #[arg(long)]
        curve: Option<String>,
        #[arg(long)]
        subnet_id: Option<String>,
        #[arg(long)]
        url: Option<String>,
    },
    #[command(name = "insert-share", about = "Insert a recovery key share")]
    InsertShare {
        #[arg(long)]
        encryption_key: String,
        #[arg(long, default_value_t = String::from("test_session_id"))]
        session_id: String,
        #[arg(long)]
        decryption_key_share: String,
        #[arg(long)]
        curve: String,
        #[arg(long, default_value_t = String::from("test_subnet_id"))]
        subnet_id: String,
        #[arg(long, default_value_t = String::from("test_url"))]
        url: String,
    },
    #[command(name = "import", about = "Import recovery shares from a previously exported file")]
    ImportSharesFromFile {
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
        #[arg(short, long)]
        import_password: Option<String>,
    },
    #[command(name = "export", about = "Export recovery shares to an encrypted file")]
    ExportSharesToFile {
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
        #[arg(short, long)]
        export_password: Option<String>,
    },
    #[command(name = "upload", about = "Upload recovery shares to a node")]
    UploadDecryptionShare {
        #[arg(short, long, value_name = "STRING")]
        key_type: String,
        #[arg(short, long, value_name = "FILE")]
        ciphertext_file: PathBuf,
        #[arg(short, long, value_name = "ENCRYPTION_KEY")]
        encryption_key: String,
    },
    #[command(name = "recover", about = "Extract tars and upload recovery shares all nodes")]
    Recover {
        #[arg(short, long, value_name = "DIRECTORY")]
        directory: PathBuf,
        #[arg(short, long, value_name = "SESSION_ID")]
        session_id: String,
    },
    #[command(
        name = "mnemonic",
        about = "Use a mnemonic to generate the wallet key replacing the current one"
    )]
    Mnemonic {
        #[arg(short, long, value_name = "MNEMONIC")]
        phrase: String,
    },

    #[command(
        name = "contract-resolver",
        about = "Set the resolver contract address for network context"
    )]
    ContractResolver {
        #[arg(short, long, value_name = "CONTRACT_RESOLVER_ADDRESS")]
        address: String,
    },
    #[command(name = "set-configuration", about = "sets the local configuration properties")]
    SetConfig {
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
        #[arg(short, long, value_name = "RPC_URL")]
        rpc_url: String,
        #[arg(short, long, value_name = "ADDRESS")]
        chain_id: u64,
        #[arg(short, long, value_name = "ENVIRONMENT")]
        env: u8,
    },
    #[command(
        name = "get-node-status",
        about = "reads the recovery status of the nodes from the contract"
    )]
    GetNodeStatus,
    #[command(
        name = "decrypt-share",
        about = "create a decryption share from a local file but do not upload it to the node"
    )]
    DecryptShare {
        #[arg(short, long, value_name = "STRING")]
        key_type: String,
        #[arg(short, long, value_name = "CIPHERTEXT_FILE")]
        ciphertext_file: PathBuf,
        #[arg(short, long, value_name = "SHARE_FILE")]
        share_file: Option<PathBuf>,
        #[arg(short, long, value_name = "OUTPUT_SHARE_FILE")]
        output_share_file: PathBuf,
        #[arg(short, long, value_name = "ENCRYPTION_KEY")]
        encryption_key: String,
    },
    #[command(
        name = "merge-decryption-shares",
        about = "Merge decryption shares from a local file"
    )]
    MergeDecryptionShares {
        #[arg(short, long, value_name = "STRING")]
        key_type: String,
        #[arg(short, long, value_name = "BLINDER_FILE")]
        blinder: PathBuf,
        #[arg(short, long, value_name = "CIPHERTEXT_FILE")]
        ciphertext_file: PathBuf,
        #[arg(short, long, value_name = "SHARE_FILES")]
        decrypted_share_files: Vec<PathBuf>,
        #[arg(short, long, value_name = "OUTPUT_SHARE_FILE")]
        output_file: PathBuf,
    },
    #[command(name = "info", about = "Print information about this wallet")]
    Info,
}
