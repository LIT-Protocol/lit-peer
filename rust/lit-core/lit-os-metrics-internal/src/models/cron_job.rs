use super::OsMetric;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The structure of a cron job
/// This structure is used to convert the results of the query to a structured format
/// The fields in this struct must match the fields in the query
/// If the query is ever changed, this struct will need to be updated
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CronJob {
    /// The cron job command to run
    pub command: String,
    /// The cron file the job is in
    pub cron_file: String,
    /// The day of the month the job runs
    pub day_of_month: String,
    /// The day of the week the job runs
    pub day_of_week: String,
    /// The event the job runs
    pub event: String,
    /// The hour the job runs
    pub hour: String,
    /// The minute the job runs
    pub minute: String,
    /// The month the job runs
    pub month: String,
    /// The query time
    pub query_time: Option<usize>,
}

impl TryFrom<&BTreeMap<String, String>> for CronJob {
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
            command: value.get("command").ok_or("missing command")?.clone(),
            cron_file: value.get("cron_file").ok_or("missing cron_file")?.clone(),
            day_of_month: value.get("day_of_month").ok_or("missing day_of_month")?.clone(),
            day_of_week: value.get("day_of_week").ok_or("missing day_of_week")?.clone(),
            event: value.get("event").ok_or("missing event")?.clone(),
            hour: value.get("hour").ok_or("missing hour")?.clone(),
            minute: value.get("minute").ok_or("missing minute")?.clone(),
            month: value.get("month").ok_or("missing month")?.clone(),
            query_time: parse_optional("query_time")?,
        })
    }
}

impl From<&CronJob> for BTreeMap<String, String> {
    fn from(value: &CronJob) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert("command".to_string(), value.command.clone());
        map.insert("cron_file".to_string(), value.cron_file.clone());
        map.insert("day_of_month".to_string(), value.day_of_month.clone());
        map.insert("day_of_week".to_string(), value.day_of_week.clone());
        map.insert("event".to_string(), value.event.clone());
        map.insert("hour".to_string(), value.hour.clone());
        map.insert("minute".to_string(), value.minute.clone());
        map.insert("month".to_string(), value.month.clone());
        map.insert(
            "query_time".to_string(),
            value.query_time.map(|v| v.to_string()).unwrap_or_default(),
        );
        map
    }
}

impl OsMetric for CronJob {
    const NAME: &'static str = "os.cron_jobs";
}
