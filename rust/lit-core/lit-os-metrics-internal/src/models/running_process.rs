use super::OsMetric;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The structure of a running process
/// This structure is used to convert the results of the query to a structured format
/// The fields in this struct must match the fields in the query
/// If the query is ever changed, this struct will need to be updated
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunningProcess {
    /// The command line of the process
    pub cmdline: String,
    /// The current working directory of the process
    pub cwd: String,
    /// The effective group name of the process
    pub effective_groupname: String,
    /// The effective username of the process
    pub effective_username: String,
    /// The environment variables of the process
    pub environment: Vec<RunningProcessEnvironmentVariable>,
    /// The group name of the process
    pub group: String,
    /// The group ID of the process
    pub group_id: Option<usize>,
    /// The MD5 hash of the process
    pub md5: String,
    /// The memory used by the process
    pub mem_used: Option<isize>,
    /// The number of files on disk
    pub on_disk: Option<isize>,
    /// The open files of the process
    pub open_files: BTreeMap<String, serde_json::Value>,
    /// The parent process ID
    pub parent: Option<usize>,
    /// The parent process name
    pub parent_name: String,
    /// The path of the process
    pub path: String,
    /// The process name
    pub process: String,
    /// The process group ID
    pub process_group: Option<usize>,
    /// The process ID
    pub process_id: Option<usize>,
    /// The process start time
    pub process_start_time: Option<usize>,
    /// The query time
    pub query_time: Option<usize>,
    /// The SHA1 hash of the process
    pub sha1: String,
    /// The SHA256 hash of the process
    pub sha256: String,
    /// The system time of the process
    pub system_time: Option<isize>,
    /// The user of the process
    pub user: String,
    /// The user ID of the process
    pub user_id: Option<usize>,
    /// The user time of the process
    pub user_time: Option<isize>,
}

/// The structure of a running process environment variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningProcessEnvironmentVariable {
    pub variable_name: String,
    pub value: String,
}

impl TryFrom<&BTreeMap<String, String>> for RunningProcess {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
        // Helper closure for parsing Option<T> from a potentially missing or empty string
        let parse_optional = |key: &str| -> Result<Option<usize>, String> {
            match value.get(key) {
                Some(s) if !s.is_empty() => {
                    s.parse().map(Some).map_err(|e: std::num::ParseIntError| {
                        format!("failed to parse {}: {}", key, e)
                    })
                }
                _ => Ok(None), // Key missing or string is empty
            }
        };

        let parse_optional_isize = |key: &str| -> Result<Option<isize>, String> {
            match value.get(key) {
                Some(s) if !s.is_empty() => {
                    s.parse().map(Some).map_err(|e: std::num::ParseIntError| {
                        format!("failed to parse {}: {}", key, e)
                    })
                }
                _ => Ok(None), // Key missing or string is empty
            }
        };

        Ok(Self {
            cmdline: value.get("cmdline").ok_or("missing cmdline")?.clone(),
            cwd: value.get("cwd").ok_or("missing cwd")?.clone(),
            effective_groupname: value
                .get("effective_groupname")
                .ok_or("missing effective_groupname")?
                .clone(),
            effective_username: value
                .get("effective_username")
                .ok_or("missing effective_username")?
                .clone(),
            environment: serde_json::from_str(
                value.get("environment").ok_or("missing environment")?,
            )
            .unwrap_or_default(),
            group: value.get("group").ok_or("missing group")?.clone(),
            group_id: parse_optional("group_id")?,
            md5: value.get("md5").ok_or("missing md5")?.clone(),
            mem_used: parse_optional_isize("mem_used")?,
            on_disk: parse_optional_isize("on_disk")?,
            open_files: serde_json::from_str(value.get("open_files").ok_or("missing open_files")?)
                .unwrap_or_default(),
            parent: parse_optional("parent")?,
            parent_name: value.get("parent_name").ok_or("missing parent_name")?.clone(),
            path: value.get("path").ok_or("missing path")?.clone(),
            process: value.get("process").ok_or("missing process")?.clone(),
            process_group: parse_optional("process_group")?,
            process_id: parse_optional("process_id")?,
            process_start_time: parse_optional("process_start_time")?,
            query_time: parse_optional("query_time")?,
            sha1: value.get("sha1").ok_or("missing sha1")?.clone(),
            sha256: value.get("sha256").ok_or("missing sha256")?.clone(),
            system_time: parse_optional_isize("system_time")?,
            user: value.get("user").ok_or("missing user")?.clone(),
            user_id: parse_optional("user_id")?,
            user_time: parse_optional_isize("user_time")?,
        })
    }
}

impl From<&RunningProcess> for BTreeMap<String, String> {
    fn from(value: &RunningProcess) -> Self {
        let mut map = BTreeMap::new();
        map.insert("cmdline".to_string(), value.cmdline.clone());
        map.insert("cwd".to_string(), value.cwd.clone());
        map.insert("effective_groupname".to_string(), value.effective_groupname.clone());
        map.insert("effective_username".to_string(), value.effective_username.clone());
        map.insert("environment".to_string(), serde_json::to_string(&value.environment).unwrap());
        map.insert("group".to_string(), value.group.clone());
        map.insert(
            "group_id".to_string(),
            value.group_id.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("md5".to_string(), value.md5.clone());
        map.insert(
            "mem_used".to_string(),
            value.mem_used.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("on_disk".to_string(), value.on_disk.map(|v| v.to_string()).unwrap_or_default());
        map.insert("open_files".to_string(), serde_json::to_string(&value.open_files).unwrap());
        map.insert("parent".to_string(), value.parent.map(|v| v.to_string()).unwrap_or_default());
        map.insert("parent_name".to_string(), value.parent_name.clone());
        map.insert("path".to_string(), value.path.clone());
        map.insert("process".to_string(), value.process.clone());
        map.insert(
            "process_group".to_string(),
            value.process_group.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert(
            "process_id".to_string(),
            value.process_id.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert(
            "process_start_time".to_string(),
            value.process_start_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert(
            "query_time".to_string(),
            value.query_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("sha1".to_string(), value.sha1.clone());
        map.insert("sha256".to_string(), value.sha256.clone());
        map.insert(
            "system_time".to_string(),
            value.system_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("user".to_string(), value.user.clone());
        map.insert("user_id".to_string(), value.user_id.map(|v| v.to_string()).unwrap_or_default());
        map.insert(
            "user_time".to_string(),
            value.user_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map
    }
}

impl OsMetric for RunningProcess {
    const NAME: &'static str = "os.running_process";
}
