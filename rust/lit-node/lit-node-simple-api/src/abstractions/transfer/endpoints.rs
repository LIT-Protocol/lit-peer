//! Rocket endpoints for transfer (balance and send).

use crate::abstractions::transfer::evm;
use crate::abstractions::transfer::non_evm;

use super::chain_info::Chain;
use super::models::{GetBalanceResponse, TransferRequest, TransferResponse};
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{Route, get, http::Status, post, routes};
use std::sync::Arc;
use tracing::{error, info};

pub fn routes() -> Vec<Route> {
    routes![get_api_key_balance, get_pkp_balance, send]
}

/// GET /get_balance/<api_key>/<chain> — get balance for an address on a chain.
#[get("/get_api_key_balance/<api_key>/<chain>")]
async fn get_api_key_balance(testnet: &State<Arc<Testnet>>, api_key: &str, chain: &str) -> Result<Json<GetBalanceResponse>, Status> {
    
    let testnet = testnet.inner();
    let chain = match Chain::try_from_str(chain) {
        Ok(chain) => chain,
        Err(e) => {
            error!("Invalid chain: {:?}", e);
            return Err(e);
        },
    };

    match chain.info().is_evm {
        true => evm::get_api_key_balance(testnet, api_key, chain).await,
        false => non_evm::get_api_key_balance(testnet, api_key, chain).await,        
    }
}

#[get("/get_pkp_balance/<pkp_public_key>/<chain>")]
async fn get_pkp_balance( pkp_public_key: &str, chain: &str) -> Result<Json<GetBalanceResponse>, Status> {
    let chain = match Chain::try_from_str(chain) {
        Ok(chain) => chain,
        Err(e) => {
            error!("Invalid chain: {:?}", e);
            return Err(e);
        },
    };

    match chain.info().is_evm {
        true => evm::get_pkp_balance(pkp_public_key, chain).await,
        false => non_evm::get_pkp_balance(pkp_public_key, chain).await,        
    }
}

/// POST /send — send funds to a destination address on a chain.
#[post("/send", format = "json", data = "<request>")]
async fn send(testnet: &State<Arc<Testnet>>, validator_collection: &State<Arc<ValidatorCollection>>, request: Json<TransferRequest>) -> Result<Json<TransferResponse>, Status> {

    info!("request: {:?}", request);
    let validator_collection = validator_collection.inner();
    let testnet = testnet.inner();
    
    let chain = match Chain::try_from_str(request.chain.as_str()) {
        Ok(chain) => chain,
        Err(e) => {
            error!("Invalid chain: {:?}", e);
            return Err(e);
        },
    };

    match chain.info().is_evm {
        true => evm::send(testnet, validator_collection, &request, chain).await,
        false => non_evm::send(testnet, validator_collection, &request, chain).await,
    }
}
