/**
 * Lit Node Simple API - JavaScript Transfer SDK
 *
 * Wrapper for transfer endpoints in abstractions/transfer/endpoints.rs.
 * Routes are mounted at /transfer/v1/ (see src/main.rs).
 */

/**
 * @typedef {Object} GetBalanceResponse
 * @property {string} address - Wallet address
 * @property {string} balance - Balance as string
 * @property {string} chain - Chain key (lowercase, e.g. "ethereum", "solana")
 * @property {string} symbol - Asset symbol
 */

/**
 * @typedef {Object} TransferOptions
 * @property {string} apiKey - Hex-encoded API key (from getApiKey)
 * @property {string} pkpPublicKey - PKP public key
 * @property {string} chain - Chain key (lowercase, e.g. "ethereum", "solana"); use ChainInfoItem.chain from get_chains
 * @property {string} destinationAddress - Destination address
 * @property {string} amount - Amount as string
 */

/**
 * @typedef {Object} TransferResponse
 * @property {string} txn_id - Transaction id
 * @property {boolean} success - Whether the transfer succeeded
 * @property {string} chain - Chain key (lowercase)
 * @property {string} origin_symbol - Symbol of the asset sent
 * @property {string} origin_amount - Amount sent
 * @property {string} gas - Gas used/cost
 * @property {string} timestamp - Timestamp
 * @property {string} destination_address - Destination address
 */

/**
 * @typedef {Object} GetChainsOptions
 * @property {boolean} [isEvm=true] - If true return EVM chains; if false return non-EVM chains
 * @property {boolean} [isTestnet=false] - If true (and isEvm) return testnet EVM chains only; ignored when isEvm is false
 */

/**
 * @typedef {Object} ChainInfoItem
 * @property {string} chain - Chain key (lowercase identifier for API calls, e.g. "ethereum", "bnbsmartchain")
 * @property {string} display_name - Human-readable chain name for UI
 * @property {string} token - Asset/token symbol
 */

/**
 * @typedef {Object} GetChainsResponse
 * @property {ChainInfoItem[]} chains - List of supported chains with chain, display_name, and token
 */

export class LitTransferApiClient {
  /**
   * @param {Object} options
   * @param {string} [options.baseUrl='http://localhost:8000'] - Base URL of the API
   */
  constructor({ baseUrl = 'http://localhost:8000' } = {}) {
    const base = baseUrl.replace(/\/$/, '');
    this.baseUrl = `${base}/transfer/v1`;
  }

  /**
   * GET /transfer/v1/get_api_key_balance/<api_key>/<chain>
   * Gets balance for the wallet identified by the API key on the given chain.
   * @param {string} apiKey - Hex-encoded API key (from getApiKey)
   * @param {string} chain - Chain key (lowercase, e.g. "ethereum", "yellowstone"); use ChainInfoItem.chain from get_chains
   * @returns {Promise<GetBalanceResponse>} { address, balance, chain, symbol }
   */
  async getApiKeyBalance(apiKey, chain) {
    const res = await fetch(
      `${this.baseUrl}/get_api_key_balance/${encodeURIComponent(apiKey)}/${encodeURIComponent(chain)}`
    );
    if (!res.ok) throw new Error(`get_api_key_balance failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * GET /transfer/v1/get_pkp_balance/<pkp_public_key>/<chain>
   * Gets balance for the PKP (programmable key pair) address on the given chain.
   * @param {string} pkpPublicKey - PKP public key
   * @param {string} chain - Chain key (lowercase, e.g. "ethereum", "solana"); use ChainInfoItem.chain from get_chains
   * @returns {Promise<GetBalanceResponse>} { address, balance, chain, symbol }
   */
  async getPkpBalance(pkpPublicKey, chain) {
    const res = await fetch(
      `${this.baseUrl}/get_pkp_balance/${encodeURIComponent(pkpPublicKey)}/${encodeURIComponent(chain)}`
    );
    if (!res.ok) throw new Error(`get_pkp_balance failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * GET /transfer/v1/get_address_balance/<address>/<chain>
   * Gets balance for an arbitrary address on the given chain.
   * @param {string} address - Wallet or contract address (e.g. 0x... for EVM)
   * @param {string} chain - Chain key (lowercase, e.g. "ethereum", "solana"); use ChainInfoItem.chain from get_chains
   * @returns {Promise<GetBalanceResponse>} { address, balance, chain, symbol }
   */
  async getAddressBalance(address, chain) {
    const res = await fetch(
      `${this.baseUrl}/get_address_balance/${encodeURIComponent(address)}/${encodeURIComponent(chain)}`
    );
    if (!res.ok) throw new Error(`get_address_balance failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * GET /transfer/v1/get_chains?is_evm=&is_testnet=
   * Returns the list of supported chains (EVM, non-EVM, or testnet EVM) with chain key, display_name, and token.
   * Use chain for API calls (getPkpBalance, send, etc.); use display_name for UI labels.
   * @param {GetChainsOptions} [options] - { isEvm, isTestnet }; default { isEvm: true, isTestnet: false }
   * @returns {Promise<GetChainsResponse>} { chains: { chain, display_name, token }[] }
   */
  async getAllChains(options = {}) {
    const { isEvm = true, isTestnet = false } = options;
    const params = new URLSearchParams({
      is_evm: String(isEvm),
      is_testnet: String(isTestnet),
    });
    const url = `${this.baseUrl}/get_chains?${params.toString()}`;
    const res = await fetch(url, { method: 'GET' });
    if (!res.ok) throw new Error(`get_chains failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * POST /transfer/v1/send
   * Sends funds to a destination address on a chain (PKP-signed).
   * @param {TransferOptions} options
   * @returns {Promise<TransferResponse>}
   */
  async send({ apiKey, pkpPublicKey, chain, destinationAddress, amount }) {
    const body = {
      api_key: apiKey,
      pkp_public_key: pkpPublicKey,
      chain,
      destination_address: destinationAddress,
      amount,
    };
    const res = await fetch(`${this.baseUrl}/send`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`send failed: ${res.status} ${res.statusText}`);
    return res.json();
  }
}

/**
 * Factory for a transfer client.
 * @param {string} [baseUrl='http://localhost:8000']
 * @returns {LitTransferApiClient}
 */
export function createTransferClient(baseUrl = 'http://localhost:8000') {
  return new LitTransferApiClient({ baseUrl });
}

export default LitTransferApiClient;
