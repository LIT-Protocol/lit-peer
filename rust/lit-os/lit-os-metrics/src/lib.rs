//!
//!

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
