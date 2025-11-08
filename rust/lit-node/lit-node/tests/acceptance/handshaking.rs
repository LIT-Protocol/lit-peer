use ethers::prelude::U256;
use lit_node_testnet::{TestSetupBuilder, node_collection::handshake_returns_keys};
use tracing::info;

#[tokio::test]
pub async fn test_handshaking() {
    crate::common::setup_logging();
    info!("Starting test: test_handshaking");
    let (testnet, validator_collection, _) = TestSetupBuilder::default().build().await;

    // Assert that the handshake returns keys.
    assert!(handshake_returns_keys(validator_collection.actions(), U256::from(1)).await);
    drop(testnet);
}
