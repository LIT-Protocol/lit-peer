use std::path::PathBuf;
use std::{fs, sync::Arc};

use futures::FutureExt;

use error::Result;
use lit_api_core::server::hyper::bind_unix_socket;
use lit_api_core::server::hyper::handler::router::Router;
use lit_attestation::config::LitAttestationConfig;
use lit_core::config::LitConfig;

use crate::context::{CTX_KEY_SERVICE_CTX, ServiceContext};
use crate::handler::{attestation, attestation_intent, kdf};

pub mod config;
mod context;
mod data;
mod error;
mod handler;

pub async fn start(cfg: Arc<LitConfig>, socket_path: Option<PathBuf>) -> Result<()> {
    let socket_path = socket_path.unwrap_or_else(|| cfg.attestation_service_socket_path());

    // Remove existing socket file if needed.
    if socket_path.exists() {
        fs::remove_file(&socket_path)
            .unwrap_or_else(|_| panic!("Unable to remove existing socket: {:?}", &socket_path));
    }

    #[rustfmt::skip]
    bind_unix_socket(socket_path, Router::new()
        .attach(CTX_KEY_SERVICE_CTX, Arc::new(ServiceContext::from_lit_config(cfg)
            .expect("Unable to init service context")))
        .post("/attestation/intent", move |req| {
            attestation_intent::handle_req(req).boxed()
        })
        .post("/attestation/confirm", move |req| {
            attestation::handle_req(req).boxed()
        })
        .post("/kdf", move |req| {
            kdf::handle_req(req).boxed()
        }))
        .await;

    Ok(())
}
