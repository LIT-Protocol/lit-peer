use crate::core::internal;
use crate::core::v1::models::request::{
    CombineSignatureSharesRequest, DecryptRequest, EncryptRequest, LitActionRequest,
    SignWithPKPRequest,
};
use crate::core::v1::models::response::{
    CombineSignatureSharesResponse, DecryptResponse, EncryptResponse, GetApiKeyResponse,
    HandshakeResponse, LitActionResponses, MintPkpResponse, SignWithPkpResponse,
};
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;
use ethers::types::U256;
use rocket::serde::json::Json;
use rocket::{Route, State, get, http::Status, post, routes};
use std::sync::Arc;

pub fn routes() -> Vec<Route> {
    routes![
        handshake,
        sign_with_pkp,
        get_api_key,
        mint_pkp,
        encrypt,
        decrypt,
        combine_signature_shares,
        lit_action,
        get_ledger_balance
    ]
}

#[get("/get_api_key")]
async fn get_api_key(testnet: &State<Arc<Testnet>>) -> Result<Json<GetApiKeyResponse>, Status> {
    internal::get_api_key(&testnet.inner()).await
}

#[get("/handshake")]
async fn handshake(
    testnet: &State<Arc<Testnet>>,
    validator_collection: &State<Arc<ValidatorCollection>>,
) -> Result<Json<HandshakeResponse>, Status> {
    internal::handshake(&testnet.inner(), &validator_collection.inner()).await
}

#[get("/mint_pkp/<api_key>")]
async fn mint_pkp(
    testnet: &State<Arc<Testnet>>,
    api_key: &str,
) -> Result<Json<MintPkpResponse>, Status> {
    internal::mint_pkp(&testnet.inner(), api_key).await
}

#[post("/sign_with_pkp", format = "json", data = "<sign_request>")]
async fn sign_with_pkp(
    testnet: &State<Arc<Testnet>>,
    validator_collection: &State<Arc<ValidatorCollection>>,
    sign_request: Json<SignWithPKPRequest>,
) -> Result<Json<SignWithPkpResponse>, Status> {
    internal::sign_with_pkp(
        &testnet.inner(),
        &validator_collection.inner(),
        sign_request,
    )
    .await
}

#[post("/lit_action", format = "json", data = "<lit_action_request>")]
async fn lit_action(
    testnet: &State<Arc<Testnet>>,
    validator_collection: &State<Arc<ValidatorCollection>>,
    lit_action_request: Json<LitActionRequest>,
) -> Result<Json<LitActionResponses>, Status> {
    internal::lit_action(
        &testnet.inner(),
        &validator_collection.inner(),
        lit_action_request,
    )
    .await
}

#[post("/encrypt", format = "json", data = "<encrypt_request>")]
async fn encrypt(
    testnet: &State<Arc<Testnet>>,
    validator_collection: &State<Arc<ValidatorCollection>>,
    encrypt_request: Json<EncryptRequest>,
) -> Result<Json<EncryptResponse>, Status> {
    internal::encrypt(
        &testnet.inner(),
        &validator_collection.inner(),
        encrypt_request,
    )
    .await
}

#[post("/decrypt", format = "json", data = "<decrypt_request>")]
async fn decrypt(
    testnet: &State<Arc<Testnet>>,
    validator_collection: &State<Arc<ValidatorCollection>>,
    decrypt_request: Json<DecryptRequest>,
) -> Result<Json<DecryptResponse>, Status> {
    internal::decrypt(
        &testnet.inner(),
        &validator_collection.inner(),
        decrypt_request,
    )
    .await
}

#[post(
    "/combine_signature_shares",
    format = "json",
    data = "<combine_signature_shares_request>"
)]
async fn combine_signature_shares(
    combine_signature_shares_request: Json<CombineSignatureSharesRequest>,
) -> Result<Json<CombineSignatureSharesResponse>, Status> {
    internal::combine_signature_shares(combine_signature_shares_request).await
}

#[get("/get_ledger_balance/<api_key>")]
async fn get_ledger_balance(
    testnet: &State<Arc<Testnet>>,
    api_key: &str,
) -> Result<Json<String>, Status> {
    internal::get_ledger_balance(testnet, api_key).await
}