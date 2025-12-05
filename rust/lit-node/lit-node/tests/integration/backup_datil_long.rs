use crate::common::recovery_party::SiweSignature;
use chrono::{Duration, Utc};
use ethers::prelude::{H160, U256};
use ethers::types::Address;
use hex::FromHex;
use k256::ecdsa::{SigningKey, VerifyingKey};
use lit_blockchain::contracts::pubkey_router::RootKey;
use lit_core::config::CFG_ADMIN_OVERRIDE_NAME;
use lit_node::auth::auth_material::JsonAuthSigExtended;
use lit_node::endpoints::auth_sig::LITNODE_ADMIN_RES;
use lit_node::peers::peer_state::models::NetworkState;
use lit_node::tss::common::restore::NodeRecoveryStatus;

use lit_node_core::{CurveType, JsonAuthSig};
use lit_node_testnet::TestSetupBuilder;
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::testnet::actions::RootKeyConfig;
use lit_node_testnet::validator::ValidatorCollection;
use reqwest::Client;
use rocket::serde::Serialize;
use sha3::{Keccak256, digest::Digest};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::task::JoinSet;
use tracing::info;

const TARBALL_NAME: &str = "lit_backup_encrypted_keys.tar.gz";

// Notes:
// This test is designed to test the recovery of a Datil backup into a Naga network.
// The datil based lit-recovery binary is used to recover the keyset from the datilbackup and upload the keyset to the nodes.
// This is not the same as the lit-recovery project that exists in this repository.
// This binary can be found athttps://github.com/LIT-Protocol/lit-recovery/pull/60
// which is the branch "Introduce staker_address_to_url_map"

#[tokio::test]
async fn recover_datil_into_naga_test() {
    unsafe {
        std::env::set_var(
            "IPFS_API_KEY",
            "NkOJGWDsFcLTn7gXH37bS85HIMJJ4-d-r2qVHJWBXOXyxJYtG7FbyXATZCEAyf2s",
        );
    }
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024) // 32MB stack
        .spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(end_to_end_test(3, 3));
        })
        .unwrap()
        .join()
        .unwrap();
}

const BLINDERS_0_BLS: &str = "5dec372f39c2f083a98e47924e7db47fbb6e8fb9be4b0a6eb4c5841ec3415f2f";
const BLINDERS_0_K256: &str = "8A3B338BF130B8C1B4D71C1692548A5F1F8E51A39520FA773D8028012BE25794";
const BLINDERS_1_BLS: &str = "0ec33d19020bd39d4825f51137b49d6da4e4aa93778f17fc0541fe20aa8874d1";
const BLINDERS_1_K256: &str = "36C0AC74655E4E4F1089701D8F7DEC5BA408D1921ABD5DF518847C5C7E57EEA4";
const BLINDERS_2_BLS: &str = "5e7779db0cd406fc1a9fcd376d455a031771052f11d18068dfc4caa5deb82016";
const BLINDERS_2_K256: &str = "578B3BD51C7DA42E7ADC4FFC70914B08B617C87B491EDBC6C8015FC9C3EF9887";

async fn end_to_end_test(number_of_nodes: usize, recovery_party_size: usize) {
    let realm_id = U256::from(1);
    let admin_signing_key = create_node_operator_admin_signing_key().await;

    crate::common::setup_logging();

    let (testnet, mut validator_collection, _end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(number_of_nodes)
        .build()
        .await;

    let backup_directory = create_recovery_directory();

    validator_collection
        .actions()
        .wait_for_epoch(realm_id, U256::from(2))
        .await;

    testnet.actions().sleep_millis(5000).await;

    let (realm_id, identifier, description) = (
        U256::from(1),
        "datil-keyset".to_string(),
        "Datil Key ".to_string(),
    );
    let keyset_id = identifier.clone();
    let root_key_configs = vec![
        RootKeyConfig {
            curve_type: CurveType::BLS,
            count: 1,
        },
        RootKeyConfig {
            curve_type: CurveType::K256,
            count: 10,
        },
    ];
    let result = validator_collection
        .actions()
        .add_keyset(realm_id, identifier, description, root_key_configs)
        .await;
    assert!(result.is_ok(), "Failed to add keyset `{}`", keyset_id);

    let tx = validator_collection
        .actions()
        .contracts()
        .pubkey_router
        .admin_reset_root_keys(
            testnet.actions().contracts().staking.address(),
            keyset_id.clone(),
        );
    tx.send().await.unwrap();
    let tx = validator_collection
        .actions()
        .contracts()
        .pubkey_router
        .admin_set_root_keys(
            testnet.actions().contracts().staking.address(),
            keyset_id.clone(),
            datil_root_keys(),
        );
    tx.send().await.unwrap();

    // stop old nodes but leave the test net up. Setting the network to restore state
    // should stop all the nodes
    info!("Setting network state to Restore");
    validator_collection
        .actions()
        .set_epoch_state(realm_id, NetworkState::Restore as u8)
        .await
        .unwrap();

    info!("Making sure that {} nodes are offline", number_of_nodes);
    for i in 0..number_of_nodes {
        let validator = validator_collection.get_validator_by_idx_mut(i);
        assert!(validator.is_node_offline());
    }

    // Since we're using the exact same contract state as before the nodes got shut down, we need to
    // allow the nodes to register their attested wallets on their next boot.
    let actions = validator_collection.actions();
    let current_validators = actions.get_current_validators(realm_id).await;
    actions
        .admin_set_register_attested_wallet_disabled_for_validators(current_validators, false)
        .await
        .expect("Failed to set register attested wallet disabled for validators");

    // nodes start in restore mode and reuse the same testnet
    info!("Restarting the nodes");
    let validator_collection2 = ValidatorCollection::builder()
        .num_staked_nodes(number_of_nodes)
        .pause_network_while_building(false)
        .build(&testnet)
        .await
        .expect("Failed to build validator collection");

    actions.sleep_millis(5000).await;

    // Use the admin endpoint to upload the backup and blinders
    let client = reqwest::ClientBuilder::new()
        .tls_sni(false)
        .build()
        .unwrap();

    let downloaded_blinders = get_downloaded_blinders();

    // Blinders need to be in-place before the key backups are uploaded
    info!("Uploading blinders to nodes");
    upload_blinders_to_nodes(
        &admin_signing_key,
        &testnet,
        &client,
        &downloaded_blinders,
        &validator_collection2,
    )
    .await;

    // Key backups need to be in-place before the decryption shares are uploaded
    info!("Uploading backups to nodes");
    upload_key_backups_to_nodes(
        &admin_signing_key,
        &testnet,
        &client,
        &validator_collection2,
        &backup_directory,
    )
    .await;

    upload_decryption_shares_to_nodes(recovery_party_size).await;
    info!("Decryption shares uploaded");

    // Wait until all keys are restored
    validator_collection
        .actions()
        .wait_for_recovery_status(NodeRecoveryStatus::AllKeysAreRestored as u8)
        .await;
    info!("All the nodes restored all the keys!");


    // sleep 1000 ms to allow the key shares to be written to disk
    validator_collection.actions().sleep_millis(1000).await;    
    
    let state = actions.get_state(realm_id.as_u64()).await;
    info!("State: {:?}", state);
    let r=  actions.set_epoch_state(realm_id, NetworkState::Active as u8).await;
    assert!(r.is_ok(), "Failed to set epoch state to Active");
    // Fast forward the network by 300 seconds, and wait for the new node to be active - effectively waiting for the next epoch.
    let current_epoch = validator_collection.actions().get_current_epoch(realm_id).await;
    validator_collection.actions().increase_blockchain_timestamp(300).await;   
    validator_collection.actions().wait_for_epoch(realm_id, current_epoch + 1).await;
    info!("Key shares should now be written to disk!");
    
}

fn datil_root_keys() -> Vec<RootKey> {
    vec![
            RootKey {
                key_type: U256::from(1),
                pubkey: ethers::types::Bytes::from_hex("0xb500ba119f643feb1981d26ffe7235288fdd39c36d6ebd35aebea7a5f92a812798513c1ae710461a6d229c59a782e375").unwrap(),
            },
            RootKey {
                key_type: U256::from(2),
                pubkey: ethers::types::Bytes::from_hex("0x02a11f8d29fabb49b5bbcd92159698afe4f136bab8b4a33f8606a71bd03bd6dc27").unwrap(),
            },
            RootKey {
                key_type: U256::from(2),
                pubkey: ethers::types::Bytes::from_hex("0x02cd471f410f17f1e932886a90effbb522a7841d9107d256c034cfa04020ba64c6").unwrap(),
            },
            RootKey {
                key_type: U256::from(2),
                pubkey: ethers::types::Bytes::from_hex("0x02d63650585b90ae80acde8fc4c638c4db0a00945f9b1c40024c92064cd99bdbbe").unwrap(),
            },
            RootKey {
                key_type: U256::from(2),
                pubkey: ethers::types::Bytes::from_hex("0x03a9e669a6f3b662a6b91fcb3cfa08608ab705e83b9b01bbf4fc4c2fcac3163b23").unwrap(),
            },
            RootKey {
                key_type: U256::from(2),
                pubkey: ethers::types::Bytes::from_hex("0x03d16416e913ba7adc1ccd58c36ff9f2130fa64d36e510551af70fb1be2174bb74").unwrap(),
            },
            RootKey {
                key_type: U256::from(2),
                pubkey: ethers::types::Bytes::from_hex("0x022e26c96cdeabee0930344a08cf3ee290c9efb3344fc8d50e460706ef7b55c518").unwrap(),
            },
            RootKey {
                key_type: U256::from(2),
                pubkey: ethers::types::Bytes::from_hex("0x027b98e8d099788fae7d9dc79865f28d4ddc0f630c6c593e5e8d7ef94c0285d729").unwrap(),
            },
            RootKey {
                key_type: U256::from(2),
                pubkey: ethers::types::Bytes::from_hex("0x033c8c0840302669019a6d0d12108caa6b0581a1d96022d4ea87ab203fba94cf1e").unwrap(),
            },
            RootKey {
                key_type: U256::from(2),
                pubkey: ethers::types::Bytes::from_hex("0x039af7bc7d673c899cc45ec5e30ba518be438931e9acb916fef7a336b9954687e9").unwrap(),
            },
            RootKey {
                key_type: U256::from(2),
                pubkey: ethers::types::Bytes::from_hex("0x023403362ef1a693967858606e0cd9c5a67b30d5bd3a1a70a960c1286c15c8f68a").unwrap(),
            },
        ]
}
async fn upload_decryption_shares_to_nodes(recovery_party_size: usize) {
    use tokio::process::Command;

    for i in 0..recovery_party_size {
        let share_db_path = format!(
            "./tests/test_data/datil_recovery_into_naga/lit-recovery-data/sdb{}.db3",
            i + 1
        );
        let keyringdb = format!(
            "--file=./tests/test_data/datil_recovery_into_naga/keyringdb/{}",
            i + 1
        );

        let mut command = Command::new("./tests/test_data/datil_recovery_into_naga/lit-recovery");

        command.env("SHARE_DB_PATH", &share_db_path)
        .arg("--password=a")
        .arg(keyringdb)
        .arg("recover")
        .arg("--bls12381g1-encryption-key")
        .arg("b0aa1aeaf1f4fa72e59905a4d0723ce4b6f53a277f75b38c9ae87a31fa7d40825c22b83dd18e821a316303e69681ee66")
        .arg("--secp256k1-encryption-key")
        .arg("02a220b4caab1baa5d0b24612743803f1b40980ad56b7904ed83da3e012eb366a2")
        .arg("--directory")
        .arg("tests/test_data/datil_recovery_into_naga/backups");

        println!("command: {:?}", command);
        let output = command.output().await.unwrap();
        if !output.stderr.is_empty() {
            println!(
                "stdout of lit-recovery tool: {}",
                String::from_utf8(output.stdout).unwrap()
            );
            println!(
                "stderr of lit-recovery tool: {}",
                String::from_utf8(output.stderr).unwrap()
            );
            panic!("lit-recovery tool encountered an error.");
        }
    }
}

async fn upload_key_backups_to_nodes(
    admin_signing_key: &SigningKey,
    testnet: &Testnet,
    client: &Client,
    validator_collection: &ValidatorCollection,
    backup_directory: &PathBuf,
) {
    let validators = validator_collection.get_active_validators().await.unwrap();
    let mut join_set = JoinSet::new();
    // Download the backups and blinders
    for &validator in validators.iter() {
        let public_address = validator.public_address();
        let chain_id = testnet.chain_id;
        let client = client.clone();
        let admin_signing_key = admin_signing_key.clone();
        let backup_directory = backup_directory.clone();
        join_set.spawn(async move {
            let url = format!("http://{}", public_address.clone());
            let auth_sig =
                generate_admin_auth_sig(&admin_signing_key, chain_id, &url, &public_address);
            let json_body = serde_json::to_string(&auth_sig.auth_sig).unwrap();

            let tar_file = backup_directory.join(format!("{}{}", public_address, TARBALL_NAME));
            let file = tokio::fs::File::open(tar_file).await.unwrap();

            info!("Uploading backup for validator {}", public_address);
            let response = client
                .post(format!("{}/web/admin/set_key_backup", url))
                .header("Content-Type", "application/octet-stream")
                .header(
                    "x-auth-sig",
                    data_encoding::BASE64URL.encode(json_body.as_bytes()),
                )
                .body(file)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            debug!("Response: {}", response);
            let response: serde_json::Value = serde_json::from_str(&response).unwrap();
            let success = response.as_object().unwrap().get("success");
            let success = if let Some(success) = success {
                success.as_str().map(|s| s == "true").unwrap_or_default()
            } else {
                false
            };
            (public_address, success)
        });
    }
    while let Some(node_info) = join_set.join_next().await {
        let (public_address, success) = node_info.unwrap();
        info!("Node {} received tar backup: {}", public_address, success);
        assert!(success);
    }
}

#[derive(Clone, Default, Serialize)]
struct DatilBlinders {
    bls_blinder: String,
    k256_blinder: String,
}

fn get_downloaded_blinders() -> HashMap<String, DatilBlinders> {
    let mut blinders0 = DatilBlinders::default();
    blinders0.bls_blinder = BLINDERS_0_BLS.to_string();
    blinders0.k256_blinder = BLINDERS_0_K256.to_string();

    let mut blinders1 = DatilBlinders::default();
    blinders1.bls_blinder = BLINDERS_1_BLS.to_string();
    blinders1.k256_blinder = BLINDERS_1_K256.to_string();

    let mut blinders2 = DatilBlinders::default();
    blinders2.bls_blinder = BLINDERS_2_BLS.to_string();
    blinders2.k256_blinder = BLINDERS_2_K256.to_string();

    let mut map = HashMap::new();
    map.insert(String::from("127.0.0.1:7470"), blinders0);
    map.insert(String::from("127.0.0.1:7471"), blinders1);
    map.insert(String::from("127.0.0.1:7472"), blinders2);
    map
}

async fn upload_blinders_to_nodes(
    admin_signing_key: &SigningKey,
    testnet: &Testnet,
    client: &Client,
    downloaded_blinders: &HashMap<String, DatilBlinders>,
    validator_collection2: &ValidatorCollection,
) {
    let validators = validator_collection2.get_active_validators().await.unwrap();
    let mut join_set = JoinSet::new();

    for &validator in validators.iter() {
        let public_address = validator.public_address();
        let admin_signing_key = admin_signing_key.clone();
        let chain_id = testnet.chain_id;
        let client = client.clone();
        let blinders = downloaded_blinders[&public_address].clone();

        join_set.spawn(async move {
            // Send the blinders to the node operators
            let url = format!("http://{}/web/admin/set_blinders", public_address);
            let auth_sig =
                generate_admin_auth_sig(&admin_signing_key, chain_id, &url, &public_address);
            let auth_sig = serde_json::to_string(&auth_sig.auth_sig).unwrap();

            let json_body = serde_json::to_string(&blinders).unwrap();

            info!(
                "{} Sending blinders: {}",
                public_address,
                serde_json::to_string_pretty(&blinders).unwrap()
            );
            info!("Sending blinders to validator: {}", url);
            let response = client
                .post(url)
                .header("Content-Type", "application/octet-stream")
                .header(
                    "x-auth-sig",
                    data_encoding::BASE64URL.encode(auth_sig.as_bytes()),
                )
                .body(json_body)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            info!("Response: {}", response);
            public_address
        });
    }
    while let Some(node_info) = join_set.join_next().await {
        let public_address = node_info.unwrap();
        info!("Node {} received blinders", public_address);
    }
}

fn create_recovery_directory() -> PathBuf {
    let mut backup_directory = std::env::current_dir().unwrap();
    backup_directory.push("tests/test_data/datil_recovery_into_naga/backups");
    backup_directory
}

async fn create_node_operator_admin_signing_key() -> SigningKey {
    let admin_signing_key = SigningKey::random(&mut rand::rngs::OsRng);
    let admin_address = admin_signing_key.to_eth_address_str();

    tokio::fs::write(
        format!("./{}.toml", CFG_ADMIN_OVERRIDE_NAME),
        format!(
            r#"[node]
admin_address = "{}"
    "#,
            admin_address
        ),
    )
    .await
    .unwrap();

    info!(
        "Starting backup recovery test with admin_address = {}",
        admin_address
    );
    admin_signing_key
}

fn generate_admin_auth_sig(
    signing_key: &SigningKey,
    chain_id: u64,
    uri: &str,
    domain: &str,
) -> JsonAuthSigExtended {
    let address = signing_key.to_eth_address_str();
    let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.fZ").to_string();
    let expiration = (Utc::now() + Duration::days(1)) // Sets 1 day expiration
        .format("%Y-%m-%dT%H:%M:%S%.fZ")
        .to_string();
    let siwe_message = siwe::Message {
        domain: domain.parse().unwrap(),
        address: signing_key.to_eth_address().0,
        statement: None,
        uri: uri.parse().unwrap(),
        version: siwe::Version::V1,
        chain_id,
        nonce: "AAAAAAAAAAAAAAAAA".into(),
        issued_at: now.parse().unwrap(),
        expiration_time: Some(expiration.parse().unwrap()),
        not_before: None,
        request_id: None,
        resources: vec![LITNODE_ADMIN_RES.parse().unwrap()],
    };
    let signed_message = siwe_message.to_string();

    let (signature, recovery_id) = signing_key.sign_siwe(signed_message.as_bytes());
    let mut buffer = [0u8; 65];
    buffer[..64].copy_from_slice(&signature.to_bytes());
    buffer[64] = recovery_id.to_byte();
    JsonAuthSigExtended {
        auth_sig: JsonAuthSig::new(
            hex::encode(&buffer),
            "web3.eth.personal.sign".to_string(),
            signed_message,
            address,
            None,
        ),
    }
}

trait EthereumAddress {
    fn to_eth_address_str(&self) -> String {
        let address = fmt_address(&self.to_eth_address().0);
        let mut buffer = String::new();
        buffer.push('0');
        buffer.push('x');
        buffer.push_str(&String::from_utf8(address.to_vec()).unwrap());
        buffer
    }

    fn to_eth_address(&self) -> Address;
}

impl EthereumAddress for VerifyingKey {
    fn to_eth_address(&self) -> Address {
        let pub_key_pt = self.to_encoded_point(false);
        let digest = keccak256(&pub_key_pt.as_bytes()[1..]);
        let last_20 = <[u8; 20]>::try_from(&digest[12..]).unwrap();
        H160::from_slice(&last_20)
    }
}

impl EthereumAddress for SigningKey {
    fn to_eth_address(&self) -> Address {
        let public_key = self.verifying_key();
        public_key.to_eth_address()
    }
}

fn fmt_address(bytes: &[u8; 20]) -> [u8; 40] {
    let mut buffer = [0u8; 40];
    hex::encode_to_slice(bytes, &mut buffer).unwrap();

    let checksum = keccak256(&buffer);

    for i in 0..buffer.len() {
        let byte = checksum[i / 2];
        let nibble = 0xf & if i & 1 == 0 { byte >> 4 } else { byte };
        if nibble >= 8 {
            buffer[i] = buffer[i].to_ascii_uppercase();
        }
    }
    buffer
}

fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::default();
    hasher.update(bytes);
    hasher.finalize().into()
}
