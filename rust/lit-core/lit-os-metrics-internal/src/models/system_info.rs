use crate::models::OsMetric;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    query_time: String,
    hostname: String,
    uuid: String,
    cpu_type: String,
    cpu_subtype: String,
    cpu_brand: String,
    cpu_physical_cores: String,
    cpu_logical_cores: String,
    physical_memory: String,
    hardware_vendor: String,
    hardware_model: String,
    hardware_version: String,
    hardware_serial: String,
    board_vendor: String,
    board_model: String,
    board_version: String,
    board_serial: String,
    computer_name: String,
    local_hostname: String,
}

impl OsMetric for SystemInfo {
    const NAME: &'static str = "system_info";
}

impl TryFrom<&BTreeMap<String, String>> for SystemInfo {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        Ok(Self {
            query_time: value
                .get("query_time")
                .cloned()
                .ok_or_else(|| "Missing query_time".to_string())?,
            hostname: value
                .get("hostname")
                .cloned()
                .ok_or_else(|| "Missing hostname".to_string())?,
            uuid: value.get("uuid").cloned().ok_or_else(|| "Missing uuid".to_string())?,
            cpu_type: value
                .get("cpu_type")
                .cloned()
                .ok_or_else(|| "Missing cpu_type".to_string())?,
            cpu_subtype: value
                .get("cpu_subtype")
                .cloned()
                .ok_or_else(|| "Missing cpu_subtype".to_string())?,
            cpu_brand: value
                .get("cpu_brand")
                .cloned()
                .ok_or_else(|| "Missing cpu_brand".to_string())?,
            cpu_physical_cores: value
                .get("cpu_physical_cores")
                .cloned()
                .ok_or_else(|| "Missing cpu_physical_cores".to_string())?,
            cpu_logical_cores: value
                .get("cpu_logical_cores")
                .cloned()
                .ok_or_else(|| "Missing cpu_logical_cores".to_string())?,
            physical_memory: value
                .get("physical_memory")
                .cloned()
                .ok_or_else(|| "Missing physical_memory".to_string())?,
            hardware_vendor: value
                .get("hardware_vendor")
                .cloned()
                .ok_or_else(|| "Missing hardware_vendor".to_string())?,
            hardware_model: value
                .get("hardware_model")
                .cloned()
                .ok_or_else(|| "Missing hardware_model".to_string())?,
            hardware_version: value
                .get("hardware_version")
                .cloned()
                .ok_or_else(|| "Missing hardware_version".to_string())?,
            hardware_serial: value
                .get("hardware_serial")
                .cloned()
                .ok_or_else(|| "Missing hardware_serial".to_string())?,
            board_vendor: value
                .get("board_vendor")
                .cloned()
                .ok_or_else(|| "Missing board_vendor".to_string())?,
            board_model: value
                .get("board_model")
                .cloned()
                .ok_or_else(|| "Missing board_model".to_string())?,
            board_version: value
                .get("board_version")
                .cloned()
                .ok_or_else(|| "Missing board_version".to_string())?,
            board_serial: value
                .get("board_serial")
                .cloned()
                .ok_or_else(|| "Missing board_serial".to_string())?,
            computer_name: value
                .get("computer_name")
                .cloned()
                .ok_or_else(|| "Missing computer_name".to_string())?,
            local_hostname: value
                .get("local_hostname")
                .cloned()
                .ok_or_else(|| "Missing local_hostname".to_string())?,
        })
    }
}
