use std::io::{stdout, Read};
use std::fs::{self, File};
use std::path::Path;
use crossterm::{
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, Dataset, Chart, Axis, GraphType, Paragraph},
    style::{Color, Modifier, Style},
    layout::{Constraint, Direction, Layout},
    Terminal,
    symbols,
};
use serde_json::Value;
use anyhow::{Result, Context};

mod formatter;
mod parser;
mod query;
mod config;

use parser::parse_json;
use config::Config;

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

enum AppState {
    Menu,
    ExecutingOption,
}

struct App<'a> {
    items: Vec<ListItem<'a>>,
    state: ratatui::widgets::ListState,
    app_state: AppState,
    output: Vec<String>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        let items = OPTIONS.iter().map(|&i| ListItem::new(i)).collect();
        let mut state = ratatui::widgets::ListState::default();
        state.select(Some(0));
        App { 
            items, 
            state, 
            app_state: AppState::Menu,
            output: Vec::new(),
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn main() -> Result<()> {
    let config = config::read_config()?;
    
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app, &config);

    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    config: &Config,
) -> Result<()> {
    // Generate the data once, outside the loop
    let chart_data = generate_chart_data(config).unwrap_or_else(|_| vec![(0.0, 0.0)]);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20)
                ].as_ref())
                .split(f.area());

            let items = List::new(app.items.clone())
                .block(Block::default().title("Parson Menu").borders(Borders::ALL))
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");

            f.render_stateful_widget(items, chunks[0], &mut app.state);

            match app.app_state {
                AppState::Menu => {
                    let dataset = Dataset::default()
                        .name("JSON Data")
                        .marker(symbols::Marker::Braille)
                        .graph_type(GraphType::Line)
                        .style(Style::default().fg(Color::Cyan))
                        .data(&chart_data);

                    let chart = Chart::new(vec![dataset])
                        .block(Block::default().title("Data Visualization").borders(Borders::ALL))
                        .x_axis(Axis::default().title("File Index").bounds([0.0, 10.0]))
                        .y_axis(Axis::default().title("Value").bounds([0.0, 10.0]));

                    f.render_widget(chart, chunks[1]);
                },
                AppState::ExecutingOption => {
                    let output = Paragraph::new(app.output.join("\n"))
                        .block(Block::default().title("Output").borders(Borders::ALL))
                        .wrap(ratatui::widgets::Wrap { trim: true });
                    f.render_widget(output, chunks[1]);
                }
            }

            let help = Paragraph::new("↑↓: Navigate | Enter: Select | q: Quit")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(help, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match app.app_state {
                AppState::Menu => {
                    match key.code {
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Enter => {
                            if let Some(selected) = app.state.selected() {
                                app.app_state = AppState::ExecutingOption;
                                app.output = execute_option(selected, config)?;
                            }
                        }
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                },
                AppState::ExecutingOption => {
                    if key.code == KeyCode::Enter {
                        app.app_state = AppState::Menu;
                    }
                }
            }
        }
    }
    Ok(())
}

fn execute_option(selected: usize, config: &Config) -> Result<Vec<String>> {
    let mut output = Vec::new();
    match selected {
        0 => fast_reading(config, &mut output)?,
        1 => data_extraction(config, &mut output)?,
        2 => data_validation(config, &mut output)?,
        3 => file_compression(config, &mut output)?,
        4 => multi_file_processing(config, &mut output)?,
        5 => custom_data_types(config, &mut output)?,
        6 => error_checking(config, &mut output)?,
        7 => pretty_printing(config, &mut output)?,
        8 => multi_language_support(config, &mut output)?,
        9 => speed_optimization(config, &mut output)?,
        _ => {}
    }
    Ok(output)
}

// Update all your functions to take &mut Vec<String> as an additional parameter and use it instead of println!
fn fast_reading(config: &Config, output: &mut Vec<String>) -> Result<()> {
    output.push("Fast Reading selected.".to_string());
    let input_dir = Path::new(&config.folders.input_folder);
    
    if !input_dir.exists() {
        output.push(format!("Error: Input directory '{}' does not exist.", config.folders.input_folder));
        return Ok(());
    }

    let entries = fs::read_dir(input_dir).context(format!("Failed to read directory: {}", config.folders.input_folder))?;

    let mut file_count = 0;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            file_count += 1;
            let mut file = File::open(&path).context(format!("Failed to open file: {:?}", path))?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).context(format!("Failed to read file: {:?}", path))?;
            
            let json: Value = parse_json(&contents)?;
            output.push(format!("File: {:?}", path.file_name().unwrap()));
            output.push("First few keys:".to_string());
            if let Some(obj) = json.as_object() {
                for (key, _) in obj.iter().take(5) {
                    output.push(format!("- {}", key));
                }
            }
            output.push(String::new());
        }
    }

    if file_count == 0 {
        output.push("No JSON files found in the input directory.".to_string());
    } else {
        output.push(format!("Processed {} JSON file(s).", file_count));
    }

    Ok(())
}

// Update other functions similarly...

fn generate_chart_data(config: &Config) -> Result<Vec<(f64, f64)>> {
    let input_dir = Path::new(&config.folders.input_folder);
    let mut data = Vec::new();

    let entries = fs::read_dir(input_dir)?;
    for (index, entry) in entries.enumerate() {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            let contents = fs::read_to_string(&path)?;
            let json: Value = serde_json::from_str(&contents)?;
            
            // Assuming we're interested in a numeric value from the JSON
            if let Some(value) = json.get("some_key").and_then(|v| v.as_f64()) {
                data.push((index as f64, value));
            }
        }
    }

    Ok(data)
}

// Add these stub functions after the `fast_reading` function

fn data_extraction(_config: &Config, output: &mut Vec<String>) -> Result<()> {
    output.push("Data Extraction not implemented yet.".to_string());
    Ok(())
}

fn data_validation(_config: &Config, output: &mut Vec<String>) -> Result<()> {
    output.push("Data Validation not implemented yet.".to_string());
    Ok(())
}

fn file_compression(_config: &Config, output: &mut Vec<String>) -> Result<()> {
    output.push("File Compression not implemented yet.".to_string());
    Ok(())
}

fn multi_file_processing(_config: &Config, output: &mut Vec<String>) -> Result<()> {
    output.push("Multi-File Processing not implemented yet.".to_string());
    Ok(())
}

fn custom_data_types(_config: &Config, output: &mut Vec<String>) -> Result<()> {
    output.push("Custom Data Types not implemented yet.".to_string());
    Ok(())
}

fn error_checking(_config: &Config, output: &mut Vec<String>) -> Result<()> {
    output.push("Error Checking not implemented yet.".to_string());
    Ok(())
}

fn pretty_printing(_config: &Config, output: &mut Vec<String>) -> Result<()> {
    output.push("Pretty Printing not implemented yet.".to_string());
    Ok(())
}

fn multi_language_support(_config: &Config, output: &mut Vec<String>) -> Result<()> {
    output.push("Multi-Language Support not implemented yet.".to_string());
    Ok(())
}

fn speed_optimization(_config: &Config, output: &mut Vec<String>) -> Result<()> {
    output.push("Speed Optimization not implemented yet.".to_string());
    Ok(())
}
