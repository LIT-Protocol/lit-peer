use super::{GaugeMetric, OsMetric};
use lit_observability::opentelemetry::KeyValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The structure of an installed debian package
/// This structure is used to convert the results of the query to a structured format
/// The fields in this struct must match the fields in the query
/// If the query is ever changed, this struct will need to be updated
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DebianPackage {
    /// The architecture of the package
    pub arch: String,
    /// The name of the package
    pub name: String,
    /// The source code of the package
    pub package_source: String,
    /// The priority of the package
    pub priority: String,
    /// The time the query was run
    pub query_time: Option<usize>,
    /// The revision of the package
    pub revision: String,
    /// The section of the package
    pub section: String,
    /// The size of the package in bytes
    pub size: Option<usize>,
    pub version: String,
}

impl OsMetric for DebianPackage {
    const NAME: &'static str = "os.installed_debian_packages";
}

impl GaugeMetric for DebianPackage {
    fn gauge_value(&self) -> Option<f64> {
        Some(1.0)
    }

    fn gauge_labels(&self) -> Vec<KeyValue> {
        vec![
            KeyValue::new("arch", self.arch.clone()),
            KeyValue::new("name", self.name.clone()),
            KeyValue::new("package_source", self.package_source.clone()),
            KeyValue::new("priority", self.priority.clone()),
            KeyValue::new("revision", self.revision.clone()),
            KeyValue::new("section", self.section.clone()),
            KeyValue::new("version", self.version.clone()),
            KeyValue::new("size", self.size.map(|v| v.to_string()).unwrap_or_default()),
        ]
    }
}

impl TryFrom<&BTreeMap<String, String>> for DebianPackage {
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
            arch: value.get("arch").ok_or("missing arch")?.clone(),
            name: value.get("name").ok_or("missing name")?.clone(),
            package_source: value.get("package_source").ok_or("missing package_source")?.clone(),
            priority: value.get("priority").ok_or("missing priority")?.clone(),
            query_time: parse_optional("query_time")?,
            revision: value.get("revision").ok_or("missing revision")?.clone(),
            section: value.get("section").ok_or("missing section")?.clone(),
            size: parse_optional("size")?,
            version: value.get("version").ok_or("missing version")?.clone(),
        })
    }
}

impl From<&DebianPackage> for BTreeMap<String, String> {
    fn from(value: &DebianPackage) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert("arch".to_string(), value.arch.clone());
        map.insert("name".to_string(), value.name.clone());
        map.insert("package_source".to_string(), value.package_source.clone());
        map.insert("priority".to_string(), value.priority.clone());
        map.insert(
            "query_time".to_string(),
            value.query_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("revision".to_string(), value.revision.clone());
        map.insert("section".to_string(), value.section.clone());
        map.insert("size".to_string(), value.size.map(|v| v.to_string()).unwrap_or_default());
        map.insert("version".to_string(), value.version.clone());
        map
    }
}
