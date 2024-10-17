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
- Interactive menu with arrow key navigation
- Automatic processing of JSON files from 'JSON go here' to 'JSON fresh here' folders
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

1. Fast Reading: Quickly processes all JSON files from 'JSON go here' to 'JSON fresh here' folders.
2. Data Extraction: Extract specific data from JSON files using key paths.
3. Data Validation: Validate JSON files and get a summary of valid and invalid files.
4. File Compression: Compress JSON files with selectable compression levels.
5. Multi-File Processing: Process multiple files based on a specified pattern.
6. Custom Data Types: Apply custom data types to specific JSON keys.
7. Error Checking: Check for errors in JSON files and display detailed reports.
8. Pretty Printing: Format JSON files with custom indentation.
9. Multi-Language Support: Simulate support for multiple languages (placeholder functionality).
10. Speed Optimization: Simulate different optimization levels for JSON processing.

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

## Project Structure

- `.gitignore`: Specifies intentionally untracked files to ignore
- `config.toml`: Contains configuration for non-Replit deployments
- `README.md`: This file, containing project information and usage instructions

## Third-Party Libraries

Parson uses the following third-party library:

- [simd-json](https://github.com/simd-lite/simd-json): A high-performance JSON parser that leverages SIMD instructions.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Third-Party License Notices

simd-json is licensed under the Apache License, Version 2.0. The full text of the license can be found at [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgements

We would like to thank the contributors and maintainers of the simd-json library for their excellent work in creating a high-performance JSON parsing solution.
