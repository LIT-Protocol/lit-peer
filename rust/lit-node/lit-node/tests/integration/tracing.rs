use crate::common::auth_sig::get_session_sigs_for_auth;
use crate::common::lit_actions::execute_lit_action_session_sigs;
use lit_node_core::request::JsonExecutionRequest;
use lit_node_core::response::{GenericResponse, JsonExecutionResponse};
use lit_node_core::{
    Invocation, LitAbility, LitResourceAbilityRequest, LitResourceAbilityRequestResource,
    LitResourcePrefix, NodeSet,
};
use lit_node_testnet::TestSetupBuilder;
use lit_node_testnet::node_collection::get_identity_pubkeys_from_node_set;
use rand::Rng;
use rand_core::OsRng;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};
use tracing::info;

#[tokio::test]
pub async fn test_privacy_mode_logging() {
    crate::common::setup_logging();

    info!("Starting privacy mode test");

    // Setup testnet
    let (_testnet, validator_collection, end_user) = TestSetupBuilder::default().build().await;

    let node_set = validator_collection.random_threshold_nodeset().await;
    let node_set_map = get_identity_pubkeys_from_node_set(&node_set).await;

    let realm_id = ethers::types::U256::from(1);
    let epoch = validator_collection
        .actions()
        .get_current_epoch(realm_id)
        .await
        .as_u64();

    // Read the lit action script
    let lit_action_code =
        std::fs::read_to_string("./tests/lit_action_scripts/broadcast_and_collect.js")
            .expect("failed to read broadcast_and_collect.js");
    let lit_action_code = data_encoding::BASE64.encode(lit_action_code.as_bytes());

    let (pubkey, _token_id, _eth_address) = end_user.first_pkp().info();
    let mut js_params = serde_json::Map::new();
    js_params.insert("publicKey".to_string(), pubkey.into());

    // Generate session sigs
    let session_sigs_and_node_set = get_session_sigs_for_auth(
        &node_set_map,
        vec![LitResourceAbilityRequest {
            resource: LitResourceAbilityRequestResource {
                resource: "*".to_string(),
                resource_prefix: LitResourcePrefix::LA.to_string(),
            },
            ability: LitAbility::LitActionExecution.to_string(),
        }],
        Some(end_user.wallet.clone()),
        None,
        None,
    );

    // Get log readers for the nodes
    let mut log_readers = validator_collection.log_readers();

    // Test 1: Execute without privacy mode
    info!("=== TEST 1: Executing lit action WITHOUT privacy mode ===");

    // Get initial log positions
    let initial_log_positions: Vec<u64> = log_readers
        .iter_mut()
        .map(|reader| reader.stream_position().unwrap_or(0))
        .collect();

    let execute_resp_no_privacy = execute_lit_action_session_sigs(
        Some(lit_action_code.clone()),
        None,
        Some(serde_json::Value::Object(js_params.clone())),
        None,
        &session_sigs_and_node_set,
        epoch,
    )
    .await
    .expect("failed to execute lit action without privacy mode");

    assert!(!execute_resp_no_privacy.is_empty());
    assert!(execute_resp_no_privacy[0].ok);

    // Wait a bit for logs to be written
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    // Read new logs from all nodes
    let logs_no_privacy: Vec<String> = log_readers
        .iter_mut()
        .enumerate()
        .flat_map(|(idx, reader)| {
            let start_pos = initial_log_positions.get(idx).copied().unwrap_or(0);
            read_logs_from_reader(reader, start_pos)
        })
        .collect();

    info!("Logs without privacy mode: {} lines", logs_no_privacy.len());

    // Verify we have some logs
    assert!(
        !logs_no_privacy.is_empty(),
        "Should have logs without privacy mode"
    );

    // Check for some expected log content (like endpoint paths, execution info, etc.)
    let has_endpoint_log = logs_no_privacy
        .iter()
        .any(|line| line.contains("/web/execute") || line.contains("POST /web/execute"));
    assert!(has_endpoint_log, "Should have endpoint logs");

    // Test 2: Execute WITH privacy mode using header
    info!("=== TEST 2: Executing lit action WITH privacy mode (header) ===");

    // Get log positions before privacy mode request
    let log_positions_before_privacy: Vec<u64> = log_readers
        .iter_mut()
        .map(|reader| reader.stream_position().unwrap_or(0))
        .collect();

    // Execute with privacy mode using custom header
    // We need to manually construct the request with the privacy mode header
    let nodes: Vec<NodeSet> = session_sigs_and_node_set
        .iter()
        .map(|sig_and_nodeset| sig_and_nodeset.node.clone())
        .collect();

    let my_private_key = OsRng.r#gen();
    let mut execute_request = lit_sdk::ExecuteFunctionRequest::new()
        .url_prefix(lit_sdk::UrlPrefix::Http)
        .add_custom_header("X-Privacy-Mode", "true")
        .node_set(
            session_sigs_and_node_set
                .iter()
                .map(|sig_and_nodeset| {
                    let execute_request = JsonExecutionRequest {
                        auth_sig: lit_node_core::AuthSigItem::Single(
                            sig_and_nodeset.session_sig.clone(),
                        ),
                        code: Some(lit_action_code.clone()),
                        ipfs_id: None,
                        js_params: Some(serde_json::Value::Object(js_params.clone())),
                        auth_methods: None,
                        epoch,
                        node_set: nodes.clone(),
                        invocation: Invocation::Sync,
                    };
                    lit_sdk::EndpointRequest {
                        node_set: sig_and_nodeset.node.clone(),
                        identity_key: sig_and_nodeset.identity_key,
                        body: execute_request,
                    }
                })
                .collect::<Vec<_>>(),
        )
        .build()
        .expect("failed to build execute request");

    let execute_resp_privacy = execute_request
        .send(&my_private_key)
        .await
        .expect("failed to execute lit action with privacy mode");

    let results: Vec<GenericResponse<JsonExecutionResponse>> =
        execute_resp_privacy.results().to_owned();
    assert!(!results.is_empty());
    assert!(results[0].ok);

    // Wait a bit for logs to be written
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    // Read new logs from all nodes
    let logs_with_privacy: Vec<String> = log_readers
        .iter_mut()
        .enumerate()
        .flat_map(|(idx, reader)| {
            let start_pos = log_positions_before_privacy.get(idx).copied().unwrap_or(0);
            read_logs_from_reader(reader, start_pos)
        })
        .collect();

    info!("Logs with privacy mode: {} lines", logs_with_privacy.len());

    // Verify we have the privacy mode endpoint log
    let has_privacy_endpoint_log = logs_with_privacy.iter().any(|line| {
        line.contains("privacy_mode_request")
            || (line.contains("method") && line.contains("path") && line.contains("POST"))
    });
    assert!(
        has_privacy_endpoint_log,
        "Should have privacy_mode_request log"
    );

    // Verify that detailed logs are filtered out
    // Check that we don't have detailed execution logs that were present without privacy mode
    let has_detailed_execution_logs = logs_with_privacy.iter().any(|line| {
        // Look for logs that would contain sensitive execution details
        // Exclude the privacy_mode_request log from this check
        !line.contains("privacy_mode_request")
            && (line.contains("executing lit action")
                || line.contains("POST /web/execute/v2")
                || (line.contains("execute") && (line.contains("debug") || line.contains("trace"))))
    });

    // We should NOT have detailed execution logs when privacy mode is on
    assert!(
        !has_detailed_execution_logs,
        "Should not have detailed execution logs when privacy mode is enabled"
    );

    // Filter out privacy_mode_request logs for content comparison
    let non_privacy_logs: Vec<_> = logs_with_privacy
        .iter()
        .filter(|line| !line.contains("privacy_mode_request"))
        .collect();

    info!("Non-privacy logs count: {}", non_privacy_logs.len());

    // Compare: logs without privacy should have more content than logs with privacy
    // (excluding the privacy_mode_request log)
    let no_privacy_content: usize = logs_no_privacy.iter().map(|l| l.len()).sum();
    let with_privacy_content: usize = non_privacy_logs.iter().map(|l| l.len()).sum();

    info!("Content length without privacy: {}", no_privacy_content);
    info!(
        "Content length with privacy (excluding privacy_mode_request): {}",
        with_privacy_content
    );

    assert!(
        no_privacy_content > with_privacy_content,
        "Logs with privacy mode should have less content than without privacy mode. \
         No privacy: {}, With privacy: {}",
        no_privacy_content,
        with_privacy_content
    );

    info!("Privacy mode test completed successfully");
}

fn read_logs_from_reader(reader: &mut BufReader<File>, start_position: u64) -> Vec<String> {
    // Seek to start position
    if reader
        .seek(std::io::SeekFrom::Start(start_position))
        .is_err()
    {
        return Vec::new();
    }

    reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect()
}
