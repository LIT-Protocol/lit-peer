use lit_core::config::LitConfig;
use lit_core::error::Result;
use lit_node_common::config::{CFG_KEY_SIGNING_ROUND_TIMEOUT_MS_DEFAULT, LitNodeConfig};
#[cfg(all(feature = "proxy_chatter", feature = "testing"))]
use std::path::PathBuf;
use std::{sync::Arc, time::Duration};
use tonic::transport::Channel;
use url::Url;

#[cfg(all(feature = "proxy_chatter", feature = "testing"))]
use crate::error::parser_err;
use crate::tasks::chatter_sender::INTERNAL_CHATTER_PORT_OFFSET;
use crate::{
    error::unexpected_err,
    p2p_comms::web::chatter_server::chatter::chatter_service_client::ChatterServiceClient,
};
#[cfg(all(feature = "proxy_chatter", feature = "testing"))]
use lit_node_common::proxy_mapping::{ClientProxyMapping, PROXY_MAPPING_PATH};

pub struct ChatterClientFactory;

impl ChatterClientFactory {
    pub async fn new_client(
        dest_url: Url,
        cfg: Arc<LitConfig>,
    ) -> Result<ChatterServiceClient<Channel>> {
        let dest_grpc_url = get_grpc_url_from_http_url(dest_url.clone());
        #[cfg(not(all(feature = "proxy_chatter", feature = "testing")))]
        {
            ChatterClientFactory::new_default_client(dest_grpc_url, cfg).await
        }

        #[cfg(all(feature = "proxy_chatter", feature = "testing"))]
        {
            if cfg.enable_proxied_chatter_client()? {
                ChatterClientFactory::new_proxied_client(dest_grpc_url, cfg).await
            } else {
                ChatterClientFactory::new_default_client(dest_grpc_url, cfg).await
            }
        }
    }

    pub async fn new_default_client(
        dest_peer: Url,
        lit_config: Arc<LitConfig>,
    ) -> Result<ChatterServiceClient<Channel>> {
        debug!("Creating a new grpc client");
        let uri = dest_peer.as_str().parse().expect("Failed to parse URL");
        let timeout = match lit_config.chatter_client_timeout() {
            Ok(t) => Duration::from_secs(t),
            Err(e) => {
                warn!("Failed to get chatter client timeout: {:?}", e);
                Duration::from_millis(CFG_KEY_SIGNING_ROUND_TIMEOUT_MS_DEFAULT as u64)
            }
        };
        debug!("GRPC client timeout {} ms", timeout.as_millis());
        match Channel::builder(uri)
            .timeout(timeout)
            .keep_alive_while_idle(true)
            .keep_alive_timeout(timeout)
            .tcp_keepalive(Some(timeout))
            .connect_timeout(timeout)
            .connect()
            .await
        {
            Ok(channel) => Ok(ChatterServiceClient::new(channel)),
            Err(e) => Err(unexpected_err(
                e,
                Some(format!("Failed to connect to peer: {}", dest_peer)),
            )),
        }
    }

    #[cfg(all(feature = "proxy_chatter", feature = "testing"))]
    pub async fn new_proxied_client(
        dest_url: Url,
        lit_config: Arc<LitConfig>,
    ) -> Result<ChatterServiceClient<Channel>> {
        // Check if config file for proxy mappings exists.
        let path = PathBuf::from(PROXY_MAPPING_PATH);

        if path.exists() {
            // Read config file for proxy mappings.
            let proxy_config = ClientProxyMapping::try_from(path.as_path())?;

            // Check if our address is in the proxy mappings.
            let prefix = lit_config.http_prefix_when_talking_to_other_nodes();
            let our_url = Url::parse(&format!("{}{}", prefix, lit_config.external_addr()?))
                .map_err(|e| parser_err(e, Some("Unable to parse our external addr".into())))?;

            if let Some(our_proxy_config) = proxy_config.proxy_mappings().get(&our_url).cloned() {
                let dest_proxy_url = match our_proxy_config.get(&dest_url) {
                    Some(d) => d,
                    None => {
                        return ChatterClientFactory::new_default_client(dest_url, lit_config)
                            .await;
                    }
                };
                return ChatterClientFactory::new_default_client(
                    dest_proxy_url.clone(),
                    lit_config,
                )
                .await;
            }
        }
        ChatterClientFactory::new_default_client(dest_url, lit_config).await
    }
}

pub fn get_grpc_url_from_http_url(mut url: Url) -> Url {
    let new_port = url
        .port_or_known_default()
        .expect("Unable to parse http port")
        + INTERNAL_CHATTER_PORT_OFFSET;
    url.set_port(Some(new_port))
        .expect("Failed to set new port");
    url
}
