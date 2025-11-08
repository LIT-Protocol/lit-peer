pub mod client;
pub mod manager;
pub mod models;
pub mod routes;
pub mod runtime;
pub mod testnet_instance;
pub mod transport;
use lit_node_testnet::setup_logging;

use manager::start_runtime;

pub fn main() {
    // setup the logger
    setup_logging();
    // start the runtime
    start_runtime(transport::TransportType::HTTP).unwrap();
}
