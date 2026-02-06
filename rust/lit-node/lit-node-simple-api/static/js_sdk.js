/**
 * Lit Node Simple API - JavaScript SDK
 *
 * Wrapper for the v1 API endpoints defined in lit-node-simple-api.
 * Types match core/v1/models (request.rs, response.rs) and core/v1/endpoints.rs.
 */

// --- Request types (match core/v1/models/request.rs) ---

/**
 * @typedef {Object} SignWithPkpOptions
 * @property {string} apiKey - Hex-encoded API key (from getApiKey)
 * @property {string} pkpPublicKey - PKP public key
 * @property {string} message - Message to sign
 */

/**
 * @typedef {Object} LitActionOptions
 * @property {string} apiKey - Hex-encoded API key (from getApiKey)
 * @property {string} code - Lit action JavaScript code
 * @property {Object} [jsParams] - Optional JSON params passed to the lit action
 */

/**
 * @typedef {Object} EncryptOptions
 * @property {string} apiKey - Hex-encoded API key (from getApiKey)
 * @property {string} message - Plaintext message to encrypt
 */

/**
 * @typedef {Object} DecryptOptions
 * @property {string} apiKey - Hex-encoded API key (from getApiKey)
 * @property {string[]} shares - Array of hex-encoded decryption share strings
 * @property {string} ciphertext - Base64-encoded ciphertext (from encrypt)
 * @property {string} dataToEncryptHash - Hex-encoded SHA-256 hash of the original plaintext
 */

/**
 * @typedef {Object} CombineSignatureSharesOptions
 * @property {string} apiKey - Hex-encoded API key (from getApiKey)
 * @property {JsonPKPSigningResponse[]} shares - Array of signing response objects (from signWithPkp response.shares)
 */

/**
 * Single PKP signing response (lit_node_core::response::JsonPKPSigningResponse; camelCase in JSON).
 * @typedef {Object} JsonPKPSigningResponse
 * @property {boolean} success - Whether the signing succeeded
 * @property {number[]|Uint8Array} signedData - Signed data bytes
 * @property {Object} signatureShare - SignableOutput (e.g. EcdsaSignedMessageShare with signature_share)
 */

// --- Response types (match core/v1/models/response.rs) ---

/**
 * @typedef {Object} GetApiKeyResponse
 * @property {string} api_key - Hex-encoded API key
 * @property {string} wallet_address - Wallet address for the API key
 */

/**
 * @typedef {Object} HandshakeResponse
 * @property {string[]} responses - Handshake responses from validators
 */

/**
 * @typedef {Object} MintPkpResponse
 * @property {string} pkp_public_key - Minted PKP public key
 */

/**
 * @typedef {Object} SignWithPkpResponse
 * @property {JsonPKPSigningResponse[]} shares - Array of PKP signing responses (pass to combineSignatureShares)
 * @property {string} curve_type - Curve type used for signing (e.g. EcdsaK256Sha256)
 */

/**
 * @typedef {Object} SignWithPkpResponseItem - Single signing result within a lit action
 * @property {JsonPKPSigningResponse[]} shares - Signature share objects
 * @property {string} curve_type - Curve type
 */

/**
 * @typedef {Object} LitActionResponse - Single lit action execution result
 * @property {SignWithPkpResponseItem[]} signatures - Signing results from the action
 * @property {string} response - Action response payload
 * @property {string} logs - Action logs
 */

/**
 * @typedef {Object} LitActionResponses - Top-level lit_action endpoint response
 * @property {LitActionResponse[]} responses - One entry per execution (e.g. per node)
 */

/**
 * @typedef {Object} EncryptResponse
 * @property {string} ciphertext - Base64-encoded ciphertext
 * @property {string} data_to_encrypt_hash - Hex-encoded SHA-256 hash of the plaintext (use for decrypt)
 */

/**
 * @typedef {Object} DecryptResponse
 * @property {string} decrypted_text - Decrypted plaintext
 */

/**
 * @typedef {Object} CombineSignatureSharesResponse
 * @property {string} signature - Hex-encoded combined signature
 * @property {string} r - ECDSA r component (hex)
 * @property {string} s - ECDSA s component (hex)
 * @property {number} v - ECDSA v component
 * @property {number} recovery_id - Recovery id byte
 */

// --- Transfer (abstractions/transfer); mount transfer routes to use ---

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

/**
 * @typedef {Object} GetBalanceResponse
 * @property {string} address - Wallet address
 * @property {string} balance - Balance as string
 * @property {string} chain - Chain identifier
 * @property {string} symbol - Asset symbol
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
   * @returns {Promise<SignWithPkpResponse>} { shares, curve_type }
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
   * @returns {Promise<LitActionResponses>} { responses: LitActionResponse[] }
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

  /**
   * POST /encrypt
   * Encrypts a message (time-lock encryption) for the wallet identified by the API key.
   * @param {EncryptOptions} options
   * @returns {Promise<EncryptResponse>} { ciphertext, data_to_encrypt_hash }
   */
  async encrypt({ apiKey, message }) {
    const body = { api_key: apiKey, message };
    const res = await fetch(`${this.baseUrl}/encrypt`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`encrypt failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * POST /decrypt
   * Decrypts ciphertext using decryption shares (from network/nodes).
   * @param {DecryptOptions} options
   * @returns {Promise<DecryptResponse>} { decrypted_text }
   */
  async decrypt({ apiKey, shares, ciphertext, dataToEncryptHash }) {
    const body = {
      api_key: apiKey,
      shares,
      ciphertext,
      data_to_encrypt_hash: dataToEncryptHash,
    };
    const res = await fetch(`${this.baseUrl}/decrypt`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`decrypt failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * POST /combine_signature_shares
   * Combines signature shares (pass signWithPkp response.shares as-is).
   * @param {CombineSignatureSharesOptions} options
   * @returns {Promise<CombineSignatureSharesResponse>} { signature, r, s, v, recovery_id }
   */
  async combineSignatureShares({ apiKey, shares }) {
    const body = { api_key: apiKey, shares };
    const res = await fetch(`${this.baseUrl}/combine_signature_shares`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`combine_signature_shares failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * GET /get_balance/<api_key>/<chain>
   * Gets balance for the wallet identified by the API key on the given chain.
   * Requires transfer routes to be mounted.
   * @param {string} apiKey - Hex-encoded API key (from getApiKey)
   * @param {string} chain - Chain identifier (e.g. "Ethereum", "Solana")
   * @returns {Promise<GetBalanceResponse>} { address, balance, chain, symbol }
   */
  async getBalance(apiKey, chain) {
    const res = await fetch(
      `${this.baseUrl}/get_balance/${encodeURIComponent(apiKey)}/${encodeURIComponent(chain)}`
    );
    if (!res.ok) throw new Error(`get_balance failed: ${res.status} ${res.statusText}`);
    return res.json();
  }

  /**
   * POST /send
   * Sends funds to a destination address on a chain (PKP-signed). Requires transfer routes to be mounted.
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
 * Factory for a default client (e.g. for script usage).
 * @param {string} [baseUrl='http://localhost:8000']
 * @returns {LitNodeSimpleApiClient}
 */
export function createClient(baseUrl = 'http://localhost:8000') {
  return new LitNodeSimpleApiClient({ baseUrl });
}

export default LitNodeSimpleApiClient;
