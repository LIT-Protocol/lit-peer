use rand::{Rng, distributions::Alphanumeric};
use rocket::response::status;
use rocket::serde::json::{Json, Value, serde_json::json};
use rocket::{State, get, http::Status, post};

use crate::models::TestNetClientCommand;
use crate::shiva_client::ShivaClient;

use super::models::{TestNetCreateRequest, TestNetInfo, TestNetResponse, TestNetState};

#[post("/test/create/testnet", format = "json", data = "<create_request>")]
pub(crate) async fn create_testnet(
    client: &State<ShivaClient>,
    create_request: Json<TestNetCreateRequest>,
) -> status::Custom<Value> {
    let testnet_ids = client.get_testnet_ids().await;
    let testnet_ids = testnet_ids.unwrap();
    if testnet_ids.len() > 1 {
        return bad_request(
            "".to_string(),
            TestNetClientCommand::CreateTestnet,
            vec!["Currently only a single testnet may be managed at a time".to_string()],
        );
    }

    let session_id: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(36)
        .map(char::from)
        .collect();

    if create_request.custom_build_path.is_none()
        || create_request.lit_action_server_custom_build_path.is_none()
    {
        return bad_request(
            session_id,
            TestNetClientCommand::CreateTestnet,
            vec![
                "Must provide lit action and lit node binaries for running custom builds"
                    .to_string(),
            ],
        );
    };

    match client
        .create_testnets(session_id.to_string(), create_request.0)
        .await
    {
        Ok(state) => status::Custom(
            Status::Ok,
            json!(TestNetResponse::<()> {
                testnet_id: session_id,
                command: TestNetClientCommand::CreateTestnet,
                body: Some(()),
                last_state_observed: Some(format_state(&state)),
                ..Default::default()
            }),
        ),
        Err(e) => error_response(session_id, TestNetClientCommand::CreateTestnet, e),
    }
}

#[get("/test/delete/testnet/<id>")]
pub(crate) async fn delete_testnet(client: &State<ShivaClient>, id: &str) -> status::Custom<Value> {
    let del_status = client.delete_testnet(id.to_string()).await;

    match del_status {
        Ok(state) => status::Custom(
            Status::Ok,
            json!(TestNetResponse::<bool> {
                testnet_id: id.to_string(),
                command: TestNetClientCommand::Shutdown,
                body: Some(true),
                last_state_observed: Some(format_state(&state)),
                ..Default::default()
            }),
        ),
        Err(e) => error_response(id.to_string(), TestNetClientCommand::Shutdown, e),
    }
}

#[get("/test/poll/testnet/<id>")]
pub(crate) async fn poll_testnet(client: &State<ShivaClient>, id: &str) -> status::Custom<Value> {
    let poll_status = client.poll_testnet_state(id.to_string()).await;

    match poll_status {
        Ok(status) => status::Custom(
            Status::Ok,
            json!(TestNetResponse::<String> {
                testnet_id: id.to_string(),
                command: TestNetClientCommand::Poke,
                body: Some(format!("{:?}", status)),
                last_state_observed: Some(format_state(&status)),
                ..Default::default()
            }),
        ),
        Err(e) => error_response(id.to_string(), TestNetClientCommand::Poke, e),
    }
}

#[get("/test/get/info/testnet/<id>")]
pub(crate) async fn get_info_testnet(
    client: &State<ShivaClient>,
    id: &str,
) -> status::Custom<Value> {
    let info_status = client.get_testnet_info(id.to_string()).await;
    match info_status {
        Ok(testnet_info) => match client.get_testnet_state(id.to_string()).await {
            Ok(status) => status::Custom(
                Status::Ok,
                json!(TestNetResponse::<TestNetInfo> {
                    testnet_id: id.to_string(),
                    command: TestNetClientCommand::GetInfo,
                    body: Some(testnet_info),
                    last_state_observed: Some(format_state(&status)),
                    ..Default::default()
                }),
            ),
            Err(e) => error_response(id.to_string(), TestNetClientCommand::GetInfo, e),
        },
        Err(e) => error_response(id.to_string(), TestNetClientCommand::GetInfo, e),
    }
}

#[get("/test/get/testnets")]
pub(crate) async fn get_testnets(client: &State<ShivaClient>) -> status::Custom<Value> {
    let testnets = client.get_testnet_ids().await;
    match testnets {
        Ok(ids) => status::Custom(Status::Ok, json!(ids.clone())),
        Err(e) => error_response("".to_string(), TestNetClientCommand::GetTestnets, e),
    }
}

#[get("/test/action/stop/random/<id>")]
pub(crate) async fn stop_random_node_testnet(
    client: &State<ShivaClient>,
    id: &str,
) -> status::Custom<Value> {
    let stop_status = client.stop_random_node(id.to_string()).await;
    match stop_status {
        Ok(status) => {
            let current_state = client.get_testnet_state(id.to_string()).await;
            status::Custom(
                Status::Ok,
                json!(TestNetResponse::<usize> {
                    testnet_id: id.to_string(),
                    command: TestNetClientCommand::StopRandom,
                    body: Some(status),
                    last_state_observed: current_state.ok().as_ref().map(format_state),
                    ..Default::default()
                }),
            )
        }
        Err(e) => error_response(id.to_string(), TestNetClientCommand::StopRandom, e),
    }
}

#[get("/test/action/stop/random/wait/<id>")]
pub(crate) async fn stop_random_node_and_wait_testnet(
    client: &State<ShivaClient>,
    id: &str,
) -> status::Custom<Value> {
    let wait_status = client.stop_random_node_wait(id.to_string()).await;
    match wait_status {
        Ok(status) => {
            let state = client.get_testnet_state(id.to_string()).await;
            status::Custom(
                Status::Ok,
                json!(TestNetResponse::<usize> {
                    testnet_id: id.to_string(),
                    command: TestNetClientCommand::StopRandomAndWait,
                    body: Some(status),
                    last_state_observed: state.ok().as_ref().map(format_state),
                    ..Default::default()
                }),
            )
        }
        Err(e) => error_response(id.to_string(), TestNetClientCommand::StopRandomAndWait, e),
    }
}

#[get("/test/action/transition/epoch/wait/<id>")]
pub(crate) async fn transition_epoch_and_wait(
    client: &State<ShivaClient>,
    id: &str,
) -> status::Custom<Value> {
    let transition_status = client.transition_epoch_wait(id.to_string()).await;
    match transition_status {
        Ok(status) => {
            let current_state = client.get_testnet_state(id.to_string()).await;
            status::Custom(
                Status::Ok,
                json!(TestNetResponse::<bool> {
                    testnet_id: id.to_string(),
                    command: TestNetClientCommand::TransitionEpochAndWait,
                    body: Some(status),
                    last_state_observed: current_state.ok().as_ref().map(format_state),
                    ..Default::default()
                }),
            )
        }
        Err(e) => error_response(
            id.to_string(),
            TestNetClientCommand::TransitionEpochAndWait,
            e,
        ),
    }
}

fn error_response(
    id: String,
    command: TestNetClientCommand,
    error: anyhow::Error,
) -> status::Custom<Value> {
    status::Custom(
        Status::InternalServerError,
        json!(TestNetResponse::<()> {
            testnet_id: id,
            command: command,
            body: Some(()),
            last_state_observed: Some("UNKNOWN".to_string()),
            errors: Some(vec![error.to_string()]),
            ..Default::default()
        }),
    )
}

fn bad_request(
    id: String,
    command: TestNetClientCommand,
    errors: Vec<String>,
) -> status::Custom<Value> {
    status::Custom(
        Status::BadRequest,
        json!(TestNetResponse::<()> {
            testnet_id: id,
            command,
            body: Some(()),
            last_state_observed: Some("BAD_REQUEST".to_string()),
            errors: Some(errors),
            ..Default::default()
        }),
    )
}

fn format_state(state: &TestNetState) -> String {
    format!("{:?}", state)
}
