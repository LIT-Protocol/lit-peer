use chrono::{DateTime, NaiveDateTime, Utc};

use crate::models::chain::{SimpleTx, blockscout, otter};
use std::error::Error;

pub async fn get_tx_list_async(
    rpc_api_type: u32,
    chain_api_url: &str,
    address: &str,
    block_start: u64,
    block_end: u64,
    start_date: Option<NaiveDateTime>,
    end_date: Option<NaiveDateTime>,
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
                start_date,
                end_date,
                page,
                page_size,
                false,
            )
            .await
            .expect("Error getting tx list");

            let mut results: Vec<SimpleTx> = bs_txs
                .result
                .iter()
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

            if include_internal_transactions {
                let min_block_number = results
                    .iter()
                    .map(|tx| tx.block_number.parse::<u64>().unwrap())
                    .min()
                    .unwrap();
                let max_block_number = results
                    .iter()
                    .map(|tx| tx.block_number.parse::<u64>().unwrap())
                    .max()
                    .unwrap();

                log::info!("min_block_number: {:?}", min_block_number);
                log::info!("max_block_number: {:?}", max_block_number);
                log::info!("start_date: {:?}", start_date);
                log::info!("end_date: {:?}", end_date);

                let bs_internal_txs = blockscout::RpcCalls::get_tx_list_async(
                    chain_api_url,
                    address,
                    min_block_number,
                    max_block_number,
                    None,
                    None,
                    1,
                    1000,
                    true,
                )
                .await
                .expect("Error getting internal tx list");

                results.extend(bs_internal_txs.result.iter().map(|tx| SimpleTx {
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
                }));
            };

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

pub async fn get_block_number(
    rpc_api_type: u32,
    chain_api_url: &str,
    block_timestamp: String,
) -> Result<u64, Box<dyn Error>> {
    if rpc_api_type != 1 {
        return Err("Invalid RPC API Type".into());
    }

    let block_number =
        blockscout::RpcCalls::get_block_number(chain_api_url, block_timestamp).await?;
    Ok(block_number.result.blockNumber.parse::<u64>().unwrap())
}

pub async fn get_datetime_from_block_number(
    rpc_api_type: u32,
    chain_api_url: &str,
    block_number: u64,
) -> Result<DateTime<Utc>, Box<dyn Error>> {
    if rpc_api_type != 1 {
        return Err("Invalid RPC API Type".into());
    }

    let block_reward =
        blockscout::RpcCalls::get_rewards_by_block_number(chain_api_url, block_number).await?;

    Ok(DateTime::<Utc>::from_timestamp(block_reward.result.timeStamp as i64, 0).unwrap())
}

pub async fn get_lastest_block_number(
    rpc_api_type: u32,
    chain_api_url: &str,
) -> Result<u128, Box<dyn Error>> {
    if rpc_api_type != 1 {
        return Err("Invalid RPC API Type".into());
    }

    let block_number = blockscout::RpcCalls::get_lastest_block_number(chain_api_url).await?;
    Ok(block_number.result)
}
