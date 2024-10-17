use serde_json::Value;
use anyhow::Result;

#[derive(Clone)]
pub enum OutputFormat {
    Raw,
    GoogleCloudAI,
    PrettyJson,
}

pub fn format_output(value: &Value, format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Raw => Ok(value.to_string()),
        OutputFormat::GoogleCloudAI => format_for_google_cloud_ai(value),
        OutputFormat::PrettyJson => serde_json::to_string_pretty(value).map_err(|e| e.into()),
    }
}

fn format_for_google_cloud_ai(value: &Value) -> Result<String> {
    let formatted = serde_json::json!({
        "payload": {
            "data": value,
        },
        "metadata": {
            "format": "json",
            "source": "parson",
        }
    });

    serde_json::to_string_pretty(&formatted)
        .map_err(|e| e.into())
}
