use serde_json::Value;
use anyhow::{Result, anyhow};

pub fn query_json(json: &Value, query: &str) -> Result<Value> {
    let parts: Vec<&str> = query.split('.').collect();
    let mut current = json;

    for part in parts {
        current = match current {
            Value::Object(obj) => obj.get(part).ok_or_else(|| anyhow!("Key '{}' not found in JSON object", part))?,
            Value::Array(arr) => {
                if let Ok(index) = part.parse::<usize>() {
                    arr.get(index).ok_or_else(|| anyhow!("Index {} out of bounds for JSON array", index))?
                } else {
                    return Err(anyhow!("Invalid array index: {}", part));
                }
            },
            _ => return Err(anyhow!("Cannot query '{}' on non-object and non-array value", part)),
        };
    }

    Ok(current.clone())
}
