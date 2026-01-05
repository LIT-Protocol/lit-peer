use lit_api_core::context::{HEADER_KEY_X_CORRELATION_ID, HEADER_KEY_X_REQUEST_ID};
use lit_observability::logging::{RequestContext, get_request_context, set_request_context};
use rocket::http::HeaderMap;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Value;
use uuid::Uuid;

/// Rocket request guard that extracts HTTP headers from the incoming request.
pub struct RequestHeaders<'r> {
    pub headers: HeaderMap<'r>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders<'r> {
    type Error = Value;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RequestHeaders {
            headers: request.headers().clone(),
        })
    }
}

/// Extracts request context from headers, existing context, or generates fallback.
///
/// Priority: headers > existing span context > new fallback ID.
/// Fallback IDs use "LD-" prefix (Lit Default) to distinguish from client-provided IDs.
fn extract_request_context(headers: &RequestHeaders<'_>) -> RequestContext {
    let x_request_id = headers.headers.get_one(HEADER_KEY_X_REQUEST_ID);
    let x_correlation_id = headers.headers.get_one(HEADER_KEY_X_CORRELATION_ID);

    // Use headers if present
    if x_request_id.is_some() || x_correlation_id.is_some() {
        let correlation_id = x_correlation_id.or(x_request_id).map(String::from);
        let request_id = x_request_id.or(x_correlation_id).map(String::from);
        return RequestContext::new(request_id, correlation_id);
    }

    // Check if another guard already set context (e.g., Tracing guard)
    if let Some(existing_ctx) = get_request_context()
        && existing_ctx.has_context()
    {
        return existing_ctx;
    }

    // Generate fallback ID
    let fallback_id = format!("LD-{}", Uuid::new_v4());
    RequestContext::new(Some(fallback_id.clone()), Some(fallback_id))
}

/// Sets request context on the current span from request headers.
///
/// Use this at the start of endpoints that accept `RequestHeaders` so the
/// handler span carries request context for log injection.
pub fn set_request_id_on_span(headers: &RequestHeaders<'_>) {
    let request_ctx = extract_request_context(headers);
    set_request_context(request_ctx.request_id, request_ctx.correlation_id);
}
