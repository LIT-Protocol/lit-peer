use ethers::signers::{LocalWallet, Signer};
use ethers::types::U256;
use ethers::utils::keccak256;
use lit_core::utils::binary::hex_to_bytes;
use lit_node_core::SigningScheme;
use lit_node_testnet::common::lit_actions::generate_session_sigs_and_execute_lit_action;
use lit_node_testnet::common::pkp::{generate_session_sigs_and_send_signing_requests, recombine_shares_using_wasm};
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::node_collection::{get_identity_pubkeys_from_node_set, handshake_nodes};
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;
use rocket::serde::json::Json;
use rocket::{Route, State, http::Status, get, post, routes};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

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
        "api_key": hex::encode(secret_key)
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

    
    Ok(serde_json::json!({
        "message": "Hello, world!"
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

    let shares = decrypt_request.shares.clone();
    let shares: Vec<Vec<u8>> = shares.iter().map(|share| hex_to_bytes(share).unwrap()).collect();

    Ok(serde_json::json!({
        "message": "Hello, world!"
    }))
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