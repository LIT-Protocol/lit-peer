//! Rocket endpoints for the swap intents API (v1).

use super::models::{
    AcceptQuoteRequest, AcceptQuoteResponse, GetQuoteRequest, GetQuoteResponse,
    GetSwapStatusResponse, SwapState, TokenListResponse,
};
use rocket::serde::json::Json;
use rocket::{Route, get, http::Status, post, routes};

pub fn routes() -> Vec<Route> {
    routes![token_list, get_quote, accept_quote, get_swap_status]
}

/// GET /token_list — returns a list of supported tokens.
#[get("/token_list")]
async fn token_list() -> Result<Json<TokenListResponse>, Status> {
    Ok(Json(TokenListResponse { tokens: vec![] }))
}

/// POST /get_quote — get a quote for a swap.
#[post("/get_quote", format = "json", data = "<request>")]
async fn get_quote(request: Json<GetQuoteRequest>) -> Result<Json<GetQuoteResponse>, Status> {
    let _ = request;
    Ok(Json(GetQuoteResponse {
        quote_id: String::new(),
        quote_expiry: String::new(),
        fees: vec![],
        signed_input_hash: String::new(),
    }))
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
