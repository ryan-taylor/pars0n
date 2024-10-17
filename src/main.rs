use clap::Parser;
use std::path::PathBuf;
use anyhow::{Result, Context};

mod parser;
mod query;
mod formatter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the JSON file
    #[arg(short, long)]
    file: PathBuf,

    /// JSON key to query
    #[arg(short, long)]
    key: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let json_content = std::fs::read_to_string(&cli.file)
        .with_context(|| format!("Failed to read file: {:?}", cli.file))?;

    let parsed_json = parser::parse_json(&json_content)
        .with_context(|| "Failed to parse JSON")?;

    let query_result = query::query_json(&parsed_json, &cli.key)
        .with_context(|| format!("Failed to query JSON with key: {}", cli.key))?;

    let formatted_output = formatter::format_for_google_cloud_ai(&query_result)
        .with_context(|| "Failed to format output for Google Cloud AI")?;

    println!("{}", formatted_output);

    Ok(())
}
