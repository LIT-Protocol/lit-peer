use chrono::prelude::*;

pub fn format_timestamp(timestamp: u64) -> String {
    let dt = DateTime::<Utc>::from_timestamp(timestamp as i64, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn format_duration(interval: u64) -> String {
    let duration = std::time::Duration::from_secs(interval);
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = (duration.as_secs() / 3600) % 24;
    let days = (duration.as_secs() / 86400) % 365;
    format!("{}d {:0>2}h {:0>2}m {:0>2}s", days, hours, minutes, seconds)
}

pub fn format_timelock(interval: u64) -> String {
    let duration = std::time::Duration::from_secs(interval);
    let hours = (duration.as_secs() / 3600) % 24;
    let days = (duration.as_secs() / 86400) % 365;
    format!("{}d {:0>2}h ", days, hours)
}
