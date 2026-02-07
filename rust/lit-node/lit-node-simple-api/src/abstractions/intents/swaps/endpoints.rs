//! Rocket endpoints for the swap intents API (v1).

use std::sync::Arc;

use super::models::{
    AcceptQuoteRequest, AcceptQuoteResponse, GetQuoteRequest, GetQuoteResponse,
    GetSwapStatusResponse, SwapState, TokenListResponse,
};
use crate::abstractions::intents::swaps::contracts::quote_storage::{QuoteRequest, QuoteStorage };
use ethers::providers::Provider;
use ethers::providers::Http;
use ethers::types::H160;
use rocket::serde::json::Json;
use rocket::{Route, get, http::Status, post, routes};
use crate::abstractions::transfer::chain_info::Chain;

pub fn routes() -> Vec<Route> {
    routes![token_list, new_quote_request, accept_quote, get_swap_status]
}

/// GET /token_list — returns a list of supported tokens.
#[get("/token_list")]
async fn token_list() -> Result<Json<TokenListResponse>, Status> {
    Ok(Json(TokenListResponse { tokens: vec![] }))
}

/// POST /get_quote — get a quote for a swap.
#[post("/new_quote_request", format = "json", data = "<request>")]
async fn new_quote_request(request: Json<GetQuoteRequest>) -> Result<Json<GetQuoteResponse>, Status> {        

    let quote_request = QuoteRequest {
        from: request.from,
        origin_chain: request.origin_chain,        
        origin_symbol: request.origin_symbol,
        origin_amount: request.origin_amount,
        destination_symbol: request.destination_symbol,
        destination_chain: request.destination_chain,
        destination_amount: request.destination_amount,
        slippage: request.slippage,
        pricing_type: request.pricing_type,
        quote_deadline_seconds: request.quote_deadline_seconds,
        origin_address: request.origin_address,
        refund_address: request.refund_address,
        transaction_deadline_seconds: request.transaction_deadline_seconds,
        message: request.message,
    };


    
   let contract = get_signable_quote_contract().await.unwrap();
   let func = contract.new_quote_request(quote_request);
   let tx = func.send().await.unwrap();
   let receipt = tx.await.unwrap();
   let transaction_hash = receipt.unwrap().transaction_hash;
   let quote_request_id = transaction_hash.to_string();
   
    Ok(Json(GetQuoteResponse {
        quote_id: String::new(),
        quote_expiry: String::new(),
        fees: vec![],
        signed_input_hash: String::new(),
    }))
}

async fn fill_quote_request(request: GetQuoteRequest) -> Result<GetQuoteResponse, Status> {
    

    Ok(GetQuoteResponse {
        quote_id: quote_request_id.to_string(),
        quote_expiry: String::new(),
        fees: vec![],
        signed_input_hash: String::new(),
    })
}


/// POST /accept_quote — accept a quote and get the PKP address to send funds to.
#[post("/accept_quote", format = "json", data = "<request>")]
async fn accept_quote(request: Json<AcceptQuoteRequest>) -> Result<Json<AcceptQuoteResponse>, Status> {
    let _ = request;
    Ok(Json(AcceptQuoteResponse {
        pkp_address: String::new(),
    }))
}

/// GET /get_swap_status/<quote_id> — get the status of a swap by quote id.
#[get("/get_swap_status/<quote_id>")]
async fn get_swap_status(quote_id: &str) -> Result<Json<GetSwapStatusResponse>, Status> {
    let _ = quote_id;
    Ok(Json(GetSwapStatusResponse {
        state: SwapState::Pending,
        details: None,
    }))
}


async fn get_signable_quote_contract() -> Result<QuoteStorage<Provider<Http>>, Status> {

    let chain = Chain::Yellowstone;
    let provider = Provider::<Http>::try_from(chain.info().rpc_url).unwrap();
    let client = Arc::new(provider);
    let quote_storage_address = H160::random();
    let contract = QuoteStorage::new(quote_storage_address, client);
    Ok(contract)
}