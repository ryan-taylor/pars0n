# Parson - High-Performance JSON Parser CLI

Parson is a Rust CLI application that uses simd-lite for high-performance JSON parsing and querying. The output can be formatted in multiple ways, including a format compatible with Google Cloud AI processing. This tool is designed to be efficient and easy to use, perfect for processing large JSON files or multiple files in batch.

## Features

- Fast JSON parsing using simd-lite
- Advanced JSON querying with support for nested keys and array indexing
- Support for processing multiple JSON files in batch
- Multiple output format options:
  - Raw JSON
  - Google Cloud AI compatible format
  - Pretty-printed JSON
- Interactive menu for selecting operations
- Error handling with informative messages
- Deployable on Replit

## Installation

Parson is designed to be used on Replit. To use it, simply clone the repository and build the project using Cargo.

```bash
git clone <repository-url>
cd parson
cargo build --release
```

The built executable will be available in the `target/release` directory.

## Usage

To use Parson, run the following command:

```bash
./target/release/parson
```

This will launch the interactive menu, where you can select the desired operation using the arrow keys and Enter.

### Menu Options

1. Process All JSON Files: This option automatically processes all JSON files in the 'JSON go here' folder and outputs the results to the 'JSON fresh here' folder.
2. Data Extraction: (Functionality to be implemented)
3. Data Validation: (Functionality to be implemented)
4. Pretty Printing: (Functionality to be implemented)
5. Exit: Quit the application

## JSON File Locations

Parson uses two specific folders for input and output:

- `JSON go here`: Place your input JSON files in this folder.
- `JSON fresh here`: Processed JSON files will be output to this folder.

## Development

To contribute to Parson or modify it for your needs:

1. Clone the repository
2. Make your changes
3. Run tests: `cargo test`
4. Build the project: `cargo build --release`

## Deployment on Replit

Parson is designed to be easily deployable on Replit. To deploy:

1. Create a new Repl and select "Import from GitHub"
2. Enter the repository URL
3. Once imported, Replit will automatically detect the Rust project and set up the environment
4. Use the Run button to build and run the project

## License

[Specify your license here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
