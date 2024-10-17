use serde_json::Value;
use anyhow::{Result, anyhow};

pub fn query_json(json: &Value, key: &str) -> Result<Value> {
    match json {
        Value::Object(obj) => {
            obj.get(key)
                .cloned()
                .ok_or_else(|| anyhow!("Key '{}' not found in JSON object", key))
        }
        _ => Err(anyhow!("Root JSON value is not an object")),
    }
}
