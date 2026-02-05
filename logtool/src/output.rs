use crate::parser::LogEntry;
use anyhow::Result;
use csv::Writer;

pub fn write_csv(path: &str, logs: &[LogEntry]) -> Result<()> {
    let mut wtr = Writer::from_path(path)?;

    wtr.write_record(&["timestamp", "level", "message"])?;

    for log in logs {
        wtr.write_record(&[
            log.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            log.level.clone(),
            log.message.clone(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
