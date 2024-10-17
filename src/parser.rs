use serde_json::Value;
use anyhow::{Result, Context};

pub fn parse_json(json_content: &str) -> Result<Value> {
    serde_json::from_str(json_content)
        .with_context(|| "Failed to parse JSON")
}
