pub fn write_csv(
    path: &str,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
) -> anyhow::Result<()>{
    let mut wtr = csv::Writer::from_path(path)?;

    wtr.write_record(&headers)?;

    for row in rows {
        wtr.write_record(&row)?;
    }

    wtr.flush()?;
    Ok(())
} 