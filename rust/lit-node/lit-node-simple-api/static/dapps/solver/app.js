/**
 * Solver dapp – uses js_swaps_sdk for open swap requests, open quotes, and fill quote.
 * Stylistic reference: SWPS app. 4 pages: Account, Swap requests, Quotes, Fulfill.
 */

const STORAGE_KEY = 'solver_account';

let swapsClient = null;
let state = {
  baseUrl: 'http://localhost:8000',
  solverAddress: null,
};

function getBaseUrl() {
  return (document.getElementById('baseUrl')?.value || 'http://localhost:8000').trim().replace(/\/$/, '');
}

function initClients() {
  const baseUrl = getBaseUrl();
  if (state.baseUrl !== baseUrl) {
    state.baseUrl = baseUrl;
    swapsClient = null;
  }
}

async function getSwapsClient() {
  initClients();
  if (!swapsClient) {
    const { createSwapsClient } = await import('../../js_swaps_sdk.js');
    swapsClient = createSwapsClient(state.baseUrl);
  }
  return swapsClient;
}

function formatAddress(str) {
  if (!str || typeof str !== 'string') return '–';
  const s = String(str).trim();
  if (s.length <= 16) return s;
  return `${s.slice(0, 8)}...${s.slice(-8)}`;
}

// --- Routing ---
function getPageFromHash() {
  const hash = (window.location.hash || '#account').slice(1);
  return hash === '' ? 'account' : hash;
}

function setPage(pageId) {
  document.querySelectorAll('.page').forEach((p) => p.classList.remove('active'));
  document.querySelectorAll('.nav a').forEach((a) => a.classList.remove('active'));
  const page = document.getElementById(`page-${pageId}`);
  const link = document.querySelector(`.nav a[data-page="${pageId}"]`);
  if (page) page.classList.add('active');
  if (link) link.classList.add('active');
  window.location.hash = pageId;
  if (pageId === 'swap-requests') loadSwapRequests();
  if (pageId === 'quotes') loadQuotes();
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

// --- Account ---
function showAccountStatus(msg, type = 'info') {
  const el = document.getElementById('account-status');
  if (!el) return;
  el.textContent = msg;
  el.className = `status ${type}`;
  el.style.display = 'block';
}

function showAccountLoaded() {
  const actions = document.getElementById('account-actions');
  const loaded = document.getElementById('account-loaded');
  const display = document.getElementById('account-address-display');
  if (actions) actions.style.display = 'none';
  if (loaded) loaded.style.display = 'block';
  if (display) display.textContent = formatAddress(state.solverAddress);
  updateLogoutVisibility();
  const fulfillRefund = document.getElementById('fulfill-provider-refund');
  if (fulfillRefund) fulfillRefund.placeholder = 'Uses solver address from Account if empty';
}

function clearAccountUI() {
  const actions = document.getElementById('account-actions');
  const loaded = document.getElementById('account-loaded');
  const input = document.getElementById('account-wallet');
  if (actions) actions.style.display = 'block';
  if (loaded) loaded.style.display = 'none';
  if (input) input.value = '';
  updateLogoutVisibility();
  const fulfillRefund = document.getElementById('fulfill-provider-refund');
  if (fulfillRefund) fulfillRefund.placeholder = '0x...';
}

function saveAccountToStorage() {
  if (state.solverAddress) {
    localStorage.setItem(STORAGE_KEY, JSON.stringify({ solverAddress: state.solverAddress }));
  } else {
    localStorage.removeItem(STORAGE_KEY);
  }
}

function loadAccountFromStorage() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return;
    const data = JSON.parse(raw);
    if (data.solverAddress) {
      state.solverAddress = data.solverAddress;
      showAccountLoaded();
    }
  } catch (_) {}
}

function saveAccount() {
  const input = document.getElementById('account-wallet');
  const addr = input?.value?.trim();
  if (!addr) {
    showAccountStatus('Enter a wallet address.', 'error');
    return;
  }
  state.solverAddress = addr;
  showAccountLoaded();
  showAccountStatus('Address saved. Use it as provider refund when fulfilling.', 'success');
  saveAccountToStorage();
}

function clearAccount() {
  state.solverAddress = null;
  clearAccountUI();
  saveAccountToStorage();
  showAccountStatus('Cleared. Enter a new address to continue.', 'info');
}

function updateLogoutVisibility() {
  const btn = document.getElementById('btn-logout');
  if (btn) btn.style.display = state.solverAddress ? 'inline-flex' : 'none';
}

function logout() {
  clearAccount();
}

// --- Swap requests list ---
function showSwapRequestsStatus(msg, type = 'info') {
  const el = document.getElementById('swap-requests-status');
  if (!el) return;
  el.textContent = msg;
  el.className = `status ${type}`;
  el.style.display = msg ? 'block' : 'none';
}

function renderSwapRequest(id, sr) {
  const li = document.createElement('li');
  li.className = 'list-card';
  const from = sr.from || '–';
  const origin = `${sr.origin_amount ?? '–'} ${sr.origin_symbol ?? ''} on ${sr.origin_chain ?? ''}`;
  const dest = `${sr.destination_amount ?? '–'} ${sr.destination_symbol ?? ''} on ${sr.destination_chain ?? ''}`;
  li.innerHTML = `
    <div class="list-card-title">Swap request #${id}</div>
    <div class="list-card-row"><span class="key">From</span><span class="val">${formatAddress(from)}</span></div>
    <div class="list-card-row"><span class="key">Origin</span><span class="val">${origin}</span></div>
    <div class="list-card-row"><span class="key">Destination</span><span class="val">${dest}</span></div>
    <div class="list-card-row"><span class="key">Slippage</span><span class="val">${sr.slippage ?? '–'}</span></div>
    <button type="button" class="btn btn-secondary btn-block btn-fulfill-sr" data-id="${id}" style="margin-top: 0.5rem;">Fulfill this</button>
  `;
  return li;
}

async function loadSwapRequests() {
  const listEl = document.getElementById('swap-requests-list');
  const emptyEl = document.getElementById('swap-requests-empty');
  if (!listEl || !emptyEl) return;
  listEl.innerHTML = '';
  emptyEl.textContent = 'Loading…';
  emptyEl.style.display = 'block';
  showSwapRequestsStatus('');
  try {
    const client = await getSwapsClient();
    const res = await client.getOpenSwapRequests();
    const list = res.swap_requests || [];
    if (list.length === 0) {
      emptyEl.textContent = 'No open swap requests.';
      emptyEl.style.display = 'block';
      return;
    }
    emptyEl.style.display = 'none';
    list.forEach((sr, i) => {
      const id = i + 1;
      listEl.appendChild(renderSwapRequest(id, sr));
    });
    listEl.querySelectorAll('.btn-fulfill-sr').forEach((btn) => {
      btn.addEventListener('click', () => {
        const id = btn.getAttribute('data-id');
        document.getElementById('fulfill-swap-request-id').value = id;
        setPage('fulfill');
      });
    });
  } catch (err) {
    emptyEl.textContent = '';
    emptyEl.style.display = 'none';
    showSwapRequestsStatus(err.message || String(err), 'error');
  }
}

// --- Quotes list ---
function showQuotesStatus(msg, type = 'info') {
  const el = document.getElementById('quotes-status');
  if (!el) return;
  el.textContent = msg;
  el.className = `status ${type}`;
  el.style.display = msg ? 'block' : 'none';
}

function renderQuote(idx, q) {
  const li = document.createElement('li');
  li.className = 'list-card';
  const expiry = q.quote_expiry ? new Date(q.quote_expiry * 1000).toISOString() : '–';
  li.innerHTML = `
    <div class="list-card-title">Quote #${idx}</div>
    <div class="list-card-row"><span class="key">PKP</span><span class="val">${formatAddress(q.pkp_address)}</span></div>
    <div class="list-card-row"><span class="key">Swap request ID</span><span class="val">${q.swap_request_id ?? '–'}</span></div>
    <div class="list-card-row"><span class="key">Provider refund</span><span class="val">${formatAddress(q.provider_refund_address)}</span></div>
    <div class="list-card-row"><span class="key">Quote expiry</span><span class="val">${expiry}</span></div>
    <div class="list-card-row"><span class="key">Fees total</span><span class="val">${q.fees_total ?? '0'}</span></div>
  `;
  return li;
}

async function loadQuotes() {
  const listEl = document.getElementById('quotes-list');
  const emptyEl = document.getElementById('quotes-empty');
  if (!listEl || !emptyEl) return;
  listEl.innerHTML = '';
  emptyEl.textContent = 'Loading…';
  emptyEl.style.display = 'block';
  showQuotesStatus('');
  try {
    const client = await getSwapsClient();
    const res = await client.getOpenQuotes();
    const list = res.quotes || [];
    if (list.length === 0) {
      emptyEl.textContent = 'No open quotes.';
      emptyEl.style.display = 'block';
      return;
    }
    emptyEl.style.display = 'none';
    list.forEach((q, i) => {
      listEl.appendChild(renderQuote(i + 1, q));
    });
  } catch (err) {
    emptyEl.textContent = '';
    emptyEl.style.display = 'none';
    showQuotesStatus(err.message || String(err), 'error');
  }
}

// --- Fulfill (fill quote) ---
function showFulfillStatus(msg, type = 'info') {
  const el = document.getElementById('fulfill-status');
  if (!el) return;
  el.textContent = msg;
  el.className = `status ${type}`;
  el.style.display = msg ? 'block' : 'none';
}

async function submitFulfill(e) {
  e.preventDefault();
  const swapRequestId = document.getElementById('fulfill-swap-request-id')?.value?.trim();
  const quoteDeadline = parseInt(document.getElementById('fulfill-quote-deadline')?.value || '60', 10);
  let providerRefund = document.getElementById('fulfill-provider-refund')?.value?.trim();
  const message = document.getElementById('fulfill-message')?.value?.trim() || undefined;
  if (!swapRequestId) {
    showFulfillStatus('Enter a swap request ID.', 'error');
    return;
  }
  if (!providerRefund && state.solverAddress) providerRefund = state.solverAddress;
  if (!providerRefund) {
    showFulfillStatus('Enter provider refund address or sign in on Account with your solver address.', 'error');
    return;
  }
  const btn = document.getElementById('btn-fulfill');
  btn.disabled = true;
  showFulfillStatus('Creating quote…', 'info');
  try {
    const client = await getSwapsClient();
    const result = await client.fillQuote({
      swapRequestId,
      quoteDeadlineSeconds: quoteDeadline,
      providerRefundAddress: providerRefund,
      message,
    });
    showFulfillStatus(
      `Quote created. Quote ID: ${result.quote_id || 'N/A'}. Tx: ${result.transaction_hash || 'N/A'}`,
      'success'
    );
    loadQuotes();
  } catch (err) {
    showFulfillStatus(err.message || String(err), 'error');
  } finally {
    btn.disabled = false;
  }
}

// --- Init ---
function init() {
  initRouting();
  loadAccountFromStorage();

  document.getElementById('baseUrl')?.addEventListener('change', () => {
    initClients();
  });

  document.getElementById('btn-logout')?.addEventListener('click', logout);
  document.getElementById('btn-save-account')?.addEventListener('click', saveAccount);
  document.getElementById('btn-clear-account')?.addEventListener('click', clearAccount);
  document.getElementById('btn-copy-account')?.addEventListener('click', () => {
    if (state.solverAddress) navigator.clipboard.writeText(state.solverAddress);
  });

  document.getElementById('btn-refresh-swap-requests')?.addEventListener('click', loadSwapRequests);
  document.getElementById('btn-refresh-quotes')?.addEventListener('click', loadQuotes);

  document.getElementById('fulfill-form')?.addEventListener('submit', submitFulfill);
}

init();
