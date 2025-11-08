use crate::error::Result;
use lit_node_common::client_state::{ClientState, IdentityKey};
use lit_node_core::response::GenericResponse;
use lit_sdk::EncryptedPayload;
use rocket::http::Status;
use rocket::response::status;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct ClientSession {
    pub identity_key_used: IdentityKey,
    pub their_public_key: [u8; 32],
    pub client_state: Arc<ClientState>,
}

impl ClientSession {
    pub fn json_encrypt_response<T>(&self, response: T) -> EncryptedPayload<T>
    where
        T: Serialize + DeserializeOwned + Sync,
    {
        self.client_state
            .json_encrypt(self.identity_key_used, &self.their_public_key, response)
            .expect("Failed to encrypt response")
    }

    pub fn json_encrypt_response_status<T>(&self, response: T) -> status::Custom<Value>
    where
        T: Serialize + DeserializeOwned + Sync,
    {
        status::Custom(
            Status::Ok,
            json!(self.json_encrypt_response(GenericResponse::ok(response))),
        )
    }

    pub fn json_encrypt_err_and_code(
        &self,
        error_msg: &str,
        error_code: &str,
        status: Status,
    ) -> status::Custom<Value> {
        let msg = GenericResponse::err_and_data(error_msg.to_string(), error_code.to_string());
        status::Custom(status, json!(self.json_encrypt_response(msg)))
    }

    pub fn json_encrypt_err_response(
        &self,
        error_msg: &str,
        status: Status,
    ) -> status::Custom<Value> {
        let msg = GenericResponse::err(error_msg.to_string());
        status::Custom(status, json!(self.json_encrypt_response(msg)))
    }

    pub fn json_encrypt_err_custom_response(
        &self,
        error_msg: &str,
        handle: status::Custom<Value>,
    ) -> status::Custom<Value> {
        let msg = GenericResponse::err_and_data_json(error_msg.to_string(), handle.1);
        status::Custom(handle.0, json!(self.json_encrypt_response(msg)))
    }
}

pub trait ClientSessionHandler {
    fn json_decrypt_to_session<I>(
        &self,
        payload: &EncryptedPayload<I>,
    ) -> Result<(I, ClientSession)>
    where
        I: Serialize + DeserializeOwned + Sync;
}

impl ClientSessionHandler for Arc<ClientState> {
    fn json_decrypt_to_session<I>(
        &self,
        payload: &EncryptedPayload<I>,
    ) -> Result<(I, ClientSession)>
    where
        I: Serialize + DeserializeOwned + Sync,
    {
        let (msg, identity_key_used, their_public_key) = self.json_decrypt(payload)?;

        Ok((
            msg,
            ClientSession {
                identity_key_used,
                their_public_key,
                client_state: self.clone(),
            },
        ))
    }
}
