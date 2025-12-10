use lit_api_core::context::HEADER_KEY_X_REQUEST_ID;
use rocket::http::HeaderMap;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Value;
use tracing_opentelemetry::OpenTelemetrySpanExt;

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

/// Sets the X-Request-Id header value as an attribute on the current tracing span.
/// This ensures the request_id appears in all logs exported to GCP.
pub fn set_request_id_on_span(headers: &RequestHeaders<'_>) {
    let request_id = headers
        .headers
        .get_one(HEADER_KEY_X_REQUEST_ID)
        .unwrap_or("unknown_req_id")
        .to_string();

    let span = tracing::Span::current();
    // Set the request_id as an attribute on the OpenTelemetry span
    // This will ensure it appears in all logs exported to GCP
    span.set_attribute("request_id", request_id.clone());
    // Also record it as a tracing field for local logging
    span.record("request_id", tracing::field::display(&request_id));
}
