use crate::common::auth_sig::get_session_sigs_for_auth;
use crate::common::ecdsa::simple_single_sign_with_hd_key;
use crate::common::pkp::{
    decode_endpoint_responses, generate_session_sigs_and_send_signing_requests,
};
use crate::common::web_user_tests::{
    assert_decrypted, prepare_test_encryption_parameters,
    retrieve_decryption_key_session_sigs_with_version,
};
use ethers::prelude::U256;
use lit_node::models::RequestConditions;
use lit_node::utils::web::hash_access_control_conditions;
use lit_node_core::{
    AccessControlConditionResource, LitAbility, LitResource, LitResourceAbilityRequest,
    LitResourceAbilityRequestResource, SigningScheme, response::JsonPKPSigningResponse,
};
use lit_node_testnet::TestSetupBuilder;
use lit_node_testnet::end_user::EndUser;
use lit_node_testnet::node_collection::{get_identity_pubkeys_from_node_set, get_network_pubkey};
use lit_node_testnet::testnet::Testnet;
use lit_node_testnet::validator::ValidatorCollection;
use lit_sdk::signature::combine_and_verify_signature_shares;
const INITIAL_VALIDATORS: usize = 5;
const MAX_VALIDATORS: usize = 10;
const EPOCH_LENGTH: usize = 300;

#[tokio::test]
async fn shadow_splicing_sign_encrypt() {
    let (_testnet, validator_collection, end_user) = load_network().await;

    let pubkey = end_user.first_pkp().pubkey.clone();
    let actions = validator_collection.actions();

    let realm_id = 1u64;
    let new_realm_id = actions.add_realm().await.unwrap();

    info!("New realm ID: {}", new_realm_id);

    let inactive_validators = validator_collection
        .get_inactive_validators()
        .await
        .unwrap();
    info!(
        "Validators in Realm {}: {:?}",
        realm_id,
        validator_collection
            .get_active_validators()
            .await
            .unwrap()
            .iter()
            .map(|v| v.node_address())
    );
    info!(
        "Validators in Realm {}: {:?}",
        new_realm_id,
        inactive_validators.iter().map(|v| v.node_address())
    );

    let target_validators = inactive_validators
        .iter()
        .map(|v| v.account().staker_address)
        .collect::<Vec<_>>();

    assert!(
        simple_single_sign_with_hd_key(
            &validator_collection,
            &end_user,
            pubkey.clone(),
            SigningScheme::SchnorrEd25519Sha512,
            &vec![]
        )
        .await,
        "Failed to sign with first realm."
    );

    info!(
        "Target validators for shadow splicing: {:?}",
        target_validators
    );
    info!(
        "Setting up shadow splicing with source realm {} and target realm {} for {} validators",
        realm_id,
        new_realm_id,
        target_validators.len()
    );

    let _result = actions
        .setup_shadow_splicing(realm_id, new_realm_id, target_validators.clone())
        .await
        .unwrap();
    info!("Shadow splicing has started.");

    let _result = actions
        .wait_for_shadow_splicing_to_complete(new_realm_id, target_validators)
        .await
        .unwrap();

    // Now with two realms we perform the following tests:
    // 1. Sign a message with the original realm and verify it works
    // 2. Sign a message with the new realm and verify it works with the same pubkey
    // 3. Verify that signature shares from both realms cannot be combined to produce a valid signature for the same message
    // 4. Encrypt a message decrypt it with both realms
    let realm1_id = U256::from(realm_id);
    let realm2_id = U256::from(new_realm_id);
    let msg_to_sign = b"Test message for shadow splicing".to_vec();

    // Test 1
    info!("Testing signature from realm 1");
    let realm1_responses = signature_from_realm(
        &validator_collection,
        &end_user,
        &msg_to_sign,
        pubkey.clone(),
        realm1_id,
        SigningScheme::SchnorrEd25519Sha512,
    )
    .await;

    // Test 2
    info!("Testing signature from realm 2");
    let realm2_responses = signature_from_realm(
        &validator_collection,
        &end_user,
        &msg_to_sign,
        pubkey.clone(),
        realm2_id,
        SigningScheme::SchnorrEd25519Sha512,
    )
    .await;

    // Test 3
    info!("Testing signature shares from both realms");
    let mut shares = Vec::with_capacity(4);
    for r in &realm1_responses[..2] {
        shares.push(r.signature_share.clone());
    }
    for r in &realm2_responses[..2] {
        shares.push(r.signature_share.clone());
    }
    assert!(
        combine_and_verify_signature_shares(&shares).is_err(),
        "Combined signature shares from both realms should not produce a valid signature"
    );

    // Test 4
    info!("Testing encryption with both realms");
    let network_pubkey = get_network_pubkey(actions).await;
    let test_encryption_parameters = prepare_test_encryption_parameters();
    let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
        access_control_conditions: test_encryption_parameters.access_control_conditions.clone(),
        evm_contract_conditions: test_encryption_parameters.evm_contract_conditions.clone(),
        sol_rpc_conditions: test_encryption_parameters.sol_rpc_conditions.clone(),
        unified_access_control_conditions: test_encryption_parameters
            .unified_access_control_conditions
            .clone(),
    })
    .unwrap();
    let identity_param = AccessControlConditionResource::new(format!(
        "{}/{}",
        hashed_access_control_conditions, test_encryption_parameters.data_to_encrypt_hash
    ))
    .get_resource_key()
    .into_bytes();

    let pubkey = blsful::PublicKey::try_from(hex::decode(&network_pubkey).unwrap()).unwrap();

    let ciphertext = lit_sdk::encryption::encrypt_time_lock(
        &pubkey,
        test_encryption_parameters.to_encrypt.as_bytes(),
        &identity_param,
    )
    .expect("Unable to encrypt");

    let signer = end_user.signing_provider();

    let resource_ability_request = LitResourceAbilityRequest {
        resource: LitResourceAbilityRequestResource {
            resource: format!(
                "{}/{}",
                hashed_access_control_conditions, test_encryption_parameters.data_to_encrypt_hash
            ),
            resource_prefix: "lit-accesscontrolcondition".to_string(),
        },
        ability: LitAbility::AccessControlConditionDecryption.to_string(),
    };

    let realm1_nodes = validator_collection
        .random_threshold_nodeset_with_realm_id(realm1_id.as_u64(), &vec![])
        .await;
    let realm1_node_set = get_identity_pubkeys_from_node_set(&realm1_nodes).await;

    let realm2_nodes = validator_collection
        .random_threshold_nodeset_with_realm_id(realm2_id.as_u64(), &vec![])
        .await;
    let realm2_node_set = get_identity_pubkeys_from_node_set(&realm2_nodes).await;

    // Get session sig for auth
    let session_sigs_realm_1 = get_session_sigs_for_auth(
        &realm1_node_set,
        vec![resource_ability_request.clone()],
        Some(signer.signer().clone()),
        None,
        Some(U256::MAX), // max_price
    );
    let session_sigs_realm_2 = get_session_sigs_for_auth(
        &realm2_node_set,
        vec![resource_ability_request.clone()],
        Some(signer.signer().clone()),
        None,
        Some(U256::MAX), // max_price
    );

    let epoch = validator_collection
        .actions()
        .get_current_epoch(realm1_id)
        .await
        .as_u64();
    let decryption_resp_realm1 = retrieve_decryption_key_session_sigs_with_version(
        test_encryption_parameters.clone(),
        &session_sigs_realm_1,
        epoch,
    )
    .await;

    let epoch = validator_collection
        .actions()
        .get_current_epoch(realm2_id)
        .await
        .as_u64();
    let decryption_resp_realm2 = retrieve_decryption_key_session_sigs_with_version(
        test_encryption_parameters.clone(),
        &session_sigs_realm_2,
        epoch,
    )
    .await;
    assert_decrypted(
        &pubkey,
        identity_param.clone(),
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp_realm1,
    );
    assert_decrypted(
        &pubkey,
        identity_param,
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp_realm2,
    );
}

/// Tests that shadow splicing works correctly
/// and both realms can DKG when the epoch changes.
#[tokio::test]
async fn shadow_splicing_epoch() {
    let (_testnet, validator_collection, end_user) = load_network().await;

    let actions = validator_collection.actions();
    let pubkey = end_user.first_pkp().pubkey.clone();
    let realm_id = 1u64;
    let new_realm_id = actions.add_realm().await.unwrap();

    let inactive_validators = validator_collection
        .get_inactive_validators()
        .await
        .unwrap();

    let target_validators = inactive_validators
        .iter()
        .map(|v| v.account().staker_address)
        .collect::<Vec<_>>();

    assert!(
        simple_single_sign_with_hd_key(
            &validator_collection,
            &end_user,
            pubkey.clone(),
            SigningScheme::SchnorrEd25519Sha512,
            &vec![]
        )
        .await,
        "Failed to sign with first realm."
    );

    info!(
        "Target validators for shadow splicing: {:?}",
        target_validators
    );
    info!(
        "Setting up shadow splicing with source realm {} and target realm {} for {} validators",
        realm_id,
        new_realm_id,
        target_validators.len()
    );

    let _result = actions
        .setup_shadow_splicing(realm_id, new_realm_id, target_validators.clone())
        .await
        .unwrap();
    info!("Shadow splicing has started.");

    let _result = actions
        .wait_for_shadow_splicing_to_complete(new_realm_id, target_validators)
        .await
        .unwrap();

    actions.increase_blockchain_timestamp(EPOCH_LENGTH).await;

    let realm_id = U256::from(realm_id);
    let realm_epoch = actions.get_current_epoch(realm_id).await;
    let realm_epoch_length = actions.get_epoch_length(realm_id).await.unwrap();
    info!(
        "Old realm current epoch {}, length: {}",
        realm_epoch.as_u64(),
        realm_epoch_length.as_u64()
    );

    actions.wait_for_epoch(realm_id, realm_epoch + 1).await;

    let new_realm_id = U256::from(new_realm_id);
    let new_realm_epoch = actions.get_current_epoch(new_realm_id).await;
    let new_realm_epoch_length = actions.get_epoch_length(new_realm_id).await.unwrap();
    info!(
        "New realm current epoch {}, length: {}",
        new_realm_epoch.as_u64(),
        new_realm_epoch_length.as_u64()
    );
}

async fn signature_from_realm(
    validator_collection: &ValidatorCollection,
    end_user: &EndUser,
    msg_to_sign: &[u8],
    pubkey: String,
    realm_id: U256,
    scheme: SigningScheme,
) -> Vec<JsonPKPSigningResponse> {
    let signer = end_user.signing_provider();
    let epoch = validator_collection
        .actions()
        .get_current_epoch(realm_id)
        .await
        .as_u64();
    let realm_nodes = validator_collection
        .random_threshold_nodeset_with_realm_id(realm_id.as_u64(), &vec![])
        .await;
    // Verify that the new realm produces a valid signature just like the original realm
    tracing::info!("NodeSet: {:?}, signing_scheme: {}", realm_nodes, scheme,);
    let realm_node_set = get_identity_pubkeys_from_node_set(&realm_nodes).await;

    let expected_responses = realm_node_set.len();

    let endpoint_responses = generate_session_sigs_and_send_signing_requests(
        &realm_node_set,
        signer.signer().clone(),
        msg_to_sign.to_vec(),
        pubkey.clone(),
        epoch,
        scheme,
    )
    .await;
    assert!(endpoint_responses.len() >= expected_responses);
    assert!(
        endpoint_responses.iter().all(|e| e.ok),
        "Some of the endpoints failed to respond correctly"
    );
    let responses: Vec<JsonPKPSigningResponse> = endpoint_responses
        .into_iter()
        .map(|x| {
            assert!(x.ok);
            assert!(x.data.is_some());
            x.data.unwrap()
        })
        .collect();

    // Checks that the responses are valid and the signature verifies
    let _realm_signed_output = decode_endpoint_responses(responses.clone());
    responses
}

async fn load_network() -> (Testnet, ValidatorCollection, EndUser) {
    crate::common::setup_logging();

    let (testnet, validator_collection, end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(INITIAL_VALIDATORS)
        .num_staked_only_validators(MAX_VALIDATORS - INITIAL_VALIDATORS)
        .register_inactive_validators(true)
        .epoch_length(EPOCH_LENGTH)
        .build()
        .await;

    info!("Setting up contracts");

    info!(
        "Validator collection: {:?}",
        validator_collection.addresses()
    );

    (testnet, validator_collection, end_user)
}
