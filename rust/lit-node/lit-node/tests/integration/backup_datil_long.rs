use crate::common::auth_sig::get_session_sigs_for_auth;
use crate::common::pkp::sign_with_pkp_request;
use crate::common::recovery_party::SiweSignature;
use crate::common::web_user_tests::{
    assert_decrypted, prepare_test_encryption_parameters,
    retrieve_decryption_key_session_sigs_with_version,
};
use chrono::{Duration, Utc};
use ethers::prelude::{H160, U256};
use ethers::signers::Signer;
use ethers::types::{Address, TransactionRequest};
use hex::FromHex;
use k256::ecdsa::{SigningKey, VerifyingKey};
use lit_blockchain::contracts::pubkey_router::RootKey;
use lit_core::config::CFG_ADMIN_OVERRIDE_NAME;
use lit_node::auth::auth_material::JsonAuthSigExtended;
use lit_node::endpoints::auth_sig::LITNODE_ADMIN_RES;
use lit_node::peers::peer_state::models::NetworkState;
use lit_node::tss::common::restore::NodeRecoveryStatus;

use lit_node::tss::util::DEFAULT_KEY_SET_NAME;
use lit_node_core::{
    CurveType, JsonAuthSig, LitAbility, LitResourceAbilityRequest,
    LitResourceAbilityRequestResource, SigningScheme,
};
use lit_node_testnet::DATIL_KEY_SET_NAME;
use lit_node_testnet::TestSetupBuilder;
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::node_collection::get_identity_pubkeys_from_node_set;
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::testnet::actions::{Actions, keysets::RootKeyConfig};
use lit_node_testnet::validator::ValidatorCollection;
use reqwest::Client;
use rocket::serde::Serialize;
use sha3::{Keccak256, digest::Digest};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::task::JoinSet;
use tracing::info;

const BACKUP_ENCRYPTED_KEYS: &str = "lit_backup_encrypted_keys.tar.gz";

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

    let epoch_length = 300_usize;
    let (testnet, mut validator_collection, mut end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(number_of_nodes)
        .epoch_length(epoch_length)
        .include_datil_testnet(true)
        .build()
        .await;

    let backup_directory = create_recovery_directory();
    let actions = validator_collection.actions().clone();
    actions.wait_for_epoch(realm_id, U256::from(2)).await;

    let (realm_id, identifier, description) = (
        U256::from(1),
        "datil".to_string(),
        "Datil Key Set".to_string(),
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
    let result = actions
        .add_keyset(realm_id, identifier, description, root_key_configs)
        .await;
    assert!(result.is_ok(), "Failed to add keyset `{keyset_id}`");

    let tx = actions.contracts().pubkey_router.admin_set_root_keys(
        testnet.actions().contracts().staking.address(),
        keyset_id.clone(),
        datil_root_keys(),
    );
    tx.send().await.unwrap();

    // stop old nodes but leave the test net up. Setting the network to restore state
    // should stop all the nodes
    info!("Setting network state to Restore");
    actions
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

    let actions = validator_collection2.actions();
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
    actions
        .wait_for_recovery_status(NodeRecoveryStatus::AllKeysAreRestored as u8)
        .await;
    info!("All the nodes restored all the keys!");

    // Get and log root keys for both keysets
    let datil_root_keys = validator_collection
        .actions()
        .get_all_root_keys(DATIL_KEY_SET_NAME)
        .await;
    let naga_keyset1_root_keys = validator_collection
        .actions()
        .get_all_root_keys(DEFAULT_KEY_SET_NAME)
        .await;
    info!("Datil root keys: {:?}", datil_root_keys);
    info!("Naga keyset1 root keys: {:?}", naga_keyset1_root_keys);

    // Advance one more DKG to write key shares to disk for the restored keyset. Note that
    // restored key shares are NOT written to disk until the next DKG.

    // Fast forward time to allow nodes to start a DKG to advance to the next epoch.
    validator_collection
        .actions()
        .increase_blockchain_timestamp(epoch_length)
        .await;
    // Admin set epoch state to active to pull nodes out of the restore mode
    validator_collection
        .actions()
        .set_epoch_state(realm_id, NetworkState::NextValidatorSetLocked as u8)
        .await
        .expect("Failed to set epoch state to active");

    validator_collection
        .actions()
        .wait_for_epoch(realm_id, U256::from(3))
        .await;

    info!("Testing encrypt and decrypt with datil keyset");
    test_datil_encrypt_naga_decrypt(&validator_collection, &end_user).await;

    info!("Testing PKP signing with datil keyset");
    test_datil_keyset_pkp_signing(&testnet, &validator_collection, &mut end_user).await;
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

        println!("command: {command:?}");
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

            let tar_file =
                backup_directory.join(format!("{public_address}{BACKUP_ENCRYPTED_KEYS}"));
            let file = tokio::fs::File::open(tar_file).await.unwrap();

            info!("Uploading backup for validator {}", public_address);
            let response = client
                .post(format!("{url}/web/admin/set_key_backup"))
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
            let url = format!("http://{public_address}/web/admin/set_blinders");
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
        format!("./{CFG_ADMIN_OVERRIDE_NAME}.toml"),
        format!(
            r#"[node]
admin_address = "{admin_address}"
    "#
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
            hex::encode(buffer),
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
        buffer.push_str(core::str::from_utf8(&address).unwrap());
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

// Assertion helpers
async fn get_bls_pubkey(actions: &Actions, key_set_id: &str) -> String {
    let bls_pubkey = actions
        .get_root_keys(1, key_set_id)
        .await
        .expect("Failed to get root keys");
    assert!(!bls_pubkey.is_empty());
    bls_pubkey[0].clone()
}

async fn test_datil_encrypt_naga_decrypt(
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
) {
    // Encrypt using datil BLS pubkey
    let test_encryption_parameters = prepare_test_encryption_parameters();
    let key_set_id = DATIL_KEY_SET_NAME;
    let datil_bls_pubkey = get_bls_pubkey(validator_collection.actions(), key_set_id).await;

    let datil_bls_pubkey =
        blsful::PublicKey::try_from(hex::decode(&datil_bls_pubkey).unwrap()).unwrap();
    let message_bytes = test_encryption_parameters.to_encrypt.as_bytes();
    let ciphertext = lit_sdk::encryption::encrypt_time_lock(
        &datil_bls_pubkey,
        message_bytes,
        &test_encryption_parameters.identity_param,
    )
    .expect("Unable to encrypt");
    debug!(
        "encrypting with pubkey {} -> ciphertext: {:?}",
        datil_bls_pubkey, ciphertext
    );

    // Decrypt by specifying the datil keyset ID against the nodes
    let epoch = validator_collection
        .actions()
        .get_current_epoch(U256::from(1))
        .await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let signer = end_user.signing_provider().clone();
    let session_sigs = get_session_sigs_for_auth(
        &node_set,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: format!(
                    "{}/{}",
                    test_encryption_parameters.hashed_access_control_conditions,
                    test_encryption_parameters.data_to_encrypt_hash
                ),
                resource_prefix: "lit-accesscontrolcondition".to_string(),
            },
            ability: LitAbility::AccessControlConditionDecryption.to_string(),
        }],
        Some(signer.signer().clone()),
        None,
        Some(U256::MAX), // max_price
    );
    let decryption_resp = retrieve_decryption_key_session_sigs_with_version(
        test_encryption_parameters.clone(),
        &session_sigs,
        epoch.as_u64(),
        key_set_id,
    )
    .await;

    assert_decrypted(
        &datil_bls_pubkey,
        test_encryption_parameters.identity_param.clone(),
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp,
    );

    info!("Decryption checks passed");
}

// TODO: Need to actually set up permissions for the datil PKP before sending in the signing request.
async fn test_datil_keyset_pkp_signing(
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
    end_user: &mut EndUser,
) {
    // Let's use the mint-grant-burn pattern to properly test authing against permissions registered on the datil chain.
    // First add a non-owner wallet as a permitted address of the PKP on datil chain.
    let non_owner_end_user = EndUser::new(testnet);
    non_owner_end_user.fund_wallet_default_amount().await;
    non_owner_end_user.deposit_to_wallet_ledger_default().await;

    let datil_pkp = end_user.first_pkp();
    datil_pkp
        .add_permitted_address_to_pkp(non_owner_end_user.wallet.address(), &[U256::from(1)])
        .await
        .expect("Could not add permitted address to pkp");

    // Burn the PKP
    let burned = datil_pkp.burn_pkp().await;
    assert!(burned.is_ok());

    let pkp_address = datil_pkp.eth_address;

    // Now try signing with the permitted non-owner wallet.
    let value_to_send = 10;
    let tx = TransactionRequest::new()
        .to("0x0000000000000000000000000000000000000000"
            .parse::<Address>()
            .unwrap())
        .value(value_to_send)
        .from(pkp_address)
        .gas(21000)
        .gas_price(1000000000_u64)
        .chain_id(31337)
        .nonce(0)
        .data(vec![]);
    // let to_sign_as_sighash = tx.sighash();
    // let to_sign = to_sign_as_sighash.0.to_vec();

    let node_set = validator_collection
        .partially_random_threshold_nodeset(&vec![])
        .await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;

    let epoch = validator_collection
        .actions()
        .get_current_epoch(U256::from(1))
        .await
        .as_u64();

    let to_sign = "Testing signing with datil keyset on naga after restore!".to_string();
    let to_sign = keccak256(to_sign.as_bytes()).to_vec();

    // Make sure the end user has a PKP
    end_user.new_pkp().await.expect("Could not mint PKP");
    let pubkey = end_user.first_pkp().pubkey.clone();
    let key_set_id = &end_user.first_pkp().key_set_id;

    assert!(
        sign_with_pkp_request(
            &node_set,
            end_user.wallet.clone(),
            to_sign,
            pubkey,
            epoch,
            SigningScheme::EcdsaK256Sha256,
            key_set_id
        )
        .await
        .is_ok()
    );
}
