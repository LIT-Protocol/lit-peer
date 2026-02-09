use super::chain_info::Chain;
use super::models::{GetBalanceResponse, TransferRequest, TransferResponse};
use crate::core::internal::{combine_signature_shares, sign_with_pkp};
use crate::core::v1::models::request::{CombineSignatureSharesRequest, SignWithPKPRequest};
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::Signer;
use ethers::types::{H160, U256};
use ethers::types::{Signature, transaction::eip2718::TypedTransaction};
use ethers::utils::{eip1559_default_estimator, format_ether, keccak256, parse_ether};
use lit_core::utils::binary::{bytes_to_hex, hex_to_bytes};
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::sync::Arc;
use tracing::info;

pub async fn get_api_key_balance(
    testnet: &Arc<Testnet>,
    api_key: &str,
    chain: Chain,
) -> Result<Json<GetBalanceResponse>, Status> {
    let secret_key = hex_to_bytes(api_key).unwrap();
    let end_user = EndUser::from_secret_key(testnet, &secret_key);
    end_user
        .deposit_to_wallet_ledger(U256::from(10000000000000000u128))
        .await;
    let from = end_user.wallet.address();

    get_balance(from, chain).await
}

pub async fn get_pkp_balance(
    pkp_public_key: &str,
    chain: Chain,
) -> Result<Json<GetBalanceResponse>, Status> {
    let pkp_address = get_pkp_address(pkp_public_key).await.unwrap();
    get_balance(pkp_address, chain).await
}

pub async fn get_address_balance(
    address: &str,
    chain: Chain,
) -> Result<Json<GetBalanceResponse>, Status> {
    let address = H160::from_slice(hex_to_bytes(address).unwrap().as_slice());
    get_balance(address, chain).await
}

async fn get_balance(address: H160, chain: Chain) -> Result<Json<GetBalanceResponse>, Status> {
    let provider = Provider::<Http>::try_from(chain.info().rpc_url).unwrap();
    info!("provider: {:?}", provider);

    let block = None;
    let balance = provider.get_balance(address, block).await.unwrap();

    let balance = format_ether(balance).parse::<f64>().unwrap();

    Ok(Json(GetBalanceResponse {
        address: bytes_to_hex(&address.as_bytes()),
        balance: balance,
        chain: chain.clone(),
        symbol: chain.info().token.to_string(),
    }))
}

async fn get_pkp_address(pkp_public_key: &str) -> Result<H160, Status> {
    let pkp_address = hex::decode(&pkp_public_key.replace("0x", "")[2..]).unwrap();
    let pkp_address = keccak256(&pkp_address);
    let pkp_address = H160::from_slice(&pkp_address[12..]);

    Ok(pkp_address)
}

pub async fn send(
    testnet: &Arc<Testnet>,
    validator_collection: &Arc<ValidatorCollection>,
    request: &Json<TransferRequest>,
    chain: Chain,
) -> Result<Json<TransferResponse>, Status> {
    let provider = Provider::<Http>::try_from(chain.info().rpc_url).unwrap();

    let pkp_address = get_pkp_address(&request.pkp_public_key).await.unwrap();

    let nonce = provider
        .get_transaction_count(pkp_address, None)
        .await
        .unwrap();
    let gas_price = provider.get_gas_price().await.unwrap();

    info!("gas_price: {:?}", gas_price);
    let block = provider
        .get_block(ethers::types::BlockNumber::Latest)
        .await
        .unwrap();
    let block = block.unwrap();
    let base_fee_per_gas = block.base_fee_per_gas.unwrap();
    let gas_limit = block.gas_limit;
    info!(
        "base_fee_per_gas: {:?}, gas_limit: {:?}",
        base_fee_per_gas, gas_limit
    );

    let gas_price = U256::from(21000);
    info!("limited gas_price: {:?}", gas_price);

    let to = request
        .destination_address
        .parse::<H160>()
        .expect("Invalid destination address");
    // 1. Structure the transaction
    // let tx = TransactionRequest::new()

    let amount_in_wei = parse_ether(request.amount).unwrap();

    let mut tx = ethers::types::Eip1559TransactionRequest::new()
        .from(pkp_address)
        .to(to)
        .value(amount_in_wei)
        // .gas_price(gas_price)
        .gas(gas_price)
        .nonce(nonce)
        .chain_id(chain.info().chain_id);

    let (max_fee_per_gas, max_priority_fee_per_gas) = provider
        .estimate_eip1559_fees(Some(eip1559_default_estimator))
        .await
        .unwrap();
    info!(
        "estimated gas fees - max: {:?}, priority:{:?}",
        max_fee_per_gas, max_priority_fee_per_gas
    );
    tx = tx
        .max_fee_per_gas(max_fee_per_gas)
        .max_priority_fee_per_gas(max_priority_fee_per_gas);

    info!("tx: {:?}", tx);
    // 2. RLP encode the transaction
    let typed_tx = TypedTransaction::Eip1559(tx);
    // let typed_tx = TypedTransaction::Eip1559(tx);
    let encoded_tx = typed_tx.rlp();

    // normally we'd keccak256 the transaction and sign that, but the function we're about to call does this for us
    let encode_tx_string = format!("0x{}", bytes_to_hex(&encoded_tx.0.to_vec()));

    let signature = sign_with_pkp(
        testnet,
        validator_collection,
        Json(SignWithPKPRequest {
            api_key: request.api_key.clone(),
            message: encode_tx_string,
            pkp_public_key: request.pkp_public_key.clone(),
            signing_scheme: chain.info().signing_scheme.to_string(),
        }),
    )
    .await
    .unwrap();

    let signature_response = combine_signature_shares(Json(CombineSignatureSharesRequest {
        api_key: request.api_key.clone(),
        shares: signature.shares.clone(),
    }))
    .await
    .unwrap();

    info!("signature_response: {:?}", signature_response);
    let chain_id_offset = chain.info().chain_id * 2 + 35;
    let signature = Signature {
        r: U256::from_str_radix(&signature_response.r, 16).unwrap(),
        s: U256::from_str_radix(&signature_response.s, 16).unwrap(),
        v: signature_response.recovery_id as u64 + chain_id_offset,
    };

    info!("signature: {:?}", signature);

    let message = hex_to_bytes(&signature_response.signed_data.clone()).unwrap();
    let message = ethers::types::H256::from_slice(&message);
    let address = hex::decode(&signature_response.verifying_key[2..]).unwrap();
    let address = keccak256(&address);
    let address = H160::from_slice(&address[12..]);

    let result = signature.verify(message, address);
    info!("result for address {:?}: {:?}", address, result);
    if !result.is_ok() {
        return Err(Status::InternalServerError);
    }

    let signed_tx = typed_tx.rlp_signed(&signature);

    info!("signed_tx: {:?}", bytes_to_hex(&signed_tx));

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
        chain: chain.clone(),
        origin_symbol: chain.info().token.to_string(),
        origin_amount: request.amount.clone(),
        gas: tx_receipt.gas_used.unwrap().to_string(),
        timestamp: tx_receipt.block_number.unwrap().to_string(),
        destination_address: request.destination_address.clone(),
    }))
}
