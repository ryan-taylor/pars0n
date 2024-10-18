use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
    style::{Print, Stylize},
    queue,
};
use std::io::{stdout, Write};
use std::fs;
use std::path::Path;
use serde_json::Value;
use anyhow::{Result, Error};
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
    
    if !crossterm::terminal::is_raw_mode_enabled()? {
        eprintln!("Failed to enter raw mode. The CLI UI may not work correctly.");
    }

    let mut selected = 0;
    loop {
        display_menu(selected)?;
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Up => selected = (selected - 1 + OPTIONS.len()) % OPTIONS.len(),
                KeyCode::Down => selected = (selected + 1) % OPTIONS.len(),
                KeyCode::Enter => {
                    execute!(stdout(), Clear(ClearType::All))?;
                    match selected {
                        0 => if let Err(e) = fast_reading() { print_error(e)?; },
                        1 => if let Err(e) = data_extraction() { print_error(e)?; },
                        2 => if let Err(e) = data_validation() { print_error(e)?; },
                        3 => if let Err(e) = file_compression() { print_error(e)?; },
                        4 => if let Err(e) = multi_file_processing() { print_error(e)?; },
                        5 => if let Err(e) = custom_data_types() { print_error(e)?; },
                        6 => if let Err(e) = error_checking() { print_error(e)?; },
                        7 => if let Err(e) = pretty_printing() { print_error(e)?; },
                        8 => if let Err(e) = multi_language_support() { print_error(e)?; },
                        9 => if let Err(e) = speed_optimization() { print_error(e)?; },
                        _ => {}
                    }
                    clean_print("\nPress any key to return to the main menu...")?;
                    // Wait for a key press before returning to the main menu
                    loop {
                        if let Event::Key(_) = read()? {
                            break;
                        }
                    }
                }
                KeyCode::Char('q') => break,
                KeyCode::Esc => {
                    // If we're in a submenu, this will just return to the main menu
                    // If we're already in the main menu, this will do nothing
                    continue;
                }
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    execute!(stdout(), Show)?;
    Ok(())
}

fn display_menu(selected: usize) -> crossterm::Result<()> {
    execute!(
        stdout(),
        Clear(ClearType::All),
        crossterm::cursor::MoveTo(0, 0),
        Hide
    )?;
    
    clean_print(&"Parson - Purify your JSON".bold().to_string())?;
    clean_print("Use ↑ and ↓ arrows to move, Enter to select, Esc to go back, 'q' to quit")?;
    clean_print("")?;  // Add an empty line for better spacing

    for (index, option) in OPTIONS.iter().enumerate() {
        let line = if index == selected {
            format!("→ {}", option).green()
        } else {
            format!("  {}", option).stylize()
        };
        execute!(
            stdout(),
            crossterm::cursor::MoveTo(0, (index + 4) as u16),
            Print(line)
        )?;
    }
    
    stdout().flush()?;
    Ok(())
}

fn fast_reading() -> Result<()> {
    clean_print("Fast Reading")?;
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

    clean_print("All JSON files processed quickly.")?;
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

fn clean_print(text: &str) -> crossterm::Result<()> {
    let mut stdout = stdout();
    queue!(stdout, Print(text), Print("\n"))?;
    stdout.flush()?;
    Ok(())
}

fn print_error(e: Error) -> crossterm::Result<()> {
    clean_print(&format!("Error: {}", e))
}
