use chrono::{NaiveDateTime};
use regex::Regex;

pub struct LogEntry {
    pub timestamp: NaiveDateTime,
    pub level: String,
    pub message: String,
}


pub fn parse_line(line: &str) -> Option<LogEntry> {
    let re = Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) (\w+) (.+)").ok()?;
    let caps = re.captures(line)?;
    
    let timestamp = NaiveDateTime::parse_from_str(&caps[1], "%Y-%m-%d %H:%M:%S").ok()?;

    Some(LogEntry {
        timestamp,
        level: caps[2].to_string(),
        message: caps[3].to_string(),
    })
}