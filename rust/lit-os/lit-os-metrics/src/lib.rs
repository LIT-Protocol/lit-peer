//! Handles OS metrics
//!

#![deny(unsafe_code)]
#![warn(
    missing_docs, trivial_casts, trivial_numeric_casts, unused_import_braces,
    unused_qualifications, rust_2018_idioms, clippy::unwrap_used, clippy::mod_module_files
)]

use error::Result;
use lit_observability::opentelemetry::{Key, KeyValue, Value, global};
use lit_os_metrics_internal::*;
use serde::Serialize;
use std::{collections::BTreeMap, fmt::Debug};
use tracing::info;

mod consts;
mod error;

pub use consts::*;

/// Add a query values to a metric
pub fn add_value_metrics<T>(os_query: &OSQuery, query: String) -> Result<()>
where
    T: Debug + MetricKeyValue + for<'a> TryFrom<&'a BTreeMap<String, String>, Error = String>,
{
    let values = execute_query::<T>(os_query, query)?;
    for (i, value) in values.iter().enumerate() {
        add_value(T::NAME, (i + 1) as u64, &[value.as_key_value()]);
    }
    Ok(())
}

/// Function to handle the complex Docker container telemetry.
/// It emits a simple gauge metric and a detailed, structured log for each container.
pub fn handle_docker_container_telemetry(os_query: &OSQuery, query: String) -> Result<()> {
    // 1. Get the detailed container info
    let containers = execute_query::<DockerRunningContainers>(os_query, query)?;

    let meter = global::meter(METER_NAME);
    let counter = meter
        .u64_counter(<DockerRunningContainers as OsMetric>::NAME)
        .with_description("A counter showing running Docker containers.")
        .init();

    for container in containers {
        // 2. For each container, emit a counter metric with a value of 1.
        counter.add(
            1,
            &[
                KeyValue::new("container.name", container.container_name.clone()),
                KeyValue::new("image.name", container.image_name.clone()),
            ],
        );

        // 3. For each container, emit a detailed log record with the full JSON.
        let log_body = serde_json::to_string(&container)
            .unwrap_or_else(|e| format!("\"unable to serialize DockerRunningContainers: {}\"", e));

        info!(
            container.name = %container.container_name,
            image.name = %container.image_name,
            container.details = %log_body,
            "Docker container running"
        );
    }
    Ok(())
}

/// Function to handle the complex running process telemetry.
/// It emits a simple gauge metric and a detailed, structured log for each process.
pub fn handle_running_process_telemetry(os_query: &OSQuery, query: String) -> Result<()> {
    // 1. Get the detailed process info
    let processes = execute_query::<RunningProcess>(os_query, query)?;

    let meter = global::meter(METER_NAME);
    let counter = meter
        .u64_counter(<RunningProcess as OsMetric>::NAME)
        .with_description("A counter showing running processes.")
        .init();

    for process in processes {
        // Convert numeric values once to avoid repeated conversions
        let pid_str = process.process_id.map(|p| p.to_string()).unwrap_or_default();
        let parent_pid_str = process.parent.map(|p| p.to_string()).unwrap_or_default();
        let start_time = process.process_start_time.unwrap_or_default();
        let mem_used = process.mem_used.unwrap_or_default();

        // 2. For each process, emit a counter metric with a value of 1.
        // Note: Removed process.path to avoid high cardinality
        counter.add(
            1,
            &[
                KeyValue::new("process.name", process.process.clone()),
                KeyValue::new("process.pid", pid_str.clone()),
                KeyValue::new("process.user", process.user.clone()),
                KeyValue::new("process.parent_pid", parent_pid_str.clone()),
                KeyValue::new("process.parent_name", process.parent_name.clone()),
                // Attach curated details as a single JSON attribute
                KeyValue::new("process.details", {
                    const MAX_FIELD_LEN: usize = 256; // Max length for long string fields.

                    // Truncate long fields *before* serialization to ensure valid JSON.
                    let path = process.path.get(..MAX_FIELD_LEN).unwrap_or(&process.path);
                    let cmdline = process.cmdline.get(..MAX_FIELD_LEN).unwrap_or(&process.cmdline);
                    let effective_username = process
                        .effective_username
                        .get(..MAX_FIELD_LEN)
                        .unwrap_or(&process.effective_username);

                    // Create a minimal details object with the most critical fields.
                    let details = serde_json::json!({
                        "path": path,
                        "cmdline": cmdline,
                        "effective_username": effective_username,
                        "start_time": start_time,
                        "mem_used": mem_used,
                    });

                    // This serialization won't exceed the overall limit.
                    serde_json::to_string(&details)
                        .unwrap_or_else(|_| r#"{"error":"serialization_failed"}"#.to_string())
                }),
            ],
        );
    }
    Ok(())
}

/// Add a single value to a metric
pub fn add_value(metric_name: &'static str, value: u64, attributes: &[KeyValue]) {
    let meter = global::meter(METER_NAME);
    let counter = meter.u64_counter(metric_name);
    let counter = counter.init();
    counter.add(value, attributes);
}

/// Trait for converting information into a key value metric
pub trait MetricKeyValue: Serialize {
    /// The name of the metric
    const NAME: &'static str;
    /// Convert the metric to a key value pair
    fn as_key_value(&self) -> KeyValue {
        KeyValue::new(
            Key::new(Self::NAME),
            Value::String(serde_json::to_string(self).expect("unable to serialize").into()),
        )
    }
}

impl MetricKeyValue for CpuInfo {
    const NAME: &'static str = <CpuInfo as OsMetric>::NAME;
}
impl MetricKeyValue for CronJob {
    const NAME: &'static str = <CronJob as OsMetric>::NAME;
}
impl MetricKeyValue for DebianPackage {
    const NAME: &'static str = <DebianPackage as OsMetric>::NAME;
}
impl MetricKeyValue for DiskInfo {
    const NAME: &'static str = <DiskInfo as OsMetric>::NAME;
}
impl MetricKeyValue for EstablishedOutbound {
    const NAME: &'static str = <EstablishedOutbound as OsMetric>::NAME;
}
impl MetricKeyValue for InterfaceAddress {
    const NAME: &'static str = <InterfaceAddress as OsMetric>::NAME;
}
impl MetricKeyValue for LoginHistory {
    const NAME: &'static str = <LoginHistory as OsMetric>::NAME;
}
impl MetricKeyValue for OsInfo {
    const NAME: &'static str = <OsInfo as OsMetric>::NAME;
}
impl MetricKeyValue for RunningProcess {
    const NAME: &'static str = <RunningProcess as OsMetric>::NAME;
}
impl MetricKeyValue for MemoryInfo {
    const NAME: &'static str = <MemoryInfo as OsMetric>::NAME;
}
impl MetricKeyValue for LoadAverage {
    const NAME: &'static str = <LoadAverage as OsMetric>::NAME;
}
impl MetricKeyValue for ListeningPort {
    const NAME: &'static str = <ListeningPort as OsMetric>::NAME;
}
impl MetricKeyValue for KernelInfo {
    const NAME: &'static str = <KernelInfo as OsMetric>::NAME;
}
impl MetricKeyValue for Uptime {
    const NAME: &'static str = <Uptime as OsMetric>::NAME;
}
impl MetricKeyValue for IptablesRule {
    const NAME: &'static str = <IptablesRule as OsMetric>::NAME;
}
impl MetricKeyValue for SystemInfo {
    const NAME: &'static str = <SystemInfo as OsMetric>::NAME;
}
