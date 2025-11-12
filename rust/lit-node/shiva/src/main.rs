pub mod models;
pub mod routes;
pub mod shiva_client;
pub mod testnet_instance;

use lit_node_testnet::setup_logging;
use rocket::{launch, routes};

use crate::shiva_client::ShivaClient;

#[launch]
fn rocket() -> _ {
    // setup the logger
    setup_logging("SHVA");
    // start the web service
    let client = ShivaClient::new();
    let addr = "0.0.0.0".parse().unwrap();
    let port = 8000;
    let ws = rocket::build()
        .mount(
            "/",
            routes![
                crate::routes::create_testnet,
                crate::routes::delete_testnet,
                crate::routes::poll_testnet,
                crate::routes::get_info_testnet,
                crate::routes::stop_random_node_testnet,
                crate::routes::stop_random_node_and_wait_testnet,
                crate::routes::get_testnets,
                crate::routes::transition_epoch_and_wait
            ],
        )
        .manage(client)
        .configure(rocket::Config {
            address: addr,
            port,
            workers: 1, // Only allow the web server to use a single thread
            ..rocket::Config::default()
        });

    ws
}
