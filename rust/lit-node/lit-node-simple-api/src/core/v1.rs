
use ethers::signers::{LocalWallet, Signer};
use ethers::types::U256;
use ethers::utils::keccak256;
use lit_core::utils::binary::{bytes_to_hex, hex_to_bytes};
use lit_node_core::{AccessControlConditionItem, AccessControlConditionResource, JsonAccessControlCondition, JsonReturnValueTest, LitResource, SigningScheme};
use lit_node_testnet::common::lit_actions::generate_session_sigs_and_execute_lit_action;
use lit_node_testnet::common::pkp::{generate_session_sigs_and_send_signing_requests, recombine_shares_using_wasm};
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::node_collection::{get_identity_pubkeys_from_node_set, get_network_pubkey, handshake_nodes};
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;
use lit_rust_crypto::k256::sha2::{Sha256, Digest};
use rocket::serde::json::Json;
use rocket::{Route, State, http::Status, get, post, routes};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use base64_light::base64_decode;
use crate::core::copied::{RequestConditions, hash_access_control_conditions};

pub fn routes() -> Vec<Route> {
    routes![
        handshake, sign_with_pkp, get_api_key, mint_pkp,
        encrypt, decrypt,
        combine_signature_shares,
        lit_action
    ]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignWithPKPRequest {
    pub api_key: String,
    pub pkp_public_key: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LitActionRequest {
    pub api_key: String,
    pub code: String,
    pub js_params: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptRequest {
    pub api_key: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptRequest {
    pub api_key: String,
    pub shares: Vec<String>,
    pub ciphertext: String,
    pub data_to_encrypt_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombineSignatureSharesRequest {
    pub api_key: String,    
    pub shares: Vec<String>,
}

#[get("/get_api_key")]
async fn get_api_key(testnet: &State<Arc<Testnet>>) -> Result<serde_json::Value, Status> {
    let wallet = LocalWallet::new(&mut rand::thread_rng());
    info!("New wallet address: {:?}", wallet.address());
    let secret_key = wallet.signer().to_bytes().to_vec();

    let testnet = testnet.inner();
    let end_user = EndUser::from_secret_key(testnet, &secret_key);

    end_user.fund_wallet_default_amount().await;
    Ok(serde_json::json!({
        "api_key": format!("{}", hex::encode(secret_key)),
        "wallet_address": format!("0x{}", bytes_to_hex(wallet.address().as_bytes()))
    }))
}


#[get("/handshake")]
async fn handshake(
    testnet: &State<Arc<Testnet>>,
    validator_collection: &State<Arc<ValidatorCollection>>,
) -> Result<serde_json::Value, Status> {
    let testnet = testnet.inner();
    let validator_collection = validator_collection.inner();
    let responses = handshake_nodes(
        &validator_collection.actions(),
        U256::from(testnet.realm_id()),
    )
    .await;
    Ok(serde_json::json!({
        "responses": responses
    }))
}

#[get("/mint_pkp/<api_key>")]
async fn mint_pkp(
    testnet: &State<Arc<Testnet>>,
    api_key: &str,
) -> Result<serde_json::Value, Status> {

    let testnet = testnet.inner();
    let mut end_user = EndUser::from_secret_key(testnet, &hex_to_bytes(&api_key).unwrap());

    let key_set_id = testnet.actions().get_all_keyset_configs().await.unwrap()[0].identifier.clone();
    let pkp_info = end_user.new_pkp(&key_set_id).await.unwrap();

    let pkp = end_user.pkp_by_pubkey(pkp_info.0.clone());
    let pkp_public_key = pkp.pubkey.clone();
    let deposit_amount = U256::from(1000000000000000000u128);
    end_user.deposit_to_wallet_ledger(deposit_amount).await;
    // end_user.deposit_to_pkp_ledger(&pkp, deposit_amount).await;
    Ok(serde_json::json!({
        "pkp_public_key": pkp_public_key
    }))
}

#[post("/sign_with_pkp", format = "json", data = "<sign_request>")]
async fn sign_with_pkp(
    testnet: &State<Arc<Testnet>>,
    validator_collection: &State<Arc<ValidatorCollection>>,
    sign_request: Json<SignWithPKPRequest>,
) -> Result<serde_json::Value, Status> {
    let testnet = testnet.inner();
    let validator_collection = validator_collection.inner();
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set_with_keys = get_identity_pubkeys_from_node_set(&node_set).await;

    let to_sign = sign_request.message.as_bytes().to_vec();
    let to_sign = keccak256(to_sign.as_slice()).to_vec();
    let pubkey = sign_request.pkp_public_key.clone();
    let epoch = validator_collection
        .actions()
        .get_current_epoch(U256::from(testnet.realm_id()))
        .await
        .as_u64();
    let signing_scheme = SigningScheme::EcdsaK256Sha256;
    let key_set_id = validator_collection
        .actions()
        .get_keyset_id_for_pkp(&pubkey)
        .await
        .unwrap();

    let end_user = EndUser::from_secret_key(testnet, &hex_to_bytes(&sign_request.api_key).unwrap());
    let wallet = end_user.wallet.clone();
    
    info!("Signing with PKP: {:?}", pubkey);
    info!("Wallet address: {:?}", wallet.address());
    info!("Key set id: {:?}", key_set_id);

    let endpoint_responses = generate_session_sigs_and_send_signing_requests(
        &node_set_with_keys,
        wallet,
        to_sign.clone(),
        pubkey.clone(),
        epoch,
        signing_scheme,
        &key_set_id,
    )
    .await;

    let endpoint_responses = endpoint_responses.iter().map(|response| {
        match response.data.clone().is_some() {
            true => {
                let signature_share = match response.data.is_some() {
                    true => {
                        let signature_share = response.data.clone().unwrap().signature_share.clone();
                        let signature_share = signature_share.ecdsa_signed_message_share().unwrap().signature_share.clone();
                        signature_share
                    }
                    false => {
                        "".to_string()
                    }
                };
                signature_share
            },
            false => {
                "".to_string()
            }
        }}).collect::<Vec<_>>();
    info!("endpoint_responses: {:?}", endpoint_responses);

    Ok(serde_json::json!({
        "endpoint_responses": endpoint_responses
    }))
}

#[post("/lit_action", format = "json", data = "<lit_action_request>")]
async fn lit_action(
    testnet: &State<Arc<Testnet>>,
    validator_collection: &State<Arc<ValidatorCollection>>,
    lit_action_request: Json<LitActionRequest>,
) -> Result<serde_json::Value, Status> {
    let testnet = testnet.inner();
    let validator_collection = validator_collection.inner();

    let lit_action_code = lit_action_request.code.clone();
    let lit_action_code = data_encoding::BASE64.encode(lit_action_code.as_bytes());
    let js_params = lit_action_request.js_params.clone();   

    let end_user = EndUser::from_secret_key(testnet, &hex_to_bytes(&lit_action_request.api_key).unwrap());

    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set_with_keys = get_identity_pubkeys_from_node_set(&node_set).await;
    let epoch = validator_collection
        .actions()
        .get_current_epoch(U256::from(testnet.realm_id()))
        .await
        .as_u64();
    let key_set_id = validator_collection
        .actions()
        .get_all_keyset_configs().await.unwrap()[0].identifier.clone();
        
        let execute_resp = generate_session_sigs_and_execute_lit_action(
            &node_set_with_keys,
            end_user.wallet.clone(),
            Some(lit_action_code),
            None,
            js_params,
            None,
            epoch,
            key_set_id,
        )
        .await;

    if execute_resp.is_err() {
        return Err(Status::InternalServerError);
    }
    let execute_resp = execute_resp.unwrap();
    Ok(serde_json::json!({
        "execute_resp": execute_resp
    }))
}

#[post("/encrypt", format = "json", data = "<encrypt_request>")]
async fn encrypt(
    testnet: &State<Arc<Testnet>>,
    validator_collection: &State<Arc<ValidatorCollection>>,
    encrypt_request: Json<EncryptRequest>,
) -> Result<serde_json::Value, Status> {
    let testnet = testnet.inner();
    let validator_collection = validator_collection.inner();

    // Encrypt
    let to_encrypt = encrypt_request.message.clone();
    let mut hasher = Sha256::new();
    hasher.update(to_encrypt.as_bytes());
    let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

    // Identity param
    let end_user = EndUser::from_secret_key(testnet, &hex_to_bytes(&encrypt_request.api_key).unwrap());
    let wallet = end_user.wallet.clone();
    let wallet_address = wallet.address();
    let hex_wallet_address = bytes_to_hex(wallet_address.as_bytes());
    let chain = &testnet.chain_name;

    let identity_param = get_identity_param(hex_wallet_address, chain, &data_to_encrypt_hash);


    let network_pubkey = get_network_pubkey(validator_collection.actions()).await;
    let message_bytes = to_encrypt.as_bytes();
    let pubkey =
        lit_rust_crypto::blsful::PublicKey::try_from(&hex::decode(&network_pubkey).unwrap())
            .unwrap();    
    let ciphertext =
        lit_sdk::encryption::encrypt_time_lock(&pubkey, message_bytes, &identity_param)
            .expect("Unable to encrypt");
    info!("ciphertext: {:?}", ciphertext);

    Ok(serde_json::json!({
        "ciphertext": ciphertext
    }))

}

#[post("/decrypt", format = "json", data = "<decrypt_request>")]
async fn decrypt(
    testnet: &State<Arc<Testnet>>,
    validator_collection: &State<Arc<ValidatorCollection>>,
    decrypt_request: Json<DecryptRequest>,
) -> Result<serde_json::Value, Status> {
    let testnet = testnet.inner();
    let validator_collection = validator_collection.inner();
    let ciphertext = decrypt_request.ciphertext.clone();
    let network_pubkey = get_network_pubkey(validator_collection.actions()).await;
    let network_pubkey = hex::decode(&network_pubkey).unwrap();
    let network_pubkey = lit_rust_crypto::blsful::PublicKey::<lit_rust_crypto::blsful::Bls12381G2Impl>::try_from(&network_pubkey).unwrap();
    let shares = decrypt_request.shares.clone();
    let shares: Vec<Vec<u8>> = shares.iter().map(|share| hex_to_bytes(share).unwrap()).collect();
    let serialized_decryption_shares = shares.iter().map(|share| lit_rust_crypto::blsful::SignatureShare::try_from(share).unwrap()).collect::<Vec<_>>();
    let data_to_encrypt_hash = decrypt_request.data_to_encrypt_hash.clone();

// Identity param
    let end_user = EndUser::from_secret_key(testnet, &hex_to_bytes(&decrypt_request.api_key).unwrap());
    let wallet = end_user.wallet.clone();
    let wallet_address = wallet.address();
    let hex_wallet_address = bytes_to_hex(wallet_address.as_bytes());
    let chain = &testnet.chain_name;
    let identity_param = get_identity_param(hex_wallet_address, chain, &data_to_encrypt_hash);

    let ciphertext = serde_bare::from_slice(&base64_decode(&ciphertext)).unwrap();
    
     let decrypted = lit_sdk::encryption::verify_and_decrypt_with_signatures_shares(
        &network_pubkey,
        &identity_param,
        &ciphertext,
        &serialized_decryption_shares,
    ).unwrap();

       let result = match std::str::from_utf8(&decrypted) {
                    Ok(result) => result.to_string(),
                    Err(e) => {
                        return Err(Status::InternalServerError);
                    }
                };


    Ok(serde_json::json!({
        "derypted-text": result
    }))
}   

fn get_identity_param(hex_wallet_address: String, chain: &str, data_to_encrypt_hash: &str) -> Vec<u8> {
    
    let access_control_conditions = Some(vec![AccessControlConditionItem::Condition(
        JsonAccessControlCondition {
            contract_address: "".to_string(),
            chain: chain.to_string(),
            standard_contract_type: "".to_string(),
            method: "".to_string(),
            parameters: vec![":userAddress".to_string()],
            return_value_test: JsonReturnValueTest {
                comparator: "=".to_string(),
                value: hex_wallet_address,
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
    let identity_param = AccessControlConditionResource::new(format!(
        "{hashed_access_control_conditions}/{data_to_encrypt_hash}"
    ))
    .get_resource_key()
    .into_bytes();

    identity_param
    }


#[post("/combine_signature_shares", format = "json", data = "<combine_signature_shares_request>")]
async fn combine_signature_shares(
    combine_signature_shares_request: Json<CombineSignatureSharesRequest>,
) -> Result<serde_json::Value, Status> {

    let shares = combine_signature_shares_request.shares.clone();
    let (signature, recovery_id) = recombine_shares_using_wasm(shares).unwrap();
    let hex_signature = hex::encode(signature.to_bytes());
    
    Ok(serde_json::json!({
        "signature": hex_signature,
        "recovery_id": recovery_id.to_byte()
    }))
}