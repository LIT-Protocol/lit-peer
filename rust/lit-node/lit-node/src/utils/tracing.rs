use lit_observability::opentelemetry::global;
use lit_observability::tracing::propagation::TonicMetadataMap;
use tonic::metadata::MetadataMap;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// This function injects the current tracing context into the given Tonic metadata map for
/// propagating tracing contexts across distributed systems.
pub fn inject_tracing_metadata(tonic_metadata_map: &mut MetadataMap) {
    let cx = tracing::Span::current().context();
    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&cx, &mut TonicMetadataMap(tonic_metadata_map));
    });
}
