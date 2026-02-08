/**
 * Lit Node Simple API - JavaScript Swaps SDK
 *
 * Wrapper for swap intents API endpoints in src/abstractions/intents/swaps/endpoints.rs.
 * Mount the swaps routes at /swaps/v1/ (e.g. in main.rs: .mount("/swaps/v1/", abstractions::intents::swaps::endpoints::routes())).
 */

// --- Token list ---

/**
 * @typedef {Object} TokenListItem
 * @property {string} id - Unique identifier
 * @property {string} symbol - Token symbol
 * @property {string} blockchain - Blockchain identifier
 * @property {string} usd_equivalent_price - USD equivalent price
 * @property {string} usd_price_date - When the USD price was set
 */

/**
 * @typedef {Object} TokenListResponse
 * @property {TokenListItem[]} tokens - List of supported tokens
 */

// --- New quote request ---

/** Quote pricing type: fees from origin amount. */
export const QUOTE_PRICING_ORIGIN = 'Origin';
/** Quote pricing type: fees from destination amount. */
export const QUOTE_PRICING_DESTINATION = 'Destination';

/**
 * @typedef {Object} NewSwapRequestOptions
 * @property {string} from - Message sender (origin of the swap), hex address
 * @property {string} originChain - Origin chain key (e.g. "ethereum", "yellowstone")
 * @property {string} originSymbol - Origin token symbol
 * @property {string|number|bigint} originAmount - Origin amount
 * @property {string} destinationSymbol - Destination token symbol
 * @property {string} destinationChain - Destination chain key
 * @property {string|number|bigint} destinationAmount - Destination amount
 * @property {string|number|bigint} slippage - Acceptable slippage
 * @property {string} [pricingType='Origin'] - 'Origin' or 'Destination' (use QUOTE_PRICING_*)
 * @property {number} [quoteDeadlineSeconds=60] - How long to wait for a quote (0–60)
 * @property {string} originAddress - Origin address (hex)
 * @property {string} refundAddress - Refund address if tx fails (hex)
 * @property {number} [transactionDeadlineSeconds=0] - Max time for transaction to complete
 * @property {string} [message] - Optional message
 */

/**
 * @typedef {Object} QuoteFee
 * @property {string} [recipient] - Fee recipient
 * @property {string} amount - Fee amount
 * @property {string} [symbol] - Fee symbol
 */

/**
 * @typedef {Object} NewSwapResponse
 * @property {string} swap_request_id - Swap request id
 * @property {string} swap_request_expiry - When the quote expires
 * @property {QuoteFee[]} fees - Fee components
 * @property {string} signed_input_hash - Hash of input params signed by LIT
 * @property {string} transaction_hash - Transaction hash for the request
 */

// --- Fill quote ---

/**
 * @typedef {Object} FillQuoteRequestOptions
 * @property {string} swapRequestId - Swap request id from new_quote_request
 * @property {number} quoteDeadlineSeconds - Quote deadline in seconds
 * @property {string} providerRefundAddress - Provider refund address (hex)
 * @property {string} [message] - Optional message
 */

/**
 * @typedef {Object} FillQuoteResponse
 * @property {string} quote_id - Quote id
 * @property {string} transaction_hash - Transaction hash
 * @property {string} pkp_address - PKP address for the quote
 * @property {string} swap_request_id - Swap request id
 * @property {string} quote_expiry - Quote expiry time
 * @property {QuoteFee[]} fees - Fee components
 * @property {number} total_fees - Total fees
 * @property {string} signed_input_hash - Signed input hash
 */

// --- Accept quote ---

/**
 * @typedef {Object} AcceptQuoteRequestOptions
 * @property {string} quoteId - Quote id from fill_quote
 */

/**
 * @typedef {Object} AcceptQuoteResponse
 * @property {string} pkp_address - PKP address to send funds to
 */

// --- Get swap status ---

/** Swap state: pending. */
export const SWAP_STATE_PENDING = 'Pending';
/** Swap state: processing. */
export const SWAP_STATE_PROCESSING = 'Processing';
/** Swap state: success. */
export const SWAP_STATE_SUCCESS = 'Success';
/** Swap state: expired. */
export const SWAP_STATE_EXPIRED = 'Expired';
/** Swap state: refunded. */
export const SWAP_STATE_REFUNDED = 'Refunded';
/** Swap state: other (e.g. failed, cancelled). */
export const SWAP_STATE_OTHER = 'Other';

/**
 * @typedef {Object} SwapQuoteDetails
 * @property {string} quote_id - Quote id
 * @property {string} [origin_symbol] - Origin symbol
 * @property {string} [origin_amount] - Origin amount
 * @property {string} [destination_symbol] - Destination symbol
 * @property {string} [destination_amount] - Destination amount
 * @property {string} [pkp_address] - PKP address
 * @property {Object} [extra] - Additional fields
 */

/**
 * @typedef {Object} GetSwapStatusResponse
 * @property {string} state - One of Pending, Processing, Success, Expired, Refunded, Other
 * @property {SwapQuoteDetails|null} [details] - Swap/quote details when available
 */

export class LitSwapsApiClient {
  /**
   * @param {Object} options
   * @param {string} [options.baseUrl='http://localhost:8000'] - Base URL of the API (server root). Routes are requested at baseUrl/swaps/v1/.
   */
  constructor({ baseUrl = 'http://localhost:8000' } = {}) {
    const base = baseUrl.replace(/\/$/, '');
    this.baseUrl = `${base}/swaps/v1`;
  }

  /**
   * GET /swaps/v1/token_list
   * Returns a list of supported tokens.
   * @returns {Promise<TokenListResponse>} { tokens: TokenListItem[] }
   */
  async getTokenList() {
    const res = await fetch(`${this.baseUrl}/token_list`);
    if (!res.ok) throw new Error(`token_list failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * POST /swaps/v1/new_quote_request
   * Create a new swap request and get a swap_request_id and transaction hash.
   * @param {NewSwapRequestOptions} options
   * @returns {Promise<NewSwapResponse>}
   */
  async newQuoteRequest(options) {
    const body = {
      from: options.from,
      origin_chain: options.originChain,
      origin_symbol: options.originSymbol,
      origin_amount: String(options.originAmount),
      destination_symbol: options.destinationSymbol,
      destination_chain: options.destinationChain,
      destination_amount: String(options.destinationAmount),
      slippage: String(options.slippage),
      type: options.pricingType ?? QUOTE_PRICING_ORIGIN,
      quote_deadline_seconds: options.quoteDeadlineSeconds ?? 60,
      origin_address: options.originAddress,
      refund_address: options.refundAddress,
      transaction_deadline_seconds: options.transactionDeadlineSeconds ?? 0,
      message: options.message ?? null,
    };
    const res = await fetch(`${this.baseUrl}/new_quote_request`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`new_quote_request failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * POST /swaps/v1/fill_quote
   * Fill a quote and get the quote_id and transaction hash.
   * @param {FillQuoteRequestOptions} options
   * @returns {Promise<FillQuoteResponse>}
   */
  async fillQuote(options) {
    const body = {
      swap_request_id: options.swapRequestId,
      quote_deadline_seconds: options.quoteDeadlineSeconds,
      provider_refund_address: options.providerRefundAddress,
      message: options.message ?? null,
    };
    const res = await fetch(`${this.baseUrl}/fill_quote`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`fill_quote failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * POST /swaps/v1/accept_quote
   * Accept a quote and get the PKP address to send funds to.
   * @param {AcceptQuoteRequestOptions} options
   * @returns {Promise<AcceptQuoteResponse>}
   */
  async acceptQuote(options) {
    const body = { quote_id: options.quoteId };
    const res = await fetch(`${this.baseUrl}/accept_quote`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`accept_quote failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * GET /swaps/v1/get_swap_status/<quote_id>
   * Get the status of a swap by quote id.
   * @param {string} quoteId - Quote id from fill_quote
   * @returns {Promise<GetSwapStatusResponse>} { state, details? }
   */
  async getSwapStatus(quoteId) {
    const res = await fetch(
      `${this.baseUrl}/get_swap_status/${encodeURIComponent(quoteId)}`
    );
    if (!res.ok) throw new Error(`get_swap_status failed: ${res.status} ${res.statusText}`);
    return res.json();
  }
}

/**
 * Factory for a swaps API client.
 * @param {string} [baseUrl='http://localhost:8000'] - Server root URL
 * @returns {LitSwapsApiClient}
 */
export function createSwapsClient(baseUrl = 'http://localhost:8000') {
  return new LitSwapsApiClient({ baseUrl });
}

export default LitSwapsApiClient;
