pub mod abstractions;
pub mod core;
use lit_node_testnet::TestSetupBuilder;
use rocket::fs::{FileServer, relative};
use rocket_cors::{AllowedOrigins, Method};
use std::sync::Arc;
use std::{collections::HashSet, str::FromStr};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // this actually loads "naga-test", because that's what' sin the live_testnet.toml file
    let (testnet, validator_collection, _end_user) = TestSetupBuilder::default()
        .setup_datil_keys(false)
        .build()
        .await;

    let testnet = Arc::new(testnet);
    let validator_collection = Arc::new(validator_collection);
    let allowed_methods = HashSet::from([
        Method::from_str("Get").unwrap(),
        Method::from_str("Options").unwrap(),
        Method::from_str("Post").unwrap(),
        Method::from_str("Patch").unwrap(),
    ]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS failed to build");

    let r = rocket::build()
        .manage(testnet)
        .manage(validator_collection)
        .attach(cors)
        .mount("/core/v1/", core::v1::endpoints::routes())
        .mount("/transfer/v1/", abstractions::transfer::endpoints::routes())
        .mount("/swaps/v1/", abstractions::intents::swaps::endpoints::routes())
        .mount("/", FileServer::from(relative!("static")));
    r.launch().await?;
    Ok(())
}
