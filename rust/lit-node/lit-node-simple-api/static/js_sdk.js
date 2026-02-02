/**
 * Lit Node Simple API - JavaScript SDK
 *
 * Wrapper for the v1 API endpoints defined in lit-node-simple-api.
 * @see lit-node-simple-api/src/base/v1.rs
 */

/**
 * @typedef {Object} SignWithPkpOptions
 * @property {string} apiKey - Hex-encoded API key (from getApiKey)
 * @property {string} pkpPublicKey - PKP public key
 * @property {string} message - Message to sign
 */

/**
 * @typedef {Object} SignWithPkpResponse
 * @property {Array} endpoint_responses - Signing endpoint responses
 */

/**
 * @typedef {Object} GetApiKeyResponse
 * @property {string} api_key - Hex-encoded API key
 */

/**
 * @typedef {Object} HandshakeResponse
 * @property {Array} responses - Handshake responses from validators
 */

/**
 * @typedef {Object} MintPkpResponse
 * @property {string} pkp_public_key - Minted PKP public key
 */

/**
 * @typedef {Object} LitActionOptions
 * @property {string} apiKey - Hex-encoded API key (from getApiKey)
 * @property {string} code - Lit action JavaScript code
 * @property {Object} [jsParams] - Optional JSON params passed to the lit action
 */

/**
 * @typedef {Object} LitActionResponse
 * @property {*} execute_resp - Lit action execution response
 */

export class LitNodeSimpleApiClient {
  /**
   * @param {Object} options
   * @param {string} [options.baseUrl='http://localhost:8000'] - Base URL of the API
   */
  constructor({ baseUrl = 'http://localhost:8000' } = {}) {
    this.baseUrl = baseUrl.replace(/\/$/, '');
  }

  /**
   * GET /get_api_key
   * Generates and returns a new API key (hex-encoded wallet secret).
   * @returns {Promise<GetApiKeyResponse>}
   */
  async getApiKey() {
    const res = await fetch(`${this.baseUrl}/get_api_key`);
    if (!res.ok) throw new Error(`get_api_key failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * GET /handshake
   * Performs handshake with validators and returns their responses.
   * @returns {Promise<HandshakeResponse>}
   */
  async handshake() {
    const res = await fetch(`${this.baseUrl}/handshake`);
    if (!res.ok) throw new Error(`handshake failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * GET /mint_pkp/<api_key>
   * Mints a new PKP for the wallet identified by the API key.
   * @param {string} apiKey - Hex-encoded API key (from getApiKey)
   * @returns {Promise<MintPkpResponse>}
   */
  async mintPkp(apiKey) {
    const res = await fetch(`${this.baseUrl}/mint_pkp/${encodeURIComponent(apiKey)}`);
    if (!res.ok) throw new Error(`mint_pkp failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * POST /sign_with_pkp
   * Signs a message with the given PKP using the provided API key.
   * @param {SignWithPkpOptions} options
   * @returns {Promise<SignWithPkpResponse>}
   */
  async signWithPkp({ apiKey, pkpPublicKey, message }) {
    const body = {
      api_key: apiKey,
      pkp_public_key: pkpPublicKey,
      message,
    };
    const res = await fetch(`${this.baseUrl}/sign_with_pkp`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`sign_with_pkp failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * POST /lit_action
   * Executes a lit action with the given code and optional params.
   * @param {LitActionOptions} options
   * @returns {Promise<LitActionResponse>}
   */
  async litAction({ apiKey, code, jsParams }) {
    const body = {
      api_key: apiKey,
      code,
      js_params: jsParams ?? null,
    };
    const res = await fetch(`${this.baseUrl}/lit_action`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`lit_action failed: ${res.status} ${res.statusText}`);
    return res.json();
  }
}

/**
 * Factory for a default client (e.g. for script usage).
 * @param {string} [baseUrl='http://localhost:8000']
 * @returns {LitNodeSimpleApiClient}
 */
export function createClient(baseUrl = 'http://localhost:8000') {
  return new LitNodeSimpleApiClient({ baseUrl });
}

export default LitNodeSimpleApiClient;
