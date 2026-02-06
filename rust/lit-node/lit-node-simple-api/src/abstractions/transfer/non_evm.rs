use std::sync::Arc;

use super::chain_info::Chain;
use super::models::{GetBalanceResponse, TransferRequest, TransferResponse};
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;
use rocket::serde::json::Json;
use rocket::http::Status;


pub async fn get_balance(testnet: &Testnet, api_key: &str, chain: Chain) -> Result<Json<GetBalanceResponse>, Status> {
    let _ = (testnet, api_key, chain);

    Ok(Json(GetBalanceResponse {
        address: String::new(),
        balance: String::new(),
        chain: Chain::Ethereum,
        symbol: String::new(),
    }))
}

pub async fn send(testnet: &Arc<Testnet>, validator_collection: &Arc<ValidatorCollection>, request: &Json<TransferRequest>, chain: Chain) -> Result<Json<TransferResponse>, Status> {


    let _ = request;
    Ok(Json(TransferResponse {
        txn_id: String::new(),
        success: false,
        chain: Chain::Ethereum,
        origin_symbol: String::new(),
        origin_amount: String::new(),
        gas: String::new(),
        timestamp: String::new(),
        destination_address: String::new(),
    }))
}