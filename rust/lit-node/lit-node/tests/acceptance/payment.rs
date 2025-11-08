use crate::common::auth_sig::{
    generate_authsig_item, get_auth_sig_for_session_sig_from_nodes,
    get_auth_sig_with_payment_resources,
};
use crate::common::auth_sig::{get_session_sigs_and_node_set_for_pkp, get_session_sigs_for_auth};
use crate::common::lit_actions::{INFINITE_LOOP_LIT_ACTION_CODE, INVALID_LIT_ACTION_CODE};
use crate::common::session_sigs::VALID_SESSION_SIG_LIT_ACTION_CODE;
use crate::common::web_user_tests::{
    assert_decrypted, generate_session_sigs_execute_lit_action, prepare_test_encryption_parameters,
    retrieve_decryption_key_session_sigs,
};
use ethers::signers::Signer;
use ethers::types::{I256, U256};
use lit_node::payment::payed_endpoint::PayedEndpoint;
use lit_node::utils::encoding;
use lit_node::utils::encoding::bytes_to_hex;
use lit_node_core::{
    LitAbility, LitResourceAbilityRequest, LitResourceAbilityRequestResource, LitResourcePrefix,
    NodeSet,
};
use lit_node_testnet::TestSetupBuilder;
use lit_node_testnet::node_collection::{get_identity_pubkeys_from_node_set, get_network_pubkey};
use lit_node_testnet::testnet::actions::Actions;
use lit_node_testnet::{end_user::EndUser, testnet::Testnet, validator::ValidatorCollection};
use rand_core::OsRng;

// Constants for better code practice
const NUM_STAKED_VALIDATORS: usize = 3;
const EPOCH_LENGTH_SECONDS: u64 = 300;
const INITIAL_FUNDING_AMOUNT: &str = "100000000000000000000";
const LIT_ACTION_MIN_ESTIMATE: u64 = 49800000000000000;
const PAYMENT_PERIOD_SECONDS: u64 = 10;
const MAX_REQUESTS_PER_PERIOD: u64 = 3;
const MAX_TEST_REQUESTS: usize = 7;
const WAIT_TIME_BALANCE_CHECK_SECONDS: u64 = 3;
const WAIT_TIME_PAYMENT_DB_PERIOD_SECONDS: u64 = 11;
const PRICE_MULTIPLIER_MULTIPLE_REQUESTS: u64 = 100;
const DIVISION_FACTOR_TO_FAIL: u64 = 2;
// Any amount that is enough to run Lit Action

#[tokio::test]
async fn test_all_payment_methods_for_user() {
    // Network set-up
    crate::common::setup_logging();
    let (testnet, validator_collection, actions, node_set) = setup_testnet_for_payments().await;
    let realm_id = ethers::types::U256::from(1);
    // Encryption
    let test_encryption_parameters = prepare_test_encryption_parameters();
    let network_pubkey = get_network_pubkey(&actions).await;
    let message_bytes = test_encryption_parameters.to_encrypt.as_bytes();
    let pubkey = blsful::PublicKey::try_from(&hex::decode(&network_pubkey).unwrap()).unwrap();
    let ciphertext = lit_sdk::encryption::encrypt_time_lock(
        &pubkey,
        message_bytes,
        &test_encryption_parameters.identity_param,
    )
    .expect("Unable to encrypt");

    let resource_ability_requests = vec![LitResourceAbilityRequest {
        resource: LitResourceAbilityRequestResource {
            resource: format!(
                "{}/{}",
                test_encryption_parameters.hashed_access_control_conditions,
                test_encryption_parameters.data_to_encrypt_hash
            ),
            resource_prefix: LitResourcePrefix::ACC.to_string(),
        },
        ability: LitAbility::AccessControlConditionDecryption.to_string(),
    }];

    let mut self_pay_user = EndUser::new(&testnet);

    // 1. Self-paying SessionSig
    info!("1 - Self-Paying: Testing that the user can pay for themselves");

    let first_node_price = self_pay_user.first_node_price_from_feed(0).await;

    // 1.1. Max price < Current price
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;

    info!("1.1 - Self-Paying: Fail as max_price < current_price");
    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(self_pay_user.wallet.clone()),
        None,
        Some(first_node_price / DIVISION_FACTOR_TO_FAIL),
    );

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;
    assert!(
        !decryption_resp[0].ok,
        "Expected an error, but got a successful response"
    );
    assert!(
        decryption_resp[0]
            .error_object
            .as_ref()
            .unwrap()
            .contains("is less than the endpoint price"),
        "Should not be able to decrypt if max_price is less than the current price"
    );

    // 1.2. Max price > User's Ledger balance
    info!("1.2 - Self-Paying: Fail as max_price > Ledger balance");
    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(self_pay_user.wallet.clone()),
        None,
        Some(U256::MAX),
    );

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;

    assert!(
        !decryption_resp[0].ok,
        "Expected an error, but got a successful response"
    );
    let error = decryption_resp[0].error_object.as_ref().unwrap();
    assert!(
        error.contains(
            "balance 0 minus their pending spending of 0 is not enough to cover the minimum estimated price"
        ),
        "Should not be able to decrypt if user doesn't have the required balance"
    );
    assert!(
        error.contains("No payer in the Payment DB;  No Capacity Delegation;  Self failed to pay:"),
        "Should not be able to decrypt if all the payment methods fail"
    );

    // 1.3. Max price ≥ Current price
    info!("1.3 - Self-Paying: Pass as max_price ≥ current_price");
    self_pay_user
        .set_wallet_balance(INITIAL_FUNDING_AMOUNT)
        .await;

    self_pay_user
        .deposit_to_wallet_ledger(first_node_price * NUM_STAKED_VALIDATORS)
        .await;

    let self_pay_user_ledger_balance = self_pay_user
        .get_wallet_ledger_balance("User Ledger Balance after the deposit")
        .await;

    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(self_pay_user.wallet.clone()),
        None,
        Some(first_node_price),
    );

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;

    assert_decrypted(
        &pubkey,
        test_encryption_parameters.identity_param.clone(),
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp,
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_BALANCE_CHECK_SECONDS,
    ))
    .await;

    let self_pay_user_ledger_balance_after = self_pay_user
        .get_wallet_ledger_balance("User Ledger Balance after the request")
        .await;

    assert!(
        self_pay_user_ledger_balance_after < self_pay_user_ledger_balance,
        "User balance didn't decrease after the successful request"
    );

    // 1.4. Balance == request to withdraw
    info!("1.4 - Self-Paying: Fail as balance is requested to be withdrawn");

    let amount = first_node_price * NUM_STAKED_VALIDATORS;
    self_pay_user.deposit_to_wallet_ledger(amount).await;

    self_pay_user
        .ledger_request_withdraw(
            I256::from_raw(amount) + self_pay_user_ledger_balance_after - 1,
            "Requested to withdraw the whole balance",
        )
        .await;

    let self_pay_user_ledger_stable_balance = self_pay_user
        .get_wallet_ledger_stable_balance("User Ledger *Stable* Balance")
        .await;

    assert_eq!(self_pay_user_ledger_stable_balance, I256::from(1));

    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(self_pay_user.wallet.clone()),
        None,
        Some(first_node_price),
    );

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;

    assert!(
        !decryption_resp[0].ok,
        "Expected an error, but got a successful response"
    );
    let error = decryption_resp[0].error_object.as_ref().unwrap();
    assert!(
        error.contains(
            "balance 1 minus their pending spending of 0 is not enough to cover the minimum estimated price"
        ),
        "Should not be able to decrypt if user doesn't have the required balance"
    );

    // 1.5. Charge for a failing request
    info!("1.5 - Self-Paying: Charge for a failing request");
    self_pay_user
        .set_wallet_balance(INITIAL_FUNDING_AMOUNT)
        .await;
    assert!(self_pay_user.new_pkp().await.is_ok(), "Failed to mint PKP");

    // The price for the sign_session_key
    self_pay_user
        .deposit_to_first_pkp_ledger(first_node_price * NUM_STAKED_VALIDATORS)
        .await;

    // The price for lit-action
    self_pay_user
        .deposit_to_first_pkp_ledger(U256::from(LIT_ACTION_MIN_ESTIMATE))
        .await;

    let pkp_ledger_balance = self_pay_user
        .get_first_pkp_ledger_balance("Pkp Ledger Balance after the deposit")
        .await;

    let execute_resp = generate_session_sigs_execute_lit_action(
        &validator_collection,
        INVALID_LIT_ACTION_CODE,
        &self_pay_user,
    )
    .await
    .unwrap();

    assert!(
        !execute_resp[0].ok,
        "Expected an error, but got a successful response"
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_BALANCE_CHECK_SECONDS,
    ))
    .await;

    let pkp_ledger_balance_after = self_pay_user
        .get_first_pkp_ledger_balance("Pkp Ledger Balance after the request")
        .await;

    assert!(
        pkp_ledger_balance_after < pkp_ledger_balance,
        "User balance didn't decrease after the failed request"
    );

    // 1.6. Charge for SignSessionKey
    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;

    let auth_sig = generate_authsig_item(&self_pay_user.wallet).await.unwrap();

    let (network_pubkey, _token_id, eth_address) = self_pay_user.first_pkp().info();

    let signing_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let session_pub_key = encoding::bytes_to_hex(verifying_key.to_bytes());

    info!("Starting test: only_sign_session_sig_with_lit_actions");

    let lit_action_code =
        data_encoding::BASE64.encode(VALID_SESSION_SIG_LIT_ACTION_CODE.to_string().as_bytes());

    let resource = "Threshold/Signing".to_string();
    let resource_prefix = format!("{}://*", LitResourcePrefix::PKP);

    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), network_pubkey.to_string().into());
    js_params.insert("sigName".to_string(), "sig1".into());

    let pkp_ledger_balance_before = self_pay_user
        .get_first_pkp_ledger_balance("Pkp Ledger Balance before the deposit")
        .await;

    // The price for the sign_session_key
    self_pay_user
        .deposit_to_first_pkp_ledger(first_node_price * NUM_STAKED_VALIDATORS)
        .await;

    // The price for lit-action
    self_pay_user
        .deposit_to_first_pkp_ledger(
            U256::try_from(I256::from(LIT_ACTION_MIN_ESTIMATE) - pkp_ledger_balance_before)
                .unwrap(),
        )
        .await;

    let pkp_ledger_balance = self_pay_user
        .get_first_pkp_ledger_balance("Pkp Ledger Balance after the deposit")
        .await;

    let signing_resp = get_auth_sig_for_session_sig_from_nodes(
        &node_set,
        &auth_sig,
        false,
        &eth_address.to_fixed_bytes(),
        &network_pubkey,
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
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_BALANCE_CHECK_SECONDS,
    ))
    .await;

    let pkp_ledger_balance_after = self_pay_user
        .get_first_pkp_ledger_balance("Pkp Ledger Balance after the request")
        .await;

    assert!(
        pkp_ledger_balance_after < pkp_ledger_balance,
        "Pkp balance didn't decrease after the failed request"
    );

    // 1.7. Charge for a request which times out
    info!("Self-Paying: Charge for a failing request");
    self_pay_user
        .set_wallet_balance(INITIAL_FUNDING_AMOUNT)
        .await;
    assert!(self_pay_user.new_pkp().await.is_ok(), "Failed to mint PKP");

    // The price for the sign_session_key
    self_pay_user
        .deposit_to_first_pkp_ledger(first_node_price * NUM_STAKED_VALIDATORS)
        .await;

    // The price for lit-action
    self_pay_user
        .deposit_to_first_pkp_ledger(U256::from(LIT_ACTION_MIN_ESTIMATE))
        .await;

    let pkp_ledger_balance = self_pay_user
        .get_first_pkp_ledger_balance("Pkp Ledger Balance after the deposit")
        .await;

    let responses = generate_session_sigs_execute_lit_action(
        &validator_collection,
        INFINITE_LOOP_LIT_ACTION_CODE,
        &self_pay_user,
    )
    .await
    .unwrap();

    assert!(
        !responses[0].ok,
        "Expected an error, but got a successful response"
    );

    let error = responses[0].error_object.as_ref().unwrap();
    assert!(
        error.contains("timeout"),
        "Expected a timeout, but got a different response"
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_BALANCE_CHECK_SECONDS,
    ))
    .await;

    let pkp_ledger_balance_after = self_pay_user
        .get_first_pkp_ledger_balance("Pkp Ledger Balance after the request")
        .await;

    assert!(
        pkp_ledger_balance_after < pkp_ledger_balance,
        "Pkp balance didn't decrease after the failed request"
    );

    // 2. Capacity Delegation AuthSig
    info!("2 - Delegation: Testing that someone else can delegate authSig to the user");
    let mut delegation_user = EndUser::new(&testnet);
    delegation_user.fund_wallet_default_amount().await;
    let _delegation_user_pkp = delegation_user.new_pkp().await.unwrap();

    let mut delegation_payer = EndUser::new(&testnet);
    let _ = delegation_payer.fund_wallet_default_amount().await;
    let _delegation_payer_pkp = delegation_payer.new_pkp().await.unwrap();

    delegation_payer
        .set_wallet_balance(INITIAL_FUNDING_AMOUNT)
        .await;
    delegation_payer
        .get_first_pkp_ledger_balance("Payer Ledger Balance before the deposit")
        .await;
    delegation_payer
        .deposit_to_wallet_ledger(first_node_price * NUM_STAKED_VALIDATORS)
        .await;
    let delegation_payer_ledger_balance = delegation_payer
        .get_wallet_ledger_balance("Payer Ledger Balance after the deposit")
        .await;

    // delegate the capacity AuthSig to the user
    // to do this, we create a signature from the payer
    // and then we stick that signature into the capabilities section of the SIWE

    // 2.1. Delegation Max price < Current price
    info!("2.1 - Delegation: Fail as delegated.max_price < current_price");
    let delegation_max_price = first_node_price / DIVISION_FACTOR_TO_FAIL;

    let delegation_auth_sig = get_auth_sig_with_payment_resources(
        &delegation_payer.wallet,
        &bytes_to_hex(delegation_user.wallet.address()),
        U256::from(delegation_max_price),
        vec![PayedEndpoint::EncryptionSign],
    );

    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(delegation_user.wallet.clone()),
        Some(vec![delegation_auth_sig.clone()]),
        Some(delegation_max_price),
    );

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;
    assert!(
        !decryption_resp[0].ok,
        "Expected an error, but got a successful response"
    );
    assert!(
        decryption_resp[0]
            .error_object
            .as_ref()
            .unwrap()
            .contains("is less than the endpoint price:"),
        "Should not be able to decrypt if max_price is less than the current price"
    );

    // 2.2. Max price ≥ Current price but SessionSig price ≤ Current Price
    info!("2.2 - Delegation: Fail as session_sig.max_price < current_price < delegated.max_price");
    let delegation_auth_sig = get_auth_sig_with_payment_resources(
        &delegation_payer.wallet,
        &bytes_to_hex(delegation_user.wallet.address()),
        first_node_price,
        vec![PayedEndpoint::EncryptionSign],
    );

    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(delegation_user.wallet.clone()),
        Some(vec![delegation_auth_sig.clone()]),
        Some(first_node_price / DIVISION_FACTOR_TO_FAIL),
    );

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;

    assert!(
        !decryption_resp[0].ok,
        "Expected an error, but got a successful response"
    );
    assert!(
        decryption_resp[0]
            .error_object
            .as_ref()
            .unwrap()
            .contains("is less than the endpoint price:"),
        "Should not be able to decrypt if max_price is less than the current price"
    );

    // 2.3. SessionSig price > Max price
    info!("2.3 - Delegation: Fail as session_sig.max_price > delegated.max_price");
    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(delegation_user.wallet.clone()),
        Some(vec![delegation_auth_sig.clone()]),
        Some(U256::MAX),
    );

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;

    assert!(
        !decryption_resp[0].ok,
        "Expected an error, but got a successful response"
    );
    assert!(
        decryption_resp[0]
            .error_object
            .as_ref()
            .unwrap()
            .contains("No payer in the Payment DB;  No Capacity Delegation;  Self failed to pay"),
        "Should not be able to decrypt if sessionSig max_price is greater than the Delegation max_price"
    );

    // 2.4. Max price ≥ Current price & SessionSig price == Current Price
    info!("2.4 - Delegation: Pass as session_sig.max_price ≤ delegated.max_price ≥ current_price");
    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(delegation_user.wallet.clone()),
        Some(vec![delegation_auth_sig.clone()]),
        Some(first_node_price),
    );
    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;

    assert_decrypted(
        &pubkey,
        test_encryption_parameters.identity_param.clone(),
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp,
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_BALANCE_CHECK_SECONDS,
    ))
    .await;

    let delegation_payer_ledger_balance_after = delegation_payer
        .get_wallet_ledger_balance("Delegation Ledger Balance after the request")
        .await;

    assert!(
        delegation_payer_ledger_balance_after < delegation_payer_ledger_balance,
        "Delegator balance didn't decrease after the successful request"
    );

    // 3. PaymentDelegation contract
    info!(
        "3 - Payment DB: Testing that someone else can delegate via PaymentDB contract to the user"
    );
    let payer_db_user = EndUser::new(&testnet);
    let payer_db = EndUser::new(&testnet);

    payer_db.set_wallet_balance(INITIAL_FUNDING_AMOUNT).await;

    let _payer_db_ledger_balance = payer_db
        .get_wallet_ledger_balance("Payer Ledger Balance before the deposit")
        .await;

    payer_db
        .deposit_to_wallet_ledger(first_node_price * PRICE_MULTIPLIER_MULTIPLE_REQUESTS)
        .await;

    let payer_db_ledger_balance = payer_db
        .get_wallet_ledger_balance("Payer Ledger Balance after the deposit")
        .await;

    actions
        .create_payment_delegation_entry(
            &payer_db.wallet,
            payer_db_user.wallet.address(),
            MAX_REQUESTS_PER_PERIOD as u32,
            PAYMENT_PERIOD_SECONDS as u32,
            first_node_price.low_u128() * NUM_STAKED_VALIDATORS as u128,
        )
        .await;

    // 3.1. Session_sig.max_price ≥ payment_db.total_price / threshold
    info!("3.1 - PaymentDB: Fail as session_sig.max_price > delegated.total_max_price / threshold");
    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(payer_db_user.wallet.clone()),
        None,
        Some(first_node_price * NUM_STAKED_VALIDATORS),
    );

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;
    assert!(
        !decryption_resp[0].ok,
        "Expected an error, but got a successful response"
    );
    assert!(
        decryption_resp[0]
            .error_object
            .as_ref()
            .unwrap()
            .contains("No payer in the Payment DB;  No Capacity Delegation;  Self failed to pay:"),
        "Should not be able to decrypt if sessionSig max_price is greater than the Delegation max_price"
    );

    info!("PaymentDB- Pass as session_sig.max_price ≤ delegated.total_max_price / threshold");
    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: format!(
                    "{}/{}",
                    test_encryption_parameters.hashed_access_control_conditions,
                    test_encryption_parameters.data_to_encrypt_hash
                ),
                resource_prefix: LitResourcePrefix::ACC.to_string(),
            },
            ability: LitAbility::AccessControlConditionDecryption.to_string(),
        }],
        Some(payer_db_user.wallet.clone()),
        None,
        Some(first_node_price),
    );

    // We send 7 requests since the first 3 requests should definitely pass as it's the max usage within the 10 seconds period.
    // But the 10 second period could roll over and that will allow the next 3 requests to pass as well. Thus the request after that should always fail.
    let mut request_count = 0;
    while request_count < MAX_TEST_REQUESTS {
        info!(
            "Sending request after payment delegation entry {}",
            request_count
        );

        let decryption_resp = retrieve_decryption_key_session_sigs(
            test_encryption_parameters.clone(),
            &session_sigs_and_node_set,
            actions.get_current_epoch(realm_id).await.as_u64(),
        )
        .await;

        if !decryption_resp[0].ok {
            let error = decryption_resp[0].error_object.as_ref().unwrap();
            assert!(
                error.contains(
                    "No payer in the Payment DB;  No Capacity Delegation;  Self failed to pay"
                ) & error.contains("balance 0 minus their pending spending of 0 is not enough to cover the minimum estimated price"),
                "Request failed for some other reason apart from the max usage limit in the PaymentDB"
            );
            break;
        }

        request_count += 1;
    }

    info!(
        "We made {} requests after delegating using the PaymentDelegation contract before we hit the rate limit again",
        request_count
    );

    assert!(
        request_count > 2 && request_count < MAX_TEST_REQUESTS,
        "PaymentDB should work for at least the first 3 requests and at most the next 3 requests but it also passed at request {}",
        request_count
    );

    // We have a 10 second period, so, after 10 seconds we should be able to make 3 more requests.
    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_PAYMENT_DB_PERIOD_SECONDS,
    ))
    .await;

    let payer_db_ledger_balance_after_first_round = payer_db
        .get_wallet_ledger_balance("Payer DB Ledger Balance after the request")
        .await;

    assert!(
        payer_db_ledger_balance_after_first_round < payer_db_ledger_balance,
        "Payer DB balance didn't decrease after the successful request"
    );

    // We send 7 requests since the first 3 requests should definitely pass as it's the max usage within the 10 seconds period.
    // But the 10 second period could roll over and that will allow the next 3 requests to pass as well. Thus the request after that should always fail.
    request_count = 0;
    while request_count < MAX_TEST_REQUESTS {
        info!(
            "Sending request after payment delegation entry {}",
            request_count
        );

        let decryption_resp = retrieve_decryption_key_session_sigs(
            test_encryption_parameters.clone(),
            &session_sigs_and_node_set,
            actions.get_current_epoch(realm_id).await.as_u64(),
        )
        .await;

        if !decryption_resp[0].ok {
            let error = decryption_resp[0].error_object.as_ref().unwrap();
            assert!(
                error.contains(
                    "No payer in the Payment DB;  No Capacity Delegation;  Self failed to pay"
                ) & error.contains("balance 0 minus their pending spending of 0 is not enough to cover the minimum estimated price"),
                "Request failed for some other reason apart from the max usage limit in the PaymentDB"
            );
            break;
        }

        request_count += 1;
    }

    info!(
        "We made {} requests after delegating using the PaymentDelegation contract before we hit the rate limit again",
        request_count
    );

    assert!(
        request_count > 2 && request_count < MAX_TEST_REQUESTS,
        "PaymentDB should work for at least the first 3 requests and at most the next 3 requests but it also passed at request {}",
        request_count
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_BALANCE_CHECK_SECONDS,
    ))
    .await;

    let payer_db_ledger_balance_after_second_round = payer_db
        .get_wallet_ledger_balance("Payer DB Ledger Balance after the second requests")
        .await;

    assert!(
        payer_db_ledger_balance_after_second_round < payer_db_ledger_balance_after_first_round,
        "Payer DB balance didn't decrease after the second successful requests"
    );
}

#[tokio::test]
async fn test_all_payment_methods_for_pkp() {
    // Network set-up
    crate::common::setup_logging();
    let (testnet, _validator_collection, actions, node_set) = setup_testnet_for_payments().await;
    let realm_id = ethers::types::U256::from(1);
    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;

    let mut pkp_owner = EndUser::new(&testnet);
    pkp_owner.set_wallet_balance(INITIAL_FUNDING_AMOUNT).await;
    pkp_owner.new_pkp().await.expect("Failed to mint PKP");

    // add the PKP itself as a permitted address, so that our session sig from the PKP will be able to sign with it
    let (pubkey, _token_id, eth_address) = pkp_owner.first_pkp().info();
    let pkp = pkp_owner.pkp_by_pubkey(pubkey.clone());
    pkp.add_permitted_address_to_pkp(eth_address, &[U256::from(1)])
        .await
        .expect("Could not add permitted address to pkp");

    // Encryption
    let test_encryption_parameters = prepare_test_encryption_parameters();

    let network_pubkey = get_network_pubkey(&actions).await;
    let message_bytes = test_encryption_parameters.to_encrypt.as_bytes();

    let bls_pubkey = blsful::PublicKey::try_from(&hex::decode(&network_pubkey).unwrap()).unwrap();

    let ciphertext = lit_sdk::encryption::encrypt_time_lock(
        &bls_pubkey,
        message_bytes,
        &test_encryption_parameters.identity_param,
    )
    .expect("Unable to encrypt");

    // 1. Self-paying PKP SessionSig
    info!("Self-Paying: Testing that the PKP can pay for itself");

    let first_node_price = pkp_owner.first_node_price_from_feed(0).await;

    pkp_owner.set_wallet_balance(INITIAL_FUNDING_AMOUNT).await;

    let _pkp_ledger_balance = pkp_owner
        .get_first_pkp_ledger_balance("PKP Ledger Balance before the deposit")
        .await;

    pkp_owner
        .deposit_to_first_pkp_ledger(first_node_price * NUM_STAKED_VALIDATORS * 2)
        .await;

    let pkp_ledger_balance = pkp_owner
        .get_first_pkp_ledger_balance("PKP Ledger Balance after the deposit")
        .await;

    let resource_ability_requests = vec![LitResourceAbilityRequest {
        resource: LitResourceAbilityRequestResource {
            resource: format!(
                "{}/{}",
                test_encryption_parameters.hashed_access_control_conditions,
                test_encryption_parameters.data_to_encrypt_hash
            ),
            resource_prefix: LitResourcePrefix::ACC.to_string(),
        },
        ability: LitAbility::AccessControlConditionDecryption.to_string(),
    }];

    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey.clone(),
        eth_address,
        resource_ability_requests.clone(),
        pkp_owner.wallet.clone(),
        None,
        None,
        None,
        actions.get_current_epoch(realm_id).await.as_u64(),
        Some(first_node_price),
    )
    .await
    .expect("Could not get PKP session sigs");

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;

    assert_decrypted(
        &bls_pubkey,
        test_encryption_parameters.identity_param.clone(),
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp,
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_BALANCE_CHECK_SECONDS,
    ))
    .await;
    let pkp_ledger_balance_after = pkp_owner
        .get_first_pkp_ledger_balance("PKP Ledger Balance after the request")
        .await;

    // NOTE!: This may not be true always since it depends on when the nodes charge the user on-chain. Currently it happens in a separate task worker and it runs every `CFG_KEY_PAYMENT_INTERVAL_MS_DEFAULT` ms but for the test we override the default with `payment_interval = "1000"`
    assert!(
        pkp_ledger_balance_after < pkp_ledger_balance,
        "PKP balance didn't decrease after the successful request"
    );

    // 2. PKP SessionSig, payed by the owner
    info!("Self-Paying: Testing that the PKP can pay for itself");

    let pkp_ledger_balance = pkp_owner
        .get_first_pkp_ledger_balance("PKP Ledger Balance")
        .await;

    assert_eq!(
        pkp_ledger_balance,
        I256::zero(),
        "PKP ledger balance should be 0 in the beginning of this test."
    );

    pkp_owner
        .deposit_to_wallet_ledger(first_node_price * NUM_STAKED_VALIDATORS * 2)
        .await;

    let owner_ledger_balance = pkp_owner
        .get_wallet_ledger_balance("Owner Ledger Balance after the deposit")
        .await;

    let resource_ability_requests = vec![LitResourceAbilityRequest {
        resource: LitResourceAbilityRequestResource {
            resource: format!(
                "{}/{}",
                test_encryption_parameters.hashed_access_control_conditions,
                test_encryption_parameters.data_to_encrypt_hash
            ),
            resource_prefix: LitResourcePrefix::ACC.to_string(),
        },
        ability: LitAbility::AccessControlConditionDecryption.to_string(),
    }];

    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey.clone(),
        eth_address,
        resource_ability_requests.clone(),
        pkp_owner.wallet.clone(),
        None,
        None,
        None,
        actions.get_current_epoch(U256::from(1)).await.as_u64(),
        Some(first_node_price),
    )
    .await
    .expect("Could not get PKP session sigs");

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(U256::from(1)).await.as_u64(),
    )
    .await;

    assert_decrypted(
        &bls_pubkey,
        test_encryption_parameters.identity_param.clone(),
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp,
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_BALANCE_CHECK_SECONDS,
    ))
    .await;

    let owner_ledger_balance_after = pkp_owner
        .get_wallet_ledger_balance("Owner Ledger Balance after the request")
        .await;

    assert!(
        owner_ledger_balance_after < owner_ledger_balance,
        "Owner's ledger balance didn't decrease after the successful request"
    );

    // 3 Capacity Delegation AuthSig
    info!("Delegation: Testing that someone else can delegate authSig to the PKP");
    let delegation_payer = EndUser::new(&testnet);

    delegation_payer
        .set_wallet_balance(INITIAL_FUNDING_AMOUNT)
        .await;

    let _delegation_payer_ledger_balance = delegation_payer
        .get_wallet_ledger_balance("Delegation Ledger Balance before the deposit")
        .await;

    pkp_owner
        .deposit_to_wallet_ledger(first_node_price * NUM_STAKED_VALIDATORS)
        .await; // To cover the sign_session_key cost

    delegation_payer
        .deposit_to_wallet_ledger(first_node_price * NUM_STAKED_VALIDATORS)
        .await;

    let delegation_payer_ledger_balance = delegation_payer
        .get_wallet_ledger_balance("Delegation Ledger Balance after the deposit")
        .await;

    // delegate the capacity AuthSig to the user
    // to do this, we create a signature from the payer
    // and then we stick that signature into the capabilities section of the SIWE

    let delegation_auth_sig = get_auth_sig_with_payment_resources(
        &delegation_payer.wallet,
        &hex::encode(pkp_owner.first_pkp().eth_address),
        first_node_price,
        vec![PayedEndpoint::EncryptionSign],
    );

    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey.clone(),
        eth_address,
        resource_ability_requests.clone(),
        pkp_owner.wallet.clone(),
        Some(vec![delegation_auth_sig.clone()]),
        None,
        None,
        actions.get_current_epoch(realm_id).await.as_u64(),
        Some(first_node_price),
    )
    .await
    .expect("Could not get PKP session sigs");
    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;

    assert_decrypted(
        &bls_pubkey,
        test_encryption_parameters.identity_param.clone(),
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp,
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_BALANCE_CHECK_SECONDS,
    ))
    .await;

    let delegation_payer_ledger_balance_after = delegation_payer
        .get_wallet_ledger_balance("Delegation Ledger Balance after the request")
        .await;

    assert!(
        delegation_payer_ledger_balance_after < delegation_payer_ledger_balance,
        "Delegator balance didn't decrease after the successful request"
    );

    // 4. PaymentDelegation contract
    info!("Payment DB: Testing that someone else can delegate via PaymentDB contract to the PKP");

    let payer_db = EndUser::new(&testnet);
    payer_db.set_wallet_balance(INITIAL_FUNDING_AMOUNT).await;

    let _payer_db_ledger_balance = payer_db
        .get_wallet_ledger_balance("Payer Ledger Balance before the deposit")
        .await;

    payer_db
        .deposit_to_wallet_ledger(first_node_price * PRICE_MULTIPLIER_MULTIPLE_REQUESTS)
        .await;

    let payer_db_ledger_balance = payer_db
        .get_wallet_ledger_balance("Payer Ledger Balance after the deposit")
        .await;

    let _payment_delegation_contract_address = actions.contracts().payment_delegation.address();

    actions
        .create_payment_delegation_entry(
            &payer_db.wallet,
            pkp_owner.first_pkp().eth_address,
            MAX_REQUESTS_PER_PERIOD as u32, // Double to account for the price of sign_session_key
            PAYMENT_PERIOD_SECONDS as u32,
            first_node_price.low_u128() * NUM_STAKED_VALIDATORS as u128,
        )
        .await;

    let session_sigs_and_node_set = get_session_sigs_and_node_set_for_pkp(
        &node_set,
        pubkey,
        eth_address,
        resource_ability_requests.clone(),
        pkp_owner.wallet.clone(),
        None,
        None,
        None,
        actions.get_current_epoch(realm_id).await.as_u64(),
        Some(first_node_price),
    )
    .await
    .expect("Could not get PKP session sigs");

    // Skip the effect of the payment for the sign_session_key call.
    tokio::time::sleep(tokio::time::Duration::from_secs(PAYMENT_PERIOD_SECONDS)).await;

    // We send 7 requests since the first 3 requests should definitely pass as it's the max usage within the 10 seconds period.
    // But the 10 second period could roll over and that will allow the next 3 requests to pass as well. Thus the request after that should always fail.
    let mut request_count = 0;
    while request_count < MAX_TEST_REQUESTS {
        info!(
            "Sending request after payment delegation entry {}",
            request_count
        );

        let decryption_resp = retrieve_decryption_key_session_sigs(
            test_encryption_parameters.clone(),
            &session_sigs_and_node_set,
            actions.get_current_epoch(realm_id).await.as_u64(),
        )
        .await;

        if !decryption_resp[0].ok {
            let error = decryption_resp[0].error_object.as_ref().unwrap();
            assert!(
                error.contains(
                    "No payer in the Payment DB;  No Capacity Delegation;  Self failed to pay"
                ) & error.contains("balance 0 minus their pending spending of 0 is not enough to cover the minimum estimated price"),
                "Request failed for some other reason apart from the max usage limit in the PaymentDB"
            );
            break;
        }

        request_count += 1;
    }

    info!(
        "We made {} requests after delegating using the PaymentDelegation contract before we hit the rate limit again",
        request_count
    );

    assert!(
        request_count > 2 && request_count < MAX_TEST_REQUESTS,
        "PaymentDB should work for at least the first 3 requests and at most the next 3 requests but it also passed at request {}",
        request_count
    );

    // We have a 10 second period, so, after 10 seconds we should be able to make at least 3 more requests.
    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_PAYMENT_DB_PERIOD_SECONDS,
    ))
    .await;

    assert!(
        payer_db
            .wallet_ledger_has_decreased_from(payer_db_ledger_balance)
            .await,
        "Payer DB balance didn't decrease after the successful request"
    );

    // We send 7 requests since the first 3 requests should definitely pass as it's the max usage within the 10 seconds period.
    // But the 10 second period could roll over and that will allow the next 3 requests to pass as well. Thus the request after that should always fail.
    request_count = 0;
    while request_count < MAX_TEST_REQUESTS {
        info!(
            "Sending request after payment delegation entry {}",
            request_count
        );

        let decryption_resp = retrieve_decryption_key_session_sigs(
            test_encryption_parameters.clone(),
            &session_sigs_and_node_set,
            actions.get_current_epoch(realm_id).await.as_u64(),
        )
        .await;

        if !decryption_resp[0].ok {
            let error = decryption_resp[0].error_object.as_ref().unwrap();
            assert!(
                error.contains(
                    "No payer in the Payment DB;  No Capacity Delegation;  Self failed to pay"
                ) & error.contains("balance 0 minus their pending spending of 0 is not enough to cover the minimum estimated price"),
                "Request failed for some other reason apart from the max usage limit in the PaymentDB"
            );
            break;
        }

        request_count += 1;
    }

    info!(
        "We made {} requests after delegating using the PaymentDelegation contract before we hit the rate limit again",
        request_count
    );

    assert!(
        request_count > 2 && request_count < MAX_TEST_REQUESTS,
        "PaymentDB should work for at least the first 3 requests and at most the next 3 requests but it also passed at request {}",
        request_count
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_BALANCE_CHECK_SECONDS,
    ))
    .await;

    assert!(
        payer_db
            .wallet_ledger_has_decreased_from(payer_db_ledger_balance)
            .await,
        "Payer DB balance didn't decrease after the second successful requests"
    );
}

#[tokio::test]
async fn test_pending_payments_block_usage() {
    // Network set-up
    crate::common::setup_logging();
    let (testnet, _validator_collection, actions, node_set) =
        do_setup_testnet_for_payments(false).await;

    let realm_id = ethers::types::U256::from(1);
    // Encryption
    let test_encryption_parameters = prepare_test_encryption_parameters();
    let network_pubkey = get_network_pubkey(&actions).await;
    let message_bytes = test_encryption_parameters.to_encrypt.as_bytes();
    let pubkey = blsful::PublicKey::try_from(&hex::decode(&network_pubkey).unwrap()).unwrap();
    let ciphertext = lit_sdk::encryption::encrypt_time_lock(
        &pubkey,
        message_bytes,
        &test_encryption_parameters.identity_param,
    )
    .expect("Unable to encrypt");

    let resource_ability_requests = vec![LitResourceAbilityRequest {
        resource: LitResourceAbilityRequestResource {
            resource: format!(
                "{}/{}",
                test_encryption_parameters.hashed_access_control_conditions,
                test_encryption_parameters.data_to_encrypt_hash
            ),
            resource_prefix: LitResourcePrefix::ACC.to_string(),
        },
        ability: LitAbility::AccessControlConditionDecryption.to_string(),
    }];

    let self_pay_user = EndUser::new(&testnet);

    let threshold = 3;
    let first_node_price = self_pay_user.first_node_price_from_feed(0).await;
    let ledger_balance = first_node_price * (threshold + 1);

    let node_set = get_identity_pubkeys_from_node_set(&node_set).await;

    self_pay_user
        .set_wallet_balance(INITIAL_FUNDING_AMOUNT)
        .await;
    self_pay_user.deposit_to_wallet_ledger(ledger_balance).await;

    let self_pay_user_ledger_balance = self_pay_user
        .get_wallet_ledger_balance("User Ledger Balance after the deposit")
        .await;

    // 1. The first request passes: No pending payments
    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(self_pay_user.wallet.clone()),
        None,
        Some(first_node_price),
    );

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;

    assert_decrypted(
        &pubkey,
        test_encryption_parameters.identity_param.clone(),
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp,
    );

    tokio::time::sleep(tokio::time::Duration::from_secs(
        WAIT_TIME_BALANCE_CHECK_SECONDS,
    ))
    .await;

    let self_pay_user_ledger_balance_after = self_pay_user
        .get_wallet_ledger_balance("User Ledger Balance after the first request")
        .await;

    assert_eq!(
        self_pay_user_ledger_balance_after, self_pay_user_ledger_balance,
        "This test is only valid on a testnet which does not send the batches to the contract"
    );

    // 2. The second request passes: The ledger balance is enough, despite the pending payment
    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(self_pay_user.wallet.clone()),
        None,
        Some(first_node_price),
    );

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;

    assert_decrypted(
        &pubkey,
        test_encryption_parameters.identity_param.clone(),
        &test_encryption_parameters.to_encrypt,
        &ciphertext,
        decryption_resp,
    );

    // 3. The third request fails: The ledger balance is not enough due to the pending payment
    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set,
        resource_ability_requests.clone(),
        Some(self_pay_user.wallet.clone()),
        None,
        Some(first_node_price),
    );

    let decryption_resp = retrieve_decryption_key_session_sigs(
        test_encryption_parameters.clone(),
        &session_sigs_and_node_set,
        actions.get_current_epoch(realm_id).await.as_u64(),
    )
    .await;

    assert!(
        !decryption_resp[0].ok,
        "Expected an error, but got a successful response."
    );
    let error = decryption_resp[0].error_object.as_ref().unwrap();
    let expected_error = format!(
        "balance {} minus their pending spending of {} is not enough to cover the minimum estimated price {}",
        ledger_balance,
        first_node_price * 2,
        first_node_price * threshold
    );
    assert!(
        &error.contains(&expected_error),
        "Should not be able to decrypt if user doesn't have the required balance"
    );
}
async fn setup_testnet_for_payments() -> (Testnet, ValidatorCollection, Actions, Vec<NodeSet>) {
    do_setup_testnet_for_payments(true).await
}

pub async fn do_setup_testnet_for_payments(
    submit_batches_to_ledger: bool,
) -> (Testnet, ValidatorCollection, Actions, Vec<NodeSet>) {
    let payment_interval_ms = match submit_batches_to_ledger {
        true => None, // Use the default
        false => Some(std::i64::MAX.to_string()),
    };

    let (testnet, validator_collection, _end_user) = TestSetupBuilder::default()
        .num_staked_and_joined_validators(NUM_STAKED_VALIDATORS)
        .epoch_length(EPOCH_LENGTH_SECONDS as usize)
        .enable_payment("true".to_string())
        .payment_interval_ms(payment_interval_ms)
        .build()
        .await;

    let actions = testnet.actions().clone();
    let node_set = validator_collection.random_threshold_nodeset().await;

    (testnet, validator_collection, actions, node_set)
}
