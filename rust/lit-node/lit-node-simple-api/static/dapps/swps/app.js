/**
 * Swps dapp – interfaces with js_core_sdk and js_transfer_sdk.
 * Lit Protocol styling; 4 pages: Overview, Transfer, Swap, History.
 */

const HISTORY_KEY = 'swps_tx_history';

let coreClient = null;
let transferClient = null;
let state = {
  baseUrl: 'http://localhost:8000',
  apiKey: null,
  pkpPublicKey: null,
  walletAddress: null,
  chainList: [], // { name, token }[] from getAllChains({ isEvm: true, isTestnet: true })
};

function getBaseUrl() {
  return (document.getElementById('baseUrl').value || 'http://localhost:8000').trim().replace(/\/$/, '');
}

function initClients() {
  const baseUrl = getBaseUrl();
  if (state.baseUrl !== baseUrl) {
    state.baseUrl = baseUrl;
    coreClient = null;
    transferClient = null;
  }
}

async function getCoreClient() {
  initClients();
  if (!coreClient) {
    const { createClient } = await import('../../js_core_sdk.js');
    coreClient = createClient(state.baseUrl);
  }
  return coreClient;
}

async function getTransferClient() {
  initClients();
  if (!transferClient) {
    const { createTransferClient } = await import('../../js_transfer_sdk.js');
    transferClient = createTransferClient(state.baseUrl);
  }
  return transferClient;
}

// --- Routing ---
function getPageFromHash() {
  const hash = (window.location.hash || '#overview').slice(1);
  return hash === '' ? 'overview' : hash;
}

function setPage(pageId) {
  document.querySelectorAll('.page').forEach((p) => p.classList.remove('active'));
  document.querySelectorAll('.nav a').forEach((a) => a.classList.remove('active'));
  const page = document.getElementById(`page-${pageId}`);
  const link = document.querySelector(`.nav a[data-page="${pageId}"]`);
  if (page) page.classList.add('active');
  if (link) link.classList.add('active');
  window.location.hash = pageId;
}

function initRouting() {
  function applyRoute() {
    setPage(getPageFromHash());
  }
  window.addEventListener('hashchange', applyRoute);
  applyRoute();
  document.querySelectorAll('.nav a').forEach((a) => {
    a.addEventListener('click', (e) => {
      e.preventDefault();
      setPage(a.getAttribute('data-page'));
    });
  });
}

// --- Chains (getAllChains: isEvm=true, isTestnet=true) ---
async function loadChains() {
  try {
    const client = await getTransferClient();
    const res = await client.getAllChains({ isEvm: true, isTestnet: true });
    state.chainList = res.chains || [];
    populateChainSelect('overview-chain', state.chainList);
    populateChainSelect('transfer-chain', state.chainList);
  } catch (err) {
    console.error('loadChains failed:', err);
    state.chainList = [];
    populateChainSelect('overview-chain', []);
    populateChainSelect('transfer-chain', []);
  }
}

function populateChainSelect(selectId, chains) {
  const sel = document.getElementById(selectId);
  if (!sel) return;
  sel.innerHTML = '';
  if (!chains.length) {
    const opt = document.createElement('option');
    opt.value = '';
    opt.textContent = 'No chains loaded';
    sel.appendChild(opt);
    return;
  }
  chains.forEach((c) => {
    const opt = document.createElement('option');
    opt.value = c.name;
    opt.textContent = c.name;
    sel.appendChild(opt);
  });
}

// --- Overview ---
function showOverviewStatus(msg, type = 'info') {
  const el = document.getElementById('overview-status');
  el.textContent = msg;
  el.className = `status ${type}`;
  el.style.display = 'block';
}

function showOverviewAccount() {
  document.getElementById('overview-actions').style.display = 'none';
  const wrap = document.getElementById('overview-account');
  wrap.style.display = 'block';
  document.getElementById('overview-wallet').textContent = state.walletAddress || '–';
  document.getElementById('overview-pkp').textContent = state.pkpPublicKey
    ? `${state.pkpPublicKey.slice(0, 12)}...${state.pkpPublicKey.slice(-8)}`
    : '–';
  document.getElementById('overview-chain-wrap').style.display = 'block';
  const legend = document.getElementById('overview-balance-legend');
  if (legend) legend.style.display = 'block';
}

async function createAccount() {
  const btn = document.getElementById('btn-create-account');
  btn.disabled = true;
  showOverviewStatus('Creating account…', 'info');
  try {
    const client = await getCoreClient();
    const { api_key, wallet_address } = await client.getApiKey();
    const { pkp_public_key } = await client.mintPkp(api_key);
    state.apiKey = api_key;
    state.pkpPublicKey = pkp_public_key;
    state.walletAddress = wallet_address;
    showOverviewAccount();
    showOverviewStatus('Account created. You can refresh balances below.', 'success');
    await refreshBalances();
  } catch (err) {
    showOverviewStatus(err.message || String(err), 'error');
  } finally {
    btn.disabled = false;
  }
}

const WALLET_BALANCE_CHAIN = 'Yellowstone'; // Wallet balance always shown for Yellowstone (Litkey)

async function refreshBalances() {
  const chainEl = document.getElementById('overview-chain');
  const chain = (chainEl && chainEl.value ? chainEl.value : (state.chainList[0]?.name || '')).trim();
  if (!chain) return;
  const wrap = document.getElementById('overview-balances');
  if (!state.apiKey && !state.pkpPublicKey) {
    wrap.innerHTML = '<p class="history-empty">Load an account first.</p>';
    return;
  }
  wrap.innerHTML = '<p class="history-empty">Loading…</p>';
  try {
    const client = await getTransferClient();
    const list = document.createElement('ul');
    list.className = 'balance-list';
    if (state.apiKey) {
      const bal = await client.getApiKeyBalance(state.apiKey, WALLET_BALANCE_CHAIN);
      const li = document.createElement('li');
      li.innerHTML = `<span class="balance-symbol">API Key balance (Litkey - ${WALLET_BALANCE_CHAIN})</span><span>${bal.balance}</span>`;
      list.appendChild(li);
    }
    if (state.pkpPublicKey) {
      const bal = await client.getPkpBalance(state.pkpPublicKey, chain);
      const li = document.createElement('li');
      li.innerHTML = `<span class="balance-symbol">PKP (${bal.symbol})</span><span>${bal.balance}</span>`;
      list.appendChild(li);
    }
    wrap.innerHTML = '';
    wrap.appendChild(list);
  } catch (err) {
    wrap.innerHTML = `<p class="status error">${err.message || String(err)}</p>`;
  }
}

function loadStoredAccount() {
  try {
    const raw = localStorage.getItem('swps_account');
    if (!raw) return;
    const data = JSON.parse(raw);
    if (data.apiKey && data.pkpPublicKey) {
      state.apiKey = data.apiKey;
      state.pkpPublicKey = data.pkpPublicKey;
      state.walletAddress = data.walletAddress || null;
      showOverviewAccount();
      document.getElementById('overview-actions').style.display = 'none';
    }
  } catch (_) {}
}

function loadPastedAccount() {
  const apiKey = document.getElementById('overview-api-key').value.trim();
  const pkpKey = document.getElementById('overview-pkp-key').value.trim();
  if (!apiKey || !pkpKey) {
    showOverviewStatus('Enter both API key and PKP public key.', 'error');
    return;
  }
  state.apiKey = apiKey;
  state.pkpPublicKey = pkpKey;
  state.walletAddress = null; // not available from paste
  showOverviewAccount();
  document.getElementById('overview-actions').style.display = 'none';
  showOverviewStatus('Account loaded. Refresh balances to see data.', 'success');
  saveAccount();
}

function saveAccount() {
  if (state.apiKey && state.pkpPublicKey) {
    localStorage.setItem(
      'swps_account',
      JSON.stringify({
        apiKey: state.apiKey,
        pkpPublicKey: state.pkpPublicKey,
        walletAddress: state.walletAddress,
      })
    );
  }
}

// --- Transfer ---
function showTransferStatus(msg, type = 'info') {
  const el = document.getElementById('transfer-status');
  el.textContent = msg;
  el.className = `status ${type}`;
  el.style.display = 'block';
}

function pushHistory(record) {
  let list = [];
  try {
    const raw = sessionStorage.getItem(HISTORY_KEY);
    if (raw) list = JSON.parse(raw);
  } catch (_) {}
  list.unshift(record);
  sessionStorage.setItem(HISTORY_KEY, JSON.stringify(list.slice(0, 50)));
}

async function submitTransfer(e) {
  e.preventDefault();
  if (!state.apiKey || !state.pkpPublicKey) {
    showTransferStatus('Create an account on Overview first.', 'error');
    return;
  }
  const chainEl = document.getElementById('transfer-chain');
  const chain = chainEl ? chainEl.value.trim() : '';
  if (!chain) {
    showTransferStatus('Select a chain.', 'error');
    return;
  }
  const destination = document.getElementById('transfer-destination').value.trim();
  const amount = document.getElementById('transfer-amount').value.trim();
  const btn = document.getElementById('btn-transfer');
  btn.disabled = true;
  showTransferStatus('Sending…', 'info');
  try {
    const client = await getTransferClient();
    const result = await client.send({
      apiKey: state.apiKey,
      pkpPublicKey: state.pkpPublicKey,
      chain,
      destinationAddress: destination,
      amount,
    });
    showTransferStatus(
      result.success ? `Sent. Txn: ${result.txn_id || 'N/A'}` : `Transfer failed: ${result.txn_id || 'unknown'}`,
      result.success ? 'success' : 'error'
    );
    if (result.success) {
      pushHistory({
        txn_id: result.txn_id,
        success: result.success,
        chain: result.chain,
        origin_symbol: result.origin_symbol,
        origin_amount: result.origin_amount,
        destination_address: result.destination_address,
        from_address: state.walletAddress || null,
        timestamp: result.timestamp || new Date().toISOString(),
      });
    }
  } catch (err) {
    showTransferStatus(err.message || String(err), 'error');
  } finally {
    btn.disabled = false;
  }
}

// --- Swap (intent-style UI; no backend yet) ---
function showSwapStatus(msg, type = 'info') {
  const el = document.getElementById('swap-status');
  el.textContent = msg;
  el.className = `status ${type}`;
  el.style.display = 'block';
}

function swapFlip() {
  const from = document.getElementById('swap-token-from');
  const to = document.getElementById('swap-token-to');
  const tmp = from.value;
  from.value = to.value;
  to.value = tmp;
  document.getElementById('swap-amount-from').value = document.getElementById('swap-amount-to').value || '';
  document.getElementById('swap-amount-to').value = '';
}

function swapSubmit() {
  const from = document.getElementById('swap-token-from').value;
  const to = document.getElementById('swap-token-to').value;
  const amount = document.getElementById('swap-amount-from').value;
  if (!amount || Number(amount) <= 0) {
    showSwapStatus('Enter an amount.', 'error');
    return;
  }
  showSwapStatus(
    `Swap ${amount} ${from} → ${to} is intent-based. Connect to a swap API (e.g. NEAR Intents) to execute.`,
    'info'
  );
}

// --- History ---
function getHistoryAddress() {
  return document.getElementById('history-address').value.trim();
}

function loadHistory() {
  const address = getHistoryAddress();
  let list = [];
  try {
    const raw = sessionStorage.getItem(HISTORY_KEY);
    if (raw) list = JSON.parse(raw);
  } catch (_) {}
  const ul = document.getElementById('history-list');
  const empty = document.getElementById('history-empty');
  ul.innerHTML = '';
  const addrLower = address ? address.toLowerCase() : '';
  const forAddress = address
    ? list.filter(
        (t) =>
          (t.from_address || '').toLowerCase() === addrLower ||
          (t.destination_address || '').toLowerCase() === addrLower
      )
    : list;
  if (forAddress.length === 0) {
    empty.style.display = 'block';
    empty.textContent = address
      ? `No in-session transactions for ${address.slice(0, 10)}… Use a block explorer for full history.`
      : 'No in-session transactions. Transfers you make in this session appear here.';
    return;
  }
  empty.style.display = 'none';
  forAddress.forEach((t) => {
    const li = document.createElement('li');
    li.className = 'history-item';
    li.innerHTML = `
      <div class="txn-amount">${t.origin_amount || '–'} ${t.origin_symbol || ''} → ${(t.destination_address || '').slice(0, 10)}…</div>
      <div class="txn-id">${t.txn_id || '–'} · ${t.chain || ''} · ${t.timestamp || ''}</div>
    `;
    ul.appendChild(li);
  });
}

// --- Init ---
function init() {
  initRouting();
  loadChains();
  loadStoredAccount();
  if (state.apiKey && state.pkpPublicKey) showOverviewAccount();

  document.getElementById('baseUrl').addEventListener('change', () => {
    initClients();
    loadChains();
  });

  document.getElementById('btn-create-account').addEventListener('click', createAccount);
  document.getElementById('btn-load-account').addEventListener('click', loadPastedAccount);
  document.getElementById('btn-refresh-balance').addEventListener('click', refreshBalances);
  document.getElementById('overview-chain').addEventListener('change', refreshBalances);

  document.getElementById('transfer-form').addEventListener('submit', submitTransfer);

  document.getElementById('swap-flip').addEventListener('click', swapFlip);
  document.getElementById('btn-swap').addEventListener('click', swapSubmit);

  document.getElementById('btn-load-history').addEventListener('click', loadHistory);
  document.getElementById('btn-history-use-wallet').addEventListener('click', () => {
    if (state.walletAddress) {
      document.getElementById('history-address').value = state.walletAddress;
    }
  });
  document.getElementById('history-address').addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      loadHistory();
    }
  });

  // Persist account when we have it (e.g. after create)
  const observer = () => saveAccount();
  setInterval(observer, 2000);
}

init();
