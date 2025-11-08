use crate::models::OsMetric;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct ListeningPort {
    query_time: String,
    pid: String,
    port: String,
    protocol: String,
    family: String,
    address: String,
    fd: String,
    socket: String,
    path: String,
    process_name: Option<String>,
}

impl OsMetric for ListeningPort {
    const NAME: &'static str = "listening_ports";
}

impl TryFrom<&BTreeMap<String, String>> for ListeningPort {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Self {
            query_time: value
                .get("query_time")
                .cloned()
                .ok_or_else(|| "Missing query_time".to_string())?,
            pid: value.get("pid").cloned().ok_or_else(|| "Missing pid".to_string())?,
            port: value.get("port").cloned().ok_or_else(|| "Missing port".to_string())?,
            protocol: value
                .get("protocol")
                .cloned()
                .ok_or_else(|| "Missing protocol".to_string())?,
            family: value.get("family").cloned().ok_or_else(|| "Missing family".to_string())?,
            address: value.get("address").cloned().ok_or_else(|| "Missing address".to_string())?,
            fd: value.get("fd").cloned().ok_or_else(|| "Missing fd".to_string())?,
            socket: value.get("socket").cloned().ok_or_else(|| "Missing socket".to_string())?,
            path: value.get("path").cloned().ok_or_else(|| "Missing path".to_string())?,
            process_name: value.get("process_name").cloned(),
        })
    }
}
