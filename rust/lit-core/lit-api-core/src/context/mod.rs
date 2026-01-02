use std::collections::HashMap;
use std::fmt;
use std::future::Future;

use opentelemetry::propagation::{Extractor, Injector, TextMapPropagator};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio::task::futures::TaskLocalFuture;
use tokio::task_local;
use tracing::{Span, info_span};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

use lit_observability::logging::set_request_context;

use crate::error::{EC, Error, Result, conversion_err_code, validation_err_code};

pub const HEADER_KEY_X_CORRELATION_ID: &str = "X-Correlation-Id";
pub const HEADER_KEY_X_REQUEST_ID: &str = "X-Request-Id";
pub const HEADER_KEY_X_LIT_SDK_VERSION: &str = "X-Lit-SDK-Version";

pub const TRACKING_LOG_KEY_LIT_SDK_VERSION: &str = "lit_sdk_version";

task_local! {
    pub static TRACING: Box<dyn Tracer>;
}

/// The TracingSpan request guard creates a new tracing span for the request. If the request
/// contains a parent span ID, it will be used as the parent of this new span. Otherwise, a new
/// root span will be created.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TracingSpan {
    span: Span,
}

impl TracingSpan {
    pub fn new(span: Span) -> Self {
        Self { span }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TracingSpan {
    type Error = crate::error::Error;

    async fn from_request(
        req: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        // Extract the propagated context
        let propagator = TraceContextPropagator::new();
        // Initialize some container to hold the header information.
        let mut carrier = HashMap::new();
        // Transfer header information from request to carrier.
        for header in req.headers().iter() {
            carrier.insert(header.name().to_string(), header.value().to_string());
        }
        // Extract the context from the carrier
        let context = propagator.extract(&HeaderExtractor::from(&carrier));

        // Initialize a new span with the propagated context as the parent
        let req_method = req.method();
        let req_path = req.uri().path();
        let new_span = info_span!(
            "handle_request",
            method = req_method.as_str(),
            path = req_path.to_string(),
        );
        new_span.set_parent(context);

        Outcome::Success(TracingSpan { span: new_span })
    }
}

pub struct HeaderExtractor<'a> {
    headers: &'a HashMap<String, String>,
}

impl<'a> From<&'a HashMap<String, String>> for HeaderExtractor<'a> {
    fn from(headers: &'a HashMap<String, String>) -> Self {
        HeaderExtractor { headers }
    }
}

impl<'a> Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&'a str> {
        self.headers.get(key).map(|v| v.as_str())
    }

    fn keys(&self) -> Vec<&str> {
        self.headers.keys().map(|v| v.as_str()).collect()
    }
}

pub struct HeaderInjector<'a> {
    headers: &'a mut HashMap<String, String>,
}
impl<'a> From<&'a mut HashMap<String, String>> for HeaderInjector<'a> {
    fn from(headers: &'a mut HashMap<String, String>) -> Self {
        HeaderInjector { headers }
    }
}

impl<'a> Injector for HeaderInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        self.headers.insert(key.to_string(), value);
    }
}

pub trait Tracer: Sync + Send {
    fn correlation_id(&self) -> &String;
    fn add_field(&mut self, key: String, value: Value);
    fn apply_fields(&self, fields: &mut Map<String, Value>) -> Result<()>;
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracing {
    correlation_id: String,
    fields: Option<HashMap<String, Value>>,
}

impl Tracing {
    pub fn new(correlation_id: String) -> Self {
        Self { correlation_id, fields: None }
    }
}

impl Tracer for Tracing {
    fn correlation_id(&self) -> &String {
        &self.correlation_id
    }

    /// Add some fields to the structured logs.
    #[allow(dead_code)]
    fn add_field(&mut self, key: String, value: Value) {
        let mut fields = match self.fields.take() {
            Some(v) => v,
            None => HashMap::new(),
        };

        fields.insert(key, value);
        self.fields = Some(fields);
    }

    /// Apply the fields for structured logging (used in the logging plugins)
    fn apply_fields(&self, fields: &mut Map<String, Value>) -> Result<()> {
        if let Some(our_fields) = self.fields.as_ref() {
            for (key, value) in our_fields.iter() {
                fields.insert(key.clone(), value.clone());
            }
        }

        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Tracing {
    type Error = crate::error::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let (request_id, correlation_id) = extract_request_and_correlation_ids(req);

        // Generate fallback ID only if BOTH headers are missing
        let fallback_id = || format!("LD-{}", Uuid::new_v4());
        let correlation_id = correlation_id.unwrap_or_else(&fallback_id);
        let request_id = request_id.unwrap_or_else(|| correlation_id.clone());

        // Set request context on the current span. This centralizes context storage:
        // 1. Span extensions (RequestContext) - for log injection in OTLP and stdout
        // 2. OTel span attributes - for distributed tracing correlation
        // This ensures other components (like set_request_id_on_span) can find
        // this context via get_request_context() and avoid generating duplicate IDs.
        set_request_context(Some(request_id), Some(correlation_id.clone()));

        let mut tracing = Self::new(correlation_id);
        apply_req_tracing_fields(req, &mut tracing);

        Outcome::Success(tracing)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TracingRequired {
    correlation_id: String,
    fields: Option<HashMap<String, Value>>,
}

impl TracingRequired {
    pub fn new(correlation_id: String) -> Self {
        Self { correlation_id, fields: None }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.correlation_id.into_bytes()
    }
}

impl Tracer for TracingRequired {
    fn correlation_id(&self) -> &String {
        &self.correlation_id
    }

    /// Add some fields to the structured logs.
    fn add_field(&mut self, key: String, value: Value) {
        let mut fields = match self.fields.take() {
            Some(v) => v,
            None => HashMap::new(),
        };

        fields.insert(key, value);
        self.fields = Some(fields);
    }

    /// Apply the fields for structured logging (used in the logging plugins)
    fn apply_fields(&self, fields: &mut Map<String, Value>) -> Result<()> {
        if let Some(our_fields) = self.fields.as_ref() {
            for (key, value) in our_fields.iter() {
                fields.insert(key.clone(), value.clone());
            }
        }

        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TracingRequired {
    type Error = crate::error::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let (request_id, correlation_id) = extract_request_and_correlation_ids(req);

        // TracingRequired requires at least one header
        if let Some(correlation_id) = correlation_id {
            // Preserve distinct values when both headers are present
            let request_id = request_id.unwrap_or_else(|| correlation_id.clone());

            // Set request context (span extensions + OTel attributes) for consistency
            set_request_context(Some(request_id), Some(correlation_id.clone()));

            let mut tracing = Self::new(correlation_id);
            apply_req_tracing_fields(req, &mut tracing);

            Outcome::Success(tracing)
        } else {
            Outcome::Error((
                rocket::http::Status::ImATeapot,
                push_err_to_req(
                    req,
                    validation_err_code(
                        format!(
                            "Missing '{HEADER_KEY_X_REQUEST_ID}' or '{HEADER_KEY_X_CORRELATION_ID}' header."
                        ),
                        EC::CoreApiMissingRequiredXRequestIdHeader,
                        None,
                    ),
                ),
            ))
        }
    }
}

pub fn with_context<F>(tracing: impl Tracer + 'static, f: F) -> TaskLocalFuture<Box<dyn Tracer>, F>
where
    F: Future,
{
    TRACING.scope(Box::new(tracing), f)
}

/// Extracts both request_id and correlation_id from headers, preserving distinct values.
/// Returns (request_id, correlation_id) tuple.
/// - request_id: X-Request-Id header, falls back to X-Correlation-Id
/// - correlation_id: X-Correlation-Id header, falls back to X-Request-Id
pub(crate) fn extract_request_and_correlation_ids(req: &Request<'_>) -> (Option<String>, Option<String>) {
    let x_request_id = req.headers().get(HEADER_KEY_X_REQUEST_ID).next().map(|v| v.to_string());
    let x_correlation_id = req.headers().get(HEADER_KEY_X_CORRELATION_ID).next().map(|v| v.to_string());

    // request_id: prefer X-Request-Id, fall back to X-Correlation-Id
    let request_id = x_request_id.clone().or_else(|| x_correlation_id.clone());
    // correlation_id: prefer X-Correlation-Id, fall back to X-Request-Id
    let correlation_id = x_correlation_id.or(x_request_id);

    (request_id, correlation_id)
}

pub(crate) fn apply_req_tracing_fields(req: &Request<'_>, tracing: &mut (impl Tracer + 'static)) {
    if let Ok(v) = extract_and_verify_or_default_sdk_version(req) {
        tracing.add_field(TRACKING_LOG_KEY_LIT_SDK_VERSION.into(), Value::String(v.to_string()));
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SdkVersion {
    version: Version,
}

impl SdkVersion {
    pub fn new(version: Version) -> Self {
        Self { version }
    }

    pub fn version(&self) -> &Version {
        &self.version
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SdkVersion {
    type Error = crate::error::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match extract_and_verify_or_default_sdk_version(req) {
            Ok(v) => Outcome::Success(Self::new(v)),
            Err(e) => Outcome::Error((rocket::http::Status::ImATeapot, push_err_to_req(req, e))),
        }
    }
}

impl fmt::Display for SdkVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.version.fmt(f)
    }
}

pub(crate) fn extract_and_verify_or_default_sdk_version(req: &Request<'_>) -> Result<Version> {
    extract_sdk_version(req)
        .map(|v| match Version::parse(&v) {
            Ok(v) => {
                if v.major == 0 && v.minor == 0 && v.patch == 0 {
                    Err(push_err_to_req(
                        req,
                        validation_err_code(
                            format!(
                                "Invalid '{}' header; presented '{}'",
                                HEADER_KEY_X_LIT_SDK_VERSION, &v
                            ),
                            EC::CoreApiInvalidXSDKVersionHeader,
                            None,
                        ),
                    ))
                } else {
                    Ok(v)
                }
            }
            Err(e) => Err(conversion_err_code(e, EC::CoreApiInvalidXSDKVersionHeader, None)),
        })
        .unwrap_or_else(|| {
            // the first version to start sending this header was 1.2.61 so we can assume that if the header is missing, it's an older version
            Ok(Version::new(1, 2, 60))
        })
}

pub(crate) fn extract_sdk_version(req: &Request<'_>) -> Option<String> {
    req.headers().get(HEADER_KEY_X_LIT_SDK_VERSION).next().map(|val| val.to_string())
}

pub(crate) fn push_err_to_req(req: &Request<'_>, e: Error) -> Error {
    req.local_cache(|| e.clone());

    e
}
