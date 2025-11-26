use super::signing::sign_with_each_curve_type;
use ethers::types::U256;
use lit_node_testnet::TestSetupBuilder;
use tracing::info;

#[tokio::test]
#[doc = "Primary test to ensure that the network can add a second keyset and sign with it."]
pub async fn test_add_second_keyset() {
    crate::common::setup_logging();

    info!("Starting test: test_pkp_hd_sign_generic_key_with_epoch_change");
    let (_testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;

    let actions = validator_collection.actions();
    let pubkey = end_user.first_pkp().pubkey.clone();

    let realm_id = U256::from(1);
    let current_epoch = actions.get_current_epoch(realm_id).await;

    // check to see that we can sign
    sign_with_each_curve_type(&validator_collection, &end_user, pubkey.clone()).await;

    info!("**** Adding second keyset ****");
    // add a second keyset
    let r = actions.add_second_keyset(realm_id).await;
    assert!(r.is_ok(), "Failed to add second keyset");

    actions.sleep_millis(2000).await; // wait for the nodes to check the chain for the new keyset

    for i in 0..10 {
        let current_epoch = actions.get_current_epoch(realm_id).await;
        info!("Epoch: {}", current_epoch);

        // Fast forward the network by 300 seconds, and wait for the new node to be active - effectively waiting for the next epoch.
        actions.increase_blockchain_timestamp(300).await;

        // Wait for DKG to start and then finish, by effectively waiting for the epoch change - nodes become active once more.
        actions.wait_for_epoch(realm_id, current_epoch + 1).await;

        actions.sleep_millis(2000).await;
        // test signing
        sign_with_each_curve_type(&validator_collection, &end_user, pubkey.clone()).await;
    }

    actions.sleep_millis(2000000).await;
}
