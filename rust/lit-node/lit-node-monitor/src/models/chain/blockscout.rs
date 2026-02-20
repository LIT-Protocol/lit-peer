use chrono::NaiveDateTime;
use reqwest::Client;
use serde::de::DeserializeOwned;
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

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TokenTransfer {
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
    pub functionName: String,
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
    pub methodId: String,
    #[serde(default)]
    pub nonce: String,
    #[serde(default)]
    pub timeStamp: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub tokenDecimal: String,
    #[serde(default)]
    pub tokenID: String,
    #[serde(default)]
    pub tokenName: String,
    #[serde(default)]
    pub tokenSymbol: String,
    #[serde(default)]
    pub transactionIndex: String,
}

impl Tx {
    pub fn chain_time_stamp(&self) -> SystemTime {
        let timestamp: f64 = self.timeStamp.parse().unwrap();
        UNIX_EPOCH + std::time::Duration::from_secs(timestamp as u64)
    }
}

// struct to get a block number from a timestamp
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct BlockNumber {
    #[serde(default)]
    pub blockNumber: String,
}

// get block number details
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct BlockNumberReward {
    pub blockMiner: String,
    pub blockNumber: String,
    pub blockReward: String,
    pub timeStamp: u128,
    pub uncleInclusionReward: Option<String>,
    pub uncles: Option<Vec<String>>,
}

// wrapper for blockscout responses
#[derive(Serialize, Deserialize)]
pub struct BlockScoutResponse<T> {
    pub message: String,
    pub result: T,
    pub status: String,
}

pub enum TokenType {
    ERC20,   // ERC20 transfer
    ERC721,  // ERC721 transfer
    ERC1155, // ERC1155 transfer
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::ERC20 => "tokentx".to_string(),
            TokenType::ERC721 => "tokennfttx".to_string(),
            TokenType::ERC1155 => "token1155tx".to_string(),
        }
    }
}
pub struct RpcCalls;

impl RpcCalls {
    // Generic handler to request / respond to a blockscout API call
    async fn get_block_scout_response<T>(
        url: String,
    ) -> Result<BlockScoutResponse<T>, Box<dyn Error>>
    where
        T: Serialize + DeserializeOwned,
    {
        let client = Client::new();
        log::info!("url: {:?}", url);
        let response = client.get(&url).send().await?;
        let response_string = response.text().await?;

        // log::info!("blockScoutResponse: {:?}", response_string);
        if response_string.trim().is_empty() {
            Err("No data returned".into())
        } else {
            let block_scout_response: Result<BlockScoutResponse<T>, serde_json::Error> =
                serde_json::from_str(&response_string);

            if block_scout_response.is_err() {
                let err = block_scout_response.err().unwrap();
                log::error!("Error parsing response for url: {:?}: {:?}", url, err);
                return Err(Box::new(err));
            }

            Ok(block_scout_response.unwrap())
        }
    }

    pub async fn get_tx_list_async(
        chain_api_url: &str,
        address: &str,
        block_start: u64,
        block_end: u64,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
        page: u64,
        page_size: u64,
        internal_transactions: bool,
    ) -> Result<BlockScoutResponse<Vec<Tx>>, Box<dyn Error>> {
        let tx_list_type = match internal_transactions {
            true => "txlistinternal",
            false => "txlist",
        };

        //txlistinternal
        let mut url = format!(
            "{}?module=account&action={}&sort=dsc&startblock={}&endblock={}&address=0x{}",
            chain_api_url, tx_list_type, block_start, block_end, address
        );
        log::info!("start/end block: {}/{}", block_start, block_end);

        if !internal_transactions {
            url = format!("{}&page={}&offset={}", url, page, page_size);
        }

        if let Some(start_date) = start_date {
            let start_date_unix = NaiveDateTime::from(start_date).and_utc().timestamp();
            url = format!("{}&start_timestamp={}", url, start_date_unix);
        }
        if let Some(end_date) = end_date {
            let end_date_unix = NaiveDateTime::from(end_date).and_utc().timestamp();
            url = format!("{}&end_timestamp={}", url, end_date_unix);
        }

        Self::get_block_scout_response::<Vec<Tx>>(url).await
    }

    pub async fn get_token_transfer_list_async(
        chain_api_url: &str,
        address: &str,
        address_is_contract: bool,
        token_type: TokenType,
        block_start: u64,
        block_end: u64,
        page: u64,
        page_size: u64,
    ) -> Result<BlockScoutResponse<Vec<TokenTransfer>>, Box<dyn Error>> {
        //txlistinternal
        let mut url = format!(
            "{}?module=account&action={}&sort=dsc&startblock={}&endblock={}&",
            chain_api_url,
            token_type.to_string(),
            block_start,
            block_end,
        );
        log::info!("start/end block: {}/{}", block_start, block_end);

        url = format!("{}&page={}&offset={}", url, page, page_size);

        if address_is_contract {
            url = format!("{}&contractaddress={}", url, address);
        } else {
            url = format!("{}&address={}", url, address);
        }

        Self::get_block_scout_response::<Vec<TokenTransfer>>(url).await
    }

    pub async fn get_block_number(
        chain_api_url: &str,
        block_timestamp: String,
    ) -> Result<BlockScoutResponse<BlockNumber>, Box<dyn Error>> {
        let url = format!(
            "{}?module=block&action=getblocknobytime&timestamp={}&closest=before",
            chain_api_url, block_timestamp
        );

        Self::get_block_scout_response::<BlockNumber>(url).await
    }

    pub async fn get_rewards_by_block_number(
        chain_api_url: &str,
        block_number: u64,
    ) -> Result<BlockScoutResponse<BlockNumberReward>, Box<dyn Error>> {
        let url = format!(
            "{}?module=block&action=getblockreward&blockno={}",
            chain_api_url, block_number
        );

        Self::get_block_scout_response::<BlockNumberReward>(url).await
    }

    pub async fn get_lastest_block_number(
        chain_api_url: &str,
    ) -> Result<BlockScoutResponse<u128>, Box<dyn Error>> {
        let url = format!("{}??module=block&action=eth_block_number", chain_api_url);

        Self::get_block_scout_response::<u128>(url).await
    }
}
