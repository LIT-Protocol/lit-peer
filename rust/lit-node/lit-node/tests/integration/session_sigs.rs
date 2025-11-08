use crate::common::lit_actions::HELLO_WORLD_LIT_ACTION_CODE;
use crate::common::lit_actions::{
    assert_signed_action, execute_lit_action_auth_sig, execute_lit_action_session_sigs,
    lit_action_params,
};
use crate::common::pkp::generate_session_sigs_and_send_signing_requests;
use crate::common::web_user_tests::{
    assert_decrypted, prepare_test_encryption_parameters_with_wallet_address,
    retrieve_decryption_key_session_sigs,
};
use crate::common::{
    auth_sig::{
        generate_authsig_item, get_auth_sig_for_session_sig_from_nodes,
        get_session_sigs_and_node_set_for_pkp, get_session_sigs_for_auth,
    },
    session_sigs::{
        CUSTOM_AUTH_RESOURCE_VALID_PKP_SIGNING_LIT_ACTION_CODE,
        CUSTOM_AUTH_RESOURCE_VALID_SESSION_SIG_LIT_ACTION_CODE,
        INVALID_SESSION_SIG_LIT_ACTION_CODE, MGB_PKP_SESSION_SIG_LIT_ACTION_CODE,
        NO_AUTH_METHOD_PKP_SIGNING_LIT_ACTION_CODE, NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE,
        SIGN_ECDSA_LIT_ACTION_CODE, VALID_PKP_SIGNING_LIT_ACTION_CODE,
        VALID_SESSION_SIG_LIT_ACTION_CODE, get_pkp_sign, init_test,
    },
};

use anyhow::Result;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::U256;
use ipfs_hasher::IpfsHasher;
use lit_node::models::RequestConditions;
use lit_node::pkp::auth::AuthMethodScope;
use lit_node::pkp::auth::get_user_wallet_auth_method_from_address;
use lit_node::utils::encoding;
use lit_node::utils::web::hash_access_control_conditions;
use lit_node_core::SigningScheme;
use lit_node_core::{
    AccessControlConditionResource, AuthMaterialType, AuthMethod, AuthSigItem, LitAbility,
    LitResource, LitResourceAbilityRequest, LitResourceAbilityRequestResource, LitResourcePrefix,
};
use lit_node_testnet::TestSetupBuilder;
use lit_node_testnet::node_collection::get_identity_pubkeys_from_node_set;
use rand_core::OsRng;
use tracing::info;

#[doc = "Test that users can run a Lit Action before signing the sessionSig. Also test that you can't `signEcdsa` in that Lit Action."]
#[tokio::test]
async fn sign_session_sig_with_lit_actions() {
    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;

    let wallet = end_user.signing_provider().signer().clone();
    let auth_sig = generate_authsig_item(&wallet).await.unwrap();

    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let signing_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    info!("Starting test: only_sign_session_sig_with_lit_actions");

    let lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    let resource = "Threshold/Signing".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::PKP);

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &node_set,
        &auth_sig,
        false,
        &eth_address.to_fixed_bytes(),
        &pubkey,
        session_pub_key.clone(),
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        Some(lit_action_code),
        Some(serde_json::Value::Object(js_params.clone())),
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let responses = signing_resp.unwrap();
    for response in &responses {
        assert!(response.ok);
        let response = response.data.as_ref().unwrap();
        assert_eq!(response.result, "success");

        let siwe_message = serde_json::to_string(&response.siwe_message).unwrap();
        assert!(siwe_message.contains("'Threshold': 'Signing' for 'lit-pkp://*'")); // should contain user defined resources
        // TODO: Can add assertions for specific fields in the below resource like the actionIpfsIds should be: [QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz] & the userId should be: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
        assert!(siwe_message.contains("'Auth': 'Auth' for 'lit-resolvedauthcontext://*'"));
        // should contain resolved authContext resources
    }

    info!("Starting test: failed_ecdsa_sign_session_sig_with_lit_actions");

    let balance_before = end_user
        .get_first_pkp_ledger_balance("Balance before being used for sessionSig")
        .await;

    let lit_action_code =
        data_encoding::BASE64.encode(INVALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    let signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &node_set,
        &auth_sig,
        false,
        &eth_address.to_fixed_bytes(),
        &pubkey,
        session_pub_key,
        vec![resource],
        vec![resource_prefix],
        Some(lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let responses = signing_resp.unwrap();
    for response in &responses {
        assert!(!response.ok);
        let error = response.error.as_ref().unwrap();
        assert!(
            error.contains("You can not sign without providing an auth_sig."),
            "{:?}",
            error
        );
    }

    assert!(
        end_user
            .first_pkp_ledger_has_decreased_from(balance_before)
            .await,
        "Balance after being used for sessionSig should be less than balance before."
    );
}

#[doc = "Test that users need funds trun a Lit Action before signing the sessionSig."]
#[tokio::test]
async fn sign_session_sig_with_lit_actions_requires_payment() {
    crate::common::setup_logging();
    let (_testnet, validator_collection, end_user) = TestSetupBuilder::default()
        .fund_ledger_for_wallet(false)
        .build()
        .await;

    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;

    let wallet = end_user.signing_provider().signer().clone();
    end_user.set_wallet_balance("0").await;
    let auth_sig = generate_authsig_item(&wallet).await.unwrap();

    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let signing_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    info!("Starting test: only_sign_session_sig_with_lit_actions");

    let lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    let resource = "Threshold/Signing".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::PKP);

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &node_set,
        &auth_sig,
        false,
        &eth_address.to_fixed_bytes(),
        &pubkey,
        session_pub_key.clone(),
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        Some(lit_action_code),
        Some(serde_json::Value::Object(js_params.clone())),
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let responses = signing_resp.unwrap();
    for response in &responses {
        assert!(
            !response.ok,
            "response.ok should be false. Response: {:?}",
            response
        );
        let response_error = response.error.as_ref().unwrap();
        assert!(
            response_error.contains("unable to get payment method"),
            "response_error doesn't contain 'unable to get payment method': {:?}",
            response_error
        );
    }
}

#[doc = "Test that non-permitted users can't run any random Lit Action they want before signing the sessionSig."]
#[tokio::test]
async fn only_permitted_lit_action_can_sign_session_sig() {
    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let non_owner_wallet = LocalWallet::new(&mut OsRng);
    let auth_sig = generate_authsig_item(&non_owner_wallet).await.unwrap();

    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let signing_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    info!("Starting test: only_permitted_lit_action_can_sign_session_sig");

    let lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes()); // This Lit Action hasn't been permitted so only the owner AuthSig can sign inside it

    let resource = "Threshold/Signing".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::PKP);

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &node_set,
        &auth_sig,
        false,
        &eth_address.to_fixed_bytes(),
        &pubkey,
        session_pub_key.clone(),
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        Some(lit_action_code),
        Some(serde_json::Value::Object(js_params.clone())),
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let responses = signing_resp.unwrap();
    for response in &responses {
        assert!(!response.ok);
        assert!(response.error_object.as_ref().unwrap().contains(
            "None of the AuthMethods, AuthSig or Lit Actions meet the required scope [2]"
        ));
    }
}

#[doc = "Custom Authentication: Test that permitted Lit Action is allowed to create a sessionSig & then sign with the PKP. To test this we use a Random wallet that doesn't own the PKP as the auth_method."]
#[tokio::test]
async fn sign_pkp_with_lit_action_session_sigs() {
    crate::common::setup_logging();

    info!("Starting test: sign_pkp_with_lit_action_session_sigs");

    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    let pkp = end_user.pkp_by_pubkey(pubkey.clone());
    assert!(
        pkp.add_permitted_action_to_pkp(
            "QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz", // IPFS CID for `VALID_SESSION_SIG_LIT_ACTION_CODE`
            &[U256::from(AuthMethodScope::SignAnything as usize)]
        )
        .await
        .unwrap()
    );

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey.clone(),
        eth_address,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::PKP.to_string(),
            },
            ability: LitAbility::PKPSigning.to_string(),
        }],
        non_owner_wallet,
        None,
        Some(lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
        None,
    )
    .await
    .expect("Could not get session sigs");

    let pkp_signing_resp = get_pkp_sign(
        &node_set,
        Some(session_sigs),
        None,
        false,
        "Hello Lit".to_string(),
        pubkey,
    )
    .await;

    let signed_data = [
        134, 95, 114, 41, 198, 115, 171, 24, 245, 116, 158, 255, 141, 16, 61, 47, 54, 189, 142, 61,
        205, 85, 131, 39, 97, 86, 253, 25, 102, 251, 205, 246,
    ]; // Hello Lit encoded
    for resp in pkp_signing_resp.unwrap() {
        assert!(resp.ok);
        assert!(resp.data.is_some());
        assert_eq!(resp.data.as_ref().unwrap().signed_data, signed_data);
    }
}

#[doc = "Custom Authorization: Test that permitted Lit Action is allowed to create a sessionSig & only the other permitted Lit Action is allowed to sign with the PKP. To test this we use a Random wallet that doesn't own the PKP as the auth_method."]
#[tokio::test]
async fn sign_lit_actions_with_lit_action_session_sig() {
    info!("Starting test: sign_lit_actions_with_lit_action_session_sig");

    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let session_sig_lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    let pkp = end_user.pkp_by_pubkey(pubkey.clone());
    // For signing Session Key i.e. Personal Message
    assert!(
        pkp.add_permitted_action_to_pkp(
            "QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz", // IPFS CID for `VALID_SESSION_SIG_LIT_ACTION_CODE`
            &[U256::from(AuthMethodScope::SignPersonalMessage as usize)]
        )
        .await
        .unwrap()
    );

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey.clone(),
        eth_address,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        non_owner_wallet,
        None,
        Some(session_sig_lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
        None,
    )
    .await
    .expect("Could not get session sigs");

    // For signing inside Lit Actions i.e. signing anything
    assert!(
        pkp.add_permitted_action_to_pkp(
            "QmVKJuxhU5V9xNTD2kMKp9EoZwSuqFHmJvdDSHGDRpee9r", // IPFS CID for `VALID_PKP_SIGNING_LIT_ACTION_CODE`
            &[U256::from(AuthMethodScope::SignAnything as usize)]
        )
        .await
        .unwrap()
    );

    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        lit_action_params(VALID_PKP_SIGNING_LIT_ACTION_CODE.to_string(), pubkey)
            .await
            .expect("Could not get lit action params");

    let execute_resp = execute_lit_action_session_sigs(
        Some(lit_action_code),
        ipfs_id, // None
        js_params,
        auth_methods, // None
        &session_sigs_and_node_set,
        2,
    )
    .await
    .expect("Could not execute lit action");

    let action_result = assert_signed_action(&validator_collection, execute_resp).await;
    assert!(action_result.is_ok());

    let action_result = action_result.unwrap();
    assert!(action_result, "The action should have returned true");
}

#[doc = "Can't sign without being permitted despite having run a Lit Action sessionSig."]
#[tokio::test]
async fn only_permitted_can_sign_with_lit_action_session_sig() {
    info!("Starting test: only_permitted_can_sign_with_lit_action_session_sig");

    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let realm_id = U256::from(1);
    let epoch = validator_collection
        .actions()
        .get_current_epoch(realm_id)
        .await
        .as_u64();
    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let session_sig_lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    let pkp = end_user.pkp_by_pubkey(pubkey.clone());
    // For signing Session Key i.e. Personal Message
    assert!(
        pkp.add_permitted_action_to_pkp(
            "QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz", // IPFS CID for `VALID_SESSION_SIG_LIT_ACTION_CODE`
            &[U256::from(AuthMethodScope::SignPersonalMessage as usize)]
        )
        .await
        .unwrap()
    );

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey.clone(),
        eth_address,
        vec![
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::LA.to_string(),
                },
                ability: LitAbility::LitActionExecution.to_string(),
            },
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::PKP.to_string(),
                },
                ability: LitAbility::PKPSigning.to_string(),
            },
        ],
        non_owner_wallet,
        None,
        Some(session_sig_lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
        None,
    )
    .await
    .expect("Could not get session sigs");

    let (lit_action_code, ipfs_id, js_params, auth_methods) = lit_action_params(
        VALID_PKP_SIGNING_LIT_ACTION_CODE.to_string(),
        pubkey.clone(),
    )
    .await
    .expect("Could not get lit action params");

    let execute_resp = execute_lit_action_session_sigs(
        Some(lit_action_code),
        ipfs_id, // None
        js_params,
        auth_methods, // None
        &session_sigs_and_node_set,
        epoch,
    )
    .await
    .expect("Could not execute lit action");

    for resp in execute_resp {
        assert!(!resp.ok);
        assert!(resp.error.as_ref().unwrap().contains(
            "None of the AuthMethods, AuthSig or Lit Actions meet the required scope [1]"
        ));
    }

    let pkp_signing_resp = get_pkp_sign(
        &node_set,
        Some(session_sigs_and_node_set),
        None,
        false,
        "Hello Lit".to_string(),
        pubkey,
    )
    .await;

    for resp in pkp_signing_resp.unwrap() {
        assert!(!resp.ok);
        assert!(resp.error_object.as_ref().unwrap().contains(
            "None of the AuthMethods, AuthSig or Lit Actions meet the required scope [1]"
        ));
    }
}

#[doc = "Custom Authorization: Return Custom Auth Resource in executeJs. Test that permitted Lit Action is allowed to create a sessionSig & only the other permitted Lit Action is allowed to sign with the PKP. To test this we use a Random wallet that doesn't own the PKP as the auth_method."]
#[tokio::test]
async fn sign_lit_actions_with_custom_auth_resource_lit_action_session_sig() {
    info!("Starting test: sign_lit_actions_with_custom_auth_resource_lit_action_session_sig");

    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let session_sig_lit_action_code = data_encoding::BASE64.encode(
        CUSTOM_AUTH_RESOURCE_VALID_SESSION_SIG_LIT_ACTION_CODE
            .to_string()
            .as_bytes(),
    );

    let pkp = end_user.pkp_by_pubkey(pubkey.clone());
    // For signing Session Key i.e. Personal Message
    assert!(
        pkp.add_permitted_action_to_pkp(
            "QmRxUzYX52zEko9nvvtkdA6k8jU36enwwTVgW9ZwbdsUHY", // IPFS CID for `CUSTOM_AUTH_RESOURCE_VALID_SESSION_SIG_LIT_ACTION_CODE`
            &[U256::from(AuthMethodScope::SignPersonalMessage as usize)]
        )
        .await
        .unwrap()
    );

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey.clone(),
        eth_address,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        non_owner_wallet,
        None,
        Some(session_sig_lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
        None,
    )
    .await
    .expect("Could not get session sigs");

    // For signing inside Lit Actions i.e. signing anything
    assert!(
        pkp.add_permitted_action_to_pkp(
            "QmXVyzMZF1SFp6T284Mnuw1CW2VXq7Zbe1UZsX8kyXDgz5", // IPFS CID for `CUSTOM_AUTH_RESOURCE_VALID_PKP_SIGNING_LIT_ACTION_CODE`
            &[U256::from(AuthMethodScope::SignAnything as usize)]
        )
        .await
        .unwrap()
    );

    let (lit_action_code, ipfs_id, js_params, auth_methods) = lit_action_params(
        CUSTOM_AUTH_RESOURCE_VALID_PKP_SIGNING_LIT_ACTION_CODE.to_string(),
        pubkey,
    )
    .await
    .expect("Could not get lit action params");

    let execute_resp = execute_lit_action_session_sigs(
        Some(lit_action_code),
        ipfs_id, // None
        js_params,
        auth_methods, // None
        &session_sigs_and_node_set,
        2,
    )
    .await
    .expect("Could not execute lit action");

    let action_result = assert_signed_action(&validator_collection, execute_resp).await;
    assert!(action_result.is_ok());

    let action_result = action_result.unwrap();
    assert!(action_result, "The action should have returned true");
}

#[doc = "Custom Authorization: Create sessionSig with Custom Auth without providing any AuthMethods. Test pkpSign is allowed to it."]
#[tokio::test]
async fn sign_pkp_with_no_auth_method_lit_action_session_sig() {
    info!("Starting test: sign_pkp_with_no_auth_method_lit_action_session_sig");

    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let session_sig_lit_action_code = data_encoding::BASE64.encode(
        NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE
            .to_string()
            .as_bytes(),
    );

    let pkp = end_user.pkp_by_pubkey(pubkey.clone());
    assert!(
        pkp.add_permitted_action_to_pkp(
            "QmWLP9ojXrHJrFHnvMJv12HScFoz7R8kcYAECjtcpaJM2Y", // IPFS CID for `NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE`
            &[U256::from(AuthMethodScope::SignAnything as usize)]
        )
        .await
        .unwrap()
    );

    let mut js_params = serde_json::Map::new();
    js_params.insert("customAccessToken".to_string(), "lit".to_string().into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey.clone(),
        eth_address,
        vec![
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::LA.to_string(),
                },
                ability: LitAbility::LitActionExecution.to_string(),
            },
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::PKP.to_string(),
                },
                ability: LitAbility::PKPSigning.to_string(),
            },
        ],
        non_owner_wallet,
        None,
        Some(session_sig_lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
        None,
    )
    .await
    .expect("Could not get session sigs");

    let pkp_signing_resp = get_pkp_sign(
        &node_set,
        Some(session_sigs_and_node_set),
        None,
        false,
        "Hello Lit".to_string(),
        pubkey,
    )
    .await;

    let signed_data = [
        134, 95, 114, 41, 198, 115, 171, 24, 245, 116, 158, 255, 141, 16, 61, 47, 54, 189, 142, 61,
        205, 85, 131, 39, 97, 86, 253, 25, 102, 251, 205, 246,
    ]; // Hello Lit encoded
    for resp in pkp_signing_resp.unwrap() {
        assert!(resp.ok);
        assert_eq!(resp.data.as_ref().unwrap().signed_data, signed_data);
    }
}

#[doc = "Custom Authorization: Create sessionSig with Custom Auth without providing any AuthMethods. Use it to executeJs and sign within that Lit Action."]
#[tokio::test]
async fn sign_lit_actions_with_no_auth_method_lit_action_session_sig() {
    info!("Starting test: sign_lit_actions_with_no_auth_method_lit_action_session_sig");

    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let session_sig_lit_action_code = data_encoding::BASE64.encode(
        NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE
            .to_string()
            .as_bytes(),
    );

    let pkp = end_user.pkp_by_pubkey(pubkey.clone());
    // For signing Session Key i.e. Personal Message
    assert!(
        pkp.add_permitted_action_to_pkp(
            "QmWLP9ojXrHJrFHnvMJv12HScFoz7R8kcYAECjtcpaJM2Y", // IPFS CID for `NO_AUTH_METHOD_SESSION_SIG_LIT_ACTION_CODE`
            &[U256::from(AuthMethodScope::SignPersonalMessage as usize)]
        )
        .await
        .unwrap()
    );

    let mut js_params = serde_json::Map::new();
    js_params.insert("customAccessToken".to_string(), "lit".to_string().into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey.clone(),
        eth_address,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        non_owner_wallet,
        None,
        Some(session_sig_lit_action_code),
        Some(serde_json::Value::Object(js_params)),
        2,
        None,
    )
    .await
    .expect("Could not get session sigs");

    // For signing inside Lit Actions i.e. signing anything
    assert!(
        pkp.add_permitted_action_to_pkp(
            "QmZiUB1YmZBqXhjQswH2bE53GRCmcgYeKeGKN6bP1Zewy4", // IPFS CID for `NO_AUTH_METHOD_PKP_SIGNING_LIT_ACTION_CODE`
            &[U256::from(AuthMethodScope::SignAnything as usize)]
        )
        .await
        .unwrap()
    );

    let (lit_action_code, ipfs_id, js_params, auth_methods) = lit_action_params(
        NO_AUTH_METHOD_PKP_SIGNING_LIT_ACTION_CODE.to_string(),
        pubkey,
    )
    .await
    .expect("Could not get lit action params");

    let execute_resp = execute_lit_action_session_sigs(
        Some(lit_action_code),
        ipfs_id, // None
        js_params,
        auth_methods, // None
        &session_sigs_and_node_set,
        2,
    )
    .await
    .expect("Could not execute lit action");

    let action_result = assert_signed_action(&validator_collection, execute_resp).await;
    assert!(action_result.is_ok());

    let action_result = action_result.unwrap();
    assert!(action_result, "The action should have returned true");
}

#[doc = "Test pkpSign with EOA sessionSig."]
#[tokio::test]
async fn sign_pkp_with_eoa_session_sigs() {
    info!("Starting test: sign_pkp_with_eoa_session_sigs");

    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;

    info!("Node Set: {:?}", node_set);

    let wallet = end_user.wallet.clone();

    let (pubkey, _token_id, _eth_address) = end_user.first_pkp().info();

    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        vec![
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::PKP.to_string(),
                },
                ability: LitAbility::PKPSigning.to_string(),
            },
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::LA.to_string(),
                },
                ability: LitAbility::LitActionExecution.to_string(),
            },
        ],
        Some(wallet.clone()),
        None,
        None,
    );

    let pkp_signing_resp = get_pkp_sign(
        &node_set,
        Some(session_sigs_and_node_set),
        None,
        false,
        "Hello Lit".to_string(),
        pubkey,
    )
    .await;

    let signed_data = [
        134, 95, 114, 41, 198, 115, 171, 24, 245, 116, 158, 255, 141, 16, 61, 47, 54, 189, 142, 61,
        205, 85, 131, 39, 97, 86, 253, 25, 102, 251, 205, 246,
    ]; // Hello Lit encoded
    for resp in pkp_signing_resp.unwrap() {
        assert!(resp.ok);
        assert!(resp.data.is_some());
        assert_eq!(resp.data.as_ref().unwrap().signed_data, signed_data);
    }
}

#[doc = "Test executeJs with EOA sessionSig."]
#[tokio::test]
async fn execute_js_with_eoa_session_sigs() {
    info!("Starting test: execute_js_with_eoa_session_sigs");

    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let wallet = end_user.wallet.clone();

    let (pubkey, _token_id, _eth_address) = end_user.first_pkp().info();

    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        Some(wallet.clone()),
        None,
        None,
    );

    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        lit_action_params(HELLO_WORLD_LIT_ACTION_CODE.to_string(), pubkey)
            .await
            .expect("Could not get lit action params");

    let execute_resp = execute_lit_action_session_sigs(
        Some(lit_action_code),
        ipfs_id, // None
        js_params,
        auth_methods, // None
        &session_sigs_and_node_set,
        2,
    )
    .await
    .expect("Could not execute lit action");

    let action_result = assert_signed_action(&validator_collection, execute_resp).await;
    assert!(action_result.is_ok());

    let action_result = action_result.unwrap();
    assert!(action_result, "The action should have returned true");
}

#[doc = "Custom Decryption: Decrypt with PKP Wallet that run a Lit Action to create a sessionSig. This shows that the authentication can happen via PKP & the ACC is just the PKP Wallet Address."]
#[tokio::test]
async fn decrypt_with_lit_action_session_sig() {
    info!("Starting test: decrypt_with_lit_action_session_sig");

    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    let pkp = end_user.pkp_by_pubkey(pubkey.clone());
    assert!(
        pkp.add_permitted_action_to_pkp(
            "QmNZQXmY2VijUPfNrkC6zWykBnEniDouAeUpFi9r6aaqNz", // IPFS CID for `VALID_SESSION_SIG_LIT_ACTION_CODE`
            &[U256::from(AuthMethodScope::SignAnything as usize)]
        )
        .await
        .unwrap()
    );

    let test_encryption_params =
        prepare_test_encryption_parameters_with_wallet_address(encoding::bytes_to_hex(eth_address));

    let network_pubkey = lit_node_testnet::node_collection::get_network_pubkey_from_node_set(
        node_set.iter().map(|(n, _)| n),
    )
    .await;

    let message_bytes = test_encryption_params.to_encrypt.as_bytes();
    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: test_encryption_params.access_control_conditions.clone(),
        evm_contract_conditions: test_encryption_params.evm_contract_conditions.clone(),
        sol_rpc_conditions: test_encryption_params.sol_rpc_conditions.clone(),
        unified_access_control_conditions: test_encryption_params
            .unified_access_control_conditions
            .clone(),
    })
    .unwrap();
    let identity_param = AccessControlConditionResource::new(format!(
        "{}/{}",
        hashed_access_control_conditions, test_encryption_params.data_to_encrypt_hash
    ))
    .get_resource_key()
    .into_bytes();

    let bls_pubkey = blsful::PublicKey::try_from(&hex::decode(&network_pubkey).unwrap()).unwrap();
    // Encrypt
    let ciphertext =
        lit_sdk::encryption::encrypt_time_lock(&bls_pubkey, message_bytes, &identity_param)
            .expect("Unable to encrypt");
    info!("ciphertext: {:?}", ciphertext);

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let non_owner_wallet = LocalWallet::new(&mut OsRng);

    // Get session sig for auth
    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey.clone(),
        eth_address,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                // resource: "*".to_string(),
                resource: format!(
                    "{}/{}",
                    hashed_access_control_conditions, test_encryption_params.data_to_encrypt_hash
                ),
                resource_prefix: LitResourcePrefix::ACC.to_string(),
            },
            ability: LitAbility::AccessControlConditionDecryption.to_string(),
        }],
        non_owner_wallet.clone(),
        None,
        Some(lit_action_code.clone()),
        Some(serde_json::Value::Object(js_params.clone())),
        2,
        None,
    )
    .await
    .expect("Could not get session sigs");

    let epoch = validator_collection
        .actions()
        .get_current_epoch(U256::from(1))
        .await
        .as_u64();

    // Retrieve decrypted key
    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_params.clone(),
        &session_sigs_and_node_set,
        epoch,
    )
    .await;

    // Assert decryption
    assert_decrypted(
        &bls_pubkey,
        identity_param,
        &test_encryption_params.to_encrypt,
        &ciphertext,
        decryption_resp,
    );
}

#[doc = "V1 endpoints should not accept AuthSig."]
#[ignore]
#[tokio::test]
async fn test_v1_endpoints_api_constraints() {
    info!("Starting test: test_v1_endpoints_api_constraints");

    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let wallet = end_user.wallet.clone();
    let auth_sig = generate_authsig_item(&wallet).await.unwrap();

    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let signing_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    let resource = "*/*".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::LA);

    info!("Starting test: Can't provide Authsig/SessionSig to sign_session_key");
    let session_sig_resp = get_auth_sig_for_session_sig_from_nodes(
        &node_set,
        &auth_sig,
        true,
        &eth_address.to_fixed_bytes(),
        &pubkey,
        session_pub_key.clone(),
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        None,
        None,
        2, // Hardcoded as at other places in the tests
    )
    .await
    .expect("Could not get SessionKey signed");

    assert!(!session_sig_resp[0].ok);

    info!("Starting test: Can't provide Authsig to pkp_sign");
    let pkp_signing_resp = get_pkp_sign(
        &node_set,
        None,
        Some(auth_sig.clone()),
        false,
        "Hello Lit".to_string(),
        pubkey.clone(),
    )
    .await
    .expect("Could not get PKP sign");

    assert!(!pkp_signing_resp[0].ok);
    assert!(
        pkp_signing_resp[0]
            .error_object
            .as_ref()
            .unwrap()
            .contains("NodeCannotProvideAuthSigForEndpoint")
    );

    info!("Starting test: Can't provide AuthMethod to pkp_sign");
    let pkp_signing_resp = get_pkp_sign(
        &node_set,
        None,
        Some(auth_sig.clone()),
        true,
        "Hello Lit".to_string(),
        pubkey.clone(),
    )
    .await
    .expect("Could not get PKP sign");

    assert!(!pkp_signing_resp[0].ok);
    assert!(
        pkp_signing_resp[0]
            .error_object
            .as_ref()
            .unwrap()
            .contains("NodeCannotProvideAuthMethodForEndpoint")
    );

    info!("Starting test: Can't provide Authsig to execute_js");
    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        lit_action_params(HELLO_WORLD_LIT_ACTION_CODE.to_string(), pubkey.clone())
            .await
            .expect("Could not get lit action params");

    let realm_id = U256::from(1);
    let epoch = validator_collection
        .actions()
        .get_current_epoch(realm_id)
        .await
        .as_u64();

    let execute_resp = execute_lit_action_auth_sig(
        &node_set,
        Some(lit_action_code),
        ipfs_id, // None
        js_params,
        auth_methods, // None
        auth_sig.clone(),
        epoch,
    )
    .await;

    assert!(!execute_resp[0].ok);

    info!("Starting test: Can't provide AuthMethod to execute_js");
    let (lit_action_code, ipfs_id, js_params, _auth_methods) =
        lit_action_params(HELLO_WORLD_LIT_ACTION_CODE.to_string(), pubkey)
            .await
            .expect("Could not get lit action params");

    let auth_methods = Some(vec![AuthMethod {
        auth_method_type: 1,
        access_token: serde_json::to_string(&auth_sig).unwrap(),
    }]);

    let execute_resp = execute_lit_action_auth_sig(
        &node_set,
        Some(lit_action_code),
        ipfs_id, // None
        js_params,
        auth_methods,
        auth_sig,
        epoch,
    )
    .await;

    assert!(!execute_resp[0].ok);
}

#[doc = "Initial signSessionKey: AuthMethod with permitted Address and permitted AuthMethods (Address with lit suffixed)"]
#[tokio::test]
async fn sign_session_key_auth_method() {
    info!("Starting test: sign_session_key_auth_method");

    let (_testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;

    let (pubkey, _token_id, eth_address) = end_user.first_pkp().info();

    let signing_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    // Resources don't matter here
    let resource = "*/*".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::PKP);

    // Permit address_to_permit_wallet address
    info!("Permit bare address");
    let address_to_permit_wallet = LocalWallet::new(&mut OsRng);
    let address_to_permit_auth_sig = generate_authsig_item(&address_to_permit_wallet)
        .await
        .unwrap();

    let pkp = end_user.first_pkp();
    let _is_permitted_address_to_permit_wallet = pkp
        .add_permitted_address_to_pkp(address_to_permit_wallet.address(), &[U256::from(1)])
        .await
        .expect("Could not add permitted address to pkp");

    let address_to_permit_signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &node_set,
        &address_to_permit_auth_sig,
        false,
        &eth_address.to_fixed_bytes(),
        &pubkey,
        session_pub_key.clone(),
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        None,
        None,
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let address_to_permit_signing_resp = address_to_permit_signing_resp.unwrap();
    for response in &address_to_permit_signing_resp {
        assert!(response.ok);
    }

    info!("Permit lower case suffixed address");
    let lowercase_auth_method_to_permit_wallet = LocalWallet::new(&mut OsRng);
    let lowercase_auth_method_to_permit_auth_sig =
        generate_authsig_item(&lowercase_auth_method_to_permit_wallet)
            .await
            .unwrap();

    let lowercase_auth_method_to_permit_wallet_address = &format!(
        "0x{}",
        hex::encode(lowercase_auth_method_to_permit_wallet.address().as_bytes())
    ); // Need to add "0x" explicitly

    let is_permitted_lowercase_auth_method_to_permit_wallet = pkp
        .add_permitted_address_auth_method_to_pkp(
            get_user_wallet_auth_method_from_address(
                lowercase_auth_method_to_permit_wallet_address,
            )
            .unwrap(),
            &[U256::from(AuthMethodScope::SignPersonalMessage as usize)],
        )
        .await;
    assert!(is_permitted_lowercase_auth_method_to_permit_wallet.unwrap());

    let lowercase_auth_method_to_permit_signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &node_set,
        &lowercase_auth_method_to_permit_auth_sig,
        false,
        &eth_address.to_fixed_bytes(),
        &pubkey,
        session_pub_key.clone(),
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        None,
        None,
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let lowercase_auth_method_to_permit_signing_resp =
        lowercase_auth_method_to_permit_signing_resp.unwrap();
    for response in &lowercase_auth_method_to_permit_signing_resp {
        assert!(response.ok);
    }

    info!("Permit checked sum suffixed address");
    let checkedsum_auth_method_to_permit_wallet = LocalWallet::new(&mut OsRng);
    let checkedsum_auth_method_to_permit_auth_sig =
        generate_authsig_item(&checkedsum_auth_method_to_permit_wallet)
            .await
            .unwrap();

    let checkedsum_address =
        get_wallet_address_from_auth_sig_item(&checkedsum_auth_method_to_permit_auth_sig);
    assert!(checkedsum_address.is_ok());

    let is_permitted_checkedsum_auth_method_to_permit_wallet = pkp
        .add_permitted_address_auth_method_to_pkp(
            // get_user_wallet_auth_method_from_address(lowercase_auth_method_to_permit_wallet_address).unwrap(),
            get_user_wallet_auth_method_from_address(&checkedsum_address.unwrap()).unwrap(),
            &[U256::from(AuthMethodScope::SignPersonalMessage as usize)],
        )
        .await;
    assert!(is_permitted_checkedsum_auth_method_to_permit_wallet.unwrap());

    let checkedsum_auth_method_to_permit_signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &node_set,
        &checkedsum_auth_method_to_permit_auth_sig,
        false,
        &eth_address.to_fixed_bytes(),
        &pubkey,
        session_pub_key.clone(),
        vec![resource.clone()],
        vec![resource_prefix.clone()],
        None,
        None,
        2, // Hardcoded as at other places in the tests
    )
    .await;

    let checkedsum_auth_method_to_permit_signing_resp =
        checkedsum_auth_method_to_permit_signing_resp.unwrap();
    for response in &checkedsum_auth_method_to_permit_signing_resp {
        assert!(response.ok);
    }
}

fn get_wallet_address_from_auth_sig_item(auth_sig_item: &AuthSigItem) -> Result<String> {
    match auth_sig_item {
        AuthSigItem::Single(json_auth_sig) => match json_auth_sig.auth_material_type {
            AuthMaterialType::WalletSig => Ok(json_auth_sig.address.clone()),
            _ => Err(anyhow::anyhow!("Can only pass Wallet Sig")),
        },
        AuthSigItem::Multiple(_) => Err(anyhow::anyhow!("Can't pass multiple AuthSigs")),
    }
}

#[doc = "MGB PKP can be used only for creating a sessionSig but not for signing. It can be used to sign within another MGB PKP's permitted Lit Action"]
#[tokio::test]
pub async fn session_sig_only_mbg_pkp() {
    info!("Starting test: session_sig_only_mbg_pkp");

    let (testnet, validator_collection, end_user) = init_test().await;
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let realm_id = ethers::types::U256::from(1);
    let actions = validator_collection.actions();
    let epoch = actions.get_current_epoch(realm_id).await.as_u64();

    info!("MGB PKP for sessionSig");

    // Simply returns the response `LitActions.setResponse({response:"true"});`. We can add any custom auth logic we want in this Lit Action
    // Even though the PKP has `scope = 1` since the permitted Lit Action doesn't use `signEcdsa()` it can't be used for signing
    let ipfs_cid = "QmUvLFoQggpYsVaPs8Wig6CyTb8GtTwHfHhDsrvNBjFVLP"; // MGB_PKP_SESSION_SIG_LIT_ACTION_CODE

    let mgb_pkp = end_user
        .mint_grant_and_burn_next_pkp(ipfs_cid)
        .await
        .unwrap();

    let wallet_balance = end_user.get_wallet_balance().await;
    let fund_balance = wallet_balance / 10;
    end_user.deposit_to_pkp_ledger(&mgb_pkp, fund_balance).await;

    let auth_pubkey = mgb_pkp.pubkey;
    let auth_eth_address = mgb_pkp.eth_address;

    info!(
        "Funded MGB PKP {:?} with {:?}",
        auth_eth_address, fund_balance
    );

    info!("MGB PKP used for generating sessionSig");
    let non_owner_wallet = LocalWallet::new(&mut OsRng).with_chain_id(testnet.chain_id);

    let session_sig_lit_action_code =
        data_encoding::BASE64.encode(MGB_PKP_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());
    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        auth_pubkey.clone(),
        auth_eth_address.into(),
        vec![
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: "lit-litaction".to_string(),
                },
                ability: LitAbility::LitActionExecution.to_string(),
            },
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: "lit-pkp".to_string(),
                },
                ability: LitAbility::PKPSigning.to_string(),
            },
        ],
        non_owner_wallet,
        None,
        Some(session_sig_lit_action_code),
        None,
        2,
        Some(U256::MAX),
    )
    .await
    .expect("Could not get session sigs");

    info!("MGB PKP can't be used for signing even by the original minter since it has been burnt");
    let to_sign = vec![
        84, 104, 105, 115, 32, 109, 101, 115, 115, 97, 103, 101, 32, 105, 115, 32, 101, 120, 97,
        99, 116, 108, 121, 32, 51, 50, 32, 98, 121, 116, 101, 115,
    ];

    let signing_resp = generate_session_sigs_and_send_signing_requests(
        &node_set,
        end_user.signing_provider().signer().clone(), // Original minter
        to_sign.clone(),
        auth_pubkey.clone(),
        epoch,
        SigningScheme::EcdsaK256Sha256,
    )
    .await;

    for resp in signing_resp {
        assert!(!resp.ok);
        assert!(resp.error_object.as_ref().unwrap().contains(
            "None of the AuthMethods, AuthSig or Lit Actions meet the required scope [1]"
        ));
    }

    info!("MGB PKP for signing");
    let ipfs_cid = "QmRwN9GKHvCn4Vk7biqtr6adjXMs7PzzYPCzNCRjPFiDjm";
    let mgb_pkp_info = end_user
        .mint_grant_and_burn_next_pkp(ipfs_cid)
        .await
        .unwrap();
    let mgb_pubkey = mgb_pkp_info.pubkey;

    info!("Can use a MGB PKP to sign within the Permitted Lit Action of a different MGB PKP");
    let mut js_params = serde_json::Map::new();
    js_params.insert(
        "toSign".to_string(),
        [
            84, 104, 105, 115, 32, 109, 101, 115, 115, 97, 103, 101, 32, 105, 115, 32, 101, 120,
            97, 99, 116, 108, 121, 32, 51, 50, 32, 98, 121, 116, 101, 115,
        ]
        .into(),
    );
    js_params.insert("publicKey".to_string(), mgb_pubkey.into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let js_params = Some(serde_json::Value::Object(js_params));

    let _execute_resp = execute_lit_action_session_sigs(
        None,
        Some(ipfs_cid.to_string()),
        js_params,
        None,
        &session_sigs_and_node_set,
        2,
    )
    .await
    .expect("Could not execute lit action");
}

#[doc = "Can only run a permitted Lit Action when the resource is explicitly permitted"]
#[tokio::test]
async fn explicit_resource_permission_required_for_lit_action() {
    info!("Starting test: explicit_resource_permission_required_for_lit_action");

    let (_testnet, validator_collection, end_user) = init_test().await;

    let (pubkey, _token_id, _eth_address) = end_user.first_pkp().info();

    // the lit action we're going to test is VALID_PKP_SIGNING_LIT_ACTION_CODE
    // so let's derive the IPFS CID for it
    let lit_action_code = SIGN_ECDSA_LIT_ACTION_CODE.to_string();
    let ipfs_hasher = IpfsHasher::default();
    let ipfs_cid = ipfs_hasher.compute(lit_action_code.as_bytes());
    info!("IPFS CID that is being permitted: {:?}", ipfs_cid);

    let pkp = end_user.pkp_by_pubkey(pubkey.clone());
    // For signing Session Key i.e. Personal Message
    assert!(
        pkp.add_permitted_action_to_pkp(
            &ipfs_cid,
            &[U256::from(AuthMethodScope::SignAnything as usize)]
        )
        .await
        .unwrap()
    );

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    // get local session sigs for non owner wallet
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
    let session_sigs = get_session_sigs_for_auth(
        &node_set,
        vec![
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: "*".to_string(),
                    resource_prefix: LitResourcePrefix::PKP.to_string(),
                },
                ability: LitAbility::PKPSigning.to_string(),
            },
            LitResourceAbilityRequest {
                resource: LitResourceAbilityRequestResource {
                    resource: ipfs_cid.to_string(),
                    resource_prefix: LitResourcePrefix::LA.to_string(),
                },
                ability: LitAbility::LitActionExecution.to_string(),
            },
        ],
        Some(end_user.wallet.clone()),
        None,
        None,
    );

    let (lit_action_code, ipfs_id, js_params, auth_methods) =
        lit_action_params(SIGN_ECDSA_LIT_ACTION_CODE.to_string(), pubkey.clone())
            .await
            .expect("Could not get lit action params");

    let execute_resp = execute_lit_action_session_sigs(
        Some(lit_action_code),
        ipfs_id, // None
        js_params,
        auth_methods, // None
        &session_sigs,
        2,
    )
    .await
    .expect("Could not execute lit action");

    let action_result = assert_signed_action(&validator_collection, execute_resp).await;
    assert!(action_result.is_ok());

    let action_result = action_result.unwrap();
    assert!(
        action_result == true,
        "The action should have returned true"
    );
}
