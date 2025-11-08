use super::OsMetric;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The structure of the OS information
/// This structure is used to convert the results of the query to a structured format
/// The fields in this struct must match the fields in the query
/// If the query is ever changed, this struct will need to be updated
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OsInfo {
    /// The architecture of the OS
    pub arch: String,
    /// The build id of the OS
    pub build: String,
    /// The codename of the OS
    pub codename: String,
    /// Extra information about the OS if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// The major version of the OS
    pub major: String,
    /// The minor version of the OS
    pub minor: String,
    /// The name of the OS
    pub name: String,
    /// The patch version of the OS
    pub patch: String,
    /// The platform of the OS
    pub platform: String,
    /// The platform-like the OS
    pub platform_like: String,
    /// The query time
    pub query_time: Option<usize>,
    /// The version of the OS
    pub version: String,
}

impl TryFrom<&BTreeMap<String, String>> for OsInfo {
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
            build: value.get("build").ok_or("missing build")?.clone(),
            codename: value.get("codename").ok_or("missing codename")?.clone(),
            extra: value.get("extra").cloned(),
            major: value.get("major").ok_or("missing major")?.clone(),
            minor: value.get("minor").ok_or("missing minor")?.clone(),
            name: value.get("name").ok_or("missing name")?.clone(),
            patch: value.get("patch").ok_or("missing patch")?.clone(),
            platform: value.get("platform").ok_or("missing platform")?.clone(),
            platform_like: value.get("platform_like").ok_or("missing platform_like")?.clone(),
            query_time: parse_optional("query_time")?,
            version: value.get("version").ok_or("missing version")?.clone(),
        })
    }
}

impl From<&OsInfo> for BTreeMap<String, String> {
    fn from(value: &OsInfo) -> Self {
        let mut map = BTreeMap::new();
        map.insert("arch".to_string(), value.arch.clone());
        map.insert("build".to_string(), value.build.clone());
        map.insert("codename".to_string(), value.codename.clone());
        if let Some(extra) = &value.extra {
            map.insert("extra".to_string(), extra.clone());
        }
        map.insert("major".to_string(), value.major.clone());
        map.insert("minor".to_string(), value.minor.clone());
        map.insert("name".to_string(), value.name.clone());
        map.insert("patch".to_string(), value.patch.clone());
        map.insert("platform".to_string(), value.platform.clone());
        map.insert("platform_like".to_string(), value.platform_like.clone());
        map.insert(
            "query_time".to_string(),
            value.query_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("version".to_string(), value.version.clone());
        map
    }
}

impl OsMetric for OsInfo {
    const NAME: &'static str = "os_info";
}
