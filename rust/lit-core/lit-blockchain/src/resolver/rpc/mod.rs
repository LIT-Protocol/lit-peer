use crate::error::{Result, config_err, unexpected_err};
use crate::resolver::rpc::config::{RpcConfig, RpcEntry, RpcKind};
use arc_swap::ArcSwap;
use futures::Future;
use futures::stream::FuturesUnordered;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::json;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use std::time::SystemTime;
use tracing::trace;
use url::Url;

use ethers::prelude::*;
use ethers::providers::Provider;
use futures::FutureExt;
use im::HashMap;
use lit_core::error::Unexpected;
use reqwest::Method;

pub mod config;

pub static RPC_RESOLVER: Lazy<ArcSwap<RpcResolver>> = Lazy::new(|| {
    ArcSwap::from(Arc::new(RpcResolver::load().expect("failed to load RPC resolver")))
});

static HEALTH_REQUEST_ID: AtomicUsize = AtomicUsize::new(0);

pub static ENDPOINT_MANAGER: Lazy<StandardRpcHealthcheckPoller> =
    Lazy::new(|| StandardRpcHealthcheckPoller::new(&RPC_RESOLVER, &HEALTH_REQUEST_ID));

static HTTP_CLIENT: OnceLock<scc::HashIndex<String, Arc<Provider<Http>>>> = OnceLock::new();

pub struct StandardRpcHealthcheckPoller<'a> {
    latencies: ArcSwap<im::hashmap::HashMap<RpcEntry, Latency>>,
    rpc_resolver: &'a Lazy<ArcSwap<RpcResolver>>,
    health_request_id: &'a AtomicUsize,
}

impl<'a> StandardRpcHealthcheckPoller<'a> {
    pub fn new(
        rpc_resolver: &'a Lazy<ArcSwap<RpcResolver>>, health_request_id: &'a AtomicUsize,
    ) -> Self {
        StandardRpcHealthcheckPoller {
            rpc_resolver,
            health_request_id,
            latencies: Self::load_latencies_from_rpc_resolver(rpc_resolver),
        }
    }
}

impl<'a> RpcHealthcheckPoller for StandardRpcHealthcheckPoller<'a> {
    fn get_latencies(&self) -> &ArcSwap<HashMap<RpcEntry, Latency>> {
        &self.latencies
    }
    fn get_rpc_resolver(&self) -> &Lazy<ArcSwap<RpcResolver>> {
        self.rpc_resolver
    }
    fn get_health_request_id(&self) -> &AtomicUsize {
        self.health_request_id
    }
    async fn healthcheck(
        &self, host: &str, kind: RpcKind, req_body: Option<&serde_json::Value>,
    ) -> Result<Duration> {
        let mut request = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| unexpected_err(e, None))?
            .request(
                match kind {
                    RpcKind::EVM | RpcKind::SOLANA => Method::POST,
                    RpcKind::IPFS | RpcKind::COSMOS => Method::GET,
                },
                host,
            )
            .header("Content-Type", "application/json");

        if let Some(req_body) = req_body {
            request =
                request.body(serde_json::to_string(req_body).map_err(|e| unexpected_err(e, None))?);
        }

        let start = SystemTime::now();
        let response = request.send().await.map_err(|e| unexpected_err(e, None))?;
        let finish = SystemTime::now();

        if !response.status().is_success() {
            return Err(unexpected_err(
                format!("http response from {} was not 200 OK", host),
                None,
            ));
        }

        match kind {
            RpcKind::EVM => {
                #[derive(Deserialize)]
                struct Response {
                    result: Option<String>,
                }
                let bytes = response.bytes().await.map_err(|e| unexpected_err(e, None))?;
                let parsed_response = serde_json::from_slice::<Response>(&bytes)
                    .map_err(|e| unexpected_err(e, None))?;
                parsed_response.result.expect_or_err(String::from(
                    "jsonrpc response from eth_blockNumber did not contain a result",
                ))
            }
            RpcKind::IPFS => {
                // Note (Harry): Add more checks here for IPFS
                Ok("content".to_string())
            }
            RpcKind::SOLANA => {
                #[derive(Deserialize)]
                struct Response {
                    result: Option<String>,
                    error: Option<serde_json::Value>,
                }
                let bytes = response.bytes().await.map_err(|e| unexpected_err(e, None))?;
                let parsed_response = serde_json::from_slice::<Response>(&bytes)
                    .map_err(|e| unexpected_err(e, None))?;

                if parsed_response.error.is_some() {
                    return Err(unexpected_err("Solana node reported unhealthy status", None));
                }

                parsed_response.result.expect_or_err(String::from(
                    "jsonrpc response from Solana node did not contain a result",
                ))
            }
            RpcKind::COSMOS => {
                #[derive(Deserialize)]
                struct Response {
                    result: Option<serde_json::Value>,
                }
                let bytes = response.bytes().await.map_err(|e| unexpected_err(e, None))?;
                let parsed_response = serde_json::from_slice::<Response>(&bytes)
                    .map_err(|e| unexpected_err(e, None))?;

                if parsed_response.result.is_none() {
                    return Err(unexpected_err(
                        "Cosmos node health check response did not contain a result", None,
                    ));
                }

                Ok("healthy".to_string())
            }
        }?;

        finish.duration_since(start).map_err(|e| unexpected_err(e, None))
    }
}

pub trait RpcHealthcheckPoller: Sync {
    fn get_latencies(&self) -> &ArcSwap<im::hashmap::HashMap<RpcEntry, Latency>>;

    fn get_rpc_resolver(&self) -> &Lazy<ArcSwap<RpcResolver>>;

    fn get_health_request_id(&self) -> &AtomicUsize;

    fn healthcheck(
        &self, host: &str, kind: RpcKind, req_body: Option<&serde_json::Value>,
    ) -> impl Future<Output = Result<Duration>> + Send;

    fn measure_latency_of<F>(
        thing: F,
    ) -> impl Future<Output = (<F as Future>::Output, Result<Duration>)>
    where
        F: Future,
    {
        let a = SystemTime::now();
        thing.map(move |ret| {
            let b = SystemTime::now();
            (ret, b.duration_since(a).map_err(|e| unexpected_err(e, None)))
        })
    }

    fn poll_rpcs_for_latency(&self) -> impl std::future::Future<Output = ()> + Send {
        async move {
            trace!("polling RPCs for latency");
            let rpc_resolver_struct = self.get_rpc_resolver();
            let rpc_resolver = rpc_resolver_struct.load_full();
            let chains = rpc_resolver.config.chains().clone();
            let mut tasks = FuturesUnordered::new();
            for (_chain, entries) in chains {
                // trace!("polling chain {} for latency with {} rpcs", chain, entries.len());
                for rpc_entry in entries {
                    let rpc_entry_clone = rpc_entry.clone();
                    let u = rpc_entry.url().clone();
                    let kind = rpc_entry.kind().clone();
                    let fut = async move {
                        let id = self.get_health_request_id().fetch_add(1, Ordering::SeqCst);
                        let healthcheck = match kind {
                            RpcKind::EVM => {
                                let request_body = json!({
                                    "jsonrpc": "2.0",
                                    "method": "eth_blockNumber",
                                    "params": [],
                                    "id": id
                                });
                                self.healthcheck(u.as_str(), kind, Some(&request_body)).await
                            }
                            RpcKind::IPFS => {
                                let mut ipfs_url =
                                    Url::parse(&u).expect("URL Must parse as config verified");
                                ipfs_url.set_path("/ipfs/bafkqablimvwgy3y");
                                self.healthcheck(ipfs_url.as_str(), kind, None).await
                            }
                            RpcKind::SOLANA => {
                                let request_body = json!({
                                    "jsonrpc": "2.0",
                                    "method": "getHealth",
                                    "params": [],
                                    "id": id,
                                });
                                self.healthcheck(u.as_str(), kind, Some(&request_body)).await
                            }
                            RpcKind::COSMOS => {
                                let mut cosmos_url =
                                    Url::parse(&u).expect("URL Must parse as config verified");
                                cosmos_url.set_path("/health");
                                self.healthcheck(cosmos_url.as_str(), kind, None).await
                            }
                        };

                        // let (h, latency) = Self::measure_latency_of(healthcheck).await;
                        let latency = match healthcheck {
                            Ok(latency) => {
                                // trace!("RPC Health check for {} returned as healthy", u);
                                Latency::Healthy(latency)
                            }
                            Err(e) => {
                                trace!("RPC Health check for {} returned as unhealthy {}", u, e);
                                Latency::Unhealthy
                            }
                        };

                        Ok((rpc_entry_clone, latency))
                    };
                    tasks.push(fut);
                }
            }

            let mut results: Vec<Result<(RpcEntry, Latency)>> = vec![];
            while let Some(result) = tasks.next().await {
                results.push(result);
            }

            let mut latencies = self.get_latencies().load_full().deref().clone();
            for result in results {
                let (rpc_entry, latency) = match result {
                    Ok((u, l)) => (u, l),
                    Err(e) => {
                        trace!("Unexpected error polling RPC for latency: {:?}", e);
                        continue;
                    }
                };

                trace!("RPC Health check for {} returned as {:?}", rpc_entry.url(), latency);

                if let Some(existing_latency_entry) = latencies.get_mut(&rpc_entry) {
                    *existing_latency_entry = latency;
                } else {
                    latencies.insert(rpc_entry.clone(), latency);
                }
            }
            self.get_latencies().store(latencies.into());
        }
    }
    fn load_latencies_from_rpc_resolver(
        rpc_resolver: &Lazy<ArcSwap<RpcResolver>>,
    ) -> ArcSwap<HashMap<RpcEntry, Latency>> {
        ArcSwap::from(Arc::new({
            let resolver = rpc_resolver.load();
            let chains = resolver.config.chains();
            let key_values = chains
                .values()
                .flat_map(|v| v.iter().rev())
                .zip((0..).map(|t| Duration::MAX.saturating_sub(Duration::from_secs(t))))
                .map(|(k, v)| (k.clone(), Latency::Healthy(v)));
            let mut m = im::hashmap::HashMap::new();
            m.extend(key_values);
            m
        }))
    }
    fn rpc_entry<C>(&self, chain_name: C) -> Result<RpcEntry>
    where
        C: AsRef<str>,
    {
        let latencies = self.get_latencies().load();
        let resolver = self.get_rpc_resolver().load();
        resolver
            .resolve(chain_name.as_ref())?
            .iter()
            .min_by_key(|entry| latencies.get(entry))
            .ok_or(config_err(
                format!("No RPC entry exists for chain id: {}", chain_name.as_ref()),
                None,
            ))
            .cloned()
    }

    fn get_provider<C>(&self, chain_name: C) -> Result<Arc<Provider<Http>>>
    where
        C: AsRef<str>,
    {
        let entry = self.rpc_entry(chain_name)?;
        create_provider(&entry)
    }

    fn rpc_url<C>(&self, chain_name: C) -> Result<String>
    where
        C: AsRef<str>,
    {
        let entry = self.rpc_entry(chain_name)?;
        Ok(entry.url().to_string())
    }
}

pub struct RpcResolver {
    config: RpcConfig,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Latency {
    Healthy(Duration),
    Unhealthy,
}

impl RpcResolver {
    fn new(config: RpcConfig) -> Self {
        Self { config }
    }

    fn load() -> Result<Self> {
        let config = RpcConfig::load()?;
        config.verify()?;

        Ok(Self::new(config))
    }

    // Resolve
    pub fn resolve<C>(&self, chain_name: C) -> Result<&Vec<RpcEntry>>
    where
        C: AsRef<str>,
    {
        self.config.chains().get(chain_name.as_ref()).ok_or_else(|| {
            config_err(format!("unable to resolve RPC for chain id: {}", chain_name.as_ref()), None)
        })
    }

    pub fn resolve_entry<C>(&self, chain_name: C, index: usize) -> Result<&RpcEntry>
    where
        C: AsRef<str>,
    {
        let entries = self.resolve(chain_name.as_ref())?;
        match entries.get(index) {
            None => Err(config_err(
                format!("no RPC index in entries for chain id: {}, {}", chain_name.as_ref(), index),
                None,
            )),
            Some(value) => Ok(value),
        }
    }

    // Reload
    pub fn reload() -> Result<()> {
        let config = RpcConfig::load()?;
        config.verify()?;
        let mut latencies = ENDPOINT_MANAGER.get_latencies().load_full().deref().clone();
        let rpc_entries: std::collections::HashSet<&RpcEntry> =
            config.chains().values().flat_map(|v| v.iter()).collect();

        latencies.retain(|e, _| rpc_entries.contains(e));
        for (d, rpc_entry) in config.chains().values().flat_map(|v| {
            v.iter().enumerate().rev().map(|(i, v)| {
                (Duration::MAX.saturating_sub(Duration::from_secs(164 + i as u64)), v)
            })
        }) {
            if !latencies.contains_key(rpc_entry) {
                latencies.insert(rpc_entry.clone(), Latency::Healthy(d));
            }
        }

        RPC_RESOLVER.store(Arc::new(RpcResolver::new(config)));
        ENDPOINT_MANAGER.get_latencies().store(latencies.into());

        Ok(())
    }
}

fn create_provider(rpc_entry: &RpcEntry) -> Result<Arc<Provider<Http>>> {
    let http_cache = HTTP_CLIENT.get_or_init(|| scc::HashIndex::default());

    let guard = scc::ebr::Guard::new();
    let provider = match http_cache.peek(rpc_entry.url(), &guard) {
        Some(provider) => provider.clone(),
        None => {
            let provider = Arc::new(rpc_provider(rpc_entry)?);
            http_cache
                .insert(rpc_entry.url().to_owned(), provider.clone())
                .map_err(|_| unexpected_err("how does it already exist?", None))?;
            provider
        }
    };

    Ok(provider)
}

fn rpc_provider(rpc_entry: &RpcEntry) -> Result<Provider<Http>> {
    let mut header_map = reqwest::header::HeaderMap::new();

    if let Some(headers) = rpc_entry.headers() {
        let h: Result<Vec<_>> = headers
            .iter()
            .map(|(k, v)| {
                let k = reqwest::header::HeaderName::try_from(k)
                    .map_err(|e| config_err(e, Some("Not a valid header key".into())))?;
                let v = reqwest::header::HeaderValue::try_from(v)
                    .map_err(|e| config_err(e, Some("Not a valid header value".into())))?;
                Ok((k, v))
            })
            .collect();
        header_map = reqwest::header::HeaderMap::from_iter(h?);
    }

    if let Some(apikey) = rpc_entry.apikey() {
        // Consider marking security-sensitive headers with `set_sensitive`.
        let mut auth_value = reqwest::header::HeaderValue::from_str(apikey.as_str())
            .map_err(|e| config_err(e, Some("Not a valid api key value".into())))?;
        auth_value.set_sensitive(true);
        header_map.insert(reqwest::header::AUTHORIZATION, auth_value);
    }

    // get a client builder
    let client = reqwest::Client::builder()
        .use_rustls_tls()
        .pool_max_idle_per_host(50)
        .pool_idle_timeout(Duration::from_secs(30))
        .timeout(Duration::from_secs(10))
        .default_headers(header_map)
        .build()
        .map_err(|e| config_err(e, Some("Could not create provider client".into())))?;

    let url = Url::parse(rpc_entry.url())
        .map_err(|e| config_err(e, Some("Could not get RPC URL".into())))?;

    let mut provider = Provider::new(Http::new_with_client(url, client));
    provider.set_interval(Duration::from_secs(1));

    Ok(provider)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::env;

    fn get_test_config() -> Option<String> {
        let eth_mainnet = env::var("TEST_ETH_MAINNET_RPC").ok()?;
        let eth_goerli = env::var("TEST_ETH_GOERLI_RPC").ok()?;
        let solana = env::var("TEST_SOLANA_RPC").ok()?;
        let ipfs = env::var("TEST_IPFS_RPC").ok()?;
        let cosmos = env::var("TEST_COSMOS_RPC").ok()?;

        Some(format!(
            r#"
            [[chains.ethereum]]
            url = "{eth_mainnet}"
            kind = "EVM"
            [[chains.ethereum]]
            url = "https://eth-mainnet-backup.test.com"
            kind = "EVM"

            [[chains.goerli]]
            url = "{eth_goerli}"
            kind = "EVM"

            [[chains.solana]]
            url = "{solana}"
            kind = "SOLANA"

            [[chains.ipfs]]
            url = "{ipfs}"
            kind = "IPFS"

            [[chains.cosmos]]
            url = "{cosmos}"
            kind = "COSMOS"
        "#
        ))
    }

    #[tokio::test]
    async fn test_healthcheck_poll() -> Result<()> {
        let Some(config_str) = get_test_config() else {
            eprintln!("Skipping RPC healthcheck tests - environment variables not set");
            return Ok(());
        };

        let config: RpcConfig = toml::from_str(&config_str).unwrap();
        RPC_RESOLVER.store(Arc::new(RpcResolver::new(config)));

        // Test EVM healthcheck
        let eth_result = ENDPOINT_MANAGER
            .healthcheck(
                &env::var("TEST_ETH_MAINNET_RPC").unwrap(),
                RpcKind::EVM,
                Some(&json!({
                    "jsonrpc": "2.0",
                    "method": "eth_blockNumber",
                    "params": [],
                    "id": 1
                })),
            )
            .await;
        assert!(eth_result.is_ok(), "Ethereum RPC healthcheck failed");

        // Test Solana healthcheck
        let solana_result = ENDPOINT_MANAGER
            .healthcheck(
                &env::var("TEST_SOLANA_RPC").unwrap(),
                RpcKind::SOLANA,
                Some(&json!({
                    "jsonrpc": "2.0",
                    "method": "getHealth",
                    "params": [],
                    "id": 1
                })),
            )
            .await;
        assert!(solana_result.is_ok(), "Solana RPC healthcheck failed");

        Ok(())
    }

    use std::result::Result as StdResult;

    #[test]
    fn test_config_validation() -> Result<()> {
        // Test invalid URL
        let invalid_url_config = r#"
            [[chains.ethereum]]
            url = "not-a-url"
            kind = "EVM"
        "#;
        let config: StdResult<RpcConfig, _> = toml::from_str(invalid_url_config);
        assert!(config.is_ok(), "Should parse with invalid URL");
        assert!(config.unwrap().verify().is_err(), "Should reject invalid URL during verification");

        // Test missing URL
        let missing_url_config = r#"
            [[chains.ethereum]]
            kind = "EVM"
        "#;
        let config: StdResult<RpcConfig, _> = toml::from_str(missing_url_config);
        assert!(config.is_err(), "Shouldn't parse missing URL");

        // Test non-HTTPS URL for non-excluded chain
        let non_https_config = r#"
            [[chains.ethereum]]
            url = "http://example.com"
            kind = "EVM"
        "#;
        let config: StdResult<RpcConfig, _> = toml::from_str(non_https_config);
        assert!(config.is_ok(), "Should parse non-HTTPS URL");
        assert!(
            config.unwrap().verify().is_err(),
            "Should reject non-HTTPS URL during verification"
        );

        // Test HTTP URL for excluded chain (should pass)
        let excluded_http_config = r#"
            [[chains.hardhat]]
            url = "http://localhost:8545"
            kind = "EVM"
        "#;
        let config: StdResult<RpcConfig, _> = toml::from_str(excluded_http_config);
        assert!(config.is_ok(), "Should parse HTTP URL for excluded chain");
        assert!(config.unwrap().verify().is_ok(), "Should allow HTTP URL for excluded chain");

        // Test wrong RPC kind for required type
        let wrong_kind_config = r#"
            [[chains.ipfs_gateways]]
            url = "https://ipfs.example.com"
            kind = "EVM"
        "#;
        let config: StdResult<RpcConfig, _> = toml::from_str(wrong_kind_config);
        assert!(config.is_ok(), "Should parse wrong RPC kind");
        assert!(
            config.unwrap().verify().is_err(),
            "Should reject wrong RPC kind during verification"
        );

        // Test empty chains
        let empty_config = r#"
            chains = {}
        "#;
        let config: StdResult<RpcConfig, _> = toml::from_str(empty_config);
        assert!(config.is_ok(), "Should parse empty config");
        assert!(
            config.unwrap().verify().is_err(),
            "Should reject empty chains during verification"
        );

        Ok(())
    }

    #[test]
    fn test_rpc_resolver() -> Result<()> {
        let Some(config_str) = get_test_config() else {
            eprintln!("Skipping RPC resolver tests - environment variables not set");
            return Ok(());
        };

        let config: RpcConfig = toml::from_str(&config_str)?;
        let resolver = RpcResolver::new(config);

        // Test chain resolution
        let eth_entries = resolver.resolve("ethereum")?;
        assert_eq!(eth_entries.len(), 2, "Should have two Ethereum RPCs");

        // Test invalid chain
        assert!(resolver.resolve("invalid_chain").is_err());

        // Test specific entry resolution
        let eth_entry = resolver.resolve_entry("ethereum", 0)?;
        assert_eq!(eth_entry.kind(), &RpcKind::EVM);

        Ok(())
    }

    #[test]
    fn test_provider_creation() -> Result<()> {
        let Some(config_str) = get_test_config() else {
            eprintln!("Skipping provider creation tests - environment variables not set");
            return Ok(());
        };

        let config: RpcConfig = toml::from_str(&config_str)?;
        let entry = config.chains().get("ethereum").unwrap()[0].clone();

        let provider = create_provider(&entry)?;
        assert!(provider.url().as_str().starts_with("https://"), "Provider URL should be HTTPS");

        Ok(())
    }
}
