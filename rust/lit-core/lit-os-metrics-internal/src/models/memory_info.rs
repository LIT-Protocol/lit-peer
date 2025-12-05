use crate::models::{GaugeMetric, OsMetric};
use lit_observability::opentelemetry::KeyValue;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct MemoryInfo {
    query_time: String,
    memory_total: String,
    memory_free: String,
    buffers: String,
    cached: String,
    swap_total: String,
    swap_free: String,
}

impl OsMetric for MemoryInfo {
    const NAME: &'static str = "memory_info";
}

impl GaugeMetric for MemoryInfo {
    fn gauge_value(&self) -> Option<f64> {
        // memory_free is in bytes, convert to meaningful value
        self.memory_free.parse::<f64>().ok()
    }

    fn gauge_labels(&self) -> Vec<KeyValue> {
        vec![
            KeyValue::new("memory_total", self.memory_total.clone()),
            KeyValue::new("buffers", self.buffers.clone()),
            KeyValue::new("cached", self.cached.clone()),
            KeyValue::new("swap_total", self.swap_total.clone()),
            KeyValue::new("swap_free", self.swap_free.clone()),
        ]
    }
}

impl TryFrom<&BTreeMap<String, String>> for MemoryInfo {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Self {
            query_time: value
                .get("query_time")
                .cloned()
                .ok_or_else(|| "Missing query_time".to_string())?,
            memory_total: value
                .get("memory_total")
                .cloned()
                .ok_or_else(|| "Missing memory_total".to_string())?,
            memory_free: value
                .get("memory_free")
                .cloned()
                .ok_or_else(|| "Missing memory_free".to_string())?,
            buffers: value.get("buffers").cloned().ok_or_else(|| "Missing buffers".to_string())?,
            cached: value.get("cached").cloned().ok_or_else(|| "Missing cached".to_string())?,
            swap_total: value
                .get("swap_total")
                .cloned()
                .ok_or_else(|| "Missing swap_total".to_string())?,
            swap_free: value
                .get("swap_free")
                .cloned()
                .ok_or_else(|| "Missing swap_free".to_string())?,
        })
    }
}
