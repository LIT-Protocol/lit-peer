use crate::common::web_user_tests::{
    test_encryption_decryption_session_sigs, test_lit_action_session_sigs,
};
use lit_node_testnet::{DEFAULT_KEY_SET_NAME, TestSetupBuilder};

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
    test_encryption_decryption_session_sigs(&validator_collection, &vec![], &end_user).await;

    info!("Testing lit actions with BLS session sigs");
    test_lit_action_session_sigs(&validator_collection, &end_user).await;
}

#[tokio::test]
async fn test_web_user_with_auth_methods() {
    crate::common::setup_logging();
    let (_testnet, validator_collection, mut end_user) =
        TestSetupBuilder::default().force_deploy(true).build().await;

    let auth_methods = vec![];

    let pkp_details = end_user
        .new_pkp_and_add_auth_methods(DEFAULT_KEY_SET_NAME, &auth_methods)
        .await;
    info!("PKP details: {:?}", pkp_details);
}
