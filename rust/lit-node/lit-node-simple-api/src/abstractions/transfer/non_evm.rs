use std::sync::Arc;

use super::chain_info::Chain;
use super::models::{GetBalanceResponse, TransferRequest, TransferResponse};
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;
use rocket::http::Status;
use rocket::serde::json::Json;

pub async fn get_api_key_balance(
    _testnet: &Testnet,
    _api_key: &str,
    chain: Chain,
) -> Result<Json<GetBalanceResponse>, Status> {
    let _ = (_testnet, _api_key, chain);

    Ok(Json(GetBalanceResponse {
        address: String::new(),
        balance: 0.0,
        chain: Chain::Ethereum,
        symbol: String::new(),
    }))
}

pub async fn get_pkp_balance(
    pkp_public_key: &str,
    chain: Chain,
) -> Result<Json<GetBalanceResponse>, Status> {
    Ok(Json(GetBalanceResponse {
        address: pkp_public_key.to_string(),
        balance: 0.0,
        chain: chain.clone(),
        symbol: String::new(),
    }))
}

pub async fn get_address_balance(
    address: &str,
    chain: Chain,
) -> Result<Json<GetBalanceResponse>, Status> {
    Ok(Json(GetBalanceResponse {
        address: address.to_string(),
        balance: 0.0,
        chain: chain.clone(),
        symbol: String::new(),
    }))
}

pub async fn send(
    _testnet: &Arc<Testnet>,
    _validator_collection: &Arc<ValidatorCollection>,
    request: &Json<TransferRequest>,
    _chain: Chain,
) -> Result<Json<TransferResponse>, Status> {
    let _ = request;
    Ok(Json(TransferResponse {
        txn_id: String::new(),
        success: false,
        chain: Chain::Ethereum,
        origin_symbol: String::new(),
        origin_amount: 0.0,
        gas: String::new(),
        timestamp: String::new(),
        destination_address: String::new(),
    }))
}
