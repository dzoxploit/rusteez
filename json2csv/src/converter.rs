use serde_json::Value;
use std::collections::BTreeSet;

pub fn convert(json: Value) -> anyhow::Result<(Vec<String>, Vec<Vec<String>>)> {
     let arr = json.as_array().ok_or_else(|| anyhow::anyhow!("JSON must be array"))?;

    let mut headers = BTreeSet::new();
    for obj in arr {
        if let Some(map) = obj.as_object() {
            for key in map.keys() {
                headers.insert(key.clone());
            }
        }
    }

    let headers: Vec<String> = headers.into_iter().collect();

    let rows = arr.iter().map(|obj| {
        let map = obj.as_object().unwrap();
        headers.iter()
            .map(|h| map.get(h).map(|v| v.to_string()).unwrap_or_default())
            .collect()
    }).collect();

    Ok((headers, rows))
}