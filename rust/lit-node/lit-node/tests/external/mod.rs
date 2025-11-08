use ethers::signers::{LocalWallet, Signer};
use ethers::types::U256;
use lit_core::utils::binary::bytes_to_hex;
use lit_node_testnet::TestSetupBuilder;
use lit_node_testnet::end_user::EndUser;
use maplit::hashset;
use rocket::State;
use rocket_cors::{AllowedOrigins, Method};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct SecretState {
    wallet_address: String,
    wallet_secret: String,
    pubkey: String,
}

#[tokio::test]
#[ignore]
#[doc = "This test is used to load a network for external tests.  It also launches a rocket server that serves up a PKP and a wallet that controls it."]
async fn load_network_for_external_tests() {
    crate::common::setup_logging();

    let (testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;

    publish_pkp_to_rocket(&end_user).await;
    // tokio::time::sleep(std::time::Duration::from_secs(100000)).await;
    info!("Testnet chain id: {:?}", testnet.chain_id);
    info!(
        "Validator collection size: {:?}",
        validator_collection.size()
    );
    info!(
        "End user first pkp token id: {:?}",
        end_user.first_pkp().token_id
    );
}

#[tokio::test]
#[ignore]
#[doc = "This test is used to load a network for external tests.  It will run for a little more than 1 day, and is not intended to be run as part of the test suite."]
async fn load_network_for_external_tests_with_additional_nodes() {
    crate::common::setup_logging();

    const INITIAL_VALIDATORS: usize = 5;
    const MAX_VALIDATORS: usize = 10;
    const EPOCH_LENGTH: usize = 300;
    // Setup the network

    let (testnet, validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(INITIAL_VALIDATORS)
        .num_staked_only_validators(MAX_VALIDATORS - INITIAL_VALIDATORS)
        .register_inactive_validators(true)
        .epoch_length(EPOCH_LENGTH)
        .build()
        .await;

    publish_pkp_to_rocket(&end_user).await;

    info!("Testnet chain id: {:?}", testnet.chain_id);
    info!(
        "Validator collection size: {:?}",
        validator_collection.size()
    );
    info!(
        "End user first pkp token id: {:?}",
        end_user.first_pkp().token_id
    );

    // tokio::time::sleep(std::time::Duration::from_secs(100000)).await;
}

#[doc = "This function publishes a PKP to the rocket server and returns the secret that controls it."]
async fn publish_pkp_to_rocket(end_user: &EndUser) {
    let wallet = LocalWallet::new(&mut rand_core::OsRng);
    let addr_to_add = wallet.address();
    let pubkey = end_user.first_pkp().pubkey.clone();
    end_user
        .first_pkp()
        .add_permitted_address_to_pkp(addr_to_add, &[U256::from(1)])
        .await
        .expect("Could not add permitted address to pkp");
    info!("Started network for external tests");

    let secret = bytes_to_hex(wallet.signer().as_nonzero_scalar().to_bytes());
    info!("Wallet address that controls a minted PKP: {}", addr_to_add);
    info!("Secret that controls a minted PKP: {}", secret);

    let state = SecretState {
        wallet_address: bytes_to_hex(addr_to_add.to_fixed_bytes()),
        wallet_secret: secret.clone(),
        pubkey: pubkey.clone(),
    };

    let allowed_methods = hashset! {
    Method::from_str("Get").unwrap(),
    Method::from_str("Post").unwrap(),
    Method::from_str("Patch").unwrap()};

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: allowed_methods,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS failed to build");

    let r = rocket::build()
        .manage(state)
        .manage(cors)
        .mount("/", routes![index]);
    r.launch().await.unwrap();
}

#[get("/")]
fn index(secret_state: &State<SecretState>) -> serde_json::Value {
    serde_json::json!(secret_state.inner())
}
