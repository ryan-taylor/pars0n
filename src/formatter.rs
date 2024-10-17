use serde_json::Value;
use anyhow::Result;

pub fn format_for_google_cloud_ai(value: &Value) -> Result<String> {
    let formatted = serde_json::json!({
        "payload": {
            "data": value,
        },
        "metadata": {
            "format": "json",
            "source": "json_parser_cli",
        }
    });

    serde_json::to_string_pretty(&formatted)
        .map_err(|e| e.into())
}
