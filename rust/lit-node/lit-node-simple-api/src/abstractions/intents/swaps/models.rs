//! Request and response models for the swap intents API.

use serde::{Deserialize, Serialize};

// ============== TokenList ==============

/// Request for `/v1/TokenList`. No parameters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenListRequest {}

/// A supported token in the token list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenListItem {
    /// Unique identifier (e.g. for updates when symbol changes).
    pub id: String,
    pub symbol: String,
    pub blockchain: String,
    /// USD equivalent price.
    pub usd_equivalent_price: String,
    /// When the USD price was set.
    pub usd_price_date: String,
}

/// Response for `/v1/TokenList`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenListResponse {
    pub tokens: Vec<TokenListItem>,
}

// ============== GetQuote ==============

/// How the quote is priced; determines where fees are taken from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum QuotePricingType {
    /// Destination amount stays the same; origin amount increases.
    Origin,
    /// Origin sent stays the same; destination amount is reduced.
    Destination,
}

/// Request for `/v1/GetQuote`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuoteRequest {
    /// Message sender (origin of the swap).
    pub from: String,
    pub origin_symbol: String,
    pub origin_amount: String,
    pub destination_symbol: String,
    pub destination_amount: String,
    /// Acceptable pricing slippage.
    pub slippage: String,
    /// How the quote is priced (where fees are taken from).
    #[serde(rename = "type")]
    pub pricing_type: QuotePricingType,
    /// How long to wait for a quote (0–60 seconds).
    pub quote_deadline_seconds: u8,
    /// Origin address (e.g. for cross-reference with the transaction).
    pub origin_address: Option<String>,
    /// Where to refund if the transaction does not go through.
    pub refund_address: Option<String>,
    /// Max time for the transaction to complete; after this, revert. Optional.
    pub transaction_deadline_seconds: Option<u64>,
    /// Optional message.
    pub message: Option<String>,
}

/// A single fee component in a quote.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteFee {
    /// Who receives the fee (e.g. protocol, relay).
    pub recipient: Option<String>,
    pub amount: String,
    pub symbol: Option<String>,
}

/// Response for `/v1/GetQuote`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuoteResponse {
    pub quote_id: String,
    /// When the quote expires.
    pub quote_expiry: String,
    /// List of fee components.
    pub fees: Vec<QuoteFee>,
    /// Hash of input parameters signed by LIT (e.g. with network key).
    pub signed_input_hash: String,
}

// ============== AcceptQuote ==============

/// Request for `/v1/AcceptQuote`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptQuoteRequest {
    pub quote_id: String,
}

/// Response for `/v1/AcceptQuote`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptQuoteResponse {
    /// PKP address to send the funds to.
    pub pkp_address: String,
}

// ============== GetSwapStatus ==============

/// Request for `/v1/GetSwapStatus`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSwapStatusRequest {
    pub quote_id: String,
}

/// State of a swap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SwapState {
    Pending,
    Processing,
    Success,
    Expired,
    Refunded,
    /// Catch-all for other states (e.g. Failed, Cancelled).
    Other,
}

/// Swap and quote details returned with status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapQuoteDetails {
    pub quote_id: String,
    pub origin_symbol: Option<String>,
    pub origin_amount: Option<String>,
    pub destination_symbol: Option<String>,
    pub destination_amount: Option<String>,
    pub pkp_address: Option<String>,
    /// Additional status-specific fields as needed.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Response for `/v1/GetSwapStatus`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSwapStatusResponse {
    pub state: SwapState,
    /// Swap and quote details when available.
    pub details: Option<SwapQuoteDetails>,
}
