use chrono::{NaiveDateTime};

// Function to format seconds into "HH:MM:SS" format
pub fn format_duration(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

// Function to calculate the duration between two timestamps (in seconds)
pub fn calculate_duration(start: &str, end: &str) -> Option<i64> {
    let start = NaiveDateTime::parse_from_str(start, "%Y-%m-%d %H:%M:%S").ok()?;
    let end = NaiveDateTime::parse_from_str(end, "%Y-%m-%d %H:%M:%S").ok()?;
    Some((end - start).num_seconds())
}
