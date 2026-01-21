use reqwest::Url;
use tracing::trace;

const ADDRESSABLE_PROXY_SPACE_PER_NODE: usize = 30;
const FIRST_PROXY_STARTING_PORT: usize = 11000;
const PORT_TO_OFFSET_FROM: usize = 7470;

pub struct ClientProxyConfiguration;

impl ClientProxyConfiguration {
    /// Given the port of this node and the URL of the peer to
    /// communicate with, determine which proxy to route this request.
    ///
    /// Works by assigning `ADDRESSABLE_PROXY_SPACE_PER_NODE` ports per node to use for addressing peers.
    ///
    /// Starting port for all proxies is `FIRST_PROXY_STARTING_PORT`.
    ///
    /// Example:
    ///   - Given port 7471 wants to communicate to port 7478, route requests
    ///     to 11038.
    ///   - Given port 7478 wants to communicate to port 7471, route requests
    ///     to 11241.
    pub fn determine_local_proxy(source_port: usize, dest_port: usize) -> Url {
        // Determine offset for dest_url port.
        let dest_port_offset: usize = dest_port - PORT_TO_OFFSET_FROM;

        // If dest_port_offset is greater than ADDRESSABLE_PROXY_SPACE_PER_NODE,
        // there is no space for a proxy to route this request to the dest. Simply
        // route the request directly, without proxy.
        if dest_port_offset >= ADDRESSABLE_PROXY_SPACE_PER_NODE {
            trace!(
                "No space for a proxy to route this request, directing request straight to dest at port {:?}",
                dest_port
            );
            return get_local_url_from_port(dest_port);
        }

        // Determine offset from 7470 for source_port
        let source_port_offset = source_port - PORT_TO_OFFSET_FROM;

        // Determine starting port for proxies for this source_port.
        let source_proxies_starting_port: usize =
            FIRST_PROXY_STARTING_PORT + (source_port_offset * ADDRESSABLE_PROXY_SPACE_PER_NODE);

        // Determine exact port to proxy requests to.
        let proxy_port = source_proxies_starting_port + dest_port_offset;
        get_local_url_from_port(proxy_port)
    }
}

pub fn get_local_url_from_port(port: usize) -> Url {
    Url::parse(format!("http://127.0.0.1:{}", port).as_str()).expect("Failed to parse local url")
}
