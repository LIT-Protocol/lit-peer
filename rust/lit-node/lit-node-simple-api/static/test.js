/**
 * Simple test: create API key → mint PKP → sign "hello world" with that PKP → execute lit action.
 * - Node: node test.js (API_BASE_URL env optional). Ensure the simple API server is running.
 * - Browser: load from index.html; set window.LIT_SIMPLE_API_BASE_URL or use the page input.
 */

import { createClient } from './js_sdk.js';

function getBaseUrl() {
  if (typeof process !== 'undefined' && process.env && process.env.API_BASE_URL) return process.env.API_BASE_URL;
  if (typeof window !== 'undefined' && window.LIT_SIMPLE_API_BASE_URL) return window.LIT_SIMPLE_API_BASE_URL;
  return 'http://localhost:8000';
}

/**
 * Run the test flow. Exported for use from index.html.
 * @param {string} [baseUrl] - Override base URL (default: getBaseUrl())
 * @returns {Promise<void>}
 */
export async function runTests(baseUrl = getBaseUrl()) {
  const client = createClient(baseUrl);

  console.log('1. Getting API key...');
  const { api_key } = await client.getApiKey();
  console.log('   api_key:', api_key.slice(0, 18) + '...');

  console.log('2. Minting PKP...');
  const { pkp_public_key } = await client.mintPkp(api_key);
  console.log('   pkp_public_key:', pkp_public_key);

  console.log('3. Signing "hello world" with PKP...');
  const { endpoint_responses } = await client.signWithPkp({
    apiKey: api_key,
    pkpPublicKey: pkp_public_key,
    message: 'hello world',
  });
  console.log('   endpoint_responses count:', endpoint_responses?.length ?? 0);
  if (endpoint_responses?.length) {
    console.log('   first response keys:', Object.keys(endpoint_responses[0]));
  }

  console.log('4. Executing lit action...');
  const litActionCode = `
    const go = async () => {
      Lit.Actions.setResponse({ response: JSON.stringify("Hello from lit action!") });
    };
    go();
  `;
  const { execute_resp } = await client.litAction({
    apiKey: api_key,
    code: litActionCode,
    jsParams: { testParam: 'hello' },
  });
  console.log('   execute_resp:', JSON.stringify(execute_resp, null, 2));

  // console.log('5. Encrypt...');
  // const plaintext = 'secret message for encryption test';
  // const { ciphertext } = await client.encrypt({ apiKey: api_key, message: plaintext });
  // console.log('   ciphertext (first 60 chars):', (ciphertext || '').slice(0, 60) + '...');

  console.log('6. Combine signature shares...');
  const shares = (endpoint_responses || []).filter((s) => s && s.length > 0);
  if (shares.length > 0) {
    const { signature, recovery_id } = await client.combineSignatureShares({ apiKey: api_key, shares });
    console.log('   signature (first 40 chars):', (signature || '').slice(0, 40) + '...');
    console.log('   recovery_id:', recovery_id);
  } else {
    console.log('   (no signature shares in endpoint_responses; skipping combine)');
  }

  console.log('Done.');
}

// Run when executed directly (e.g. node test.js)
if (typeof window === 'undefined') {
  runTests().catch((err) => {
    console.error('Test failed:', err.message);
    process.exit(1);
  });
}
