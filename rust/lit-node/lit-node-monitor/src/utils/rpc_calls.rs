use crate::models::chain::{SimpleTx, blockscout, otter};
use std::error::Error;

pub async fn get_tx_list_async(
    rpc_api_type: u32,
    chain_api_url: &str,
    address: &str,
    block_start: u64,
    block_end: u64,
    include_internal_transactions: bool,
    page: u64,
    page_size: u64,
) -> Result<Vec<SimpleTx>, Box<dyn Error>> {
    let txs: Vec<SimpleTx> = match rpc_api_type {
        1 => {
            let bs_txs = blockscout::RpcCalls::get_tx_list_async(
                chain_api_url,
                address,
                block_start,
                block_end,
                page,
                page_size,
                false,
            )
            .await
            .expect("Error getting tx list");

            let bs_internal_txs = match include_internal_transactions {
                true => blockscout::RpcCalls::get_tx_list_async(
                    chain_api_url,
                    address,
                    block_start,
                    block_end,
                    page,
                    page_size,
                    true,
                )
                .await
                .expect("Error getting internal tx list"),
                false => blockscout::BlockScoutResponse {
                    result: vec![],
                    status: "".to_string(),
                    message: "".to_string(),
                },
            };

            let all_txs = bs_txs.result.iter().chain(bs_internal_txs.result.iter());

            let mut results: Vec<SimpleTx> = all_txs
                .map(|tx| SimpleTx {
                    block_hash: tx.blockHash.clone(),
                    block_number: tx.blockNumber.clone(),
                    from: tx.from.clone(),
                    to: tx.to.clone(),
                    gas: tx.gas.clone(),
                    gas_price: tx.gasPrice.clone(),
                    hash: tx.hash.clone(),
                    input: tx.input.clone(),
                    is_error: tx.isError.clone(),
                    nonce: tx.nonce.clone(),
                    time_stamp: tx.timeStamp.clone(),
                    transaction_index: tx.transactionIndex.clone(),
                })
                .collect();
            results.sort_by(|f, s| {
                s.time_stamp
                    .parse::<u128>()
                    .unwrap()
                    .cmp(&f.time_stamp.parse::<u128>().unwrap())
            });
            // results.sort_by_key(|tx| tx.time_stamp.parse::<u128>().unwrap());
            results
        }
        2 => {
            let ot_tx =
                otter::RpcCalls::get_tx_list_async(chain_api_url, address, block_start, block_end)
                    .await;

            log::info!("ot_tx: {:?}", ot_tx);

            let ot_result = ot_tx.unwrap();
            ot_result
                .txs
                .iter()
                .map(|tx| SimpleTx {
                    block_hash: tx.blockHash.clone(),
                    block_number: tx.blockNumber.clone(),
                    from: tx.from.as_ref().unwrap_or(&"".to_string()).to_string(),
                    to: tx.to.as_ref().unwrap_or(&"".to_string()).to_string(),
                    gas: tx.gas.clone(),
                    gas_price: tx.gasPrice.clone(),
                    hash: tx.hash.clone(),
                    input: tx.input.clone(),
                    is_error: "0".to_string(),
                    nonce: tx.nonce.clone(),
                    // timeStamp = otterscan_response.receipts.Where(x => x.transactionHash == tx.hash).FirstOrDefault().timestamp.ToString(),
                    // time_stamp: tx.nonce.clone(),
                    time_stamp: "0".to_string(),
                    // ot_result
                    //     .receipts
                    //     .iter()
                    //     .find(|r| r.transactionHash == tx.hash)
                    //     .unwrap()
                    //     .timestamp
                    //     .to_string(),
                    transaction_index: tx.transactionIndex.clone(),
                })
                .collect()
        }
        _ => return Err("Invalid RPC API Type".into()),
    };

    Ok(txs)
}
