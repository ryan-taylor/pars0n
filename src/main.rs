use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use std::fs;
use std::path::Path;
use serde_json::Value;
use anyhow::Result;

const OPTIONS: [&str; 10] = [
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

fn main() -> crossterm::Result<()> {
    let mut selected = 0;
    loop {
        display_menu(selected)?;
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Up => selected = (selected - 1 + OPTIONS.len()) % OPTIONS.len(),
                KeyCode::Down => selected = (selected + 1) % OPTIONS.len(),
                KeyCode::Enter => {
                    match selected {
                        0 => if let Err(e) = fast_reading() { eprintln!("Error: {}", e); },
                        1 => if let Err(e) = data_extraction() { eprintln!("Error: {}", e); },
                        2 => if let Err(e) = data_validation() { eprintln!("Error: {}", e); },
                        3 => if let Err(e) = file_compression() { eprintln!("Error: {}", e); },
                        4 => if let Err(e) = multi_file_processing() { eprintln!("Error: {}", e); },
                        5 => if let Err(e) = custom_data_types() { eprintln!("Error: {}", e); },
                        6 => if let Err(e) = error_checking() { eprintln!("Error: {}", e); },
                        7 => if let Err(e) = pretty_printing() { eprintln!("Error: {}", e); },
                        8 => if let Err(e) = multi_language_support() { eprintln!("Error: {}", e); },
                        9 => if let Err(e) = speed_optimization() { eprintln!("Error: {}", e); },
                        _ => {}
                    }
                    read()?; // Wait for a key press before returning to the menu
                }
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }
    execute!(stdout(), Show)?; // Show cursor before exiting
    Ok(())
}

fn display_menu(selected: usize) -> crossterm::Result<()> {
    execute!(stdout(), Clear(ClearType::All), Hide)?;
    println!("Parson - Purify your JSON");
    println!("Use ↑ and ↓ arrows to move, Enter to select, 'q' to quit\n");

    for (index, option) in OPTIONS.iter().enumerate() {
        if index == selected {
            println!("→ {}", option);
        } else {
            println!("  {}", option);
        }
    }
    stdout().flush()?;
    Ok(())
}

fn fast_reading() -> Result<()> {
    println!("Fast Reading");
    let input_dir = Path::new("JSON go here");
    let output_dir = Path::new("JSON fresh here");

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let content = fs::read_to_string(&path)?;
            let parsed: Value = serde_json::from_str(&content)?;
            
            let output_path = output_dir.join(path.file_name().unwrap());
            let output = serde_json::to_string(&parsed)?;
            fs::write(output_path, output)?;
        }
    }

    println!("All JSON files processed quickly.");
    println!("Press any key to return to the main menu...");
    Ok(())
}

fn data_extraction() -> Result<()> {
    println!("Data Extraction");
    println!("Enter JSON key or path (e.g., 'user.name' or '/data/0/id'):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let query = input.trim();

    let input_dir = Path::new("JSON go here");
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let content = fs::read_to_string(&path)?;
            let parsed: Value = serde_json::from_str(&content)?;
            if let Some(result) = parsed.pointer(query) {
                println!("{}: {}", path.display(), result);
            }
        }
    }

    println!("Press any key to return to the main menu...");
    Ok(())
}

fn data_validation() -> Result<()> {
    println!("Data Validation");
    let input_dir = Path::new("JSON go here");
    let mut valid_count = 0;
    let mut invalid_count = 0;

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let content = fs::read_to_string(&path)?;
            match serde_json::from_str::<Value>(&content) {
                Ok(_) => valid_count += 1,
                Err(_) => invalid_count += 1,
            }
        }
    }

    println!("{} valid, {} invalid files", valid_count, invalid_count);
    println!("Press any key to return to the main menu...");
    Ok(())
}

fn file_compression() -> Result<()> {
    println!("File Compression");
    println!("Compression not implemented yet.");
    println!("Press any key to return to the main menu...");
    Ok(())
}

fn multi_file_processing() -> Result<()> {
    println!("Multi-File Processing");
    println!("Enter file pattern (e.g., '*.json' or 'user_*.json'):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let pattern = input.trim();

    let input_dir = Path::new("JSON go here");
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap().to_str().unwrap().contains(pattern) {
            println!("Processing: {}", path.display());
        }
    }

    println!("Press any key to return to the main menu...");
    Ok(())
}

fn custom_data_types() -> Result<()> {
    println!("Custom Data Types");
    println!("Custom data types not implemented yet.");
    println!("Press any key to return to the main menu...");
    Ok(())
}

fn error_checking() -> Result<()> {
    println!("Error Checking");
    println!("Error checking not implemented yet.");
    println!("Press any key to return to the main menu...");
    Ok(())
}

fn pretty_printing() -> Result<()> {
    println!("Pretty Printing");
    println!("Enter indentation spaces (2-8):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let spaces: usize = input.trim().parse()?;

    let input_dir = Path::new("JSON go here");
    let output_dir = Path::new("JSON fresh here");

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let content = fs::read_to_string(&path)?;
            let parsed: Value = serde_json::from_str(&content)?;
            
            let output_path = output_dir.join(path.file_name().unwrap());
            let output = serde_json::to_string_pretty(&parsed)?;
            let indented = output.lines().map(|line| " ".repeat(spaces) + line).collect::<Vec<_>>().join("\n");
            fs::write(output_path, indented)?;
        }
    }

    println!("All JSON files pretty printed.");
    println!("Press any key to return to the main menu...");
    Ok(())
}

fn multi_language_support() -> Result<()> {
    println!("Multi-Language Support");
    println!("Multi-language support not implemented yet.");
    println!("Press any key to return to the main menu...");
    Ok(())
}

fn speed_optimization() -> Result<()> {
    println!("Speed Optimization");
    println!("Speed optimization not implemented yet.");
    println!("Press any key to return to the main menu...");
    Ok(())
}
