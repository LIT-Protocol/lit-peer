use crate::models::{GaugeMetric, OsMetric};
use lit_observability::opentelemetry::KeyValue;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct KernelInfo {
    query_time: String,
    version: String,
    arguments: String,
    path: String,
    device: String,
}

impl OsMetric for KernelInfo {
    const NAME: &'static str = "kernel_info";
}

impl GaugeMetric for KernelInfo {
    fn gauge_value(&self) -> Option<f64> {
        Some(1.0)
    }

    fn gauge_labels(&self) -> Vec<KeyValue> {
        vec![
            KeyValue::new("version", self.version.clone()),
            KeyValue::new("arguments", self.arguments.clone()),
            KeyValue::new("path", self.path.clone()),
            KeyValue::new("device", self.device.clone()),
        ]
    }
}

impl TryFrom<&BTreeMap<String, String>> for KernelInfo {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Self {
            query_time: value
                .get("query_time")
                .cloned()
                .ok_or_else(|| "Missing query_time".to_string())?,
            version: value.get("version").cloned().ok_or_else(|| "Missing version".to_string())?,
            arguments: value
                .get("arguments")
                .cloned()
                .ok_or_else(|| "Missing arguments".to_string())?,
            path: value.get("path").cloned().ok_or_else(|| "Missing path".to_string())?,
            device: value.get("device").cloned().ok_or_else(|| "Missing device".to_string())?,
        })
    }
}
