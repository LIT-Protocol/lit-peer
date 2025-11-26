use rocket::Request;
use rocket::fairing::AdHoc;
use tracing::info;

thread_local! {
    static PRIVACY_MODE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Extract privacy_mode from query parameters or headers
fn extract_privacy_mode(req: &Request<'_>) -> bool {
    // Check query parameters first
    if let Some(Ok(true)) = req.query_value::<bool>("privacy_mode") {
        return true;
    }

    // Check headers
    if let Some(privacy_mode_header) = req.headers().get_one("X-Privacy-Mode") {
        if privacy_mode_header.eq_ignore_ascii_case("true") || privacy_mode_header == "1" {
            return true;
        }
    }

    false
}

/// Check if privacy mode is enabled for the current request
pub fn is_privacy_mode_enabled() -> bool {
    PRIVACY_MODE.with(|cell| cell.get())
}

/// Create a fairing that sets privacy mode state for the request
pub fn privacy_mode_fairing() -> impl rocket::fairing::Fairing {
    AdHoc::on_request("Privacy Mode", |req, _| {
        Box::pin(async move {
            let privacy_enabled = extract_privacy_mode(req);

            // If privacy mode is enabled, log just the endpoint path for metrics
            // This must happen before we set privacy mode, otherwise the log will be filtered
            if privacy_enabled {
                let method = req.method().as_str();
                let path = req.uri().path().as_str();
                info!(method = method, path = path, "privacy_mode_request");
            }

            // Store in thread-local state so the tracing layer can check it
            PRIVACY_MODE.with(|cell| cell.set(privacy_enabled));
        })
    })
}

/// A tracing layer that filters out all events and spans when privacy mode is enabled
pub struct PrivacyModeLayer;

impl<S> tracing_subscriber::Layer<S> for PrivacyModeLayer
where
    S: tracing::Subscriber,
{
    fn enabled(
        &self,
        _metadata: &tracing::Metadata<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        // Disable all tracing when privacy mode is enabled
        !is_privacy_mode_enabled()
    }

    fn on_event(
        &self,
        _event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Events are filtered by enabled() above
    }

    fn on_new_span(
        &self,
        _attrs: &tracing::span::Attributes<'_>,
        _id: &tracing::span::Id,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Spans are filtered by enabled() above
    }
}
