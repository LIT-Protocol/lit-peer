//! Rocket endpoints for the swap intents API (v1).

use std::sync::Arc;

use super::models::{
    AcceptQuoteRequest, AcceptQuoteResponse, GetOpenQuotesResponse, GetOpenSwapRequestsResponse,
    GetSwapStatusResponse, NewSwapRequest, NewSwapResponse, QuoteBalancesResponse, QuoteData,
    SwapRequestData, SwapState, TokenListResponse,
};
use crate::abstractions::intents::swaps::QUOTE_STORAGE_ADDRESS;
use crate::abstractions::intents::swaps::MANAGER_PRIVATE_KEY;
use crate::abstractions::intents::swaps::contracts::quote_storage::{Quote, QuoteStorage, SwapRequest};
use crate::abstractions::intents::swaps::models::{
    FillQuoteRequest, FillQuoteResponse,
};
use crate::abstractions::transfer::chain_info::Chain;
use crate::abstractions::transfer::models::TransferRequest;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Http;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{H160, I256, U256};
use ethers::utils::format_ether;
use ethers::utils::parse_ether;
use lit_core::utils::binary::{bytes_to_hex, hex_to_bytes};
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;
use rocket::State;
use rocket::serde::json::Json;
use rocket::{Route, get, http::Status, post, routes};
use tracing::error;
use tracing::info;

pub fn routes() -> Vec<Route> {
    routes![
        get_contract_address,
        token_list,
        new_swap_request,
        fill_quote_request,
        accept_quote,
        get_swap_status,
        get_open_swap_requests,
        get_open_quotes,
        get_quote_balances,
        attempt_swap_request
    ]
}

/// GET /token_list — returns a list of supported tokens.
#[get("/token_list")]
async fn token_list() -> Result<Json<TokenListResponse>, Status> {
    Ok(Json(TokenListResponse { tokens: vec![] }))
}

/// POST /get_quote — get a quote for a swap.
#[post("/new_quote_request", format = "json", data = "<request>")]
async fn new_swap_request( testnet: &State<Arc<Testnet>>, request: Json<NewSwapRequest>) -> Result<Json<NewSwapResponse>, Status> {
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

    let testnet = testnet.inner();
    let mut swap_manager = EndUser::from_secret_key(testnet, &hex_to_bytes(MANAGER_PRIVATE_KEY).unwrap());
    if swap_manager.get_wallet_ledger_balance("inquiry").await < I256::from(100000000000000_i64) {
        swap_manager.deposit_to_wallet_ledger_default().await;
    }
    let key_set_id = testnet.actions().get_all_keyset_configs().await.unwrap()[0]
        .identifier
        .clone();
    let pkp = swap_manager.new_pkp(key_set_id.as_str()).await.unwrap();


    let origin_amount = parse_ether(request.origin_amount.to_string()).unwrap();
    let destination_amount = parse_ether(request.destination_amount.to_string()).unwrap();

    let swap_request = SwapRequest {
        from: from,
        origin_chain: request.origin_chain.clone(),
        origin_symbol: request.origin_symbol.clone(),
        origin_amount: origin_amount,
        destination_symbol: request.destination_symbol.clone(),
        destination_chain: request.destination_chain.clone(),
        destination_amount: destination_amount,
        slippage: (request.slippage.trunc() as u32).into(),
        pricing_type: request.pricing_type.into(),
        quote_deadline_seconds: request.quote_deadline_seconds.into(),
        origin_address: origin_address,
        refund_address: refund_address,
        transaction_deadline_seconds: request.transaction_deadline_seconds.into(),
        pkp_address: pkp.2.clone(),
        pkp_token_id: pkp.1,
        message: request.message.clone().unwrap_or_default(),
    };

    let contract  = get_signable_quote_contract().await.unwrap();
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

/// GET /get_open_swap_requests — returns open swap requests from the contract via getRecentSwapRequests.
#[get("/get_open_swap_requests")]
async fn get_open_swap_requests() -> Result<Json<GetOpenSwapRequestsResponse>, Status> {
    let contract = get_signable_quote_contract().await?;
    let count: U256 = contract.open_swap_requests_count().call().await.map_err(|e| {
        error!("swap_request_counter failed: {:?}", e);
        Status::InternalServerError
    })?;

    if count.is_zero() {
        return Ok(Json(GetOpenSwapRequestsResponse {
            swap_requests: vec![],
        }));
    }
    let list: Vec<SwapRequest> = contract
        .get_recent_swap_requests(count)
        .call()
        .await
        .map_err(|e| {
            error!("get_recent_swap_requests failed: {:?}", e);
            Status::InternalServerError
        })?;
    let swap_requests = list.iter().map(swap_request_to_data).collect();
    Ok(Json(GetOpenSwapRequestsResponse { swap_requests }))
}

/// GET /get_open_quotes — returns open quotes from the contract via getRecentQuotes.
#[get("/get_open_quotes")]
async fn get_open_quotes() -> Result<Json<GetOpenQuotesResponse>, Status> {
    let contract = get_signable_quote_contract().await?;
    let count: U256 = contract.open_quotes_count().call().await.map_err(|e| {
        error!("quote_counter failed: {:?}", e);
        Status::InternalServerError
    })?;
    if count.is_zero() {
        return Ok(Json(GetOpenQuotesResponse { quotes: vec![] }));
    }
    let list: Vec<Quote> = contract.get_recent_quotes(count).call().await.map_err(|e| {
        error!("get_recent_quotes failed: {:?}", e);
        Status::InternalServerError
    })?;

    let swap_request_ids: Vec<U256> = list.iter().map(|q| q.swap_request_id).collect(); 
    info!("swap request ids: {:?}", swap_request_ids);
    let swap_requests: Vec<SwapRequest> = contract.get_requests_by_ids(swap_request_ids).call().await.map_err(|e| {
        error!("get_requests_by_ids failed: {:?}", e);
        Status::InternalServerError
    })?;

    info!("swap_requests: {:?}", swap_requests);

    let quotes = list.iter().zip(swap_requests.iter()).map(|(q, sr)| quote_to_data(q, sr)).collect();
    info!("quotes: {:?}", quotes);
    Ok(Json(GetOpenQuotesResponse { quotes }))
}

#[get("/get_contract_address")]
fn get_contract_address() -> Result<Json<String>, Status> {
    Ok(Json(QUOTE_STORAGE_ADDRESS.to_string()))
}

/// GET /get_quote_balances/<quote_id> — get quote data and PKP balance on source and destination chains.
#[get("/get_quote_balances/<quote_id>")]
async fn get_quote_balances(quote_id: &str) -> Result<Json<QuoteBalancesResponse>, Status> {
    let quote_id_u256 = U256::from_dec_str(quote_id).map_err(|_| {
        error!("Invalid quote_id: {}", quote_id);
        Status::BadRequest
    })?;

    let contract = get_signable_quote_contract().await?;
    let quote = contract.get_quote(quote_id_u256).call().await.map_err(|e| {
        error!("get_quote failed: {:?}", e);
        Status::InternalServerError
    })?;

    let swap_request: SwapRequest = contract.get_swap_request(quote.swap_request_id).call().await.map_err(|e| {
        error!("get_swap_request failed: {:?}", e);
        Status::InternalServerError
    })?;

    let src_chain = Chain::try_from_str(swap_request.origin_chain.as_str()).map_err(|e| {
        error!("Unsupported origin_chain: {:?}", e);
        Status::BadRequest
    })?;
    let dst_chain = Chain::try_from_str(swap_request.destination_chain.as_str()).map_err(|e| {
        error!("Unsupported destination_chain: {:?}", e);
        Status::BadRequest
    })?;

    let pkp_address = swap_request.pkp_address;
    let src_provider = Provider::<Http>::try_from(src_chain.info().rpc_url).map_err(|e| {
        error!("Failed to create source chain provider: {:?}", e);
        Status::InternalServerError
    })?;
    let dst_provider = Provider::<Http>::try_from(dst_chain.info().rpc_url).map_err(|e| {
        error!("Failed to create destination chain provider: {:?}", e);
        Status::InternalServerError
    })?;

    let src_balance = src_provider.get_balance(pkp_address, None).await.map_err(|e| {
        error!("get_balance (source) failed: {:?}", e);
        Status::InternalServerError
    })?;
    let dst_balance = dst_provider.get_balance(pkp_address, None).await.map_err(|e| {
        error!("get_balance (destination) failed: {:?}", e);
        Status::InternalServerError
    })?;

    info!("src_balance: {:?}, dst_balance: {:?}", src_balance, dst_balance);
    info!("swap_request.origin_amount: {:?}, swap_request.destination_amount: {:?}", swap_request.origin_amount, swap_request.destination_amount);

    Ok(Json(QuoteBalancesResponse {
        pkp_address: format!("{:?}", pkp_address),
        source_chain: swap_request.origin_chain.clone(),
        destination_chain: swap_request.destination_chain.clone(),
        source_balance_wei: src_balance.to_string(),
        destination_balance_wei: dst_balance.to_string(),
        source_balance_sufficient: src_balance >= U256::from(swap_request.origin_amount),
        destination_balance_sufficient: dst_balance >= U256::from(swap_request.destination_amount),
    }))
}

#[get("/attempt_swap_request/<quote_id>")]
async fn attempt_swap_request(testnet: &State<Arc<Testnet>>,validator_collection: &State<Arc<ValidatorCollection>>, quote_id: &str) -> Result<Json<String>, Status> {

    let contract = get_signable_quote_contract().await?;
    let quote = contract.get_quote(U256::from_dec_str(quote_id).unwrap()).call().await.map_err(|e| {
        error!("get_quote failed: {:?}", e);
        Status::InternalServerError
    })?;

    let swap_request: SwapRequest = contract.get_swap_request(quote.swap_request_id).call().await.map_err(|e| {
        error!("get_swap_request failed: {:?}", e);
        Status::InternalServerError
    })?;

    let src_chain = Chain::try_from_str(swap_request.origin_chain.as_str()).unwrap();
    let dst_chain = Chain::try_from_str(swap_request.destination_chain.as_str()).unwrap();    
    let src_amount = swap_request.origin_amount;
    let dst_amount = swap_request.destination_amount;
    let pkp_address = swap_request.pkp_address;
    let pkp_token_id = swap_request.pkp_token_id;

    info!("pkp_token_id: {:?}", pkp_token_id);

    let end_user = EndUser::from_secret_key(testnet, &hex_to_bytes(MANAGER_PRIVATE_KEY).unwrap());
    let pkp_public_key = end_user.lookup_pkp_by_token_id(pkp_token_id).await.expect("Failed to lookup PKP by token id");
    // let slippage = swap_request.slippage.as_u128();
    // let pricing_type = swap_request.pricing_type;
    // let quote_deadline_seconds = swap_request.quote_deadline_seconds.as_u128();
    // let transaction_deadline_seconds = swap_request.transaction_deadline_seconds.as_u128();

    let src_providor = Provider::<Http>::try_from(src_chain.info().rpc_url).unwrap();
    let dst_provider = Provider::<Http>::try_from(dst_chain.info().rpc_url).unwrap();

    let src_balance = src_providor.get_balance(pkp_address, None).await.unwrap();
    let dst_balance = dst_provider.get_balance(pkp_address, None).await.unwrap();

    info!("src_balance on {:?}: {:?}, dst_balance on {:?}: {:?}", src_chain.info().chain_name, src_balance, dst_chain.info().chain_name, dst_balance);


    if src_balance < U256::from(src_amount) {
        error!("Insufficient funds for source chain: {}", src_chain.info().chain_id);
        return Err(Status::PaymentRequired);
    }
    if dst_balance < U256::from(dst_amount) {
        error!("Insufficient funds for destination chain: {}", dst_chain.info().chain_id);
        return Err(Status::PaymentRequired);
    }

    use crate::abstractions::transfer::endpoints::internal_send;
    let validator_collection = validator_collection.inner();
    let testnet = testnet.inner();

    let src_request = TransferRequest {
        api_key: MANAGER_PRIVATE_KEY.to_string(),
        pkp_public_key: pkp_public_key.clone(),
        chain: src_chain.info().chain.to_string(),
        destination_address: bytes_to_hex(&quote.provider_refund_address.to_fixed_bytes()),
        amount: format_ether(src_amount - 30000).parse::<f64>().unwrap(),
    };
    internal_send(testnet, validator_collection, &Json(src_request)).await?;

    let dst_request = TransferRequest {
        api_key: MANAGER_PRIVATE_KEY.to_string(),
        pkp_public_key: pkp_public_key.clone(),
        chain: dst_chain.info().chain.to_string(),
        destination_address: bytes_to_hex(&swap_request.origin_address.to_fixed_bytes()),
        amount: format_ether(dst_amount - 30000).parse::<f64>().unwrap(),
    };
    internal_send(testnet, validator_collection, &Json(dst_request)).await?;

    Ok(Json(String::new()))
}

fn swap_request_to_data(sr: &SwapRequest) -> SwapRequestData {
    SwapRequestData {
        from: format!("{:?}", sr.from),
        pkp_address: format!("{:?}", sr.pkp_address),
        origin_symbol: sr.origin_symbol.clone(),
        origin_chain: sr.origin_chain.clone(),
        origin_amount:  format_ether(sr.origin_amount).parse::<f64>().unwrap(),
        destination_symbol: sr.destination_symbol.clone(),
        destination_chain: sr.destination_chain.clone(),
        destination_amount: format_ether(sr.destination_amount).parse::<f64>().unwrap(),
        slippage: sr.slippage.as_u128(),
        pricing_type: sr.pricing_type,
        quote_deadline_seconds: sr.quote_deadline_seconds.as_u128(),
        origin_address: format!("{:?}", sr.origin_address),
        refund_address: format!("{:?}", sr.refund_address),
        transaction_deadline_seconds: sr.transaction_deadline_seconds.as_u128(),
        message: sr.message.clone(),
    }
}

fn quote_to_data(q: &Quote, sr: &SwapRequest) -> QuoteData {   
    QuoteData {
        swap_request_id: q.swap_request_id.as_u128(),
        provider_refund_address: format!("{:?}", q.provider_refund_address),
        quote_expiry: q.quote_expiry.as_u64(),
        created_at: q.created_at.as_u64(),
        fees_total: q.fees_total.as_u128(),
        swap_request_data: swap_request_to_data(sr),
    }
}

async fn get_signable_quote_contract() -> Result<QuoteStorage<SignerMiddleware<Provider<Http>, LocalWallet>>, Status> {
    let chain = Chain::Yellowstone;
    let chain_info = chain.info();
    let secret = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let secret = hex_to_bytes(secret).unwrap();

    let wallet = LocalWallet::from_bytes(&secret).unwrap().with_chain_id(chain_info.chain_id); 

    let provider = Provider::<Http>::try_from(chain_info.rpc_url).unwrap();
    let signing_provider = SignerMiddleware::new(provider.clone(), wallet);

    let client = Arc::new(signing_provider);
    let quote_storage_address = hex_to_bytes(QUOTE_STORAGE_ADDRESS).unwrap();
    let quote_storage_address = H160::from_slice(&quote_storage_address);
    let contract = QuoteStorage::new(quote_storage_address, client);
    Ok(contract)
}
