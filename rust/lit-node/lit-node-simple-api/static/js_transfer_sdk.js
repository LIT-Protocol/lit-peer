/**
 * Lit Node Simple API - JavaScript Transfer SDK
 *
 * Wrapper for transfer endpoints in abstractions/transfer/endpoints.rs.
 * Requires transfer routes to be mounted (get_api_key_balance, get_pkp_balance, send).
 */

/**
 * @typedef {Object} GetBalanceResponse
 * @property {string} address - Wallet address
 * @property {string} balance - Balance as string
 * @property {string} chain - Chain identifier (e.g. "Ethereum", "Solana")
 * @property {string} symbol - Asset symbol
 */

/**
 * @typedef {Object} TransferOptions
 * @property {string} apiKey - Hex-encoded API key (from getApiKey)
 * @property {string} pkpPublicKey - PKP public key
 * @property {string} chain - Chain identifier (e.g. "Ethereum", "Solana")
 * @property {string} destinationAddress - Destination address
 * @property {string} amount - Amount as string
 */

/**
 * @typedef {Object} TransferResponse
 * @property {string} txn_id - Transaction id
 * @property {boolean} success - Whether the transfer succeeded
 * @property {string} chain - Chain identifier
 * @property {string} origin_symbol - Symbol of the asset sent
 * @property {string} origin_amount - Amount sent
 * @property {string} gas - Gas used/cost
 * @property {string} timestamp - Timestamp
 * @property {string} destination_address - Destination address
 */

export class LitTransferApiClient {
  /**
   * @param {Object} options
   * @param {string} [options.baseUrl='http://localhost:8000'] - Base URL of the API
   */
  constructor({ baseUrl = 'http://localhost:8000' } = {}) {
    this.baseUrl = baseUrl.replace(/\/$/, '');
  }

  /**
   * GET /get_api_key_balance/<api_key>/<chain>
   * Gets balance for the wallet identified by the API key on the given chain.
   * @param {string} apiKey - Hex-encoded API key (from getApiKey)
   * @param {string} chain - Chain identifier (e.g. "Ethereum", "Solana")
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
   * GET /get_pkp_balance/<pkp_public_key>/<chain>
   * Gets balance for the PKP (programmable key pair) address on the given chain.
   * @param {string} pkpPublicKey - PKP public key
   * @param {string} chain - Chain identifier (e.g. "Ethereum", "Solana")
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
   * POST /send
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
