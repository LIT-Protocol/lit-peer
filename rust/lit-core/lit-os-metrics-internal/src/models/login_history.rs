use super::{GaugeMetric, OsMetric};
use lit_observability::opentelemetry::KeyValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The structure of a login history
/// This structure is used to convert the results of the query to a structured format
/// The fields in this struct must match the fields in the query
/// If the query is ever changed, this struct will need to be updated
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginHistory {
    /// The process id
    pub pid: Option<usize>,
    /// The query time
    pub query_time: Option<usize>,
    /// The source of the login
    pub src: String,
    /// The time of the login
    pub time: Option<isize>,
    /// The terminal
    pub tty: String,
    /// The user
    pub user: String,
    /// The utmp type
    pub utmp_type: Option<usize>,
    /// The utmp type name
    pub utmp_type_name: String,
}

impl TryFrom<&BTreeMap<String, String>> for LoginHistory {
    type Error = String;

    fn try_from(value: &BTreeMap<String, String>) -> Result<Self, Self::Error> {
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
            pid: parse_optional_usize("pid")?,
            query_time: parse_optional_usize("query_time")?,
            tty: value.get("tty").ok_or("missing tty")?.clone(),
            user: value.get("user").ok_or("missing user")?.clone(),
            utmp_type: parse_optional_usize("utmp_type")?,
            utmp_type_name: value.get("utmp_type_name").ok_or("missing utmp_type_name")?.clone(),
            src: value.get("src").ok_or("missing src")?.clone(),
            time: parse_optional_isize("time")?,
        })
    }
}

impl From<&LoginHistory> for BTreeMap<String, String> {
    fn from(value: &LoginHistory) -> Self {
        let mut map = BTreeMap::new();
        map.insert("pid".to_string(), value.pid.map(|v| v.to_string()).unwrap_or_default());
        map.insert(
            "query_time".to_string(),
            value.query_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("tty".to_string(), value.tty.clone());
        map.insert("user".to_string(), value.user.clone());
        map.insert(
            "utmp_type".to_string(),
            value.utmp_type.map(|v| v.to_string()).unwrap_or_default(),
        );
        map.insert("utmp_type_name".to_string(), value.utmp_type_name.clone());
        map.insert("src".to_string(), value.src.clone());
        map.insert("time".to_string(), value.time.map(|v| v.to_string()).unwrap_or_default());
        map
    }
}

impl OsMetric for LoginHistory {
    const NAME: &'static str = "os.login_history";
}

impl GaugeMetric for LoginHistory {
    fn gauge_value(&self) -> Option<f64> {
        Some(1.0)
    }

    fn gauge_labels(&self) -> Vec<KeyValue> {
        vec![
            KeyValue::new("user", self.user.clone()),
            KeyValue::new("tty", self.tty.clone()),
            KeyValue::new("src", self.src.clone()),
            KeyValue::new("utmp_type_name", self.utmp_type_name.clone()),
        ]
    }
}
