use lit_api_core::context::{Tracing, with_context};
use rocket::Route;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Status {
    pub ok: bool,
}

#[get("/status")]
pub(crate) async fn ep_status(tracing: Tracing) -> Json<Status> {
    with_context(tracing, async move {
        // TODO:
        Json(Status { ok: true })
    })
    .await
}

pub(crate) fn routes() -> Vec<Route> {
    routes![ep_status]
}
