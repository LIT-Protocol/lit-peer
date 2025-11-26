pub mod litactions {
    use crate::common::auth_sig::{
        generate_authsig, get_session_sigs_and_node_set_for_pkp, get_session_sigs_for_auth,
    };
    use crate::common::lit_actions::{
        execute_lit_action_session_sigs, generate_pkp_check_is_permitted_pkp_action,
        generate_session_sigs_and_execute_lit_action,
    };
    use crate::common::pkp::{SignedDatak256, recombine_shares_using_wasm};
    use crate::common::setup_logging;
    use base64_light::base64_encode_bytes;
    use lit_core::utils::binary::bytes_to_hex;
    use lit_node::models::RequestConditions;
    use lit_node_core::{
        ControlConditionItem, EVMContractCondition, JsonAccessControlCondition, JsonAuthSig,
        JsonReturnValueTest, JsonReturnValueTestV2, LitAbility, LitActionPriceComponent,
        LitResource, LitResourceAbilityRequest, LitResourceAbilityRequestResource,
        LitResourcePrefix, SigningScheme, UnifiedAccessControlCondition,
        UnifiedAccessControlConditionItem, constants::CHAIN_LOCALCHAIN,
    };
    use lit_node_testnet::end_user::EndUser;
    use lit_node_testnet::testnet::Testnet;
    use lit_node_testnet::validator::ValidatorCollection;
    use lit_node_testnet::{TestSetupBuilder, testnet::actions::Actions};

    use ethers::signers::Wallet;
    use ethers::types::U256;
    use ipfs_hasher::IpfsHasher;
    use k256::ecdsa::SigningKey;
    use lit_node::utils::web::hash_access_control_conditions;
    use lit_node_core::response::JsonExecutionResponse;
    use lit_node_core::{
        AccessControlConditionResource, EcdsaSignedMessageShare, SignableOutput,
        response::GenericResponse,
    };
    use lit_node_testnet::node_collection::{
        get_identity_pubkeys_from_node_set, get_network_pubkey,
    };
    use lit_sdk::signature::SignedDataOutput;
    use rocket::form::validate::Contains;
    use serde_json::Value;
    use sha2::{Digest, Sha256};
    use test_case::test_case;
    use tracing::info;

    // for legibility
    type LaPC = LitActionPriceComponent;

    const LAPC_DBC: &[LitActionPriceComponent] =
        &[LaPC::Broadcasts, LaPC::Decrypts, LaPC::ContractCalls];
    const LAPC_BC: &[LitActionPriceComponent] = &[LaPC::Broadcasts, LaPC::ContractCalls];
    const LAPC_SB: &[LitActionPriceComponent] = &[LaPC::Signatures, LaPC::Broadcasts];
    // Notes:
    // - The 2 tests inside test_pkp_permissions_is_cid_registered_and_can_it_sign, is covered by "sign_child_lit_action" & "fail_sign_non_hashed_message".
    // - The original encrypt test wasn't a good integration test - it attempted to compare against a known pubkey, but integration tests generate new keys each time.  encrypt & decrypt tests cover this functionality.

    #[test_case("broadcast_and_collect", &[LaPC::Broadcasts], &all_response_match, &standard_acc, true, "*", true)] /* Success */
    #[test_case("check_conditions_with_auth_sig", &[LaPC::ContractCalls], &all_response_match, &standard_acc, true, "true", true)] /* Success */
    #[test_case("check_conditions_without_auth_sig", &[LaPC::ContractCalls], &all_response_match, &standard_acc, false,  "true", true)] /* Success <<< BUT CHECK */
    #[test_case("current_ipfs_id_substitution", LAPC_DBC, &all_response_match, &ipfs_acc, true, "hello this is a test", true)] /* Success */
    #[test_case("decrypt_and_combine_with_access_denied",LAPC_BC, &action_failed_with_error, &impossible_acc, true, "Access control conditions check failed", false)] /* Success */
    #[test_case("decrypt_and_combine_with_auth_sig", LAPC_DBC, &all_response_match, &standard_acc, true, "hello this is a test", true)] /* Success */
    #[test_case("decrypt_and_combine_without_auth_sig", LAPC_DBC, &all_response_match, &standard_acc, false, "*", true)]
    #[test_case("decrypt_to_single_node", LAPC_DBC, &single_valid, &standard_acc, true, "hello this is a test", true)]
    #[test_case("get_rpc_url", &[], &all_response_match, &standard_acc,true, "https://api.node.glif.io/rpc/v1", true)] /* local rpc config */
    #[test_case("multiple_sign_and_combine_ecdsa", LAPC_SB, &valid_sign_and_combine, &standard_acc, true, "", false)]
    #[test_case("multiple_sign_and_combine_ed25519", LAPC_SB, &valid_sign_and_combine, &standard_acc, true, "", false)]
    #[test_case("multiple_sign_and_combine_blsg1", LAPC_SB, &valid_sign_and_combine, &standard_acc, true, "", false)]
    #[test_case("run_once_and_collect_responses", &[LaPC::Broadcasts, LaPC::Fetches], &all_response_match, &standard_acc,true, "*", true)]
    #[test_case("run_once", &[LaPC::Fetches], &all_response_match, &standard_acc,true, "*", true)]
    #[test_case("sign_and_combine_ecdsa", LAPC_SB, &all_response_match, &standard_acc,true, "*", true)]
    #[test_case("sign_hello_world", &[LaPC::Signatures], &valid_sign_no_combine, &standard_acc, true, "", false)]
    #[test_case("sign_child_lit_action", &[LaPC::Signatures, LaPC::CallDepth], &valid_sign_no_combine, &standard_acc, true, "", false)]
    #[test_case("fail_sign_non_hashed_message", &[LaPC::Signatures], &action_failed_with_error, &standard_acc, true, "Message length to be signed is not 32 bytes", false)]
    #[tokio::test]
    // #[ignore]
    pub async fn lit_action_from_file(
        file_name: &str,
        price_components: &[LitActionPriceComponent],
        fn_assertion: &dyn Fn(
            Vec<GenericResponse<JsonExecutionResponse>>,
            serde_json::Map<String, Value>,
            &str,
        ) -> bool,
        fn_accs: &dyn Fn(&str, &Actions) -> Vec<UnifiedAccessControlConditionItem>,
        include_auth_sig: bool,
        value: &str,
        wrap_in_quotes: bool,
    ) {
        setup_logging();
        let (testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;
        lit_action_from_file_preloaded(
            price_components,
            &validator_collection,
            &testnet,
            &end_user,
            file_name,
            fn_assertion,
            fn_accs,
            include_auth_sig,
            value,
            wrap_in_quotes,
        )
        .await;
    }

    pub async fn lit_action_from_file_preloaded(
        price_components: &[LitActionPriceComponent],
        validator_collection: &ValidatorCollection,
        _testnet: &Testnet,
        end_user: &EndUser,
        file_name: &str,
        fn_assertion: &dyn Fn(
            Vec<GenericResponse<JsonExecutionResponse>>,
            serde_json::Map<String, Value>,
            &str,
        ) -> bool,
        fn_accs: &dyn Fn(&str, &Actions) -> Vec<UnifiedAccessControlConditionItem>,
        include_auth_sig: bool,
        value: &str,
        wrap_in_quotes: bool,
    ) -> u8 {
        info!("Starting test: {}.js", file_name);
        let file_with_path = &format!("./tests/lit_action_scripts/{}.js", file_name);

        let actions = validator_collection.actions();
        let node_set = validator_collection.random_threshold_nodeset().await;
        let node_set = get_identity_pubkeys_from_node_set(&node_set).await;

        let realm_id = ethers::types::U256::from(1);
        let epoch = validator_collection
            .actions()
            .get_current_epoch(realm_id)
            .await
            .as_u64();
        let lit_action_code = std::fs::read_to_string(file_with_path).unwrap();

        // this isn't used for all actions, but it gets ignored when we pass it in for unrelated actions
        let (access_control_conditions, ciphertext, data_to_encrypt_hash, auth_sig) =
            get_encryption_decryption_test_params(
                end_user.wallet.clone(),
                &actions,
                value,
                &lit_action_code,
                fn_accs,
            )
            .await;

        let (pubkey, _token_id, _eth_address, _key_set_id) = end_user.first_pkp().info();

        let lit_action_code = data_encoding::BASE64.encode(lit_action_code.as_bytes());
        // per above, there are more params than needed for some actions, but they are ignored
        let mut js_params = serde_json::Map::new();
        js_params.insert("publicKey".to_string(), pubkey.into());
        js_params.insert("sigName".to_string(), "sig1".into());
        js_params.insert("ciphertext".to_string(), ciphertext.into());
        js_params.insert("dataToEncryptHash".to_string(), data_to_encrypt_hash.into());
        js_params.insert(
            "accessControlConditions".to_string(),
            serde_json::to_value(access_control_conditions.unwrap()).unwrap(),
        );
        if include_auth_sig {
            js_params.insert(
                "authSig".to_string(),
                serde_json::to_value(auth_sig).unwrap(),
            );
        };
        let params = js_params.clone();
        let js_params = Some(serde_json::Value::Object(js_params));

        let ipfs_id = None;
        let auth_methods = None;
        // this is the account that minted the PKP above.

        // run
        let execute_resp = generate_session_sigs_and_execute_lit_action(
            &node_set,
            end_user.wallet.clone(),
            Some(lit_action_code),
            ipfs_id,
            js_params,
            auth_methods,
            epoch,
        )
        .await;

        let value = if wrap_in_quotes {
            format!("\"{}\"", value)
        } else {
            value.to_string()
        };

        let execute_resp = execute_resp.unwrap();
        if execute_resp.len() > 0 {
            if execute_resp[0].ok {
                assert!(
                    check_payment_details(&execute_resp, price_components),
                    "Payment details are not correct."
                );
            }
        }
        assert!(fn_assertion(
            execute_resp,
            params, // this is the params that were passed into the lit action
            &value,
        ));

        1
    }

    // functions that check the response values

    fn check_payment_details(
        execute_resp: &Vec<GenericResponse<JsonExecutionResponse>>,
        price_components: &[LitActionPriceComponent],
    ) -> bool {
        let response_count = execute_resp.len();
        let mut all_price_components = price_components.to_vec();
        all_price_components.push(LitActionPriceComponent::BaseAmount);
        all_price_components.push(LitActionPriceComponent::MemoryUsage);
        // let payment_configs = validator_collection.actions().get_lit_action_price_configs().await;

        let payment_details = execute_resp
            .iter()
            .map(|r| {
                let payment_details = r.data.as_ref().unwrap().payment_detail.as_ref().unwrap();
                payment_details
                    .iter()
                    .map(|p| p.clone())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let payment_details = payment_details.iter().flatten().collect::<Vec<_>>();

        let base_amount_count = payment_details
            .iter()
            .filter(|p| p.component == LitActionPriceComponent::BaseAmount)
            .count();
        assert!(
            base_amount_count >= response_count,
            "Base amount count less than response count.  One or more nodes did not pay the base amount."
        );
        info!(
            "Base amount count passed: {} from {} nodes",
            base_amount_count, response_count
        );

        let memory_usage_count = payment_details
            .iter()
            .filter(|p| p.component == LitActionPriceComponent::MemoryUsage)
            .count();
        assert!(
            memory_usage_count >= response_count,
            "Memory usage count less than response count.  One or more nodes did not pay the memory usage."
        );
        info!(
            "Memory usage count passed: {} from {} nodes",
            memory_usage_count, response_count
        );

        for price_component in price_components {
            let count = payment_details
                .iter()
                .filter(|p| p.component == *price_component)
                .count();

            // in tests, the fetches often happen in run-once, so we check only that at least one is present.
            if price_component == &LitActionPriceComponent::Fetches {
                assert!(count > 0, "Fetches count is 0");
            } else {
                assert!(
                    count >= response_count,
                    "Price component {:?} count less than response count.  One or more nodes did not pay the {:?}",
                    price_component,
                    price_component
                );
            }
            info!(
                "Price component {:?} count passed: {} from {} nodes",
                price_component, count, response_count
            );
        }

        // find payment_details that are not in the price_components
        let mut not_found = vec![];

        for payment_detail in payment_details {
            if !all_price_components.contains(&payment_detail.component) {
                if !not_found.contains(&payment_detail.component) {
                    not_found.push(payment_detail.component);
                }
            }
        }
        if not_found.len() > 0 {
            error!("Price components not found: {:?}", not_found);
            return false;
        }

        true
    }

    #[doc = "Checks that all the 'response' values match and that each node provides a 'success' result of true"]
    fn all_response_match(
        execute_resp: Vec<GenericResponse<JsonExecutionResponse>>,
        _params: serde_json::Map<String, Value>,
        value: &str,
    ) -> bool {
        let values = vec![value.to_string(); execute_resp.len()];
        count_matching_responses(execute_resp.len(), true, execute_resp, &values)
    }

    #[doc = "Checks that the 'response' values of a single node  match incoming data and that each node provides a 'success' result that is true"]
    fn single_valid(
        execute_resp: Vec<GenericResponse<JsonExecutionResponse>>,
        _params: serde_json::Map<String, Value>,
        value: &str,
    ) -> bool {
        let values = vec![value.to_string(); execute_resp.len()];
        count_matching_responses(1, true, execute_resp, &values)
    }

    #[doc = "Validate multiple signatures"]
    fn valid_sign_and_combine(
        execute_resp: Vec<GenericResponse<JsonExecutionResponse>>,
        _params: serde_json::Map<String, Value>,
        _value: &str,
    ) -> bool {
        for resp in execute_resp {
            assert!(resp.ok);
            assert!(resp.data.is_some());
            let data = resp.data.as_ref().unwrap();
            assert!(data.success);
            info!("resp: {:?}", data);
            // info!("Response object: {:?}", response_obj);
            let sig_names = vec!["sig1", "sig2"];
            let response: Value = serde_json::from_str(&data.response).unwrap();
            let response = response.as_object().unwrap();
            for sig_name in sig_names {
                let sig = response.get(sig_name).unwrap();

                if let Some(r) = sig.get("r") {
                    assert_eq!(r.as_str().unwrap().len(), 64);
                }
                if let Some(s) = sig.get("s") {
                    assert_eq!(s.as_str().unwrap().len(), 64);
                }

                if let Some(v) = sig.get("v") {
                    // only applies to ecdsa
                    let v = v.as_u64().unwrap();
                    assert!(v == 0 || v == 1, "V recovery param must be 1 or 0");
                }
                if let Some(value) = sig.get("value") {
                    assert_eq!(value.as_str().unwrap().len(), 216);
                }
            }
        }
        true
    }

    #[doc = "Validate single signature"]
    pub fn valid_sign_no_combine(
        execute_resp: Vec<GenericResponse<JsonExecutionResponse>>,
        _params: serde_json::Map<String, Value>,
        _value: &str,
    ) -> bool {
        // collect the shares into a struct and a set of string that can be used to recombine using the WASM module.
        // currently designed to handle just a single siganture.
        let mut shares = vec![];
        for resp in execute_resp {
            assert!(resp.ok);
            let data = resp.data.as_ref().unwrap();
            info!("json_object: {:?}", data);
            // let signed_data = json_object.get("signedData").unwrap().as_str().unwrap();

            let la_signed_data = data.signed_data.get("sig1").unwrap().clone();
            let sig_name = la_signed_data.sig_name.clone();
            let la_signed_data: SignableOutput =
                serde_json::from_str(&la_signed_data.signature_share).unwrap();
            let la_signed_data: EcdsaSignedMessageShare =
                la_signed_data.ecdsa_signed_message_share().unwrap();
            let share_id = if let Ok(scalar) =
                serde_json::from_str::<k256::Scalar>(&la_signed_data.share_id)
            {
                scalar
            } else if let Ok(num) = la_signed_data.share_id.parse::<u64>() {
                info!("Share id is a number");
                k256::Scalar::from(num)
            } else {
                warn!("No share id found");
                k256::Scalar::ZERO
            };

            let scalar_primitive = elliptic_curve::ScalarPrimitive::<k256::Secp256k1>::from_slice(
                &hex::decode(&la_signed_data.digest).unwrap(),
            )
            .unwrap();
            let data_signed = k256::Scalar::from(scalar_primitive);

            let signed_data: SignedDatak256 = SignedDatak256 {
                sig_type: la_signed_data.sig_type,
                data_signed,
                signature_share: serde_json::from_str(&la_signed_data.signature_share).unwrap(),
                share_id,
                big_r: serde_json::from_str(&la_signed_data.big_r).unwrap(),
                public_key: serde_json::from_str(&la_signed_data.public_key).unwrap(),
                sig_name,
            };
            info!("signed_data: {:?}", signed_data);
            shares.push(serde_json::to_string(&signed_data).unwrap());
        }

        let (signature, recovery_id) =
            recombine_shares_using_wasm(shares).expect("Failed to recombine shares");

        info!("Signature: {:?}", signature);
        info!("Recovery ID: {:?}", recovery_id);
        true
    }

    #[doc = "Counts the number of responses that match the expected values and returns true if the count matches the expected quantity"]
    fn count_matching_responses(
        qty: usize,
        is_success: bool,
        execute_resp: Vec<GenericResponse<JsonExecutionResponse>>,
        values: &Vec<String>,
    ) -> bool {
        let results = execute_resp
            .into_iter()
            .map(|r| {
                assert!(r.ok, "Expected response to succeed but got: {:?}", r);
                assert!(
                    r.data.is_some(),
                    "Expected response to have data but got: {:?}",
                    r
                );
                r.data.unwrap()
            })
            .collect::<Vec<_>>();

        let mut valid = 0;
        for (i, result) in results.iter().enumerate() {
            if result.success != is_success {
                error!(
                    "Bad success: {} - Looking for {}",
                    result.success, is_success
                );
                return false;
            }
            if result.response != values[i] && values[i] != "\"*\"" {
                error!(
                    "Bad response: |{}| - Looking for {}",
                    result.response, values[i]
                );
            } else {
                valid += 1;
            }
        }
        qty == valid
    }

    #[doc = "Checks for an invalid authsig."]
    fn action_failed_with_error(
        execute_resp: Vec<GenericResponse<JsonExecutionResponse>>,
        _params: serde_json::Map<String, Value>,
        value: &str,
    ) -> bool {
        let mut err_count = 0;
        for result in execute_resp.iter() {
            if result.ok {
                error!("Success returned: {} - Looking for false", result.ok);
                return false;
            }
            if result.error.as_ref().unwrap().contains(value)
                || result.error_object.as_ref().unwrap().contains(value)
            {
                err_count += 1;
            }
        }

        err_count == execute_resp.len()
    }

    fn standard_acc(
        _lit_action_code: &str,
        _actions: &Actions,
    ) -> Vec<UnifiedAccessControlConditionItem> {
        vec![UnifiedAccessControlConditionItem::Condition(
            UnifiedAccessControlCondition::JsonAccessControlCondition(JsonAccessControlCondition {
                contract_address: "".to_string(),
                chain: CHAIN_LOCALCHAIN.to_string(),
                standard_contract_type: "".to_string(),
                method: "eth_getBalance".to_string(),
                parameters: vec![":userAddress".to_string(), "latest".to_string()],
                return_value_test: JsonReturnValueTest {
                    comparator: ">".to_string(),
                    value: "0".to_string(),
                },
            }),
        )]
    }

    // create a condition that will always fail.  we check that they have greater than the max UINT256 value which is impossible
    fn impossible_acc(
        _lit_action_code: &str,
        _actions: &Actions,
    ) -> Vec<UnifiedAccessControlConditionItem> {
        vec![UnifiedAccessControlConditionItem::Condition(
        UnifiedAccessControlCondition::JsonAccessControlCondition(JsonAccessControlCondition {
            contract_address: "".to_string(),
            chain: CHAIN_LOCALCHAIN.to_string(),
            standard_contract_type: "".to_string(),
            method: "eth_getBalance".to_string(),
            parameters: vec![":userAddress".to_string(), "latest".to_string()],
            return_value_test: JsonReturnValueTest {
                comparator: ">".to_string(),
                value:
                    "115792089237316195423570985008687907853269984665640564039457584007913129639935"
                        .to_string(),
            },
        }),
    )]
    }

    fn ipfs_acc(
        lit_action_code: &str,
        _actions: &Actions,
    ) -> Vec<UnifiedAccessControlConditionItem> {
        let ipfs_hasher = IpfsHasher::default();
        let cid = ipfs_hasher.compute(lit_action_code.as_bytes());
        let derived_ipfs_id = cid;

        // create a condition that checks if the current IPFS ID is equal to the derived IPFS ID
        vec![UnifiedAccessControlConditionItem::Condition(
            UnifiedAccessControlCondition::JsonAccessControlCondition(JsonAccessControlCondition {
                contract_address: "".to_string(),
                chain: CHAIN_LOCALCHAIN.to_string(),
                standard_contract_type: "".to_string(),
                method: "".to_string(),
                parameters: vec![":currentActionIpfsId".to_string()],
                return_value_test: JsonReturnValueTest {
                    comparator: "=".to_string(),
                    value: derived_ipfs_id,
                },
            }),
        )]
    }

    // create a condition that will check the number of validators in the current epoch
    // this currently isn't used in a test, but suspect it will be.
    #[allow(dead_code)]
    fn evm_acc(
        _lit_action_code: &str,
        actions: &Actions,
    ) -> Vec<UnifiedAccessControlConditionItem> {
        let staking_contract_address =
            format!("0x{}", bytes_to_hex(actions.contracts().staking.address()));
        let abi = r#"
        {
            "inputs": [],
            "name": "getNonShadowValidatorsInCurrentEpochLength",
            "outputs": [
              {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
              }
            ],
            "stateMutability": "view",
            "type": "function"
          }
        "#;

        let eth_abi: ethabi::Function = serde_json::from_str(abi).unwrap();

        // create a condition that will use EVM contract conditions
        vec![UnifiedAccessControlConditionItem::Condition(
            UnifiedAccessControlCondition::EVMContractCondition(EVMContractCondition {
                contract_address: staking_contract_address,
                chain: CHAIN_LOCALCHAIN.to_string(),
                function_name: "getNonShadowValidatorsInCurrentEpochLength".to_string(),
                function_params: vec![],
                function_abi: eth_abi,
                return_value_test: JsonReturnValueTestV2 {
                    key: "".to_string(),
                    comparator: "=".to_string(),
                    value: "3".to_string(),
                },
            }),
        )]
    }

    pub async fn get_encryption_decryption_test_params(
        wallet: Wallet<SigningKey>,
        actions: &Actions,
        to_encrypt: &str,
        lit_actions_code: &str,
        fn_accs: &dyn Fn(&str, &Actions) -> Vec<UnifiedAccessControlConditionItem>,
    ) -> (
        Option<Vec<ControlConditionItem<UnifiedAccessControlCondition>>>,
        String,
        String,
        JsonAuthSig,
    ) {
        let unified_access_control_conditions = Some(fn_accs(lit_actions_code, actions));
        let mut hasher = Sha256::new();
        hasher.update(to_encrypt.as_bytes());
        let data_to_encrypt_hash = bytes_to_hex(hasher.finalize());

        // Get auth sig for auth
        // let wallet = LocalWallet::new(&mut OsRng);
        let auth_sig = generate_authsig(&wallet)
            .await
            .expect("Couldn't generate auth sig");

        // Encrypt.
        let network_pubkey = get_network_pubkey(actions).await;
        let message_bytes = to_encrypt.as_bytes();
        let hashed_access_control_conditions = hash_access_control_conditions(RequestConditions {
            access_control_conditions: None,
            evm_contract_conditions: None,
            sol_rpc_conditions: None,
            unified_access_control_conditions: unified_access_control_conditions.clone(),
        })
        .unwrap();
        let identity_param = AccessControlConditionResource::new(format!(
            "{}/{}",
            hashed_access_control_conditions, data_to_encrypt_hash
        ))
        .get_resource_key()
        .into_bytes();

        debug!("Identity parameter: {:?}", identity_param);
        let pubkey = blsful::PublicKey::try_from(&hex::decode(&network_pubkey).unwrap()).unwrap();

        let ciphertext =
            lit_sdk::encryption::encrypt_time_lock(&pubkey, message_bytes, &identity_param)
                .expect("Unable to encrypt");
        debug!("ciphertext: {:?}", ciphertext);

        (
            unified_access_control_conditions,
            base64_encode_bytes(&serde_bare::to_vec(&ciphertext).unwrap()),
            data_to_encrypt_hash,
            auth_sig,
        )
    }

    #[tokio::test]
    #[ignore]
    async fn test_pkp_has_permission_for_permitted_action() {
        setup_logging();

        let (_testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;
        const IPFS_CID: &str = "QmZuSixiCCkttPDvHKZdAw2gNby111rNEq1aHTozkqztSg";
        let res =
            generate_pkp_check_is_permitted_pkp_action(IPFS_CID, &validator_collection, &end_user)
                .await;
        //res
        assert!(res.is_ok());
        assert!(res.unwrap());
    }

    #[doc = "Signing with MGB PKP within its permitted Lit Action"]
    #[test_case(true, "Anyone can sign with a MGB PKP within its permitted Lit Action")]
    #[test_case(
        false,
        "Any other PKP can sign with a different MGB PKP within its permitted Lit Action"
    )]
    #[tokio::test]
    pub async fn session_sig_with_mgb_pkp_lit_action(
        use_eoa_session_sig: bool,
        test_description: &str,
    ) {
        setup_logging();
        info!(test_description);

        let ipfs_cid = "QmRwN9GKHvCn4Vk7biqtr6adjXMs7PzzYPCzNCRjPFiDjm";

        let (testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;
        let node_set = validator_collection.random_threshold_nodeset().await;
        let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
        let realm_id = ethers::types::U256::from(1);
        let _epoch = validator_collection
            .actions()
            .get_current_epoch(realm_id)
            .await
            .as_u64();

        let mgb_pkp = end_user
            .mint_grant_and_burn_next_pkp(ipfs_cid)
            .await
            .unwrap();

        let mgb_pubkey = mgb_pkp.pubkey;

        let mut js_params = serde_json::Map::new();
        js_params.insert(
            "toSign".to_string(),
            [
                84, 104, 105, 115, 32, 109, 101, 115, 115, 97, 103, 101, 32, 105, 115, 32, 101,
                120, 97, 99, 116, 108, 121, 32, 51, 50, 32, 98, 121, 116, 101, 115,
            ]
            .into(),
        );
        js_params.insert("publicKey".to_string(), mgb_pubkey.into());
        js_params.insert("sigName".to_string(), "sig1".into());

        let params = js_params.clone();
        let js_params = Some(serde_json::Value::Object(js_params));

        // set up a new user and fund their wallet and ledger
        let non_owner_end_user = EndUser::new(&testnet);
        non_owner_end_user.fund_wallet_default_amount().await;
        non_owner_end_user.deposit_to_wallet_ledger_default().await;
        let non_owner_wallet = non_owner_end_user.wallet; // Since anyone can call the permitted Lit Action and sign with the PKP

        let session_sigs_and_node_set = match use_eoa_session_sig {
            true => {
                info!("Generating EOA session sig");
                get_session_sigs_for_auth(
                    &node_set,
                    vec![LitResourceAbilityRequest {
                        resource: LitResourceAbilityRequestResource {
                            resource: "*".to_string(),
                            resource_prefix: LitResourcePrefix::LA.to_string(),
                        },
                        ability: LitAbility::LitActionExecution.to_string(),
                    }],
                    Some(non_owner_wallet),
                    None,
                    None,
                )
            }
            false => {
                info!(
                    "Mint a new PKP with a different owner used only to call the permitted Lit Action of the MGB PKP"
                );

                let mut second_owner_end_user = EndUser::new(&testnet);
                second_owner_end_user.fund_wallet_default_amount().await;
                second_owner_end_user
                    .deposit_to_wallet_ledger_default()
                    .await;

                let _ = second_owner_end_user
                    .new_pkp()
                    .await
                    .expect("Could not mint next pkp");
                let second_owner_pkp_info = second_owner_end_user.first_pkp().info();
                let second_owner_pkp_pubkey = second_owner_pkp_info.0;
                let second_owner_pkp_eth_address = second_owner_pkp_info.2;

                info!("get_session_sigs_and_node_set_for_pkp");
                get_session_sigs_and_node_set_for_pkp(
                    &node_set,
                    second_owner_pkp_pubkey.clone(),
                    second_owner_pkp_eth_address,
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
                    second_owner_end_user.signing_provider().signer().clone(),
                    None,
                    None,
                    None,
                    2,
                    Some(U256::MAX),
                )
                .await
                .expect("Could not get session sigs")
            }
        };

        info!("execute_lit_action_session_sigs");
        let execute_resp = execute_lit_action_session_sigs(
            None,
            Some(ipfs_cid.to_string()),
            js_params,
            None,
            &session_sigs_and_node_set,
            2,
        )
        .await
        .expect("Could not execute lit action");

        assert!(valid_sign_no_combine(execute_resp, params, ""));
    }

    #[tokio::test]
    async fn sign_as_action() {
        setup_logging();
        let (_testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;
        let file_with_path = "./tests/lit_action_scripts/sign_as_lit_action.js";
        let lit_action_code = std::fs::read_to_string(file_with_path).unwrap();
        let action_ipfs_id = lit_sdk::compute_ipfs_hash(&lit_action_code);
        let actions = validator_collection.actions();
        let node_set = validator_collection.random_threshold_nodeset().await;
        let node_set = get_identity_pubkeys_from_node_set(&node_set).await;
        let epoch = actions.get_current_epoch(U256::from(1)).await;
        let unified_access_control_conditions = Some(standard_acc(&lit_action_code, actions));
        let auth_sig = generate_authsig(&end_user.wallet)
            .await
            .expect("Couldn't generate auth sig");
        let (pubkey, _token_id, _eth_address, _key_set_id) = end_user.first_pkp().info();
        let lit_action_code = data_encoding::BASE64.encode(lit_action_code.as_bytes());

        let mut js_params = serde_json::Map::new();
        js_params.insert("publicKey".to_string(), pubkey.clone().into());
        js_params.insert("sigName".to_string(), "sig1".into());
        js_params.insert(
            "accessControlConditions".to_string(),
            serde_json::to_value(unified_access_control_conditions.as_ref().unwrap()).unwrap(),
        );
        js_params.insert(
            "authSig".to_string(),
            serde_json::to_value(&auth_sig).unwrap(),
        );

        let file_with_path = "./tests/lit_action_scripts/get_action_public_key.js";
        let get_action_public_key_code = std::fs::read_to_string(file_with_path).unwrap();
        let get_action_public_key_code =
            data_encoding::BASE64.encode(get_action_public_key_code.as_bytes());

        let file_with_path = "./tests/lit_action_scripts/verify_action_signature.js";
        let verify_action_signature_code = std::fs::read_to_string(file_with_path).unwrap();
        let verify_action_signature_code =
            data_encoding::BASE64.encode(verify_action_signature_code.as_bytes());

        for signing_scheme in [
            SigningScheme::EcdsaK256Sha256,
            SigningScheme::EcdsaP256Sha256,
            SigningScheme::EcdsaP384Sha384,
            SigningScheme::SchnorrEd25519Sha512,
            SigningScheme::SchnorrK256Sha256,
            SigningScheme::SchnorrP256Sha256,
            SigningScheme::SchnorrP384Sha384,
            SigningScheme::SchnorrK256Taproot,
            SigningScheme::SchnorrRistretto25519Sha512,
            SigningScheme::SchnorrEd448Shake256,
            SigningScheme::SchnorrRedJubjubBlake2b512,
            SigningScheme::SchnorrRedDecaf377Blake2b512,
            SigningScheme::SchnorrkelSubstrate,
            SigningScheme::Bls12381G1ProofOfPossession,
        ] {
            let msg_len = if signing_scheme.ecdsa_message_len() > 0 {
                signing_scheme.ecdsa_message_len()
            } else {
                32
            };
            let mut js_params = js_params.clone();
            js_params.insert(
                "signingScheme".to_string(),
                signing_scheme.to_string().into(),
            );
            js_params.insert("toSign".to_string(), vec![1u8; msg_len].into());
            let js_params = Some(Value::Object(js_params.clone()));

            let execute_resp = generate_session_sigs_and_execute_lit_action(
                &node_set,
                end_user.wallet.clone(),
                Some(lit_action_code.clone()),
                None,
                js_params,
                None,
                epoch.as_u64(),
            )
            .await
            .unwrap();
            let root_keys;
            let curve_type = signing_scheme.curve_type();
            loop {
                if let Some(rk) = actions.get_root_keys(curve_type as u8, None).await {
                    root_keys = rk;
                    break;
                }
                let _r = tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            let lit_action_public_key = lit_sdk::signature::get_lit_action_public_key(
                signing_scheme,
                &action_ipfs_id,
                &root_keys,
            )
            .unwrap();
            debug!("lit_action_public_key: {}", lit_action_public_key);

            let mut signed_outputs = Vec::with_capacity(execute_resp.len());
            for ex in &execute_resp {
                assert!(ex.ok, "response returned invalid: {:?}", ex);
                assert!(
                    ex.data.is_some(),
                    "response didn't return a result: {:?}",
                    ex
                );
                let response = ex.data.as_ref().unwrap();
                assert!(
                    response.success,
                    "execution response returned false: {:?}",
                    response
                );
                let outer: String = serde_json::from_str(&response.response).unwrap();
                let output = serde_json::from_str::<SignedDataOutput>(&outer).unwrap();
                assert_eq!(output.verifying_key, lit_action_public_key);
                signed_outputs.push(output);
            }

            let first = &signed_outputs[0];
            for output in &signed_outputs[1..] {
                assert_eq!(first.signature, output.signature);
                assert_eq!(first.signed_data, output.signed_data);
                assert_eq!(first.recovery_id, output.recovery_id);
                assert_eq!(first.verifying_key, output.verifying_key);
            }

            assert!(lit_sdk::signature::verify_signature(signing_scheme, first).is_ok());

            let mut pk_params = serde_json::Map::new();
            pk_params.insert("publicKey".to_string(), pubkey.clone().into());
            pk_params.insert(
                "accessControlConditions".to_string(),
                serde_json::to_value(unified_access_control_conditions.as_ref().unwrap()).unwrap(),
            );
            pk_params.insert(
                "authSig".to_string(),
                serde_json::to_value(&auth_sig).unwrap(),
            );
            pk_params.insert(
                "signingScheme".to_string(),
                signing_scheme.to_string().into(),
            );
            pk_params.insert("actionIpfsCid".to_string(), action_ipfs_id.clone().into());
            let pk_params = Some(Value::Object(pk_params));
            let pk_execute_resp = generate_session_sigs_and_execute_lit_action(
                &node_set,
                end_user.wallet.clone(),
                Some(get_action_public_key_code.clone()),
                None,
                pk_params,
                None,
                epoch.as_u64(),
            )
            .await
            .unwrap();

            for ex in pk_execute_resp {
                assert!(ex.ok, "response returned invalid: {:?}", ex);
                assert!(
                    ex.data.is_some(),
                    "response didn't return a result: {:?}",
                    ex
                );
                let response = ex.data.as_ref().unwrap();
                assert!(
                    response.success,
                    "execution response returned false: {:?}",
                    response
                );
                let outer: String = serde_json::from_str(&response.response).unwrap();
                assert_eq!(outer, first.verifying_key);
            }

            let mut pk_params = serde_json::Map::new();
            pk_params.insert("publicKey".to_string(), pubkey.clone().into());
            pk_params.insert(
                "accessControlConditions".to_string(),
                serde_json::to_value(unified_access_control_conditions.as_ref().unwrap()).unwrap(),
            );
            pk_params.insert(
                "authSig".to_string(),
                serde_json::to_value(&auth_sig).unwrap(),
            );
            pk_params.insert(
                "signingScheme".to_string(),
                signing_scheme.to_string().into(),
            );
            pk_params.insert("actionIpfsCid".to_string(), action_ipfs_id.clone().into());
            pk_params.insert("toSign".to_string(), vec![1u8; msg_len].into());
            pk_params.insert(
                "signOutput".to_string(),
                serde_json::from_str::<String>(&execute_resp[0].data.as_ref().unwrap().response)
                    .unwrap()
                    .into(),
            );
            let pk_params = Some(Value::Object(pk_params));
            let pk_execute_resp = generate_session_sigs_and_execute_lit_action(
                &node_set,
                end_user.wallet.clone(),
                Some(verify_action_signature_code.clone()),
                None,
                pk_params,
                None,
                epoch.as_u64(),
            )
            .await
            .unwrap();

            for ex in pk_execute_resp {
                assert!(ex.ok, "response returned invalid: {:?}", ex);
                assert!(
                    ex.data.is_some(),
                    "response didn't return a result: {:?}",
                    ex
                );
                let response = ex.data.as_ref().unwrap();
                assert!(
                    response.success,
                    "execution response returned false: {:?}",
                    response
                );
                let outer: String = serde_json::from_str(&response.response).unwrap();
                assert_eq!(outer, "true");
            }
        }
    }
}
