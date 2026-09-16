//! OsQuery metrics library for emitting system metrics as OpenTelemetry gauges.

#![deny(unsafe_code)]
#![warn(
    missing_docs, trivial_casts, trivial_numeric_casts, unused_import_braces,
    unused_qualifications, rust_2018_idioms, clippy::unwrap_used, clippy::mod_module_files
)]

use error::Result;
use lit_observability::opentelemetry::global;
use lit_os_metrics_internal::*;
use std::{collections::BTreeMap, fmt::Debug};

mod consts;
mod error;

pub use consts::*;

/// Add query values as gauge metrics with proper numeric values.
/// This function should be used for metrics that have meaningful numeric values
/// like DiskInfo (free_percent), MemoryInfo (memory_free), LoadAverage (average).
pub fn add_gauge_metrics<T>(os_query: &OSQuery, query: String) -> Result<()>
where
    T: Debug + GaugeMetric + for<'a> TryFrom<&'a BTreeMap<String, String>, Error = String>,
{
    let values = execute_query::<T>(os_query, query)?;
    let meter = global::meter(METER_NAME);
    let gauge = meter.f64_gauge(T::NAME).init();

    for value in values {
        if let Some(gauge_val) = value.gauge_value() {
            gauge.record(gauge_val, &value.gauge_labels());
        }
    }
    Ok(())
}

/// Add query values as OpenTelemetry Non-Monotonic Sum metrics (Prometheus Info metrics)
/// with value 1 to indicate presence/existence.
///
/// This function should be used for metrics that represent metadata/attributes without
/// meaningful numeric values, like SystemInfo, KernelInfo, OsInfo.
///
/// This follows the [OpenTelemetry Prometheus compatibility specification](https://opentelemetry.io/docs/specs/otel/compatibility/prometheus_and_openmetrics/#info):
/// - Info metrics are converted to OTLP Non-Monotonic Sum (not Gauge)
/// - The value of 1 is intended to be viewed as a count, which should be summed together
///   when aggregating away labels
/// - Metric names MUST have the `_info` suffix to comply with the specification
///
/// The actual information is conveyed through metric attributes/labels, not the numeric value.
pub fn add_info_metrics<T>(os_query: &OSQuery, query: String) -> Result<()>
where
    T: Debug + InfoMetric + for<'a> TryFrom<&'a BTreeMap<String, String>, Error = String>,
{
    let values = execute_query::<T>(os_query, query)?;
    let meter = global::meter(METER_NAME);
    // Use UpDownCounter (Non-Monotonic Sum) as per OpenTelemetry spec for Info metrics
    let counter = meter.i64_up_down_counter(T::NAME).init();

    for value in values {
        // Use value 1 as per spec - it's intended to be viewed as a count that should be
        // summed together when aggregating away labels
        counter.add(1, &value.info_labels());
    }
    Ok(())
}
