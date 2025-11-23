use leptos::prelude::*;
// use std::time::Instant;
use crate::{
    pages::validators::Validator,
    utils::sdk_models::{JsonSDKHandshakeResponse, ResponseWrapper},
};

#[component]
pub fn ValidatorHandshake(row: RwSignal<Validator>) -> impl IntoView {
    let data = LocalResource::new(move || {
        let row = row.clone();
        async move { handshake_node(row).await }
    });

    view! {
        <p>{move || match data.get().as_deref() {
            None => view! { <p>"Loading..."</p> }.into_any(),
            Some(data) => view! { <p>{data.node_version.clone()}</p> }.into_any()
        }}</p>
    }
}

async fn handshake_node(row: RwSignal<Validator>) -> JsonSDKHandshakeResponse {
    let socket_address = row.get_untracked().socket_address.clone();
    let socket_address = format!("https://{}", socket_address);
    log::info!("Handshaking node: {:?}", socket_address);
    if socket_address.contains("0.0.0.0") {
        return JsonSDKHandshakeResponse::default();
    }

    let json_body = r#"{"clientPublicKey":"blah","challenge":"0x1234123412341234123412341234123412341234123412341234123412341234"}"#.to_string();
    let cmd = "/web/handshake";
    let request_id = &uuid::Uuid::new_v4().to_string();
    let client = reqwest::Client::new();
    let resp_string = client
        .post(format!("{}/{}", socket_address, cmd))
        .header("Content-Type", "application/json")
        .header("X-Request-Id", request_id)
        .body(json_body.clone())
        .send()
        .await;

    if resp_string.is_err() {
        log::error!("Error getting node info: {:?}", resp_string.err());
        return JsonSDKHandshakeResponse::default();
    }

    let resp = resp_string.unwrap();

    let resp_string = match resp.text().await {
        Ok(text) => text,
        Err(e) => {
            log::error!("Error getting handshake body: {:?}", e);
            return JsonSDKHandshakeResponse::default();
        }
    };
    log::info!("Response: {:?}", resp_string);

    let response_wrapper: ResponseWrapper = match serde_json::from_str(&resp_string) {
        Ok(response_wrapper) => response_wrapper,
        Err(e) => {
            log::error!("Error parsing response wrapper: {:?}", e);
            return JsonSDKHandshakeResponse::default();
        }
    };

    let handshake_result: JsonSDKHandshakeResponse = match response_wrapper.ok {
        true => match serde_json::from_value(response_wrapper.data) {
            Ok(handshake_result) => handshake_result,
            Err(e) => {
                log::error!("Error parsing handshake response: {:?}", e);
                return JsonSDKHandshakeResponse::default();
            }
        },
        false => {
            if let Some(error_object) = response_wrapper.error_object {
                let error_handshake_result: JsonSDKHandshakeResponse =
                    match serde_json::from_str(&error_object) {
                        Ok(error_handshake_result) => error_handshake_result,
                        Err(e) => {
                            log::error!("Error parsing error handshake response: {:?}", e);
                            JsonSDKHandshakeResponse::default()
                        }
                    };
                error_handshake_result
            } else {
                JsonSDKHandshakeResponse::default()
            }
        }
    };

    let row_data = row.get_untracked();
    let updated_row = Validator {
        id: row_data.id,
        host_name: row_data.host_name,
        status: "Up".to_string(),
        wallet_address: row_data.wallet_address,
        staker_address: row_data.staker_address,
        ver: handshake_result.node_version.clone(),
        socket_address: row_data.socket_address.clone(),
        commit_hash: handshake_result.git_commit_hash.clone(),
        network_public_key: handshake_result.network_public_key.clone(),
        node_identity_key: handshake_result.node_identity_key.clone(),
        epoch: handshake_result.epoch,
    };
    row.set(updated_row);

    handshake_result
}
