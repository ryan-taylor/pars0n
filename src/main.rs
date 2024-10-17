use clap::Parser;
use std::path::PathBuf;
use anyhow::{Result, Context};
use std::io::{self, Write};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
    cursor,
    style::{Color, SetForegroundColor},
};

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
    #[arg(short, long)]
    format: Option<String>,
}

fn select_output_format() -> Result<OutputFormat> {
    println!("Select output format:");
    println!("1. Raw JSON");
    println!("2. Google Cloud AI compatible");
    println!("3. Pretty JSON");

    print!("Enter your choice (1-3): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => Ok(OutputFormat::Raw),
        "2" => Ok(OutputFormat::GoogleCloudAI),
        "3" => Ok(OutputFormat::PrettyJson),
        _ => Err(anyhow::anyhow!("Invalid choice. Please select 1, 2, or 3.")),
    }
}

fn display_main_menu() -> Result<()> {
    let menu_items = vec![
        "Fast Reading",
        "Data Extraction",
        "Data Validation",
        "File Compression",
        "Multi-File Processing",
        "Custom Data Types",
        "Error Checking",
        "Pretty Printing",
        "Multi-Language Support",
        "Speed Optimization",
    ];

    let mut selected = 0;
    let mut stdout = io::stdout();

    enable_raw_mode()?;

    loop {
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        println!("Parson - Purify your JSON");
        println!("Use ↑ and ↓ arrows to move, Enter to select, 'q' to quit");
        println!();

        for (i, item) in menu_items.iter().enumerate() {
            if i == selected {
                stdout.execute(SetForegroundColor(Color::Green))?;
                println!("→ {}", item);
                stdout.execute(SetForegroundColor(Color::Reset))?;
            } else {
                println!("  {}", item);
            }
        }

        stdout.flush()?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < menu_items.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    disable_raw_mode()?;
                    return Ok(());
                }
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<()> {
    display_main_menu()?;

    let cli = Cli::parse();

    let output_format = if let Some(format) = cli.format {
        match format.as_str() {
            "raw" => OutputFormat::Raw,
            "google_cloud_ai" => OutputFormat::GoogleCloudAI,
            "pretty_json" => OutputFormat::PrettyJson,
            _ => return Err(anyhow::anyhow!("Invalid output format. Choose 'raw', 'google_cloud_ai', or 'pretty_json'")),
        }
    } else {
        select_output_format()?
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
