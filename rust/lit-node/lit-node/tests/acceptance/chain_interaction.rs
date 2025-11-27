use std::sync::Arc;

use chrono::{Duration, SecondsFormat};
use ethers::core::k256::ecdsa::SigningKey;
use lit_node_testnet::end_user::EndUser;
use std::ops::Add;

use crate::common::web_user_tests::TestEncryptionParameters;
use crate::common::web_user_tests::assert_decrypted;
use crate::common::web_user_tests::retrieve_decryption_key;

use lit_node_testnet::node_collection::{get_identity_pubkeys_from_node_set, get_network_pubkey};
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;

#[allow(dead_code)]
use ethers::contract::abigen;
use ethers::providers::Middleware;
use ethers::signers::{LocalWallet, Signer, Wallet};
use ethers::types::Bytes;
use ethers::types::TransactionRequest;
use ethers::utils::to_checksum;
use lit_core::utils::binary::hex_to_bytes;
use lit_node_testnet::TestSetupBuilder;
use rand_core::OsRng;
use sha2::{Digest, Sha256};

use lit_node::utils::encoding::bytes_to_hex;
use lit_node_core::{
    AccessControlConditionItem, AccessControlConditionResource, AuthMaterialType,
    JsonAccessControlCondition, JsonAuthSig, JsonReturnValueTest, LitResource,
    constants::{
        AUTH_SIG_DERIVED_VIA_CONTRACT_SIG, AUTH_SIG_DERIVED_VIA_CONTRACT_SIG_SHA256,
        CHAIN_LOCALCHAIN,
    },
};
use lit_rust_crypto::blsful::PublicKey;

use lit_node::models::RequestConditions;

use lit_node::utils::web::hash_access_control_conditions;

use tracing::{debug, info};

#[derive(Clone, Copy)]
enum HashType {
    Keccak256,
    Sha256,
}

impl HashType {
    fn hash(&self, message: &[u8]) -> [u8; 32] {
        match self {
            HashType::Keccak256 => {
                let hash = ethers::utils::hash_message(message).0;
                debug!("Test Keccak256 hash: {:?}", hash);
                hash
            }
            HashType::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(message);
                hasher.finalize().into()
            }
        }
    }

    fn get_auth_sig_type(&self) -> String {
        match self {
            HashType::Keccak256 => AUTH_SIG_DERIVED_VIA_CONTRACT_SIG.to_string(),
            HashType::Sha256 => AUTH_SIG_DERIVED_VIA_CONTRACT_SIG_SHA256.to_string(),
        }
    }
}

async fn test_encryption_decryption_eip1271(
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
    hash_type: HashType,
) {
    // setup our wallet
    let wallet = end_user.wallet.clone();
    let actions = validator_collection.actions();

    // deploy the eip1271 contract
    let _contract_name = "EIP1271";
    let deploy_txn = include_str!("contracts/EIP1271/EIP1271_deploytxn.hex");
    let deploy_txn = hex_to_bytes(deploy_txn).unwrap();
    let deploy_txn: Bytes = deploy_txn.into();
    let client = end_user.signing_provider();

    let tx = TransactionRequest::new()
        .data(deploy_txn)
        .chain_id(testnet.chain_id)
        .from(wallet.address());

    let pending = client.send_transaction(tx, None).await.unwrap();
    let receipt = pending.await.unwrap().unwrap();
    let contract_address = receipt.contract_address.unwrap();

    info!("EIP1271 deployed to {:?}", contract_address);

    abigen!(EIP1271, "./tests/acceptance/contracts/EIP1271/EIP1271.json");
    let contract = EIP1271::new(contract_address, Arc::new(client.clone()));

    // validate that the contract works
    let valid_result = "0x1626ba7e";
    let invalid_result = "ffffffff";
    let valid_result_bytes: Bytes = hex_to_bytes(valid_result).unwrap().into();
    let invalid_result_bytes: Bytes = hex_to_bytes(invalid_result).unwrap().into();

    // okay now let's try encrypting with this condition and then decrypting
    let chain = CHAIN_LOCALCHAIN.to_string();
    let access_control_conditions = Some(vec![AccessControlConditionItem::Condition(
        JsonAccessControlCondition {
            contract_address: "".to_string(),
            chain: chain.clone(),
            standard_contract_type: "".to_string(),
            method: "".to_string(),
            parameters: vec![":userAddress".to_string()],
            return_value_test: JsonReturnValueTest {
                comparator: "=".to_string(),
                value: format!("0x{}", bytes_to_hex(contract_address.as_bytes())),
            },
        },
    )]);

    // Get the resource key
    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: access_control_conditions.clone(),
        evm_contract_conditions: None,
        sol_rpc_conditions: None,
        unified_access_control_conditions: None,
    })
    .unwrap();

    // Encrypt
    let to_encrypt = "super secret message";
    let mut hasher = Sha256::new();
    hasher.update(to_encrypt.as_bytes());
    let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());
    let network_pubkey = get_network_pubkey(validator_collection.actions()).await;
    let message_bytes = to_encrypt.as_bytes();
    let identity_param = AccessControlConditionResource::new(format!(
        "{}/{}",
        hashed_access_control_conditions, data_to_encrypt_hash
    ))
    .get_resource_key()
    .into_bytes();
    let pubkey = PublicKey::try_from(&hex::decode(&network_pubkey).unwrap()).unwrap();
    let ciphertext =
        lit_sdk::encryption::encrypt_time_lock(&pubkey, message_bytes, &identity_param)
            .expect("Unable to encrypt");
    info!("ciphertext: {:?}", ciphertext);

    let node_set = &validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let realm_id = ethers::types::U256::from(1);
    let epoch = actions.get_current_epoch(realm_id).await.as_u64();

    info!(
        "1. FAIL: Should not decrypt since we passing a random _hash signed by the user which could be available on-chain instead of a valid unexpired SIWE message"
    );
    let message_to_sign = "Random message signed by the owner";
    let hashed_message = hash_type.hash(message_to_sign.as_bytes());

    let signature = wallet
        .sign_hash(ethers::types::TxHash(hashed_message))
        .unwrap();
    let sig_bytes: Bytes = signature.to_vec().into();

    let is_valid = contract
        .is_valid_signature(hashed_message.into(), sig_bytes.clone())
        .call()
        .await
        .unwrap();
    let is_valid_bytes: Bytes = is_valid.into();
    assert_eq!(is_valid_bytes, valid_result_bytes);

    info!(
        "1.1. isValidSignature() succeeded on-chain but the nodes won't validate this as it's not a valid unexpired SIWE message. So any random message or `_hash` signed by the user won't be permitted by the nodes"
    );
    // create a random EIP1271 authsig
    let auth_sig = JsonAuthSig::new_with_type(
        format!("0x{:}", &signature.to_string()),
        hash_type.get_auth_sig_type(),
        message_to_sign.to_string(),
        to_checksum(&contract_address, None),
        None,
        AuthMaterialType::ContractSig,
        None,
    );
    info!("1.2. Random auth_sig: {:?}", auth_sig);

    let test_encryption_params = TestEncryptionParameters {
        to_encrypt: message_to_sign.to_string(),
        data_to_encrypt_hash,
        access_control_conditions,
        evm_contract_conditions: None,
        sol_rpc_conditions: None,
        unified_access_control_conditions: None,
        chain: Some(chain.clone()),
        hashed_access_control_conditions,
        identity_param,
    };

    let decryption_resp =
        retrieve_decryption_key(&node_set, test_encryption_params.clone(), &auth_sig, epoch).await;

    for response in &decryption_resp {
        debug!("response- {:?}", response);
        assert!(!response.ok);
        let error = response.error_object.as_ref().unwrap();
        assert!(error.contains("NodeInvalidAuthSig"));
        assert!(error.contains("Parse error on SIWE"));
    }

    info!(
        "2. FAIL: Should not decrypt even though we are passing a valid unexpired SIWE message from another user which fails the EIP1271 contract check"
    );
    // create a EIP1271 authsig with a different wallet
    let non_permitted_wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);
    let siwe_message = get_siwe_message(&non_permitted_wallet);
    let siwe_message_hash = hash_type.hash(siwe_message.as_bytes());

    // User signs the hash of the SIWE message NOT the original SIWE message since `isValidSignature()` verifies the `_signature` against the `_hash`
    let siwe_signature = non_permitted_wallet
        .sign_hash(ethers::types::TxHash(siwe_message_hash))
        .unwrap();

    // validate that the contract works not for the non-permitted wallet's SIWE hash signature
    let siwe_sig_bytes: Bytes = siwe_signature.to_vec().into();
    let is_valid = contract
        .is_valid_signature(siwe_message_hash.into(), siwe_sig_bytes.clone())
        .call()
        .await
        .unwrap();
    let is_valid_bytes: Bytes = is_valid.into();
    assert_eq!(is_valid_bytes, invalid_result_bytes);

    info!(
        "2.1. SIWE sig NOT validated on-chain since it's from a non-permitted wallet as per the EIP1271 contract."
    );
    // create a valid SIWE EIP1271 authsig
    let auth_sig = JsonAuthSig::new_with_type(
        format!("0x{:}", &siwe_signature.to_string()),
        hash_type.get_auth_sig_type(),
        siwe_message,
        to_checksum(&contract_address, None),
        None,
        AuthMaterialType::ContractSig,
        None,
    );

    info!("2.2. Non-permitted SIWE auth_sig: {:?}", auth_sig);
    let decryption_resp =
        retrieve_decryption_key(&node_set, test_encryption_params.clone(), &auth_sig, epoch).await;

    for response in &decryption_resp {
        debug!("response- {:?}", response);
        assert!(!response.ok);
        let error = response.error_object.as_ref().unwrap();
        assert!(error.contains("Access control failed for Smart contract"));
        assert!(error.contains("EIP1271 Authsig failed"));
        assert!(error.contains("Return value was ffffffff."));
    }

    info!("3. PASS: Should decrypt since we passing a 'unhashed' valid SIWE message as the _hash");
    let siwe_message = get_siwe_message(&wallet);
    let siwe_message_hash = hash_type.hash(siwe_message.as_bytes());

    // User signs the hash of the SIWE message NOT the original SIWE message since `isValidSignature()` verifies the `_signature` against the `_hash`
    let siwe_signature = wallet
        .sign_hash(ethers::types::TxHash(siwe_message_hash))
        .unwrap();

    // validate that the contract works for the SIWE hash signature
    let siwe_sig_bytes: Bytes = siwe_signature.to_vec().into();
    let is_valid = contract
        .is_valid_signature(siwe_message_hash.into(), siwe_sig_bytes.clone())
        .call()
        .await
        .unwrap();
    let is_valid_bytes: Bytes = is_valid.into();
    assert_eq!(is_valid_bytes, valid_result_bytes);

    info!("3.1. SIWE sig validated on-chain since it's from the owner wallet");
    // create a valid SIWE EIP1271 authsig
    let auth_sig = JsonAuthSig::new_with_type(
        format!("0x{:}", &siwe_signature.to_string()),
        hash_type.get_auth_sig_type(),
        siwe_message,
        to_checksum(&contract_address, None),
        None,
        AuthMaterialType::ContractSig,
        None,
    );

    info!("3.2. Valid SIWE auth_sig: {:?}", auth_sig);
    let decryption_resp =
        retrieve_decryption_key(&node_set, test_encryption_params.clone(), &auth_sig, epoch).await;
    debug!("decryption_resp: {:?}", decryption_resp);

    assert_decrypted(
        &pubkey,
        test_encryption_params.identity_param.clone(),
        to_encrypt,
        &ciphertext,
        decryption_resp,
    );
}

async fn test_encryption_decryption_eip1271_keccak256(
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
) {
    info!("Testing decryption with eip1271_keccka256");
    test_encryption_decryption_eip1271(
        testnet,
        validator_collection,
        end_user,
        HashType::Keccak256,
    )
    .await;
}

async fn test_encryption_decryption_eip1271_sha256(
    testnet: &Testnet,
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
) {
    info!("Testing decryption with eip1271_sha256");
    test_encryption_decryption_eip1271(testnet, validator_collection, end_user, HashType::Sha256)
        .await;
}

#[tokio::test]
async fn test_chain_interaction() {
    crate::common::setup_logging();
    // because we are NOT generated session sigs, and the payment flow requires them, we need to disable payment
    // is this a todo to change code to support straight through authSigs for payment?
    let (testnet, validator_collection, end_user) = TestSetupBuilder::default()
        .enable_payment("false".to_string())
        .build()
        .await;

    test_encryption_decryption_eip1271_keccak256(&testnet, &validator_collection, &end_user).await;
    test_encryption_decryption_eip1271_sha256(&testnet, &validator_collection, &end_user).await;
}

fn get_siwe_message(wallet: &Wallet<SigningKey>) -> String {
    let chain_id = wallet.chain_id();
    let address = to_checksum(&wallet.address(), None);
    let now = chrono::Utc::now();
    let issue_datetime = now.to_rfc3339_opts(SecondsFormat::Millis, true);
    let expiration_datetime = now
        .add(Duration::days(1))
        .to_rfc3339_opts(SecondsFormat::Millis, true);
    let message = format!(
        "localhost wants you to sign in with your Ethereum account:
{}

This is a key for a Lit Action Test.

URI: https://localhost/
Version: 1
Chain ID: {}
Nonce: 1LF00rraLO4f7ZSIt
Issued At: {}
Expiration Time: {}",
        address, chain_id, issue_datetime, expiration_datetime
    );

    message
}
