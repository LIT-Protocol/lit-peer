use super::OsMetric;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The structure of an established outbound connection
/// This structure is used to convert the results of the query to a structured format
/// The fields in this struct must match the fields in the query
/// If the query is ever changed, this struct will need to be updated
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EstablishedOutbound {
    /// The command line used to establish the connection
    pub cmdline: String,
    /// The destination connection IP
    pub dest_connection_ip: String,
    /// The destination connection port
    pub dest_connection_port: Option<u16>,
    /// The directory of the file
    pub directory: String,
    /// The family of the connection
    pub family: String,
    /// The file path
    pub file_path: String,
    /// The size of the file in bytes
    pub file_size: Option<isize>,
    /// The md5 hash of the file
    pub md5: String,
    /// The name of the file
    pub name: String,
    /// The parent process id
    pub parent_pid: Option<usize>,
    /// The parent process name
    pub parent_process: String,
    /// The process id
    pub pid: Option<usize>,
    /// The query time
    pub query_time: Option<usize>,
    /// The sha1 hash of the file
    pub sha1: String,
    /// The sha256 hash of the file
    pub sha256: String,
    /// The source connection IP
    pub src_connection_ip: String,
    /// The source connection port
    pub src_connection_port: Option<u16>,
    /// The transport protocol
    pub transport: String,
    /// The user id the connection is running in
    pub uid: Option<usize>,
    /// The username the connection is running in
    pub username: String,
}

impl TryFrom<&BTreeMap<String, String>> for EstablishedOutbound {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let parse_optional_u16 = |key: &str| -> Result<Option<u16>, String> {
            match value.get(key) {
                Some(s) if !s.is_empty() => {
                    s.parse().map(Some).map_err(|e: std::num::ParseIntError| {
                        format!("failed to parse {}: {}", key, e)
                    })
                }
                _ => Ok(None),
            }
        };

        let parse_optional_usize = |key: &str| -> Result<Option<usize>, String> {
            match value.get(key) {
                Some(s) if !s.is_empty() => {
                    s.parse().map(Some).map_err(|e: std::num::ParseIntError| {
                        format!("failed to parse {}: {}", key, e)
                    })
                }
                _ => Ok(None),
            }
        };

        let parse_optional_isize = |key: &str| -> Result<Option<isize>, String> {
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
            cmdline: value.get("cmdline").ok_or("missing cmdline")?.clone(),
            dest_connection_ip: value
                .get("dest_connection_ip")
                .ok_or("missing dest_connection_ip")?
                .clone(),
            dest_connection_port: parse_optional_u16("dest_connection_port")?,
            directory: value.get("directory").ok_or("missing directory")?.clone(),
            family: value.get("family").ok_or("missing family")?.clone(),
            file_path: value.get("file_path").ok_or("missing file_path")?.clone(),
            file_size: parse_optional_isize("file_size")?,
            md5: value.get("md5").ok_or("missing md5")?.clone(),
            name: value.get("name").ok_or("missing name")?.clone(),
            parent_pid: parse_optional_usize("parent_pid")?,
            parent_process: value.get("parent_process").ok_or("missing parent_process")?.clone(),
            pid: parse_optional_usize("pid")?,
            query_time: parse_optional_usize("query_time")?,
            sha1: value.get("sha1").ok_or("missing sha1")?.clone(),
            sha256: value.get("sha256").ok_or("missing sha256")?.clone(),
            src_connection_ip: value
                .get("src_connection_ip")
                .ok_or("missing src_connection_ip")?
                .clone(),
            src_connection_port: parse_optional_u16("src_connection_port")?,
            transport: value.get("transport").ok_or("missing transport")?.clone(),
            uid: parse_optional_usize("uid")?,
            username: value.get("username").ok_or("missing username")?.clone(),
        })
    }
}

impl From<&EstablishedOutbound> for BTreeMap<String, String> {
    fn from(value: &EstablishedOutbound) -> Self {
        let mut map = BTreeMap::new();
        map.insert("cmdline".to_string(), value.cmdline.clone());
        map.insert("dest_connection_ip".to_string(), value.dest_connection_ip.clone());
        map.insert(
            "dest_connection_port".to_string(),
            value.dest_connection_port.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("directory".to_string(), value.directory.clone());
        map.insert("family".to_string(), value.family.clone());
        map.insert("file_path".to_string(), value.file_path.clone());
        map.insert(
            "file_size".to_string(),
            value.file_size.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("md5".to_string(), value.md5.clone());
        map.insert("name".to_string(), value.name.clone());
        map.insert(
            "parent_pid".to_string(),
            value.parent_pid.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("parent_process".to_string(), value.parent_process.clone());
        map.insert("pid".to_string(), value.pid.map(|v| v.to_string()).unwrap_or_default());
        map.insert(
            "query_time".to_string(),
            value.query_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("sha1".to_string(), value.sha1.clone());
        map.insert("sha256".to_string(), value.sha256.clone());
        map.insert("src_connection_ip".to_string(), value.src_connection_ip.clone());
        map.insert(
            "src_connection_port".to_string(),
            value.src_connection_port.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("transport".to_string(), value.transport.clone());
        map.insert("uid".to_string(), value.uid.map(|v| v.to_string()).unwrap_or_default());
        map.insert("username".to_string(), value.username.clone());
        map
    }
}

impl OsMetric for EstablishedOutbound {
    const NAME: &'static str = "os.established_outbound";
}
