//! Rocket endpoints for the swap intents API (v1).

use std::sync::Arc;

use super::models::{
    AcceptQuoteRequest, AcceptQuoteResponse, GetOpenQuotesResponse, GetOpenSwapRequestsResponse,
    GetSwapStatusResponse, NewSwapRequest, NewSwapResponse, QuoteData, SwapRequestData, SwapState,
    TokenListResponse,
};
use crate::abstractions::intents::swaps::contracts::quote_storage::{QuoteStorage, SwapRequest};
use crate::abstractions::intents::swaps::models::{
    FillQuoteRequest, FillQuoteResponse, QuotePricingType,
};
use crate::abstractions::transfer::chain_info::Chain;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::types::{H160, U256};
use lit_core::utils::binary::{bytes_to_hex, hex_to_bytes};
use rocket::serde::json::Json;
use rocket::{Route, get, http::Status, post, routes};
use tracing::error;

pub fn routes() -> Vec<Route> {
    routes![
        token_list,
        new_swap_request,
        fill_quote_request,
        accept_quote,
        get_swap_status,
        get_open_swap_requests,
        get_open_quotes
    ]
}

/// GET /token_list — returns a list of supported tokens.
#[get("/token_list")]
async fn token_list() -> Result<Json<TokenListResponse>, Status> {
    Ok(Json(TokenListResponse { tokens: vec![] }))
}

/// POST /get_quote — get a quote for a swap.
#[post("/new_quote_request", format = "json", data = "<request>")]
async fn new_swap_request(request: Json<NewSwapRequest>) -> Result<Json<NewSwapResponse>, Status> {
    if let Err(e) = Chain::try_from_str(request.origin_chain.as_str()) {
        error!("Invalid origin chain: {:?}", e);
        return Err(Status::BadRequest);
    }
    if let Err(e) = Chain::try_from_str(request.destination_chain.as_str()) {
        error!("Invalid destination chain: {:?}", e);
        return Err(Status::BadRequest);
    }

    let from = hex_to_bytes(request.from.as_str()).unwrap();
    let from = H160::from_slice(&from);

    let origin_address = hex_to_bytes(request.origin_address.as_str()).unwrap();
    let origin_address = H160::from_slice(&origin_address);

    let refund_address = hex_to_bytes(request.refund_address.as_str()).unwrap();
    let refund_address = H160::from_slice(&refund_address);

    let pricing_type = match request.pricing_type {
        QuotePricingType::Origin => 0,
        QuotePricingType::Destination => 1,
    };

    let swap_request = SwapRequest {
        from: from,
        origin_chain: request.origin_chain.clone(),
        origin_symbol: request.origin_symbol.clone(),
        origin_amount: request.origin_amount.into(),
        destination_symbol: request.destination_symbol.clone(),
        destination_chain: request.destination_chain.clone(),
        destination_amount: request.destination_amount.into(),
        slippage: request.slippage.into(),
        pricing_type: pricing_type,
        quote_deadline_seconds: request.quote_deadline_seconds.into(),
        origin_address: origin_address,
        refund_address: refund_address,
        transaction_deadline_seconds: request.transaction_deadline_seconds.into(),
        message: request.message.clone().unwrap_or_default(),
    };

    let contract: QuoteStorage<Provider<Http>> = get_signable_quote_contract().await.unwrap();
    let func = contract.new_swap_request(swap_request);
    let tx = func.send().await.unwrap();
    let swap_request_id = bytes_to_hex(&tx.0);
    let receipt = tx.await.unwrap();
    let transaction_hash = receipt.unwrap().transaction_hash;

    let duration = std::time::Duration::from_secs(request.quote_deadline_seconds.into());
    Ok(Json(NewSwapResponse {
        swap_request_id: swap_request_id.to_string(),
        transaction_hash: transaction_hash.to_string(),
        swap_request_expiry: format!(
            "{:?}",
            std::time::Instant::now().checked_add(duration).unwrap()
        ),
        fees: vec![],
        signed_input_hash: String::new(),
    }))
}

/// POST /fill_quote — fill a quote and get the transaction hash.
#[post("/fill_quote", format = "json", data = "<request>")]
async fn fill_quote_request(
    request: Json<FillQuoteRequest>,
) -> Result<Json<FillQuoteResponse>, Status> {
    let swap_request_id_str = request.swap_request_id.clone();
    let swap_request_id = request.swap_request_id.clone();
    let swap_request_id = U256::from_dec_str(swap_request_id.as_str()).unwrap();

    let provider_refund_address = hex_to_bytes(request.provider_refund_address.as_str()).unwrap();
    let provider_refund_address = H160::from_slice(&provider_refund_address);

    let contract = get_signable_quote_contract().await.unwrap();
    let func = contract.new_quote(swap_request_id, provider_refund_address);
    let tx = func.send().await.unwrap();
    let quote_id = bytes_to_hex(&tx.0);
    let receipt = tx.await.unwrap();
    let transaction_hash = receipt.unwrap().transaction_hash;

    let duration = std::time::Duration::from_secs(request.quote_deadline_seconds.into());

    let pkp_address =
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string();

    Ok(Json(FillQuoteResponse {
        quote_id: quote_id.to_string(),
        transaction_hash: transaction_hash.to_string(),
        pkp_address: pkp_address.to_string(),
        swap_request_id: swap_request_id_str,
        quote_expiry: format!(
            "{:?}",
            std::time::Instant::now().checked_add(duration).unwrap()
        ),
        fees: vec![],
        signed_input_hash: String::new(),
        total_fees: 0,
    }))
}

/// POST /accept_quote — accept a quote and get the PKP address to send funds to.
#[post("/accept_quote", format = "json", data = "<request>")]
async fn accept_quote(
    request: Json<AcceptQuoteRequest>,
) -> Result<Json<AcceptQuoteResponse>, Status> {
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

/// GET /get_open_swap_requests — returns open (all) swap requests from the contract as SwapRequestData.
#[get("/get_open_swap_requests")]
async fn get_open_swap_requests() -> Result<Json<GetOpenSwapRequestsResponse>, Status> {
    let contract = get_signable_quote_contract().await?;
    let count: U256 = contract.swap_request_counter().call().await.map_err(|e| {
        error!("swap_request_counter failed: {:?}", e);
        Status::InternalServerError
    })?;
    let count = count.as_u128();
    let mut swap_requests = Vec::with_capacity(count as usize);
    for id in 1..=count {
        let id_u256 = U256::from(id);
        let on_chain: SwapRequest = contract
            .get_swap_request(id_u256)
            .call()
            .await
            .map_err(|e| {
                error!("get_swap_request({}) failed: {:?}", id, e);
                Status::InternalServerError
            })?;
        swap_requests.push(swap_request_to_data(&on_chain));
    }
    Ok(Json(GetOpenSwapRequestsResponse { swap_requests }))
}

/// GET /get_open_quotes — returns open quotes from the contract as QuoteData.
/// Returns empty until the contract exposes a getQuote(uint256) view.
#[get("/get_open_quotes")]
async fn get_open_quotes() -> Result<Json<GetOpenQuotesResponse>, Status> {
    let _contract = get_signable_quote_contract().await?;
    // QuoteStorage has no getQuote(quoteId) view yet; quotes are in mapping(uint256 => Quote)
    // with Quote containing a mapping. Once the contract adds getQuote, populate from chain.
    Ok(Json(GetOpenQuotesResponse { quotes: vec![] }))
}

fn swap_request_to_data(sr: &SwapRequest) -> SwapRequestData {
    SwapRequestData {
        from: format!("{:?}", sr.from),
        origin_symbol: sr.origin_symbol.clone(),
        origin_chain: sr.origin_chain.clone(),
        origin_amount: sr.origin_amount.as_u128(),
        destination_symbol: sr.destination_symbol.clone(),
        destination_chain: sr.destination_chain.clone(),
        destination_amount: sr.destination_amount.as_u128(),
        slippage: sr.slippage.as_u128(),
        pricing_type: sr.pricing_type,
        quote_deadline_seconds: sr.quote_deadline_seconds.as_u128(),
        origin_address: format!("{:?}", sr.origin_address),
        refund_address: format!("{:?}", sr.refund_address),
        transaction_deadline_seconds: sr.transaction_deadline_seconds.as_u128(),
        message: sr.message.clone(),
    }
}

async fn get_signable_quote_contract() -> Result<QuoteStorage<Provider<Http>>, Status> {
    let chain = Chain::Yellowstone;
    let provider = Provider::<Http>::try_from(chain.info().rpc_url).unwrap();
    let client = Arc::new(provider);
    let quote_storage_address = H160::random();
    let contract = QuoteStorage::new(quote_storage_address, client);
    Ok(contract)
}
