use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Tx {
    #[serde(default)]
    pub blockHash: String,
    #[serde(default)]
    pub blockNumber: String,
    #[serde(default)]
    pub confirmations: String,
    #[serde(default)]
    pub contractAddress: String,
    #[serde(default)]
    pub cumulativeGasUsed: String,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub gas: String,
    #[serde(default)]
    pub gasPrice: String,
    #[serde(default)]
    pub gasUsed: String,
    #[serde(default)]
    pub hash: String,
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub isError: String,
    #[serde(default)]
    pub nonce: String,
    #[serde(default)]
    pub timeStamp: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub transactionIndex: String,
    #[serde(default)]
    pub txReceiptStatus: Option<String>,
    #[serde(default)]
    pub value: String,
}

impl Tx {
    pub fn chain_time_stamp(&self) -> SystemTime {
        let timestamp: f64 = self.timeStamp.parse().unwrap();
        UNIX_EPOCH + std::time::Duration::from_secs(timestamp as u64)
    }
}

#[derive(Serialize, Deserialize)]
pub struct BlockScoutResponse {
    pub message: String,
    pub result: Vec<Tx>,
    pub status: String,
}

pub struct RpcCalls;

impl RpcCalls {
    pub async fn get_tx_list_async(
        chain_api_url: &str,
        address: &str,
        block_start: u64,
        block_end: u64,
        page: u64,
        page_size: u64,
        internal_transactions: bool,
    ) -> Result<BlockScoutResponse, Box<dyn Error>> {
        let client = Client::new();

        let tx_list_type = match internal_transactions {
            true => "txlistinternal",
            false => "txlist",
        };
        //txlistinternal
        let url = format!(
            "{}?module=account&action={}&sort=dsc&page={}&offset={}&startblock={}&endBlock={}&address=0x{}",
            chain_api_url, tx_list_type, page, page_size, block_start, block_end, address
        );

        let response = client.get(&url).send().await?;
        let response_string = response.text().await?;

        // log::info!("blockscoutresponse: {:?}", response_string);
        if response_string.trim().is_empty() {
            Err("No data returned".into())
        } else {
            let block_scout_response: Result<BlockScoutResponse, serde_json::Error> =
                serde_json::from_str(&response_string);

            if block_scout_response.is_err() {
                let err = block_scout_response.err().unwrap();
                log::error!("Error parsing response: {:?}", err);
                return Err(Box::new(err));
            }

            Ok(block_scout_response.unwrap())
        }
    }
}
