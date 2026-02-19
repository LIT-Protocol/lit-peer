use crate::models::{GaugeMetric, OsMetric};
use lit_observability::opentelemetry::KeyValue;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct LoadAverage {
    query_time: String,
    period: String,
    average: String,
}

impl OsMetric for LoadAverage {
    const NAME: &'static str = "load_average";
}

impl GaugeMetric for LoadAverage {
    fn gauge_value(&self) -> Option<f64> {
        self.average.parse::<f64>().ok()
    }

    fn gauge_labels(&self) -> Vec<KeyValue> {
        vec![KeyValue::new("period", self.period.clone())]
    }
}

impl TryFrom<&BTreeMap<String, String>> for LoadAverage {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Self {
            query_time: value
                .get("query_time")
                .cloned()
                .ok_or_else(|| "Missing query_time".to_string())?,
            period: value.get("period").cloned().ok_or_else(|| "Missing period".to_string())?,
            average: value.get("average").cloned().ok_or_else(|| "Missing average".to_string())?,
        })
    }
}
