use crate::common::auth_sig::{get_session_sigs_and_node_set_for_pkp, get_session_sigs_for_auth};
use crate::common::lit_actions::HELLO_WORLD_LIT_ACTION_CODE;
use crate::common::lit_actions::execute_lit_action_session_sigs;
use crate::common::lit_actions::{assert_signed_action, lit_action_params};
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::node_collection::{NodeIdentityKey, get_identity_pubkeys_from_node_set};
use lit_node_testnet::node_collection::{get_network_pubkey, get_network_pubkey_from_node_set};
use lit_node_testnet::validator::ValidatorCollection;
use std::collections::HashMap;

use crate::common::auth_sig::generate_authsig;
use anyhow::Result;
use blsful::Bls12381G2Impl;
use ethers::signers::LocalWallet;
use ethers::types::U256;
use rand::Rng;
use rand_core::OsRng;
use sha2::{Digest, Sha256};

use lit_node::utils::encoding::bytes_to_hex;
use lit_node_core::{
    AccessControlConditionItem, AccessControlConditionResource, AuthSigItem,
    EVMContractConditionItem, JsonAccessControlCondition, JsonAuthSig, JsonReturnValueTest,
    LitAbility, LitResource, LitResourceAbilityRequest, LitResourceAbilityRequestResource, NodeSet,
    SolRpcConditionItem, UnifiedAccessControlCondition, UnifiedAccessControlConditionItem,
    constants::CHAIN_LOCALCHAIN, request::EncryptionSignRequest, response::EncryptionSignResponse,
};

use lit_node::models::RequestConditions;
use lit_node_core::response::JsonExecutionResponse;

use lit_node::utils::web::hash_access_control_conditions;

use super::session_sigs::SessionSigAndNodeSet;
use lit_node_core::response::GenericResponse;
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct TestEncryptionParameters {
    pub to_encrypt: String,
    pub data_to_encrypt_hash: String,
    pub access_control_conditions: Option<Vec<AccessControlConditionItem>>,
    pub evm_contract_conditions: Option<Vec<EVMContractConditionItem>>,
    pub sol_rpc_conditions: Option<Vec<SolRpcConditionItem>>,
    pub unified_access_control_conditions: Option<Vec<UnifiedAccessControlConditionItem>>,
    pub chain: Option<String>,
    pub hashed_access_control_conditions: String,
    pub identity_param: Vec<u8>,
}

pub fn prepare_test_encryption_parameters() -> TestEncryptionParameters {
    let to_encrypt = "hello this is a test";

    // sha256 the plaintext
    let mut hasher = Sha256::new();
    hasher.update(to_encrypt.as_bytes());
    let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

    let chain = Some(CHAIN_LOCALCHAIN.to_string());
    let unified_access_control_conditions =
        Some(vec![UnifiedAccessControlConditionItem::Condition(
            UnifiedAccessControlCondition::JsonAccessControlCondition(JsonAccessControlCondition {
                contract_address: "".to_string(),
                chain: CHAIN_LOCALCHAIN.to_string(),
                standard_contract_type: "".to_string(),
                method: "eth_getBalance".to_string(),
                parameters: vec![":userAddress".to_string(), "latest".to_string()],
                return_value_test: JsonReturnValueTest {
                    comparator: ">=".to_string(),
                    value: "0".to_string(),
                },
            }),
        )]);

    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: None,
        evm_contract_conditions: None,
        sol_rpc_conditions: None,
        unified_access_control_conditions: unified_access_control_conditions.clone(),
    })
    .unwrap();
    let identity_param = AccessControlConditionResource::new(format!(
        "{}/{}",
        hashed_access_control_conditions, data_to_encrypt_hash
    ))
    .get_resource_key()
    .into_bytes();

    TestEncryptionParameters {
        to_encrypt: to_encrypt.into(),
        data_to_encrypt_hash,
        access_control_conditions: None,
        evm_contract_conditions: None,
        sol_rpc_conditions: None,
        unified_access_control_conditions,
        chain,
        hashed_access_control_conditions,
        identity_param,
    }
}

pub fn prepare_test_encryption_parameters_with_wallet_address(
    wallet_address: String,
) -> TestEncryptionParameters {
    let to_encrypt = "hello this is a test to decrypt with the provided wallet address";

    // sha256 the plaintext
    let mut hasher = Sha256::new();
    hasher.update(to_encrypt.as_bytes());
    let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

    let chain = Some(CHAIN_LOCALCHAIN.to_string());

    debug!(
        "Allow only the provided wallet_addres to decrypt- {:?}",
        wallet_address
    );

    let access_control_conditions = Some(vec![AccessControlConditionItem::Condition(
        JsonAccessControlCondition {
            contract_address: "".to_string(),
            chain: CHAIN_LOCALCHAIN.to_string(),
            standard_contract_type: "".to_string(),
            method: "".to_string(),
            parameters: vec![":userAddress".to_string()],
            return_value_test: JsonReturnValueTest {
                comparator: "=".to_string(),
                value: wallet_address,
            },
        },
    )]);

    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: access_control_conditions.clone(),
        evm_contract_conditions: None,
        sol_rpc_conditions: None,
        unified_access_control_conditions: None,
    })
    .unwrap();
    let identity_param = AccessControlConditionResource::new(format!(
        "{}/{}",
        hashed_access_control_conditions, data_to_encrypt_hash
    ))
    .get_resource_key()
    .into_bytes();

    TestEncryptionParameters {
        to_encrypt: to_encrypt.into(),
        data_to_encrypt_hash,
        access_control_conditions,
        evm_contract_conditions: None,
        sol_rpc_conditions: None,
        unified_access_control_conditions: None,
        chain,
        hashed_access_control_conditions,
        identity_param,
    }
}

pub async fn test_encryption_decryption_auth_sig(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    epoch: u64,
) {
    // prepare
    let test_encryption_parameters = prepare_test_encryption_parameters();

    // Get auth sig for auth
    let wallet = LocalWallet::new(&mut OsRng);
    let auth_sig = generate_authsig(&wallet)
        .await
        .expect("Couldn't generate auth sig");

    // Encrypt.
    let message_bytes = test_encryption_parameters.to_encrypt.as_bytes();

    let network_pubkey = get_network_pubkey_from_node_set(node_set.iter().map(|(n, _)| n)).await;
    let pubkey =
        lit_sdk::lit_node_core::blsful::PublicKey::try_from(hex::decode(network_pubkey).unwrap())
            .unwrap();

    let ciphertext = lit_sdk::encryption::encrypt_time_lock(
        &pubkey,
        message_bytes,
        &test_encryption_parameters.identity_param,
    )
    .expect("Unable to encrypt");
    info!("ciphertext: {:?}", ciphertext);

    // Retrieve decrypted key
    let decryption_resp = retrieve_decryption_key(
        node_set,
        test_encryption_parameters.clone(),
        &auth_sig,
        epoch,
    )
    .await;

    // Assert decryption
    assert_decrypted(
        &pubkey,
        test_encryption_parameters.identity_param.clone(),
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp,
    );
}

pub async fn test_encryption_decryption_session_sigs(
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
) {
    let epoch = validator_collection
        .actions()
        .get_current_epoch(U256::from(1))
        .await;

    let signer = end_user.signing_provider().clone();
    // prepare
    let test_encryption_parameters = prepare_test_encryption_parameters();

    // Get the resource key
    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: test_encryption_parameters.access_control_conditions.clone(),
        evm_contract_conditions: test_encryption_parameters.evm_contract_conditions.clone(),
        sol_rpc_conditions: test_encryption_parameters.sol_rpc_conditions.clone(),
        unified_access_control_conditions: test_encryption_parameters
            .unified_access_control_conditions
            .clone(),
    })
    .unwrap();

    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    // Get session sig for auth
    let session_sigs = get_session_sigs_for_auth(
        &node_set,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: format!(
                    "{}/{}",
                    hashed_access_control_conditions,
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

    // Encrypt.
    let network_pubkey = get_network_pubkey(validator_collection.actions()).await;
    let message_bytes = test_encryption_parameters.to_encrypt.as_bytes();
    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: test_encryption_parameters.access_control_conditions.clone(),
        evm_contract_conditions: test_encryption_parameters.evm_contract_conditions.clone(),
        sol_rpc_conditions: test_encryption_parameters.sol_rpc_conditions.clone(),
        unified_access_control_conditions: test_encryption_parameters
            .unified_access_control_conditions
            .clone(),
    })
    .unwrap();
    let identity_param = AccessControlConditionResource::new(format!(
        "{}/{}",
        hashed_access_control_conditions, test_encryption_parameters.data_to_encrypt_hash
    ))
    .get_resource_key()
    .into_bytes();

    let pubkey = blsful::PublicKey::try_from(hex::decode(&network_pubkey).unwrap()).unwrap();
    let ciphertext =
        lit_sdk::encryption::encrypt_time_lock(&pubkey, message_bytes, &identity_param)
            .expect("Unable to encrypt");
    debug!(
        "encrypting with pubkey {} -> ciphertext: {:?}",
        network_pubkey, ciphertext
    );

    // Retrieve decrypted key
    let decryption_resp = retrieve_decryption_key_session_sigs_with_version(
        test_encryption_parameters.clone(),
        &session_sigs,
        epoch.as_u64(),
    )
    .await;

    assert_decrypted(
        &pubkey,
        identity_param,
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp,
    );

    info!("Decryption checks passed");
}

pub async fn retrieve_decryption_key(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    test_encryption_parameters: TestEncryptionParameters,
    auth_sig: &JsonAuthSig,
    epoch: u64,
) -> Vec<GenericResponse<EncryptionSignResponse>> {
    let payload = EncryptionSignRequest {
        access_control_conditions: test_encryption_parameters.access_control_conditions.clone(),
        evm_contract_conditions: test_encryption_parameters.evm_contract_conditions.clone(),
        sol_rpc_conditions: test_encryption_parameters.sol_rpc_conditions.clone(),
        unified_access_control_conditions: test_encryption_parameters
            .unified_access_control_conditions
            .clone(),
        chain: test_encryption_parameters.chain.clone(),
        data_to_encrypt_hash: test_encryption_parameters.data_to_encrypt_hash.clone(),
        auth_sig: AuthSigItem::Single(auth_sig.to_owned()),
        epoch,
    };
    info!("Sending payload {:?}", payload);
    let my_secret_key = rand::rngs::OsRng.r#gen();
    let response = lit_sdk::EncryptionSignRequest::new()
        .url_prefix(lit_sdk::UrlPrefix::Http)
        .node_set(
            node_set
                .iter()
                .map(|(node_set, node)| lit_sdk::EndpointRequest {
                    identity_key: *node,
                    node_set: node_set.clone(),
                    body: payload.clone(),
                })
                .collect(),
        )
        .build()
        .unwrap()
        .send(&my_secret_key)
        .await
        .unwrap();

    info!("get_encryption_key_resp: {:?}", response.results());

    response.results().to_owned()
}

pub async fn retrieve_decryption_key_session_sigs(
    test_encryption_parameters: TestEncryptionParameters,
    session_sigs_and_node_set: &Vec<SessionSigAndNodeSet>,
    epoch: u64,
) -> Vec<GenericResponse<EncryptionSignResponse>> {
    retrieve_decryption_key_session_sigs_with_version(
        test_encryption_parameters,
        session_sigs_and_node_set,
        epoch,
    )
    .await
}

pub async fn retrieve_decryption_key_session_sigs_with_version(
    test_encryption_parameters: TestEncryptionParameters,
    session_sigs_and_node_set: &Vec<SessionSigAndNodeSet>,
    epoch: u64,
) -> Vec<GenericResponse<EncryptionSignResponse>> {
    let mut endpoint_requests = Vec::new();

    // Generate JSON body for each port
    for session_sig_and_nodeset in session_sigs_and_node_set {
        let encryption_sign_request = EncryptionSignRequest {
            access_control_conditions: test_encryption_parameters.access_control_conditions.clone(),
            evm_contract_conditions: test_encryption_parameters.evm_contract_conditions.clone(),
            sol_rpc_conditions: test_encryption_parameters.sol_rpc_conditions.clone(),
            unified_access_control_conditions: test_encryption_parameters
                .unified_access_control_conditions
                .clone(),
            chain: test_encryption_parameters.chain.clone(),
            data_to_encrypt_hash: test_encryption_parameters.data_to_encrypt_hash.clone(),
            auth_sig: AuthSigItem::Single(session_sig_and_nodeset.session_sig.clone()),
            epoch,
        };

        endpoint_requests.push(lit_sdk::EndpointRequest {
            node_set: session_sig_and_nodeset.node.clone(),
            identity_key: session_sig_and_nodeset.identity_key,
            body: encryption_sign_request,
        });
    }

    let my_secret_key = rand::rngs::OsRng.r#gen();
    let response = lit_sdk::EncryptionSignRequest::new()
        .url_prefix(lit_sdk::UrlPrefix::Http)
        .node_set(endpoint_requests)
        .build()
        .unwrap()
        .send(&my_secret_key)
        .await
        .unwrap();

    debug!("get_encryption_key_resp: {:?}", response.results());

    response.results().to_owned()
}

pub fn assert_decrypted(
    network_pubkey: &blsful::PublicKey<Bls12381G2Impl>,
    identity_param: Vec<u8>,
    expected_plaintext: &str,
    ciphertext: &blsful::TimeCryptCiphertext<Bls12381G2Impl>,
    decryption_resp: Vec<GenericResponse<EncryptionSignResponse>>,
) {
    // assert_eq!(decryption_resp.len(), num_staked as usize);

    // Use decryption shares to decrypt ciphertext and check that it matches the original
    let serialized_decryption_shares = decryption_resp
        .into_iter()
        .map(|resp| {
            assert!(resp.ok);
            let parsed_resp = resp.data.unwrap();
            parsed_resp.signature_share
        })
        .collect::<Vec<_>>();
    let decrypted = lit_sdk::encryption::verify_and_decrypt_with_signatures_shares(
        network_pubkey,
        &identity_param,
        ciphertext,
        &serialized_decryption_shares,
    )
    .expect("Unable to decrypt");
    assert_eq!(
        decrypted,
        *expected_plaintext.as_bytes(),
        "Decrypted does not match expected plaintext"
    );
}

pub async fn test_lit_action_session_sigs(
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
) {
    let execute_resp = generate_session_sigs_execute_lit_action(
        validator_collection,
        HELLO_WORLD_LIT_ACTION_CODE,
        end_user,
    )
    .await
    .expect("Could not execute lit action");

    let action_result = assert_signed_action(validator_collection, execute_resp).await;

    assert!(action_result.is_ok());
    let action_result = action_result.unwrap();
    assert!(action_result, "The action should have returned true");
}

pub async fn generate_session_sigs_execute_lit_action(
    validator_collection: &ValidatorCollection,
    lit_action_code: &str,

    end_user: &EndUser,
) -> Result<Vec<GenericResponse<JsonExecutionResponse>>> {
    let (pubkey, _token_id, pkp_eth_address, _key_set_id) = end_user.first_pkp().info();
    let wallet = end_user.wallet.clone();
    // add the PKP itself as a permitted address, so that our session sig from the PKP will be able to sign with it
    end_user
        .first_pkp()
        .add_permitted_address_to_pkp(pkp_eth_address, &[U256::from(1)])
        .await
        .expect("Could not add permitted address to pkp");

    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    // Get session sig for auth
    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey.clone(),
        pkp_eth_address,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: "lit-litaction".to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        wallet.clone(),
        None,
        None,
        None,
        2,
        Some(U256::MAX), // max_price
    )
    .await
    .expect("Could not get session sigs");

    // run
    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        lit_action_params(lit_action_code.to_string(), pubkey)
            .await
            .expect("Could not get lit action params");

    execute_lit_action_session_sigs(
        Some(lit_action_code),
        ipfs_id,
        js_params,
        auth_methods,
        &session_sigs_and_node_set,
        2,
    )
    .await
}
