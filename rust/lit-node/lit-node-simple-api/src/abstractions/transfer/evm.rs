use super::chain_info::Chain;
use super::models::{GetBalanceResponse, TransferRequest, TransferResponse};
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::Signer;
use ethers::types::{H160, U256};
use lit_core::utils::binary::bytes_to_hex;
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;
use rocket::serde::json::Json;
use rocket::http::Status;
use ethers::types::{
    transaction::eip2718::TypedTransaction, TransactionRequest, Signature
};
use crate::core::internal::{combine_signature_shares, sign_with_pkp};
use crate::core::v1::models::request::{CombineSignatureSharesRequest, SignWithPKPRequest};
use std::sync::Arc;

pub async fn get_balance(testnet: &Arc<Testnet>, api_key: &str, chain: Chain) -> Result<Json<GetBalanceResponse>, Status> {
    let _ = (api_key, chain);

    let secret_key = api_key.as_bytes();
    let provider = Provider::<Http>::try_from(chain.info().rpc_url).unwrap();
    let end_user = EndUser::from_secret_key(testnet, secret_key);

    let from = end_user.wallet.address();
    let block = None;
    let balance = provider.get_balance(from, block).await.unwrap();


    Ok(Json(GetBalanceResponse {
        address: bytes_to_hex(&from.as_bytes()),
        balance: balance.to_string(),
        chain: chain.clone(),
        symbol: chain.info().token.to_string(),
    }))
}

pub async fn send(testnet: &Arc<Testnet>, validator_collection: &Arc<ValidatorCollection>, request: &Json<TransferRequest>, chain: Chain) -> Result<Json<TransferResponse>, Status> {

    let secret_key = request.api_key.as_bytes();
    let provider = Provider::<Http>::try_from(chain.info().rpc_url).unwrap();
    let end_user = EndUser::from_secret_key(testnet, secret_key);

    let nonce = provider.get_transaction_count(end_user.wallet.address(), None).await.unwrap();
    let gas_price = provider.get_gas_price().await.unwrap();
    
    let to = request.destination_address.parse::<H160>().expect("Invalid destination address");
    // 1. Structure the transaction
    let tx = TransactionRequest::new()
        .to(to)
        .value(U256::from_dec_str(request.amount.as_str()).unwrap())
        .gas_price(gas_price)
        .nonce(nonce)
        .chain_id(chain.info().chain_id);
    
    // 2. RLP encode the transaction
    let typed_tx = TypedTransaction::Legacy(tx);
    let encoded_tx = typed_tx.rlp();

    // normally we'd keccak256 the transaction and sign that, but the function we're about to call does this for us
    let encode_tx_string = hex::encode(encoded_tx);

    let signature = sign_with_pkp(testnet, validator_collection, Json(SignWithPKPRequest {
        api_key: request.api_key.clone(),
        message: encode_tx_string,
        pkp_public_key: request.pkp_public_key.clone(),
        signing_scheme: chain.info().signing_scheme.to_string(),
    })).await.unwrap();


    let signature_response = combine_signature_shares( Json(CombineSignatureSharesRequest {
        api_key: request.api_key.clone(),
        shares: signature.shares.clone(),
    })).await.unwrap();

    let signature = Signature {
        r: U256::from_str_radix(&signature_response.r, 16).unwrap(),
        s: U256::from_str_radix(&signature_response.s, 16).unwrap(),
        v: signature_response.v as u64,
    };

    let signed_tx = typed_tx.rlp_signed(&signature);    

    let pending_tx = provider.send_raw_transaction(signed_tx).await.unwrap();
    let tx_receipt = pending_tx.await.unwrap();
    if tx_receipt.is_none() {
        return Err(Status::InternalServerError);
    }
    let tx_receipt = tx_receipt.unwrap();
    

    let _ = request;
    Ok(Json(TransferResponse {
        txn_id: bytes_to_hex(&tx_receipt.transaction_hash.as_bytes()),
        success: true,
        chain: Chain::Ethereum,
        origin_symbol: chain.info().token.to_string(),
        origin_amount: request.amount.clone(),
        gas: tx_receipt.gas_used.unwrap().to_string(),
        timestamp: tx_receipt.block_number.unwrap().to_string(),
        destination_address: request.destination_address.clone(),
    }))
}