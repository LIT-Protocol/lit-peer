use crate::common::web_user_tests::{
    test_encryption_decryption_session_sigs, test_lit_action_session_sigs,
};
use lit_node_testnet::TestSetupBuilder;

use tracing::info;

#[tokio::test]
async fn test_everything_as_web_user() {
    crate::common::setup_logging();
    // use initial_node_setup if you don't have a DKG result saved.

    let (_testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;

    // FIXME: Get this test working.
    // info!("Testing JWT signing with auth sigs");
    // test_jwt_signing_auth_sig(&nc).await;
    info!("Testing decryption with session sigs");
    test_encryption_decryption_session_sigs(&validator_collection, &end_user).await;

    info!("Testing lit actions with BLS session sigs");
    test_lit_action_session_sigs(&validator_collection, &end_user).await;
}
