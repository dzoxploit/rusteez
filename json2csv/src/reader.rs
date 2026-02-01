use serde_json::Value;
use std::fs;
use anyhow::Result;

pub fn read_json(path: &str) -> Result<Value>{
    let content = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&content)?;
    Ok(json)
} 
  