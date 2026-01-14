//! Rocket fairings for request lifecycle utilities.

use lit_observability::logging::clear_task_request_context;
use rocket::Data;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

/// Clears task-local request context at request boundaries to avoid stale IDs.
pub struct RequestContextCleanupFairing;

#[rocket::async_trait]
impl Fairing for RequestContextCleanupFairing {
    fn info(&self) -> Info {
        Info {
            name: "Request Context Cleanup",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, _req: &mut Request<'_>, _data: &mut Data<'_>) {
        clear_task_request_context();
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, _res: &mut Response<'r>) {
        clear_task_request_context();
    }
}
