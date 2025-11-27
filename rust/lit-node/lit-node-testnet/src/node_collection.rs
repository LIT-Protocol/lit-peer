use super::testnet::actions::Actions;

use anyhow::Result;
use ethers::types::U256;
use ethers::utils::hex;
use futures::future::join_all;
use lit_node_core::response::GenericResponse;
use lit_node_core::response::JsonSDKHandshakeResponse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use tracing::error;
use tracing::info;
use tracing::trace;

use lit_node_common::client_state::KeyPair;
use lit_node_core::NodeSet;
use lit_sdk::EncryptedPayload;
use reqwest;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub type NodeIdentityKey = [u8; 32];

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NodeStakingStatus {
    PreviouslyStaked,
    StakedAndJoined,
    FailedToStake,
    Unstaked,
}

#[derive(Debug, Clone)]
pub struct EndpointRequest<B: Serialize + DeserializeOwned + Sync + Clone + Debug> {
    pub node: NodeSet,
    pub identity_key: NodeIdentityKey,
    pub body: B,
}

// Config that will be used for launching node.
// May either be passed as environment variables, or loaded into a config file.
#[derive(Debug)]
pub struct NodeConfig {
    pub lit_domain_name: String,
    pub rocket_port: String,
    pub staker_address: String,
    pub enable_proxied_chatter_client: Option<bool>,
}

fn populate_request_with_headers(
    request_builder: reqwest::RequestBuilder,
    custom_headers: &Option<HashMap<String, String>>,
    default_request_id: String,
) -> reqwest::RequestBuilder {
    match &custom_headers {
        Some(headers) => {
            let mut request_builder = request_builder;
            for (key, value) in headers.iter() {
                request_builder = request_builder.header(key, value);
            }

            // Set Content-Type: application/json if not set
            request_builder = match headers.get("Content-Type") {
                Some(_) => request_builder,
                None => request_builder.header("Content-Type", "application/json"),
            };

            // Set X-Request-Id: random UUID if not set
            request_builder = match headers.get("X-Request-Id") {
                Some(_) => request_builder,
                None => request_builder.header("X-Request-Id", default_request_id),
            };

            request_builder
        }
        None => request_builder
            .header("Content-Type", "application/json")
            .header("X-Request-Id", default_request_id),
    }
}

pub async fn hit_ports_with_json_body_join_all<E, D>(
    node_set: &HashMap<NodeSet, NodeIdentityKey>,
    cmd: String,
    body: E,
) -> Result<Vec<GenericResponse<D>>>
where
    E: serde::Serialize + DeserializeOwned + Sync + Clone + Debug,
    D: serde::Serialize + DeserializeOwned + Sync + Debug,
{
    let mut v = Vec::with_capacity(node_set.len());
    let mut client_keys = Vec::with_capacity(node_set.len());

    let request_id = &uuid::Uuid::new_v4().to_string();
    let client = reqwest::Client::new();
    for (node, identity_key) in node_set {
        let key_pair = KeyPair::generate();

        let payload = key_pair
            .json_encrypt(identity_key, &body)
            .expect("Failed to encrypt payload");
        let json_body = serde_json::to_string(&payload).expect("Failed to serialize payload");

        let request_builder = client.post(format!("{}/{}", node.socket_address, cmd));
        let request_builder =
            populate_request_with_headers(request_builder, &None, request_id.clone());
        let resp = request_builder.body(json_body.clone()).send();
        v.push(resp);
        client_keys.push(key_pair);
    }

    info!("Starting sign for {:?}", body);
    let results = join_all(v).await;
    info!("Finished sign for {:?}", body);

    let mut responses: Vec<GenericResponse<D>> = Vec::new();
    for (result, key_pair) in results.into_iter().zip(client_keys.iter()) {
        match result {
            Ok(resp) => {
                let json_response = resp.text().await.expect("Failed to read response text");
                let encrypted_response =
                    serde_json::from_str::<EncryptedPayload<GenericResponse<D>>>(&json_response)
                        .unwrap();
                let (response, _) = key_pair.json_decrypt(&encrypted_response).unwrap();

                responses.push(response);
            }
            Err(e) => {
                error!("Error hitting an endpoint: {:?}", e);
            }
        }
    }

    info!("responses: {:?}", responses);

    Ok(responses)
}

pub async fn get_network_pubkey(actions: &Actions) -> String {
    let realm_id = U256::from(1);

    let node_set = actions
        .get_current_validator_structs(realm_id)
        .await
        .iter()
        .map(|validator| NodeSet {
            socket_address: format!(
                "{}:{}",
                validator.ip.to_string(),
                validator.port.to_string().clone()
            ),
            value: 1,
        })
        .collect::<Vec<NodeSet>>();

    get_network_pubkey_from_node_set(node_set.iter()).await
}

pub async fn get_network_pubkey_from_node_set<'a, I>(node_set: I) -> String
where
    I: Iterator<Item = &'a NodeSet>,
{
    let response = do_handshake(node_set).await;
    let results = response.results();
    assert!(results[0].ok);

    // Parse response
    results[0].data.as_ref().unwrap().network_public_key.clone()
}

pub async fn get_identity_pubkeys_from_node_set(
    node_set: &[NodeSet],
) -> HashMap<NodeSet, NodeIdentityKey> {
    let response = do_handshake(node_set.iter()).await;
    let responses = response.results();
    let mut map = HashMap::with_capacity(responses.len());
    for (node, response) in node_set.iter().zip(responses.iter()) {
        assert!(
            response.ok,
            "Handshake response contains error: {} - {:?}, {:#?}",
            response.error.as_ref().unwrap(),
            response.data,
            response.error_object.as_ref().unwrap(),
        );
        let data = response.data.as_ref().unwrap();
        assert!(!data.node_identity_key.is_empty());
        let mut identity_key = [0u8; 32];
        let res = hex::decode_to_slice(&data.node_identity_key, &mut identity_key);
        assert!(
            res.is_ok(),
            "Failed to decode identity key from node {} into 32 bytes",
            node.socket_address
        );
        map.insert(node.clone(), identity_key);
    }
    map
}

async fn handshake_nodes(
    actions: &Actions,
    realm_id: U256,
) -> Vec<GenericResponse<JsonSDKHandshakeResponse>> {
    let validators = actions.get_current_validator_structs(realm_id).await;
    let node_set = validators
        .iter()
        .map(|validator| NodeSet {
            socket_address: format!("127.0.0.1:{}", validator.port.to_string().clone()),
            value: 1,
        })
        .collect::<Vec<NodeSet>>();

    do_handshake(node_set.iter()).await.results().to_owned()
}

pub async fn handshake_returns_keys(actions: &Actions, realm_id: U256) -> bool {
    let responses = handshake_nodes(&actions, realm_id).await;

    for response in &responses {
        // Ensure no errors
        if !response.ok {
            info!("Handshake response contains error: {:?}", response);
            return false;
        }

        let response_data = response.data.as_ref().unwrap();

        // Ensure the network public key is the correct length
        if (response_data.subnet_public_key.len() != 96)
            || (response_data.network_public_key.len() != 96)
            || (response_data.network_public_key_set.len() != 96)
        {
            info!(
                "Handshake response contains incorrect length public key: {:?}",
                response
            );
            return false;
        }
    }

    info!("Handshake response contains correct keys");
    true
}

pub async fn ensure_min_node_epoch(actions: &Actions, realm_id: U256, min_epoch: u64) -> bool {
    let responses = handshake_nodes(&actions, realm_id).await;
    for response in &responses {
        // Ensure no errors
        if !response.ok {
            return false;
        }
        let response_data = response.data.as_ref().unwrap();
        if response_data.epoch < min_epoch {
            return false;
        }
    }

    true
}

pub async fn get_node_versions(node_set: &Vec<NodeSet>) -> Vec<String> {
    let response = do_handshake(node_set.iter()).await;
    let headers = response.headers();

    // Parse response headers
    headers
        .iter()
        .map(|header| header.get("x-lit-node-version").unwrap().clone())
        .collect::<Vec<String>>()
}

async fn do_handshake<'a, I>(node_set: I) -> lit_sdk::HandshakeResponse
where
    I: Iterator<Item = &'a NodeSet>,
{
    lit_sdk::HandshakeRequest::new()
        .node_set_from_iter(node_set)
        .url_prefix(lit_sdk::UrlPrefix::Http)
        .challenge("0x1234123412341234123412341234123412341234123412341234123412341234".to_string())
        .client_public_key("blah".to_string())
        .build()
        .unwrap()
        .send()
        .await
        .unwrap()
}

/// This function is used to hit endpoints with different json bodies per port.
/// It uses the given validators if any, or gets the current validators from the chain.
pub async fn hit_endpoints_with_json_body_per_port<E, D>(
    endpoint_requests: &[EndpointRequest<E>],
    cmd: String,
    custom_headers: &Option<HashMap<String, String>>,
) -> Vec<GenericResponse<D>>
where
    E: Serialize + DeserializeOwned + Sync + Clone + Debug,
    D: Serialize + DeserializeOwned + Sync + Debug,
{
    // If the number of json bodies is not equal to the number of ports, then panic.
    // assert_eq!(json_body_vec.len(), portnames.len());

    info!("Endpoint requests count: {:?}", endpoint_requests.len());

    let request_id = uuid::Uuid::new_v4().to_string();
    let client = reqwest::Client::new();
    let futures = endpoint_requests.iter().map(|request| {
        let keypair = KeyPair::generate();
        let payload = keypair
            .json_encrypt(&request.identity_key, &request.body)
            .unwrap();

        let json_body = serde_json::to_string(&payload).unwrap();
        let request_id = request_id.clone();
        let cmd = cmd.clone();
        let client_clone = client.clone();
        async move {
            info!(
                "Endpoint url: {:?}",
                format!("http://{}/{}", request.node.socket_address, cmd)
            );
            let request_builder =
                client_clone.post(format!("http://{}/{}", request.node.socket_address, cmd));
            let request_builder =
                populate_request_with_headers(request_builder, custom_headers, request_id);
            let response = request_builder
                .body(json_body)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let encrypted_response: EncryptedPayload<GenericResponse<D>> =
                serde_json::from_str(&response).unwrap();
            let (response, _) = keypair.json_decrypt(&encrypted_response).unwrap();
            response
        }
    });
    let responses: Vec<GenericResponse<D>> = futures::future::join_all(futures).await;
    trace!("responses: {:?}", responses);
    responses
}

pub fn choose_random_indices(array_size: usize, num_random_indices: usize) -> HashSet<usize> {
    let mut indices = HashSet::new();
    for _ in 0..num_random_indices {
        let mut idx = rand::random::<usize>() % array_size;
        while indices.contains(&idx) {
            idx = rand::random::<usize>() % array_size;
        }
        indices.insert(idx);
    }
    indices
}

pub async fn get_current_validator_portnames(actions: &Actions) -> Vec<String> {
    // Fetch the portnames from the chain state
    let realm_id = U256::from(1);
    let validators = actions.get_current_validator_structs(realm_id).await;
    validators
        .iter()
        .map(|validator| validator.port.to_string().clone())
        .collect::<Vec<String>>()
}
