use clap::Parser;
use std::path::PathBuf;
use anyhow::{Result, Context};

mod parser;
mod query;
mod formatter;

use formatter::OutputFormat;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Paths to the JSON files
    #[arg(short, long, required = true, num_args = 1.., value_delimiter = ' ')]
    files: Vec<PathBuf>,

    /// JSON query string (e.g., "example.name" or "array.0.key")
    #[arg(short, long)]
    query: String,

    /// Output format (raw, google_cloud_ai, pretty_json)
    #[arg(short, long, default_value = "google_cloud_ai")]
    format: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let output_format = match cli.format.as_str() {
        "raw" => OutputFormat::Raw,
        "google_cloud_ai" => OutputFormat::GoogleCloudAI,
        "pretty_json" => OutputFormat::PrettyJson,
        _ => return Err(anyhow::anyhow!("Invalid output format. Choose 'raw', 'google_cloud_ai', or 'pretty_json'")),
    };

    for file in &cli.files {
        println!("Processing file: {:?}", file);
        
        let json_content = std::fs::read_to_string(file)
            .with_context(|| format!("Failed to read file: {:?}", file))?;

        let parsed_json = parser::parse_json(&json_content)
            .with_context(|| "Failed to parse JSON")?;

        let query_result = query::query_json(&parsed_json, &cli.query)
            .with_context(|| format!("Failed to query JSON with query: {}", cli.query))?;

        let formatted_output = formatter::format_output(&query_result, output_format.clone())
            .with_context(|| "Failed to format output")?;

        println!("{}", formatted_output);
        println!("---"); // Separator between file outputs
    }

    Ok(())
}
