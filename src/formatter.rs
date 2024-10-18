// TODO: Implement usage of OutputFormat in future iterations
#[allow(dead_code)]
#[derive(Clone)]
pub enum OutputFormat {
    PrettyJson,
    // Commented out unused match arms
    // Raw,
    // GoogleCloudAI,
}

// Commented out unused function
// fn format_for_google_cloud_ai(value: &Value) -> Result<String> {
//     let formatted = serde_json::json!({
//         "payload": {
//             "data": value,
//         },
//         "metadata": {
//             "format": "json",
//             "source": "parson",
//         }
//     });

//     serde_json::to_string_pretty(&formatted)
//         .map_err(|e| e.into())
// }
