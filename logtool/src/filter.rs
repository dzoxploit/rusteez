use chrono::{Duration, Local};
use crate::parser::LogEntry;

pub fn filter_logs(
    logs: Vec<LogEntry>,
    level: &Option<String>,
    since: &Option<String>,
) -> Vec<LogEntry> {
    logs.into_iter()
        .filter(|log| {
            let mut ok = true;

            if let Some(lvl) = level {
                ok &= &log.level == lvl;
            }

            if let Some(since_str) = since {
                if since_str.ends_with("h") {
                    let hours: i64 = since_str.trim_end_matches('h').parse().unwrap_or(0);
                    let since_time = Local::now().naive_local() - Duration::hours(hours);
                    ok &= log.timestamp >= since_time;
                }
            }

            ok  // ← PENTING: return bool
        })
        .collect()
}
