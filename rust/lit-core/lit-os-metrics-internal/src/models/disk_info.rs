use crate::OsMetric;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The disk information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiskInfo {
    /// The device name
    pub device: String,
    /// The disk path
    pub path: String,
    /// The encryption type
    pub encryption_type: String,
    /// The encryption type
    pub encrypted: String,
    /// The encryption status
    pub encryption_status: String,
    /// The free space in GB
    pub free_gb: Option<f64>,
    /// The free space in percent
    pub free_percent: Option<f64>,
    /// The disk bytes read in GB
    pub disk_gb_read: Option<f64>,
    /// The disk bytes written in GB
    pub disk_gb_written: Option<f64>,
}

impl TryFrom<&BTreeMap<String, String>> for DiskInfo {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let parse_optional_f64 = |key: &str| -> Result<Option<f64>, String> {
            match value.get(key) {
                Some(s) if !s.is_empty() => {
                    s.parse().map(Some).map_err(|e: std::num::ParseFloatError| {
                        format!("failed to parse {}: {}", key, e)
                    })
                }
                _ => Ok(None),
            }
        };

        Ok(Self {
            device: value.get("device").ok_or("missing device")?.clone(),
            path: value.get("path").ok_or("missing path")?.clone(),
            encryption_type: value.get("encryption_type").ok_or("missing encryption_type")?.clone(),
            encrypted: value.get("encrypted").ok_or("missing encrypted")?.clone(),
            encryption_status: value
                .get("encryption_status")
                .ok_or("missing encryption_status")?
                .clone(),
            free_gb: parse_optional_f64("free_gb")?,
            free_percent: parse_optional_f64("free_percent")?,
            disk_gb_read: parse_optional_f64("disk_gb_read")?,
            disk_gb_written: parse_optional_f64("disk_gb_written")?,
        })
    }
}

impl From<&DiskInfo> for BTreeMap<String, String> {
    fn from(value: &DiskInfo) -> Self {
        let mut map = BTreeMap::new();
        map.insert("device".to_string(), value.device.clone());
        map.insert("path".to_string(), value.path.clone());
        map.insert("encryption_type".to_string(), value.encryption_type.clone());
        map.insert("encrypted".to_string(), value.encrypted.clone());
        map.insert("encryption_status".to_string(), value.encryption_status.clone());
        map.insert("free_gb".to_string(), value.free_gb.map(|v| v.to_string()).unwrap_or_default());
        map.insert(
            "free_percent".to_string(),
            value.free_percent.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert(
            "disk_gb_read".to_string(),
            value.disk_gb_read.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert(
            "disk_gb_written".to_string(),
            value.disk_gb_written.map(|v| v.to_string()).unwrap_or_default(),
        );
        map
    }
}

impl OsMetric for DiskInfo {
    const NAME: &'static str = "os.disk_info";
}
