use crate::models::{GaugeMetric, OsMetric};
use lit_observability::opentelemetry::KeyValue;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct Uptime {
    days: String,
    hours: String,
    minutes: String,
    seconds: String,
    total_seconds: String,
}

impl OsMetric for Uptime {
    const NAME: &'static str = "uptime";
}

impl GaugeMetric for Uptime {
    fn gauge_value(&self) -> Option<f64> {
        self.total_seconds.parse::<f64>().ok()
    }

    fn gauge_labels(&self) -> Vec<KeyValue> {
        vec![] // Uptime is global, no labels needed
    }
}

impl TryFrom<&BTreeMap<String, String>> for Uptime {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Self {
            days: value.get("days").cloned().ok_or_else(|| "Missing days".to_string())?,
            hours: value.get("hours").cloned().ok_or_else(|| "Missing hours".to_string())?,
            minutes: value.get("minutes").cloned().ok_or_else(|| "Missing minutes".to_string())?,
            seconds: value.get("seconds").cloned().ok_or_else(|| "Missing seconds".to_string())?,
            total_seconds: value
                .get("total_seconds")
                .cloned()
                .ok_or_else(|| "Missing total_seconds".to_string())?,
        })
    }
}
