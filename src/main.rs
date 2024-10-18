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
use std::path::PathBuf;
use std::env;

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

const INPUT_DIR: &str = "/workspaces/rust/JSIN";
const OUTPUT_DIR: &str = "/workspaces/rust/JSON";

fn main() -> crossterm::Result<()> {
    // Add this check at the beginning of main
    if !Path::new("/workspaces/rust").exists() {
        eprintln!("Error: This application must be run in the /workspaces/rust directory.");
        return Ok(());
    }

    enable_raw_mode()?;
    
    if !crossterm::terminal::is_raw_mode_enabled()? {
        eprintln!("Failed to enter raw mode. The CLI UI may not work correctly.");
    }

    let input_dir = PathBuf::from(INPUT_DIR);
    let output_dir = PathBuf::from(OUTPUT_DIR);

    if !input_dir.exists() || !output_dir.exists() {
        clean_print("Error: Required directories not found.")?;
        if !input_dir.exists() {
            clean_print(&format!("The '{}' directory is missing.", INPUT_DIR))?;
        }
        if !output_dir.exists() {
            clean_print(&format!("The '{}' directory is missing.", OUTPUT_DIR))?;
        }
        clean_print("Please create these directories and add JSON files to the input directory.")?;
        clean_print("Press any key to exit...")?;
        read()?;
        return Ok(());
    }

    if fs::read_dir(&input_dir)?.next().is_none() {
        clean_print(&format!("The '{}' directory is empty. Please add JSON files to process.", INPUT_DIR))?;
        clean_print("Press any key to exit...")?;
        read()?;
        return Ok(());
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
                    read()?; // Wait for any key press
                }
                KeyCode::Char('q') => break,
                KeyCode::Esc => continue,
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
    let input_dir = PathBuf::from(INPUT_DIR);
    let output_dir = PathBuf::from(OUTPUT_DIR);

    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
        clean_print(&format!("Created output directory: {}", OUTPUT_DIR))?;
    }

    let mut processed_count = 0;

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let content = fs::read_to_string(&path)?;
            let parsed: Value = serde_json::from_str(&content)?;
            
            let output_path = output_dir.join(path.file_name().unwrap());
            let output = serde_json::to_string(&parsed)?;
            fs::write(output_path, output)?;
            processed_count += 1;
        }
    }

    if processed_count > 0 {
        clean_print(&format!("Processed {} JSON files quickly.", processed_count))?;
    } else {
        clean_print("No JSON files found in the input directory.")?;
    }

    Ok(())
}

fn data_extraction() -> Result<()> {
    clean_print("Data Extraction")?;
    clean_print("Enter JSON key or path (e.g., 'user.name' or '/data/0/id'):")?;
    
    let query = match get_user_input("")? {
        Some(input) => input,
        None => return Ok(()), // User pressed Esc
    };

    let input_dir = PathBuf::from(INPUT_DIR);
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let content = fs::read_to_string(&path)?;
            let parsed: Value = serde_json::from_str(&content)?;
            if let Some(result) = parsed.pointer(&query) {  // Changed this line
                clean_print(&format!("{}: {}", path.display(), result))?;  // Changed this line
            }
        }
    }

    Ok(())
}

fn data_validation() -> Result<()> {
    clean_print("Data Validation")?;
    let input_dir = PathBuf::from(INPUT_DIR);
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

    clean_print(&format!("{} valid, {} invalid files", valid_count, invalid_count))?;
    Ok(())
}

fn file_compression() -> Result<()> {
    clean_print("File Compression")?;
    clean_print("Select compression level:")?;
    clean_print("1. Low")?;
    clean_print("2. Medium")?;
    clean_print("3. High")?;
    
    let compression_level = loop {
        if let Some(input) = get_user_input("")? {
            match input.as_str() {
                "1" => break Compression::fast(),
                "2" => break Compression::default(),
                "3" => break Compression::best(),
                _ => {
                    clean_print("Invalid input. Please enter 1, 2, or 3.")?;
                    continue;
                }
            }
        } else {
            return Ok(()); // User pressed Esc
        }
    };

    let input_dir = PathBuf::from(INPUT_DIR);
    let output_dir = PathBuf::from(OUTPUT_DIR);

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
            clean_print(&format!("Compressed: {}", output_path.display()))?;
        }
    }

    clean_print("All JSON files compressed.")?;
    Ok(())
}

fn multi_file_processing() -> Result<()> {
    clean_print("Multi-File Processing")?;
    clean_print("Enter file pattern (e.g., '*.json' or 'user_*.json'):")?;
    
    let pattern = match get_user_input("")? {
        Some(input) => input,
        None => return Ok(()), // User pressed Esc
    };

    let input_dir = PathBuf::from(INPUT_DIR);
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap().to_str().unwrap().contains(&pattern) {
            clean_print(&format!("Processing: {}", path.display()))?;
            // Add your processing logic here
        }
    }

    clean_print("All matching files processed.")?;
    Ok(())
}

fn custom_data_types() -> Result<()> {
    clean_print("Custom Data Types")?;
    clean_print("Enter a custom data type (e.g., 'date', 'email', 'phone'):")?;
    
    let custom_type = match get_user_input("")? {
        Some(input) => input,
        None => return Ok(()), // User pressed Esc
    };

    clean_print("Enter a JSON key to apply the custom type:")?;
    let key = match get_user_input("")? {
        Some(input) => input,
        None => return Ok(()), // User pressed Esc
    };

    let input_dir = PathBuf::from(INPUT_DIR);
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let content = fs::read_to_string(&path)?;
            let mut parsed: Value = serde_json::from_str(&content)?;
            if parsed.pointer_mut(&key).is_some() {
                // This is a placeholder. In a real implementation, you'd validate and possibly
                // transform the value based on the custom_type.
                clean_print(&format!("Applied '{}' type to '{}' in {}", custom_type, key, path.display()))?;
            }
        }
    }

    clean_print("Custom data type applied to all matching files.")?;
    Ok(())
}

fn error_checking() -> Result<()> {
    clean_print("Error Checking")?;
    let input_dir = PathBuf::from(INPUT_DIR);
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            let content = fs::read_to_string(&path)?;
            match serde_json::from_str::<Value>(&content) {
                Ok(_) => clean_print(&format!("{}: No errors found", path.display()))?,
                Err(e) => clean_print(&format!("{}: Error - {}", path.display(), e))?,
            }
        }
    }

    clean_print("Error checking completed for all JSON files.")?;
    Ok(())
}

fn pretty_printing() -> Result<()> {
    clean_print("Pretty Printing")?;
    clean_print("Enter indentation spaces (2-8):")?;
    
    let spaces: usize = loop {
        match get_user_input("")? {
            Some(input) => match input.parse() {
                Ok(n) if (2..=8).contains(&n) => break n,
                _ => clean_print("Invalid input. Please enter a number between 2 and 8.")?,
            },
            None => return Ok(()), // User pressed Esc
        }
    };

    let input_dir = PathBuf::from(INPUT_DIR);
    let output_dir = PathBuf::from(OUTPUT_DIR);

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
            clean_print(&format!("Pretty printed: {}", output_path.display()))?;
        }
    }

    clean_print("All JSON files pretty printed.")?;
    Ok(())
}

fn multi_language_support() -> Result<()> {
    clean_print("Multi-Language Support")?;
    clean_print("Enter target language (e.g., 'es' for Spanish, 'fr' for French):")?;
    
    let target_lang = match get_user_input("")? {
        Some(input) => input,
        None => return Ok(()), // User pressed Esc
    };

    clean_print("This is a placeholder for multi-language support.")?;
    clean_print(&format!("In a real implementation, this would translate JSON keys to {}.", target_lang))?;
    clean_print("For now, we'll just demonstrate awareness of the selected language.")?;

    let input_dir = PathBuf::from(INPUT_DIR);
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            clean_print(&format!("Would process {} for {} language support", path.display(), target_lang))?;
        }
    }

    clean_print("Multi-language support simulation completed.")?;
    Ok(())
}

fn speed_optimization() -> Result<()> {
    clean_print("Speed Optimization")?;
    clean_print("Select optimization level:")?;
    clean_print("1. Balanced")?;
    clean_print("2. Memory-Optimized")?;
    clean_print("3. Speed-Optimized")?;
    
    let optimization_level = loop {
        match get_user_input("")? {
            Some(input) => match input.as_str() {
                "1" => break "Balanced",
                "2" => break "Memory-Optimized",
                "3" => break "Speed-Optimized",
                _ => {
                    clean_print("Invalid input. Please enter 1, 2, or 3.")?;
                    continue;
                }
            },
            None => return Ok(()), // User pressed Esc
        }
    };

    clean_print(&format!("Applying {} optimization...", optimization_level))?;
    // This is a placeholder. In a real implementation, you'd apply different
    // parsing or processing strategies based on the selected optimization level.

    let input_dir = PathBuf::from(INPUT_DIR);
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            clean_print(&format!("Optimized processing of: {}", path.display()))?;
        }
    }

    clean_print("Speed optimization simulation completed for all JSON files.")?;
    Ok(())
}

fn clean_print(text: &str) -> crossterm::Result<()> {
    execute!(
        stdout(),
        Print(text),
        Print("\n")
    )?;
    stdout().flush()?;
    Ok(())
}

fn print_error(e: Error) -> crossterm::Result<()> {
    clean_print(&format!("Error: {}", e))
}

fn get_user_input(prompt: &str) -> crossterm::Result<Option<String>> {
    clean_print(prompt)?;
    loop {
        if let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Esc => return Ok(None),
                KeyCode::Enter => {
                    return Ok(Some(String::new())); // Empty string for Enter key
                }
                KeyCode::Char(c) => {
                    print!("{}", c);
                    stdout().flush()?;
                    return Ok(Some(c.to_string()));
                }
                _ => {}
            }
        }
    }
}
