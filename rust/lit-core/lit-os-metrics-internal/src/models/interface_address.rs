use super::{GaugeMetric, OsMetric};
use lit_observability::opentelemetry::KeyValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The structure of an interface address
/// This structure is used to convert the results of the query to a structured format
/// The fields in this struct must match the fields in the query
/// If the query is ever changed, this struct will need to be updated
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InterfaceAddress {
    /// The address of the interface
    pub address: String,
    /// The name of the interface
    pub interface: String,
    /// The mac address of the interface
    pub mac: String,
    /// The query time
    pub query_time: Option<usize>,
}

impl OsMetric for InterfaceAddress {
    const NAME: &'static str = "os.interface_addresses";
}

impl GaugeMetric for InterfaceAddress {
    fn gauge_value(&self) -> Option<f64> {
        Some(1.0)
    }

    fn gauge_labels(&self) -> Vec<KeyValue> {
        vec![
            KeyValue::new("address", self.address.clone()),
            KeyValue::new("interface", self.interface.clone()),
            KeyValue::new("mac", self.mac.clone()),
        ]
    }
}

impl TryFrom<&BTreeMap<String, String>> for InterfaceAddress {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let parse_optional = |key: &str| -> Result<Option<usize>, String> {
            match value.get(key) {
                Some(s) if !s.is_empty() => {
                    s.parse().map(Some).map_err(|e: std::num::ParseIntError| {
                        format!("failed to parse {}: {}", key, e)
                    })
                }
                _ => Ok(None),
            }
        };

        Ok(Self {
            address: value.get("address").ok_or("missing address")?.clone(),
            interface: value.get("interface").ok_or("missing interface")?.clone(),
            mac: value.get("mac").ok_or("missing mac")?.clone(),
            query_time: parse_optional("query_time")?,
        })
    }
}

impl From<&InterfaceAddress> for BTreeMap<String, String> {
    fn from(value: &InterfaceAddress) -> Self {
        let mut map = BTreeMap::new();
        map.insert("address".to_string(), value.address.clone());
        map.insert("interface".to_string(), value.interface.clone());
        map.insert("mac".to_string(), value.mac.clone());
        map.insert(
            "query_time".to_string(),
            value.query_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map
    }
}
