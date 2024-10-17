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

const OPTIONS: [&str; 5] = [
    "Process All JSON Files",
    "Data Extraction",
    "Data Validation",
    "Pretty Printing",
    "Exit",
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
                        0 => process_all_json_files()?,
                        1 => data_extraction()?,
                        2 => data_validation()?,
                        3 => pretty_printing()?,
                        4 => break,
                        _ => {}
                    }
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

fn process_all_json_files() -> Result<()> {
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
            fs::write(output_path, output)?;
        }
    }

    println!("All JSON files processed successfully.");
    println!("Press any key to return to the main menu...");
    read()?;
    Ok(())
}

fn data_extraction() -> Result<()> {
    println!("Data Extraction functionality not implemented yet.");
    println!("Press any key to return to the main menu...");
    read()?;
    Ok(())
}

fn data_validation() -> Result<()> {
    println!("Data Validation functionality not implemented yet.");
    println!("Press any key to return to the main menu...");
    read()?;
    Ok(())
}

fn pretty_printing() -> Result<()> {
    println!("Pretty Printing functionality not implemented yet.");
    println!("Press any key to return to the main menu...");
    read()?;
    Ok(())
}
