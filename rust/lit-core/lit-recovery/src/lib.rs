use crate::args::Commands;
use crate::chain_manager::ChainManager;
use crate::config::RecoveryConfig;
use crate::consts::{
    ADMIN_CONTRACT_EMAIL, BLS12381G1, BLS12381G1_SIGN, DECAF377, ED448, ED25519, JUBJUB,
    KEYRING_DB_KEY_NAME, KEYRING_KEY_NAME, LIT_BACKUP_NAME_PATTERN, LIT_BACKUP_SUFFIX,
    LIT_NODE_DELETE_SHARE_ENDPOINT, LIT_NODE_DOWNLOAD_SHARE_ENDPOINT, NISTP256, NISTP384,
    RISTRETTO25519, SECP256K1,
};
use crate::decryption::{
    generate_and_send_decryption_shares_to_nodes, load_upload_shares, merge_decryption_shares,
    read_blinder, write_local_decrypt_share,
};
use crate::download::{delete_share, download_share};
use crate::error::{Error, RecoveryResult};
use crate::eth::EthereumAddress;
use crate::shares::{
    COLUMN_CURVE, COLUMN_ENCRYPTION_KEY, COLUMN_SESSION_ID, COLUMN_SUBNET_ID, COLUMN_URL,
    ShareData, ShareDatabase,
};
use arc_swap::ArcSwap;
use bip39::Mnemonic;
use blsful::inner_types::{Group, PrimeCurveAffine};
use bulletproofs::bls12_381_plus::elliptic_curve::bigint::U512;
use bulletproofs::bls12_381_plus::elliptic_curve::ops::Reduce;
use bulletproofs::blstrs_plus::Bls12381G1;
use bulletproofs::{Decaf377, Ed25519, JubJub, Ristretto25519, jubjub};
use colored::Colorize;
use cryptex::DynKeyRing;
use ed448_goldilocks_plus::Ed448;
use hex::FromHex;
use k256::Secp256k1;
use k256::ecdsa::VerifyingKey;
use lit_blockchain::contracts::backup_recovery::NextStateDownloadable;
use rand::{Rng, RngCore, rngs::OsRng};
use serde::de::DeserializeOwned;
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use vsss_rs::elliptic_curve::group::GroupEncoding;

pub mod args;
pub mod auth;
mod chain_manager;
pub mod config;
pub mod consts;
pub mod decryption;
pub mod download;
pub mod error;
pub mod eth;
pub mod io;
pub mod models;
pub mod shares;

pub struct LitRecoveryInfo {
    pub mnemonic: Option<Mnemonic>,
    pub verifying_key: VerifyingKey,
}

impl Default for LitRecoveryInfo {
    fn default() -> Self {
        Self {
            mnemonic: None,
            verifying_key: VerifyingKey::from(
                k256::PublicKey::from_affine(k256::AffinePoint::GENERATOR)
                    .expect("Failed to create default verifying key"),
            ),
        }
    }
}

impl LitRecoveryInfo {
    pub fn display(&self) {
        if let Some(mnemonic) = &self.mnemonic {
            println!(
                "This is your Mnemonic phrase. Write this down as it will not be given again:\n\t{}",
                mnemonic.phrase().green()
            );
        }
        println!(
            "Verification key: {}",
            hex::encode(self.verifying_key.to_encoded_point(false).as_bytes()).green()
        );
        println!("Address:          {}", self.verifying_key.to_eth_address().green());
    }
}

pub struct LitRecovery {
    signing_key: Mutex<soteria_rs::Protected>,
    db_key: Mutex<soteria_rs::Protected>,
    config: ArcSwap<RecoveryConfig>,
    config_path: Option<PathBuf>,
    keyring_file: Option<PathBuf>,
    password: Option<String>,
    share_db: Option<PathBuf>,
    info: ArcSwap<LitRecoveryInfo>,
}

impl Default for LitRecovery {
    fn default() -> Self {
        Self {
            signing_key: Mutex::new(soteria_rs::Protected::default()),
            db_key: Mutex::new(soteria_rs::Protected::default()),
            config: ArcSwap::new(Arc::new(RecoveryConfig {
                resolver_address: None,
                rpc_url: Some(consts::CONTRACT_CHRONICLE_RPC_URL.into()),
                chain_id: Some(consts::CONTRACT_CHRONICLE_CHAIN_ID),
                environment: Some(2), // production is 2
            })),
            config_path: None,
            keyring_file: None,
            password: None,
            share_db: None,
            info: ArcSwap::new(Arc::new(LitRecoveryInfo::default())),
        }
    }
}

impl LitRecovery {
    pub async fn new(
        key_ring: Option<PathBuf>, password: Option<String>, share_db: Option<PathBuf>,
        config_path: Option<PathBuf>,
    ) -> RecoveryResult<Self> {
        let mut recovery = LitRecovery { share_db, ..Default::default() };
        recovery.init_keys(key_ring, password).await?;
        recovery.config_path = config_path.clone();
        recovery.config.store(Arc::new(RecoveryConfig::load(config_path)?));
        Ok(recovery)
    }

    pub async fn get_config(&self) -> Arc<RecoveryConfig> {
        self.config.load_full()
    }

    pub async fn init_keys(
        &mut self, keyring_file: Option<PathBuf>, password: Option<String>,
    ) -> RecoveryResult<()> {
        self.keyring_file = keyring_file.clone();
        self.password = password.clone();
        let mut keyring = self.get_keyring()?;
        let mut secret_key_exists = false;
        if let Ok(secret_key) = keyring.get_secret(KEYRING_KEY_NAME) {
            if !secret_key.is_empty() {
                secret_key_exists = true;
                *self.signing_key.lock().await = soteria_rs::Protected::new(secret_key.as_slice());
                let key =
                    k256::ecdsa::SigningKey::from_slice(secret_key.as_slice()).map_err(|e| {
                        Error::General(format!(
                            "Failed to serialize K256 key due to error: {:?}",
                            e
                        ))
                    })?;
                self.info.rcu(|inner| {
                    Arc::new(LitRecoveryInfo {
                        verifying_key: *key.verifying_key(),
                        mnemonic: inner.mnemonic.clone(),
                    })
                });
            }
        }
        if !secret_key_exists {
            let entropy = OsRng.r#gen::<[u8; 16]>();
            let mnemonic = Mnemonic::from_entropy(&entropy, bip39::Language::English)
                .map_err(|e| Error::General(e.to_string()))?;

            let seed = bip39::Seed::new(&mnemonic, "");
            let seed = k256::WideBytes::clone_from_slice(seed.as_bytes());
            let secret = <k256::Scalar as Reduce<U512>>::reduce_bytes(&seed);
            let secret_key = k256::ecdsa::SigningKey::from_slice(&secret.to_bytes())
                .map_err(|_| Error::General("Could not create signing key".to_string()))?;

            self.info.store(Arc::new(LitRecoveryInfo {
                mnemonic: Some(mnemonic),
                verifying_key: *secret_key.verifying_key(),
            }));

            *self.signing_key.lock().await = soteria_rs::Protected::new(secret.to_bytes());
            keyring.set_secret(KEYRING_KEY_NAME, &secret.to_bytes())?;
        }

        let mut db_key = [0u8; 32];
        if let Ok(key) = keyring.get_secret(KEYRING_DB_KEY_NAME) {
            if key.is_empty() {
                OsRng.fill_bytes(&mut db_key);
                keyring.set_secret(KEYRING_DB_KEY_NAME, &db_key)?;
            } else {
                db_key.copy_from_slice(key.as_slice());
            }
        } else {
            OsRng.fill_bytes(&mut db_key);
            keyring.set_secret(KEYRING_DB_KEY_NAME, &db_key)?;
        }
        *self.db_key.lock().await = soteria_rs::Protected::new(db_key);

        self.info.load().display();
        Ok(())
    }

    pub async fn get_signing_key(&self) -> RecoveryResult<k256::ecdsa::SigningKey> {
        let mut protected = self.signing_key.lock().await;
        let unprotected = protected.unprotect();
        match unprotected {
            Some(u) => {
                if !u.as_ref().is_empty() {
                    u.signing_key().map_err(|e| Error::General(e.to_string()))
                } else {
                    Err(Error::NotFoundInKeyRing("Signing key not in keyring".to_string()))
                }
            }
            None => Err(Error::NotFoundInKeyRing("Signing key not in keyring".to_string())),
        }
    }

    pub async fn get_secret_for_wallet(&self) -> RecoveryResult<Vec<u8>> {
        self.get_signing_key().await.map(|k| k.to_bytes().to_vec())
    }

    pub async fn command(&self, command: Commands) -> RecoveryResult<()> {
        match command {
            Commands::DownloadShare => {
                let cfg = self.get_config().await;
                let sk = self.get_secret_for_wallet().await?;
                let contracts = ChainManager::new_with_signer(&sk, &cfg).await?;
                let validator_info = contracts.get_validator_struct_for_recovery_share().await;
                match validator_info {
                    Err(Error::Contract(msg)) if msg == "Contract call reverted with data: 0x" => {
                        return Err(Error::NotABackupParty(
                            "You're not a registered backup party".to_string(),
                        ));
                    }
                    Err(e) => {
                        eprintln!(
                            "An unexpected error occurred while getting the validator info from chain for share download"
                        );
                        return Err(Error::General(format!(
                            "An unexpected error occurred: {:?}",
                            e
                        )));
                    }
                    Ok(validator_info) => {
                        println!("got validator info from chain: {:?}", validator_info);
                        let remote_ip = std::net::Ipv4Addr::from(validator_info.ip);
                        let scheme = if remote_ip == std::net::Ipv4Addr::LOCALHOST {
                            "http://"
                        } else {
                            "https://"
                        };

                        let url_prefix = format!(
                            "{}{}:{}",
                            scheme,
                            std::net::Ipv4Addr::from(validator_info.ip),
                            validator_info.port,
                        );

                        let url = format!("{}{}", url_prefix, LIT_NODE_DOWNLOAD_SHARE_ENDPOINT);
                        println!("pulled url from chain data: {}", url);
                        download_share(self, &url, &contracts).await?;
                        let url = format!("{}{}", url, LIT_NODE_DELETE_SHARE_ENDPOINT);

                        let res = delete_share(self, &url).await?;
                        if res {
                            println!(
                                "Recovery share deleted from node, you now are the only holder of this keyshare"
                            );
                        } else {
                            eprintln!("Unable to delete recovery share");
                        }
                    }
                }
            }
            Commands::UploadPublicKey => {
                let cfg = self.get_config().await;
                let sk = self.get_secret_for_wallet().await?;
                let contracts = ChainManager::new_with_signer(&sk, &cfg).await?;

                let key_types = [
                    (BLS12381G1.to_string(), 1),
                    (SECP256K1.to_string(), 2),
                    (ED25519.to_string(), 3),
                    (ED448.to_string(), 4),
                    (RISTRETTO25519.to_string(), 5),
                    (NISTP256.to_string(), 6),
                    (NISTP384.to_string(), 7),
                    (JUBJUB.to_string(), 8),
                    (DECAF377.to_string(), 9),
                    (BLS12381G1_SIGN.to_string(), 10),
                ]
                .into_iter()
                .collect::<HashMap<String, usize>>();
                let next_state: NextStateDownloadable =
                    contracts.backup_recovery.get_next_backup_state().await.map_err(|e| {
                        Error::General(format!(
                            "Unable to read the next backup state from chain: {:?}",
                            e
                        ))
                    })?;
                let mut next_state_keys =
                    HashMap::with_capacity(next_state.registered_recovery_keys.len());
                for rk in &next_state.registered_recovery_keys {
                    next_state_keys.insert(rk.key_type.as_usize(), rk.pubkey.clone());
                }

                let recovery_session_id = next_state.session_id.to_string();

                let mut filter = BTreeMap::new();
                filter.insert(COLUMN_SESSION_ID, recovery_session_id.clone());
                let shares_db = self.get_shared_database().await?;
                let shares = shares_db.get_shares(
                    Some(&[COLUMN_SESSION_ID, COLUMN_ENCRYPTION_KEY, COLUMN_CURVE]),
                    Some(filter),
                )?;

                if shares.len() != next_state.registered_recovery_keys.len() {
                    return Err(Error::General(format!(
                        "Only found {} recovery keys locally, but expected {} recovery keys",
                        shares.len(),
                        next_state.registered_recovery_keys.len()
                    )));
                }

                for share in &shares {
                    let key_type = key_types[&share.curve];
                    let recovery_key =
                        next_state_keys.remove(&key_type).ok_or(Error::General(format!(
                        "The recovery state does not have a registered public key for curve '{}'",
                        share.curve
                    )))?;
                    let share_pub_key = ethers::types::Bytes::from_hex(&share.encryption_key)
                        .map_err(|e| {
                            Error::General(format!(
                                "Failed to parse {} pub key to hex: {:?}",
                                share.curve, e
                            ))
                        })?;
                    if recovery_key != share_pub_key {
                        return Err(Error::General(format!(
                            "The recovery state does not have the same public key as the local state. The recovery state public key for curve '{}' is '{}' but the local value found is '{}'",
                            share.curve, recovery_key, share.encryption_key
                        )));
                    }
                }
                if !next_state_keys.is_empty() {
                    let key_types = key_types
                        .into_iter()
                        .map(|(key, value)| (value, key))
                        .collect::<HashMap<_, _>>();
                    let mut err = "The local state is missing keys for which the recovery state has registered keys for the following curves:\n".to_string();
                    for key_type in next_state_keys.keys() {
                        let curve = key_types.get(key_type).ok_or(Error::General(format!("The recovery state has a curve type that is not known to this tool: {}", *key_type)))?;
                        err.push_str(format!("Curve '{}'\n", curve).as_str());
                    }
                    return Err(Error::General(err));
                }

                contracts.submit_pub_info_to_chain(next_state).await?;
            }
            Commands::ListShareDetails { session_id, encryption_key, curve, subnet_id, url } => {
                let shares_db = self.get_shared_database().await?;
                let mut filter = BTreeMap::new();
                if let Some(session_id) = session_id {
                    filter.insert(COLUMN_SESSION_ID, session_id);
                }
                if let Some(encryption_key) = encryption_key {
                    filter.insert(COLUMN_ENCRYPTION_KEY, encryption_key);
                }
                if let Some(curve) = curve {
                    filter.insert(COLUMN_CURVE, curve);
                }
                if let Some(subnet_id) = subnet_id {
                    filter.insert(COLUMN_SUBNET_ID, subnet_id);
                }
                if let Some(url) = url {
                    filter.insert(COLUMN_URL, url);
                }
                let filter = if filter.is_empty() { None } else { Some(filter) };
                let shares = shares_db.get_shares(
                    Some(&[
                        COLUMN_SESSION_ID, COLUMN_ENCRYPTION_KEY, COLUMN_SUBNET_ID, COLUMN_CURVE,
                        COLUMN_URL,
                    ]),
                    filter,
                )?;
                for share in shares {
                    println!(
                        "================================================================================"
                    );
                    println!("Session id: {}", share.session_id);
                    println!("Encryption Key: {}", share.encryption_key);
                    println!("Subnet ID: {}", share.subnet_id);
                    println!("URL: {}", share.url);
                    println!("Curve: {}", share.curve);
                }
                println!(
                    "================================================================================"
                );
            }
            Commands::DeleteShare { session_id, encryption_key, curve, subnet_id, url } => {
                let mut filter = BTreeMap::new();
                if let Some(session_id) = session_id {
                    filter.insert(COLUMN_SESSION_ID, session_id);
                }
                if let Some(encryption_key) = encryption_key {
                    filter.insert(COLUMN_ENCRYPTION_KEY, encryption_key);
                }
                if let Some(curve) = curve {
                    filter.insert(COLUMN_CURVE, curve);
                }
                if let Some(subnet_id) = subnet_id {
                    filter.insert(COLUMN_SUBNET_ID, subnet_id);
                }
                if let Some(url) = url {
                    filter.insert(COLUMN_URL, url);
                }
                let shares_db = self.get_shared_database().await?;
                shares_db.delete_share(&filter)?;
            }
            Commands::InsertShare {
                encryption_key,
                session_id,
                decryption_key_share,
                curve,
                subnet_id,
                url,
            } => {
                let share = ShareData {
                    session_id,
                    encryption_key,
                    decryption_key_share,
                    subnet_id,
                    curve,
                    url,
                };

                let share_db = self.get_shared_database().await?;
                share_db.insert_share(&share)?;
            }
            Commands::ImportSharesFromFile { file, import_password } => {
                println!("Importing shares from file {}", file.display());
                let new_shares_db = self.get_shared_database().await?;
                let old_shares_db = if let Some(pass) = import_password {
                    ShareDatabase::open_with_path_and_password(&file, &pass)?
                } else {
                    let mut keyring = self.get_keyring()?;
                    let signing_key = keyring.get_secret(KEYRING_KEY_NAME).map_err(|e| {
                        Error::General(format!(
                            "Failed to get secret \"{}\" due to error: {:?}",
                            KEYRING_KEY_NAME, e
                        ))
                    })?;
                    ShareDatabase::open_with_path_and_secret(&file, signing_key.as_slice())?
                };
                let shares = old_shares_db.get_shares(None, None)?;
                for share in shares {
                    new_shares_db.insert_share(&share)?;
                }
            }
            Commands::ExportSharesToFile { file, export_password } => {
                println!("Exporting shares to file {}", file.to_string_lossy());
                let old_shares_db = self.get_shared_database().await?;
                let new_shares_db = if let Some(pass) = export_password {
                    ShareDatabase::open_with_path_and_password(&file, &pass)?
                } else {
                    let mut keyring = self.get_keyring()?;
                    let signing_key = keyring.get_secret(KEYRING_KEY_NAME).map_err(|e| {
                        Error::General(format!(
                            "Failed to get secret \"{}\" due to error: {:?}",
                            KEYRING_KEY_NAME, e
                        ))
                    })?;
                    ShareDatabase::open_with_path_and_secret(&file, signing_key.as_slice())?
                };
                let shares = old_shares_db.get_shares(None, None)?;
                for share in shares {
                    new_shares_db.insert_share(&share)?;
                }
            }
            Commands::RegisterToRecoverContract => {
                self.info.load().display();
                println!("Please email the above address to {}", ADMIN_CONTRACT_EMAIL.green());
            }
            Commands::UploadDecryptionShare { key_type, ciphertext_file, encryption_key } => {
                match key_type.as_str() {
                    BLS12381G1 => {
                        generate_and_send_decryption_shares_to_nodes::<Bls12381G1>(
                            self, ciphertext_file, encryption_key,
                        )
                        .await?;
                    }
                    SECP256K1 => {
                        generate_and_send_decryption_shares_to_nodes::<Secp256k1>(
                            self, ciphertext_file, encryption_key,
                        )
                        .await?;
                    }
                    NISTP256 => {
                        generate_and_send_decryption_shares_to_nodes::<p256::NistP256>(
                            self, ciphertext_file, encryption_key,
                        )
                        .await?;
                    }
                    NISTP384 => {
                        generate_and_send_decryption_shares_to_nodes::<p384::NistP384>(
                            self, ciphertext_file, encryption_key,
                        )
                        .await?;
                    }
                    ED25519 => {
                        generate_and_send_decryption_shares_to_nodes::<Ed25519>(
                            self, ciphertext_file, encryption_key,
                        )
                        .await?;
                    }
                    RISTRETTO25519 => {
                        generate_and_send_decryption_shares_to_nodes::<Ristretto25519>(
                            self, ciphertext_file, encryption_key,
                        )
                        .await?;
                    }
                    ED448 => {
                        generate_and_send_decryption_shares_to_nodes::<Ed448>(
                            self, ciphertext_file, encryption_key,
                        )
                        .await?;
                    }
                    JUBJUB => {
                        generate_and_send_decryption_shares_to_nodes::<JubJub>(
                            self, ciphertext_file, encryption_key,
                        )
                        .await?;
                    }
                    DECAF377 => {
                        generate_and_send_decryption_shares_to_nodes::<Decaf377>(
                            self, ciphertext_file, encryption_key,
                        )
                        .await?;
                    }
                    BLS12381G1_SIGN => {
                        generate_and_send_decryption_shares_to_nodes::<Bls12381G1>(
                            self, ciphertext_file, encryption_key,
                        )
                        .await?
                    }
                    _ => {
                        println!(
                            "Key type not supported! Please use either [{}]",
                            [
                                BLS12381G1, SECP256K1, NISTP256, NISTP384, ED25519, RISTRETTO25519,
                                ED448, JUBJUB, DECAF377
                            ]
                            .join(", ")
                        );
                    }
                }
            }
            Commands::Recover { directory, session_id } => {
                self.recover(directory, session_id).await?;
            }
            Commands::Mnemonic { phrase } => {
                self.handle_mnemonic(&phrase).await?;
            }
            Commands::ContractResolver { address } => {
                let config = self.config.load();
                let new_config = Arc::new(RecoveryConfig {
                    resolver_address: Some(address),
                    ..config.as_ref().clone()
                });
                new_config.save(self.config_path.clone())?;
                self.config.store(new_config);
            }
            Commands::SetConfig { address, chain_id, rpc_url, env } => {
                let config = self.config.load();

                let new_config = Arc::new(RecoveryConfig {
                    resolver_address: Some(address.clone()),
                    chain_id: Some(chain_id),
                    rpc_url: Some(rpc_url.clone()),
                    environment: Some(env),
                    ..config.as_ref().clone()
                });

                new_config.save(self.config_path.clone())?;
                self.config.store(new_config);
            }
            Commands::GetNodeStatus => {
                let cfg = self.get_config().await;
                let sk = self.get_secret_for_wallet().await?;
                let contracts = ChainManager::new_with_signer(&sk, &cfg).await?;
                let node_status = contracts.get_node_recovery_status().await?;
                println!("{:?}", node_status);
            }
            Commands::DecryptShare {
                key_type,
                ciphertext_file,
                share_file,
                output_share_file,
                encryption_key,
            } => match key_type.to_uppercase().as_str() {
                BLS12381G1 => {
                    write_local_decrypt_share::<Bls12381G1>(
                        self, ciphertext_file, encryption_key, share_file, output_share_file,
                    )
                    .await?;
                }
                SECP256K1 => {
                    write_local_decrypt_share::<Secp256k1>(
                        self, ciphertext_file, encryption_key, share_file, output_share_file,
                    )
                    .await?;
                }
                NISTP256 => {
                    write_local_decrypt_share::<p256::NistP256>(
                        self, ciphertext_file, encryption_key, share_file, output_share_file,
                    )
                    .await?;
                }
                NISTP384 => {
                    write_local_decrypt_share::<p384::NistP384>(
                        self, ciphertext_file, encryption_key, share_file, output_share_file,
                    )
                    .await?;
                }
                ED25519 => {
                    write_local_decrypt_share::<Ed25519>(
                        self, ciphertext_file, encryption_key, share_file, output_share_file,
                    )
                    .await?;
                }
                RISTRETTO25519 => {
                    write_local_decrypt_share::<Ristretto25519>(
                        self, ciphertext_file, encryption_key, share_file, output_share_file,
                    )
                    .await?;
                }
                ED448 => {
                    write_local_decrypt_share::<Ed448>(
                        self, ciphertext_file, encryption_key, share_file, output_share_file,
                    )
                    .await?;
                }
                JUBJUB => {
                    write_local_decrypt_share::<JubJub>(
                        self, ciphertext_file, encryption_key, share_file, output_share_file,
                    )
                    .await?;
                }
                DECAF377 => {
                    write_local_decrypt_share::<Decaf377>(
                        self, ciphertext_file, encryption_key, share_file, output_share_file,
                    )
                    .await?;
                }
                BLS12381G1_SIGN => {
                    write_local_decrypt_share::<Bls12381G1>(
                        self, ciphertext_file, encryption_key, share_file, output_share_file,
                    )
                    .await?;
                }
                _ => {
                    println!(
                        "Key type not supported! Please use either [{}]",
                        [
                            BLS12381G1, SECP256K1, NISTP256, NISTP384, ED25519, RISTRETTO25519,
                            ED448, JUBJUB, DECAF377, BLS12381G1_SIGN,
                        ]
                        .join(", ")
                    );
                }
            },
            Commands::MergeDecryptionShares {
                ciphertext_file,
                key_type,
                blinder,
                decrypted_share_files,
                output_file,
            } => match key_type.to_uppercase().as_str() {
                BLS12381G1 => {
                    let blinder = read_blinder::<Bls12381G1>(blinder, "bls_blinder")?;
                    merge_decryption_shares::<Bls12381G1>(
                        ciphertext_file, blinder, decrypted_share_files, output_file, None,
                    )?;
                }
                SECP256K1 => {
                    let blinder = read_blinder::<Secp256k1>(blinder, "k256_blinder")?;
                    merge_decryption_shares::<Secp256k1>(
                        ciphertext_file, blinder, decrypted_share_files, output_file, None,
                    )?;
                }
                NISTP256 => {
                    let blinder = read_blinder::<p256::NistP256>(blinder, "p256_blinder")?;
                    merge_decryption_shares::<p256::NistP256>(
                        ciphertext_file, blinder, decrypted_share_files, output_file, None,
                    )?;
                }
                NISTP384 => {
                    let blinder = read_blinder::<p384::NistP384>(blinder, "p384_blinder")?;
                    merge_decryption_shares::<p384::NistP384>(
                        ciphertext_file, blinder, decrypted_share_files, output_file, None,
                    )?;
                }
                ED25519 => {
                    let blinder = read_blinder::<Ed25519>(blinder, "ed25519_blinder")?;
                    merge_decryption_shares::<Ed25519>(
                        ciphertext_file, blinder, decrypted_share_files, output_file, None,
                    )?;
                }
                RISTRETTO25519 => {
                    let blinder =
                        read_blinder::<Ristretto25519>(blinder, "ristretto25519_blinder")?;
                    merge_decryption_shares::<Ristretto25519>(
                        ciphertext_file, blinder, decrypted_share_files, output_file, None,
                    )?;
                }
                ED448 => {
                    let blinder = read_blinder::<Ed448>(blinder, "ed448_blinder")?;
                    merge_decryption_shares::<Ed448>(
                        ciphertext_file, blinder, decrypted_share_files, output_file, None,
                    )?;
                }
                JUBJUB => {
                    use elliptic_curve::group::cofactor::CofactorGroup;
                    // Jubjub uses a special generator for signing. Use this here
                    pub const SPENDAUTHSIG_BASEPOINT_BYTES: [u8; 32] = [
                        48, 181, 242, 170, 173, 50, 86, 48, 188, 221, 219, 206, 77, 103, 101, 109,
                        5, 253, 28, 194, 208, 55, 187, 83, 117, 182, 233, 109, 158, 1, 161, 215,
                    ];

                    let pt: jubjub::ExtendedPoint =
                        jubjub::AffinePoint::from_bytes(&SPENDAUTHSIG_BASEPOINT_BYTES)
                            .unwrap()
                            .into();
                    let generator = pt.into_subgroup().unwrap();

                    let blinder = read_blinder::<JubJub>(blinder, "jubjub_blinder")?;
                    merge_decryption_shares::<JubJub>(
                        ciphertext_file,
                        blinder,
                        decrypted_share_files,
                        output_file,
                        Some(generator),
                    )?;
                }
                DECAF377 => {
                    let blinder = read_blinder::<Decaf377>(blinder, "decaf377_blinder")?;
                    merge_decryption_shares::<Decaf377>(
                        ciphertext_file, blinder, decrypted_share_files, output_file, None,
                    )?;
                }
                _ => {
                    println!(
                        "Key type not supported! Please use either [{}]",
                        [
                            BLS12381G1, SECP256K1, NISTP256, NISTP384, ED25519, RISTRETTO25519,
                            ED448, JUBJUB, DECAF377, BLS12381G1_SIGN,
                        ]
                        .join(", ")
                    );
                }
            },
            Commands::Info => {
                self.info.load().display();
            }
        }
        Ok(())
    }

    pub async fn info(&self) -> Arc<LitRecoveryInfo> {
        self.info.load_full()
    }

    async fn get_shared_database(&self) -> RecoveryResult<ShareDatabase> {
        match self.share_db.as_ref() {
            Some(db) => ShareDatabase::open_with_path(self, db).await,
            None => ShareDatabase::open(self).await,
        }
    }

    fn get_keyring(&self) -> RecoveryResult<Box<dyn DynKeyRing>> {
        let keyring: Box<dyn DynKeyRing> = if let Some(keyring_file) = &self.keyring_file {
            let keyring_file = path_clean::clean(keyring_file);
            let mut params = cryptex::sqlcipher::ConnectionParams::default();
            params.memory = 32 * 1024;
            params.password = self
                .password
                .clone()
                .ok_or_else(|| Error::General("Password not provided".to_string()))?
                .into_bytes();
            params.salt = vec![0xEEu8; 32];
            let keyring =
                cryptex::sqlcipher::SqlCipherKeyring::with_params(&params, Some(keyring_file))?;
            Box::new(keyring)
        } else {
            Box::new(
                cryptex::get_os_keyring(env!("CARGO_PKG_NAME"))
                    .expect("Failed to create OS keyring"),
            )
        };

        Ok(keyring)
    }

    async fn handle_mnemonic(&self, phrase: &str) -> RecoveryResult<()> {
        let mnemonic = Mnemonic::from_phrase(phrase, bip39::Language::English)
            .map_err(|e| Error::General(e.to_string()))?;

        let seed = bip39::Seed::new(&mnemonic, "");
        let seed = k256::WideBytes::clone_from_slice(seed.as_bytes());
        let secret = <k256::Scalar as Reduce<U512>>::reduce_bytes(&seed);

        let secret_key = k256::ecdsa::SigningKey::from_slice(&secret.to_bytes())
            .map_err(|_| Error::General("Could not create signing key".to_string()))?;

        *self.signing_key.lock().await = soteria_rs::Protected::new(secret.to_bytes());
        let mut keyring = self.get_keyring()?;
        keyring.set_secret(KEYRING_KEY_NAME, &secret.to_bytes())?;
        let info = LitRecoveryInfo {
            mnemonic: Some(mnemonic.clone()),
            verifying_key: *secret_key.verifying_key(),
        };
        info.display();
        self.info.store(Arc::new(info));

        Ok(())
    }

    async fn recover(&self, directory: PathBuf, session_id: String) -> RecoveryResult<()> {
        // Fetch the list of .tar files in the directory.
        let info = self.info.load();
        let verifying_key = hex::encode(info.verifying_key.to_encoded_point(false).as_bytes());
        let mut path = directory.clone();
        let tar_files = fetch_tar_file_names(directory)?;
        println!("Tar files found: {:?}", tar_files);

        // Create a new directory under it, called "extracted".
        // If it exists already, delete it first and create again.
        path.push("extracted");
        // Use different directories for different lit-recovery instances.
        path.push(verifying_key);
        let _ = tokio::fs::remove_dir_all(&path).await; // may fail if the dir does not exist.
        tokio::fs::create_dir_all(&path).await.map_err(|e| {
            let err_msg = format!("Cannot create a subdirectory: {:?}", e);
            Error::General(err_msg)
        })?;
        let extracted_dir =
            path.to_str().ok_or(Error::General("Failed to stringify path".into()))?;

        // Extract the tar files.
        let mut bls_enc_key = blsful::inner_types::G1Projective::default();
        let mut secp256k1_enc_key = k256::AffinePoint::default();
        let mut nistp256_enc_key = p256::AffinePoint::default();
        let mut nistp384_enc_key = p384::AffinePoint::default();
        let mut ed25519_enc_key = vsss_rs::curve25519::WrappedEdwards::default();
        let mut ristretto25519_enc_key = vsss_rs::curve25519::WrappedRistretto::default();
        let mut ed448_enc_key = ed448_goldilocks_plus::EdwardsPoint::default();
        let mut jubjub_enc_key = jubjub::SubgroupPoint::IDENTITY;
        let mut decaf377_enc_key = decaf377::Element::IDENTITY;
        let mut bls12381g1_sign_enc_key = blsful::inner_types::G1Projective::default();

        // extract each tar file, and check the public keys and session id
        // to ensure they match
        for file in tar_files.iter() {
            println!("Extracting tar file: {}", file.display());

            let file_name = file
                .file_stem()
                .ok_or(Error::General("Failed to get file name".into()))?
                .to_str()
                .ok_or(Error::General("Failed to convert file name to string".into()))?;

            let destination = PathBuf::from(extracted_dir).join(file_name);

            tokio::fs::create_dir_all(&destination).await.map_err(|e| {
                let err_msg = format!("Cannot create a subdirectory: {:?}", e);
                Error::General(err_msg)
            })?;

            lit_core::utils::tar::read_tar_gz_strip_components_file(file, &destination, 1)
                .map_err(|e| Error::General(e.to_string()))?;

            let tar_session_id: String =
                read_from_disk(destination.clone(), consts::SESSION_ID_FN).await?;

            if session_id != tar_session_id {
                return Err(Error::General(format!(
                    "Session ID mismatch for tar file {}. Provided: {}, Found in the tar file: {}",
                    file.display(),
                    session_id,
                    tar_session_id
                )));
            }

            if bls_enc_key.is_identity().into() {
                bls_enc_key =
                    read_from_disk(destination.clone(), consts::BLS_ENCRYPTION_KEY_FN).await?;
            } else {
                let tmp_bls_enc_key: blsful::inner_types::G1Projective =
                    read_from_disk(destination.clone(), consts::BLS_ENCRYPTION_KEY_FN).await?;
                if tmp_bls_enc_key != bls_enc_key {
                    return Err(Error::General(format!(
                        "BLS Encryption Key doesn't mismatch for tar file {}. Expected '{}', Found in tar file '{}'",
                        file.display(),
                        hex::encode(bls_enc_key.to_compressed()),
                        hex::encode(tmp_bls_enc_key.to_compressed()),
                    )));
                }
            }
            if secp256k1_enc_key.is_identity().into() {
                secp256k1_enc_key =
                    read_from_disk(destination.clone(), consts::K256_ENCRYPTION_KEY_FN).await?;
            } else {
                let tmp_secp256k1_enc_key: k256::AffinePoint =
                    read_from_disk(destination.clone(), consts::K256_ENCRYPTION_KEY_FN).await?;
                if tmp_secp256k1_enc_key != secp256k1_enc_key {
                    return Err(Error::General(format!(
                        "K256 Encryption Key doesn't mismatch for tar file {}. Expected '{}', Found in tar file '{}'",
                        file.display(),
                        hex::encode(secp256k1_enc_key.to_bytes()),
                        hex::encode(tmp_secp256k1_enc_key.to_bytes()),
                    )));
                }
            }
            if nistp256_enc_key.is_identity().into() {
                nistp256_enc_key =
                    read_from_disk(destination.clone(), consts::P256_ENCRYPTION_KEY_FN).await?;
            } else {
                let tmp_nistp256_enc_key: p256::AffinePoint =
                    read_from_disk(destination.clone(), consts::P256_ENCRYPTION_KEY_FN).await?;
                if tmp_nistp256_enc_key != nistp256_enc_key {
                    return Err(Error::General(format!(
                        "P256 Encryption Key doesn't mismatch for tar file {}. Expected '{}', Found in tar file '{}'",
                        file.display(),
                        hex::encode(nistp256_enc_key.to_bytes()),
                        hex::encode(tmp_nistp256_enc_key.to_bytes()),
                    )));
                }
            }
            if nistp384_enc_key.is_identity().into() {
                nistp384_enc_key =
                    read_from_disk(destination.clone(), consts::P384_ENCRYPTION_KEY_FN).await?;
            } else {
                let tmp_nistp384_enc_key: p384::AffinePoint =
                    read_from_disk(destination.clone(), consts::P384_ENCRYPTION_KEY_FN).await?;
                if tmp_nistp384_enc_key != nistp384_enc_key {
                    return Err(Error::General(format!(
                        "P384 Encryption Key doesn't mismatch for tar file {}. Expected '{}', Found in tar file '{}'",
                        file.display(),
                        hex::encode(nistp384_enc_key.to_bytes()),
                        hex::encode(tmp_nistp384_enc_key.to_bytes()),
                    )));
                }
            }
            if ed25519_enc_key.is_identity().into() {
                ed25519_enc_key =
                    read_from_disk(destination.clone(), consts::ED25519_ENCRYPTION_KEY_FN).await?;
            } else {
                let tmp_ed25519_enc_key: vsss_rs::curve25519::WrappedEdwards =
                    read_from_disk(destination.clone(), consts::ED25519_ENCRYPTION_KEY_FN).await?;
                if tmp_ed25519_enc_key != ed25519_enc_key {
                    return Err(Error::General(format!(
                        "ED25519 Encryption Key doesn't mismatch for tar file {}. Expected '{}', Found in tar file '{}'",
                        file.display(),
                        hex::encode(ed25519_enc_key.to_bytes()),
                        hex::encode(tmp_ed25519_enc_key.to_bytes()),
                    )));
                }
            }
            if ristretto25519_enc_key.is_identity().into() {
                ristretto25519_enc_key =
                    read_from_disk(destination.clone(), consts::RISTRETTO25519_ENCRYPTION_KEY_FN)
                        .await?;
            } else {
                let tmp_ristretto25519_enc_key: vsss_rs::curve25519::WrappedRistretto =
                    read_from_disk(destination.clone(), consts::RISTRETTO25519_ENCRYPTION_KEY_FN)
                        .await?;
                if tmp_ristretto25519_enc_key != ristretto25519_enc_key {
                    return Err(Error::General(format!(
                        "Ristretto25519 Encryption Key doesn't mismatch for tar file {}. Expected '{}', Found in tar file '{}'",
                        file.display(),
                        hex::encode(ristretto25519_enc_key.to_bytes()),
                        hex::encode(tmp_ristretto25519_enc_key.to_bytes()),
                    )));
                }
            }
            if ed448_enc_key.is_identity().into() {
                ed448_enc_key =
                    read_from_disk(destination.clone(), consts::ED448_ENCRYPTION_KEY_FN).await?;
            } else {
                let tmp_ed448_enc_key: ed448_goldilocks_plus::EdwardsPoint =
                    read_from_disk(destination.clone(), consts::ED448_ENCRYPTION_KEY_FN).await?;
                if tmp_ed448_enc_key != ed448_enc_key {
                    return Err(Error::General(format!(
                        "ED448 Encryption Key doesn't mismatch for tar file {}. Expected '{}', Found in tar file '{}'",
                        file.display(),
                        hex::encode(ed448_enc_key.to_bytes()),
                        hex::encode(tmp_ed448_enc_key.to_bytes()),
                    )));
                }
            }
            if jubjub_enc_key.is_identity().into() {
                jubjub_enc_key =
                    read_from_disk(destination.clone(), consts::JUBJUB_ENCRYPTION_KEY_FN).await?;
            } else {
                let tmp_jubjub_enc_key: jubjub::SubgroupPoint =
                    read_from_disk(destination.clone(), consts::JUBJUB_ENCRYPTION_KEY_FN).await?;
                if tmp_jubjub_enc_key != jubjub_enc_key {
                    return Err(Error::General(format!(
                        "JubJub Encryption Key doesn't mismatch for tar file {}. Expected '{}', Found in tar file '{}'",
                        file.display(),
                        hex::encode(jubjub_enc_key.to_bytes()),
                        hex::encode(tmp_jubjub_enc_key.to_bytes()),
                    )));
                }
            }
            if decaf377_enc_key.is_identity() {
                decaf377_enc_key =
                    read_from_disk(destination.clone(), consts::DECAF377_ENCRYPTION_KEY_FN).await?;
            } else {
                let tmp_decaf377_enc_key: decaf377::Element =
                    read_from_disk(destination.clone(), consts::DECAF377_ENCRYPTION_KEY_FN).await?;
                if tmp_decaf377_enc_key != decaf377_enc_key {
                    return Err(Error::General(format!(
                        "Decaf377 Encryption Key doesn't mismatch for tar file {}. Expected '{}', Found in tar file '{}'",
                        file.display(),
                        hex::encode(decaf377_enc_key.to_bytes()),
                        hex::encode(tmp_decaf377_enc_key.to_bytes()),
                    )));
                }
            }
            if bls12381g1_sign_enc_key.is_identity().into() {
                bls12381g1_sign_enc_key =
                    read_from_disk(destination.clone(), consts::BLS12381G1_ENCRYPTION_KEY_FN)
                        .await?;
            } else {
                let tmp_bls12381g1_sign_enc_key: blsful::inner_types::G1Projective =
                    read_from_disk(destination.clone(), consts::BLS12381G1_ENCRYPTION_KEY_FN)
                        .await?;
                if tmp_bls12381g1_sign_enc_key != bls12381g1_sign_enc_key {
                    return Err(Error::General(format!(
                        "BLS12381G1_SIGN Encryption Key doesn't mismatch for tar file {}. Expected '{}', Found in tar file '{}'",
                        file.display(),
                        hex::encode(bls12381g1_sign_enc_key.to_bytes()),
                        hex::encode(tmp_bls12381g1_sign_enc_key.to_bytes()),
                    )));
                }
            }
        }

        // For each encrypted share in each encrypted folder, send decryption shares to the
        // corresponding node.
        let shares = fetch_encrypted_key_share_paths(path)?;
        println!("Total encrypted BLS12381G1 shares: {}", shares.bls12381g1.len());
        println!("Total encrypted Secp256k1 shares: {}", shares.secp256k1.len());
        println!("Total encrypted NISTP256 shares: {}", shares.nistp256.len());
        println!("Total encrypted NISTP384 shares: {}", shares.nistp384.len());
        println!("Total encrypted ED25519 shares: {}", shares.ed25519.len());
        println!("Total encrypted Ristretto25519 shares: {}", shares.ristretto25519.len());
        println!("Total encrypted ED448 shares: {}", shares.ed448.len());
        println!("Total encrypted JubJub shares: {}", shares.jubjub.len());
        println!("Total encrypted Decaf377 shares: {}", shares.decaf377.len());
        println!("Total encrypted BLS12381G1_SIGN shares: {}", shares.bls12381g1_sign.len());

        let mut upload_shares_by_staker_address = HashMap::new();
        load_upload_shares::<Bls12381G1>(
            self,
            hex::encode(bls_enc_key.to_compressed()),
            &shares.bls12381g1,
            &mut upload_shares_by_staker_address,
        )
        .await?;
        load_upload_shares::<Secp256k1>(
            self,
            hex::encode(secp256k1_enc_key.to_bytes()),
            &shares.secp256k1,
            &mut upload_shares_by_staker_address,
        )
        .await?;
        load_upload_shares::<p256::NistP256>(
            self,
            hex::encode(nistp256_enc_key.to_bytes()),
            &shares.nistp256,
            &mut upload_shares_by_staker_address,
        )
        .await?;
        load_upload_shares::<p384::NistP384>(
            self,
            hex::encode(nistp384_enc_key.to_bytes()),
            &shares.nistp384,
            &mut upload_shares_by_staker_address,
        )
        .await?;
        load_upload_shares::<Ed25519>(
            self,
            hex::encode(ed25519_enc_key.to_bytes().as_ref()),
            &shares.ed25519,
            &mut upload_shares_by_staker_address,
        )
        .await?;
        load_upload_shares::<Ristretto25519>(
            self,
            hex::encode(ristretto25519_enc_key.to_bytes().as_ref()),
            &shares.ristretto25519,
            &mut upload_shares_by_staker_address,
        )
        .await?;
        load_upload_shares::<Ed448>(
            self,
            hex::encode(ed448_enc_key.to_bytes()),
            &shares.ed448,
            &mut upload_shares_by_staker_address,
        )
        .await?;
        load_upload_shares::<JubJub>(
            self,
            hex::encode(jubjub_enc_key.to_bytes()),
            &shares.jubjub,
            &mut upload_shares_by_staker_address,
        )
        .await?;
        load_upload_shares::<Decaf377>(
            self,
            hex::encode(decaf377_enc_key.to_bytes()),
            &shares.decaf377,
            &mut upload_shares_by_staker_address,
        )
        .await?;
        load_upload_shares::<Bls12381G1>(
            self,
            hex::encode(bls12381g1_sign_enc_key.to_compressed()),
            &shares.bls12381g1_sign,
            &mut upload_shares_by_staker_address,
        )
        .await?;
        decryption::send_decryption_shares_to_nodes(self, &upload_shares_by_staker_address).await?;

        Ok(())
    }
}

struct EncryptedKeyShares {
    bls12381g1: Vec<PathBuf>,
    secp256k1: Vec<PathBuf>,
    nistp256: Vec<PathBuf>,
    nistp384: Vec<PathBuf>,
    ed25519: Vec<PathBuf>,
    ristretto25519: Vec<PathBuf>,
    ed448: Vec<PathBuf>,
    jubjub: Vec<PathBuf>,
    decaf377: Vec<PathBuf>,
    bls12381g1_sign: Vec<PathBuf>,
}

fn fetch_tar_file_names(directory: PathBuf) -> RecoveryResult<Vec<PathBuf>> {
    let pattern = format!("{}{}", LIT_BACKUP_NAME_PATTERN, LIT_BACKUP_SUFFIX);
    fetch_files_by_pattern(directory, &pattern)
}

fn fetch_encrypted_key_share_paths(path: PathBuf) -> RecoveryResult<EncryptedKeyShares> {
    let mut bls_shares = vec![];
    let mut k256_shares = vec![];
    let mut nistp256_shares = vec![];
    let mut nistp384_shares = vec![];
    let mut ed25519_shares = vec![];
    let mut ristretto25519_shares = vec![];
    let mut ed448_shares = vec![];
    let mut jubjub_shares = vec![];
    let mut decaf377_shares = vec![];
    let mut bls12381g1_sign_shares = vec![];
    let folders = fetch_files_by_pattern(path, LIT_BACKUP_NAME_PATTERN)?;
    for folder in folders.iter() {
        bls_shares.extend(fetch_files_by_pattern(
            folder.clone(),
            &format!("Key-H-{}-*.cbor", lit_node_core::CurveType::BLS as u8),
        )?);
        k256_shares.extend(fetch_files_by_pattern(
            folder.clone(),
            &format!("Key-H-{}-*.cbor", lit_node_core::CurveType::K256 as u8),
        )?);
        nistp256_shares.extend(fetch_files_by_pattern(
            folder.clone(),
            &format!("Key-H-{}-*.cbor", lit_node_core::CurveType::P256 as u8),
        )?);
        nistp384_shares.extend(fetch_files_by_pattern(
            folder.clone(),
            &format!("Key-H-{}-*.cbor", lit_node_core::CurveType::P384 as u8),
        )?);
        ed25519_shares.extend(fetch_files_by_pattern(
            folder.clone(),
            &format!("Key-H-{}-*.cbor", lit_node_core::CurveType::Ed25519 as u8),
        )?);
        ristretto25519_shares.extend(fetch_files_by_pattern(
            folder.clone(),
            &format!("Key-H-{}-*.cbor", lit_node_core::CurveType::Ristretto25519 as u8),
        )?);
        ed448_shares.extend(fetch_files_by_pattern(
            folder.clone(),
            &format!("Key-H-{}-*.cbor", lit_node_core::CurveType::Ed448 as u8),
        )?);
        jubjub_shares.extend(fetch_files_by_pattern(
            folder.clone(),
            &format!("Key-H-{}-*.cbor", lit_node_core::CurveType::RedJubjub as u8),
        )?);
        decaf377_shares.extend(fetch_files_by_pattern(
            folder.clone(),
            &format!("Key-H-{}-*.cbor", lit_node_core::CurveType::RedDecaf377 as u8),
        )?);
        bls12381g1_sign_shares.extend(fetch_files_by_pattern(
            folder.clone(),
            &format!("Key-H-{}-*.cbor", lit_node_core::CurveType::BLS12381G1 as u8),
        )?);
    }
    Ok(EncryptedKeyShares {
        bls12381g1: bls_shares,
        secp256k1: k256_shares,
        nistp256: nistp256_shares,
        nistp384: nistp384_shares,
        ed25519: ed25519_shares,
        ristretto25519: ristretto25519_shares,
        ed448: ed448_shares,
        jubjub: jubjub_shares,
        decaf377: decaf377_shares,
        bls12381g1_sign: bls12381g1_sign_shares,
    })
}

fn fetch_files_by_pattern(
    mut directory: PathBuf, name_pattern: &str,
) -> RecoveryResult<Vec<PathBuf>> {
    directory.push(name_pattern);

    let path_pattern =
        directory.to_str().ok_or(Error::General("Could not convert path to string".to_string()))?;

    match glob::glob(path_pattern) {
        Err(e) => {
            let err_msg = format!("Error reading glob pattern: {} - {:?}", path_pattern, e);
            Err(Error::General(err_msg))
        }
        Ok(entries) => Ok(entries.flatten().collect()),
    }
}

async fn read_from_disk<T>(mut path: PathBuf, file_name: &str) -> RecoveryResult<T>
where
    T: DeserializeOwned,
{
    path.push(file_name);
    let mut file = tokio::fs::File::open(path.clone())
        .await
        .map_err(|_| Error::General(format!("Could not open file: {}", path.display())))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .map_err(|_| Error::General(format!("Could not read file: {}", path.display())))?;
    let local_key: T = ciborium::de::from_reader(&*buffer)
        .map_err(|_| Error::General(format!("Could not deserialize file: {:?}", path)))?;
    Ok(local_key)
}
