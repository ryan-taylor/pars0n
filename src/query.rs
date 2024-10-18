use serde_json::Value;
use anyhow::{Result, anyhow};

#[allow(dead_code)]
pub fn query_json(json: &str, query: &str) -> Result<String> {
    let value: Value = serde_json::from_str(json)?;
    let result = value.pointer(query).ok_or_else(|| anyhow!("Query not found"))?.clone();
    Ok(serde_json::to_string_pretty(&result)?)
}
