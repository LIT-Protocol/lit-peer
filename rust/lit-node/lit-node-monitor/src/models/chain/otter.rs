use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Log {
    pub address: Option<String>,
    pub topics: Option<Vec<String>>,
    pub data: Option<String>,
    pub blockHash: String,
    pub blockNumber: Option<String>,
    pub transactionHash: String,
    pub transactionIndex: Option<String>,
    pub logIndex: Option<String>,
    pub transactionLogIndex: Option<String>,
    pub removed: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Receipt {
    pub transactionHash: String,
    pub transactionIndex: Option<String>,
    pub blockHash: Option<String>,
    pub blockNumber: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub cumulativeGasUsed: Option<String>,
    pub gasUsed: Option<String>,
    pub contractAddress: Option<String>,
    pub logs: Option<Vec<Log>>,
    pub status: Option<String>,
    pub logsBloom: Option<String>,
    pub effectiveGasPrice: Option<String>,
    pub timestamp: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OtterscanResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub result: OtterscanResult,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct OtterscanResult {
    pub firstPage: bool,
    pub lastPage: bool,
    pub receipts: Vec<Receipt>,
    pub txs: Vec<Tx>,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Tx {
    pub hash: String,
    pub nonce: String,
    pub blockHash: String,
    pub blockNumber: String,
    pub transactionIndex: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: String,
    pub gasPrice: String,
    pub gas: String,
    pub input: String,
    pub v: String,
    pub r: String,
    pub s: String,
    pub chainId: String,
    pub r#type: Option<String>,
    pub accessList: Option<Vec<serde_json::Value>>, // Using serde_json::Value for dynamic objects
    pub maxPriorityFeePerGas: Option<String>,
    pub maxFeePerGas: Option<String>,
}

pub struct RpcCalls;

impl RpcCalls {
    pub async fn get_tx_list_async(
        chain_url: &str,
        address: &str,
        _block_start: u64,
        _block_end: u64,
    ) -> Result<OtterscanResult, Box<dyn Error>> {
        let client = Client::new();
        let parameters = json!([address, 0, 500]);

        let resp: serde_json::Value = client
            .post(chain_url)
            .header("Content-Type", "application/json")
            .json(&json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "ots_searchTransactionsBefore",
                "params": parameters
            }))
            .send()
            .await?
            .json()
            .await?;

        log::info!("resp: {:?}", resp);

        let response: Result<OtterscanResponse, serde_json::Error> = serde_json::from_value(resp);

        if response.is_err() {
            let err = response.err().unwrap();
            log::error!("Error parsing response: {:?}", err);
            return Err(Box::new(err));
        }

        Ok(response.unwrap().result)
    }
}
