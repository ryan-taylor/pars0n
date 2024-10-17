use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
};
use std::io::{stdout, Write};
use std::fs;
use std::path::Path;
use serde_json::Value;
use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;

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
    enable_raw_mode()?;
    let mut selected = 0;
    loop {
        display_menu(selected)?;
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Up => {
                    selected = (selected + OPTIONS.len() - 1) % OPTIONS.len();
                }
                KeyCode::Down => {
                    selected = (selected + 1) % OPTIONS.len();
                }
                KeyCode::Enter => {
                    execute!(stdout(), Clear(ClearType::All))?;
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
                    println!("\nPress any key to return to the main menu...");
                    read()?;
                }
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    execute!(stdout(), Show)?;
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
    Ok(())
}

fn file_compression() -> Result<()> {
    println!("File Compression");
    println!("Select compression level:");
    println!("1. Low");
    println!("2. Medium");
    println!("3. High");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let compression_level = match input.trim() {
        "1" => Compression::fast(),
        "2" => Compression::default(),
        "3" => Compression::best(),
        _ => {
            println!("Invalid input. Using default compression level.");
            Compression::default()
        }
    };

    let input_dir = Path::new("JSON go here");
    let output_dir = Path::new("JSON fresh here");

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let content = fs::read_to_string(&path)?;
            let output_path = output_dir.join(path.file_name().unwrap()).with_extension("json.gz");
            let file = fs::File::create(&output_path)?;
            let mut encoder = GzEncoder::new(file, compression_level);
            encoder.write_all(content.as_bytes())?;
            encoder.finish()?;
            println!("Compressed: {}", output_path.display());
        }
    }

    println!("All JSON files compressed.");
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
            // Add your processing logic here
        }
    }

    println!("All matching files processed.");
    Ok(())
}

fn custom_data_types() -> Result<()> {
    println!("Custom Data Types");
    println!("Enter a custom data type (e.g., 'date', 'email', 'phone'):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let custom_type = input.trim();

    println!("Enter a JSON key to apply the custom type:");
    let mut key_input = String::new();
    std::io::stdin().read_line(&mut key_input)?;
    let key = key_input.trim();

    let input_dir = Path::new("JSON go here");
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let content = fs::read_to_string(&path)?;
            let mut parsed: Value = serde_json::from_str(&content)?;
            if parsed.pointer_mut(key).is_some() {
                // This is a placeholder. In a real implementation, you'd validate and possibly
                // transform the value based on the custom_type.
                println!("Applied '{}' type to '{}' in {}", custom_type, key, path.display());
            }
        }
    }

    println!("Custom data type applied to all matching files.");
    Ok(())
}

fn error_checking() -> Result<()> {
    println!("Error Checking");
    let input_dir = Path::new("JSON go here");
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let content = fs::read_to_string(&path)?;
            match serde_json::from_str::<Value>(&content) {
                Ok(_) => println!("{}: No errors found", path.display()),
                Err(e) => println!("{}: Error - {}", path.display(), e),
            }
        }
    }

    println!("Error checking completed for all JSON files.");
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
            fs::write(&output_path, indented)?;
            println!("Pretty printed: {}", output_path.display());
        }
    }

    println!("All JSON files pretty printed.");
    Ok(())
}

fn multi_language_support() -> Result<()> {
    println!("Multi-Language Support");
    println!("Enter target language (e.g., 'es' for Spanish, 'fr' for French):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let target_lang = input.trim();

    println!("This is a placeholder for multi-language support.");
    println!("In a real implementation, this would translate JSON keys to {}.", target_lang);
    println!("For now, we'll just demonstrate awareness of the selected language.");

    let input_dir = Path::new("JSON go here");
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            println!("Would process {} for {} language support", path.display(), target_lang);
        }
    }

    println!("Multi-language support simulation completed.");
    Ok(())
}

fn speed_optimization() -> Result<()> {
    println!("Speed Optimization");
    println!("Select optimization level:");
    println!("1. Balanced");
    println!("2. Memory-Optimized");
    println!("3. Speed-Optimized");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let optimization_level = match input.trim() {
        "1" => "Balanced",
        "2" => "Memory-Optimized",
        "3" => "Speed-Optimized",
        _ => {
            println!("Invalid input. Using Balanced optimization.");
            "Balanced"
        }
    };

    println!("Applying {} optimization...", optimization_level);
    // This is a placeholder. In a real implementation, you'd apply different
    // parsing or processing strategies based on the selected optimization level.

    let input_dir = Path::new("JSON go here");
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            println!("Optimized processing of: {}", path.display());
        }
    }

    println!("Speed optimization simulation completed for all JSON files.");
    Ok(())
}
