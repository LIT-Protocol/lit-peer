//! Rocket fairings for request lifecycle utilities.

use lit_observability::logging::clear_request_context;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};

/// Clears thread-local request context at request start and after response.
///
/// This prevents leakage when threads are reused. Span-based context is unaffected
/// because it is cleaned up with the span lifecycle.
pub struct RequestContextCleanupFairing;

#[rocket::async_trait]
impl Fairing for RequestContextCleanupFairing {
    fn info(&self) -> Info {
        Info {
            name: "Request Context Cleanup",
            kind: Kind::Request | Kind::Response,
        }
    }

    /// Clears any stale thread-local context at the start of a new request.
    async fn on_request(&self, _req: &mut Request<'_>, _data: &mut Data<'_>) {
        clear_request_context();
    }

    /// Clears thread-local request context after the response is sent.
    async fn on_response<'r>(&self, _req: &'r Request<'_>, _res: &mut Response<'r>) {
        clear_request_context();
    }
}
