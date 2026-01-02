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
        Outcome::Success(RequestHeaders { headers: request.headers().clone() })
    }
}

/// Extracts request_id and correlation_id from headers with fallback handling.
///
/// # Priority order
/// 1. **Headers present**: Uses `X-Request-Id` and/or `X-Correlation-Id` headers directly
/// 2. **Existing context**: Checks if another guard (e.g., `Tracing`) already set context
///    on the current span or its ancestors via `get_request_context()`
/// 3. **Generate fallback**: Creates a new UUID with "LD-" (Lit Default) prefix
///
/// # Preventing Fallback ID Divergence
/// When headers are absent, multiple components could potentially generate different
/// fallback IDs. This function prevents divergence by:
/// - Checking `get_request_context()` which walks the span hierarchy to find existing context
/// - Only generating a new fallback ID if no context exists anywhere in the span tree
///
/// # Span Assumptions
/// This function relies on `Span::current()` returning a span that is either:
/// - The same span where `Tracing` guard set context, OR
/// - A descendant span that can find ancestor context via `get_request_context()`
///
/// In Rocket, request guards typically run within the request handling context,
/// so `Span::current()` should be consistent across guards in the same request.
fn extract_request_context(headers: &RequestHeaders<'_>) -> RequestContext {
    let x_request_id = headers.headers.get_one(HEADER_KEY_X_REQUEST_ID);
    let x_correlation_id = headers.headers.get_one(HEADER_KEY_X_CORRELATION_ID);

    // If headers are present, use them directly
    if x_request_id.is_some() || x_correlation_id.is_some() {
        let correlation_id = x_correlation_id.or(x_request_id).map(String::from);
        let request_id = x_request_id.or(x_correlation_id).map(String::from);
        return RequestContext::new(request_id, correlation_id);
    }

    // Check if context was already set by another guard (e.g., Tracing guard)
    // This ensures both systems use the same ID even when headers are missing.
    if let Some(existing_ctx) = get_request_context()
        && existing_ctx.has_context()
    {
        return existing_ctx;
    }

    // Generate new fallback ID only if no headers and no existing context
    let fallback_id = format!("LD-{}", Uuid::new_v4());
    RequestContext::new(Some(fallback_id.clone()), Some(fallback_id))
}

/// Sets request context on the current span from request headers.
///
/// This is a convenience function for endpoints that use `RequestHeaders` guard
/// instead of the `Tracing` guard. It delegates to `set_request_context()` which
/// handles both span extensions (for log injection) and OTel span attributes
/// (for distributed tracing).
///
/// # When to use
/// - Use this when your endpoint uses `RequestHeaders` but not `Tracing` guard
/// - If your endpoint already uses `Tracing` or `TracingRequired` guard, calling
///   this is redundant (but harmless) as those guards already set context
///
/// # Note on span fields
/// Tracing span fields (via `span.record()`) are NOT used because fields must be
/// declared when the span is created. The request-level spans are created before
/// we have access to headers, so we cannot add fields dynamically.
pub fn set_request_id_on_span(headers: &RequestHeaders<'_>) {
    let request_ctx = extract_request_context(headers);
    // set_request_context handles both span extensions AND OTel attributes
    set_request_context(request_ctx.request_id, request_ctx.correlation_id);
}
