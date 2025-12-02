use crate::common::ecdsa::simple_single_sign_with_hd_key;

use ethers::types::U256;
use lit_node_core::{CurveType, SigningScheme};
use lit_node_testnet::{TestSetupBuilder, testnet::actions::RootKeyConfig};
use tracing::info;

#[tokio::test]
#[doc = "Primary test to ensure that the network can add a second keyset and sign with it."]
pub async fn test_add_second_keyset() {
    crate::common::setup_logging();

    info!("Starting test: test_pkp_hd_sign_generic_key_with_epoch_change");
    let (_testnet, validator_collection, mut end_user) = TestSetupBuilder::default().build().await;

    let actions = validator_collection.actions();
    let pubkey = end_user.first_pkp().pubkey.clone();

    let realm_id = U256::from(1);

    // check to see that we can sign
    let result = simple_single_sign_with_hd_key(
        &validator_collection,
        &end_user,
        pubkey.clone(),
        SigningScheme::EcdsaK256Sha256,
        &vec![],
    )
    .await;
    assert!(result, "Failed to sign with all nodes up.");

    let key_set_number = 2;

    let identifier = format!("naga-keyset{}-", key_set_number);
    info!("**** Adding keyset `{}` ****", identifier);

    let description = format!("Naga Keyset {}", key_set_number);

    let mut root_key_configs = vec![];
    for i in 1..5 {
        if i == 1 {
            root_key_configs.push(RootKeyConfig {
                curve_type: CurveType::BLS,
                count: 1,
            });
        } else {
            root_key_configs.push(RootKeyConfig {
                curve_type: CurveType::try_from(i).unwrap(),
                count: 2,
            });
        }
    }
    let r = actions
        .add_keyset(realm_id, identifier.clone(), description, root_key_configs)
        .await;
    assert!(r.is_ok(), "Failed to add keyset `{}`", identifier);

    let current_epoch = actions.get_current_epoch(realm_id).await;
    info!("Epoch: {}", current_epoch);

    // Fast forward the network by 300 seconds, and wait for the new node to be active - effectively waiting for the next epoch.
    actions.increase_blockchain_timestamp(300).await;

    // Wait for DKG to start and then finish, by effectively waiting for the epoch change - nodes become active once more.
    actions.wait_for_epoch(realm_id, current_epoch + 1).await;

    actions.sleep_millis(5000).await;

    let (new_pubkey, _new_token_id, _new_eth_address) = end_user
        .new_pkp_with_key_set_id(&identifier)
        .await
        .expect("Failed to mint PKP");

    // test signing
    let result = simple_single_sign_with_hd_key(
        &validator_collection,
        &end_user,
        new_pubkey.clone(),
        SigningScheme::EcdsaK256Sha256,
        &vec![],
    )
    .await;
    assert!(result, "Failed to sign with all nodes up.");

}

#[ignore]
#[tokio::test]
#[doc = "Add a lot of keysets and test signing with them."]
pub async fn test_add_a_lot_of_keysets() {
    crate::common::setup_logging();

    info!("Starting test: test_pkp_hd_sign_generic_key_with_epoch_change");
    let (_testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;

    let actions = validator_collection.actions();
    let pubkey = end_user.first_pkp().pubkey.clone();

    let realm_id = U256::from(1);

    // check to see that we can sign
    let result = simple_single_sign_with_hd_key(
        &validator_collection,
        &end_user,
        pubkey.clone(),
        SigningScheme::EcdsaK256Sha256,
        &vec![],
    )
    .await;
    assert!(result, "Failed to sign with all nodes up.");

    let mut keySetId = 2;
    for j in 0..10 {
        for i in 2..10 {
            let identifier = format!("naga-keyset{}-", keySetId);
            info!("**** Adding keyset `{}` ****", identifier);

            let description = format!("Naga Keyset {}", i);
            let root_key_configs = vec![RootKeyConfig {
                curve_type: CurveType::try_from(i).unwrap(),
                count: 2,
            }];
            let r = actions
                .add_keyset(realm_id, identifier.clone(), description, root_key_configs)
                .await;
            assert!(r.is_ok(), "Failed to add keyset `{}`", identifier);

            keySetId += 1;
        }
    }

    let current_epoch = actions.get_current_epoch(realm_id).await;
    info!("Epoch: {}", current_epoch);

    // Fast forward the network by 300 seconds, and wait for the new node to be active - effectively waiting for the next epoch.
    actions.increase_blockchain_timestamp(300).await;

    // Wait for DKG to start and then finish, by effectively waiting for the epoch change - nodes become active once more.
    actions.wait_for_epoch(realm_id, current_epoch + 1).await;

    actions.sleep_millis(5000).await;
    // test signing
    let result = simple_single_sign_with_hd_key(
        &validator_collection,
        &end_user,
        pubkey.clone(),
        SigningScheme::EcdsaK256Sha256,
        &vec![],
    )
    .await;
    assert!(result, "Failed to sign with all nodes up.");

    actions.sleep_millis(2000000).await;
}
