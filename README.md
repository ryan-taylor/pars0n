# Parson - High-Performance JSON Parser CLI

Parson is a Rust CLI application for high-performance JSON parsing and querying. This tool is designed to be efficient and easy to use, perfect for processing large JSON files or multiple files in batch.

## Features

- Fast JSON parsing
- Advanced JSON querying with support for nested keys and array indexing
- Support for processing multiple JSON files in batch
- Multiple output format options:
  - Raw JSON
  - Pretty-printed JSON
- Interactive menu with arrow key navigation
- Automatic processing of JSON files from 'JSIN' to 'JSON' folders
- Error handling with informative messages
- Docker support for easy deployment and isolation

## Installation

### Cloning the Repository

To get started with Parson, clone the repository using the following command:

```bash
git clone https://github.com/ryan-taylor/parson.git
cd parson
```

### Dependencies

Parson requires Rust and Cargo to be installed on your system. If you don't have Rust installed, you can install it by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

After installing Rust, you can install the required dependencies by running:

```bash
cargo build
```

This command will download and compile all necessary dependencies specified in the `Cargo.toml` file.

## Building the Project

To build the project, run the following command in the project root directory:

```bash
cargo build --release
```

This will create an optimized executable in the `target/release` directory.

## Running the Application

After building the project, you can run Parson using the following command:

```bash
./target/release/parson
```

This will launch the interactive menu, where you can select the desired operation using the arrow keys and Enter.

## Usage

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

## Differences from Replit Version

When running Parson outside of Replit, there are a few differences to note:

1. You need to manually create the `JSON go here` and `JSON fresh here` folders in the project root directory.
2. The application uses the `config.toml` file for configuration. Make sure this file is properly set up for your environment.
3. You may need to adjust file paths in the code if your directory structure differs from the default setup.

## Development

To contribute to Parson or modify it for your needs:

1. Clone the repository
2. Make your changes
3. Run tests: `cargo test`
4. Build the project: `cargo build --release`

## Project Structure

- `src/`: Contains the Rust source code files
- `Cargo.toml`: Rust package manifest file
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

## Setup

1. Create two folders in the project root directory:
   - `JSON go here`: Place your input JSON files in this folder.
   - `JSON fresh here`: Processed JSON files will be output to this folder.
2. Ensure the `config.toml` file is properly set up for your environment.

## Running with Docker

To run Parson using Docker while accessing files on your local machine:

1. Create two directories on your desktop:
   - `desktop_JSIN`: for input JSON files
   - `desktop_JSON`: for output files

2. Run the Docker container with volume mounting:

   ```bash
   docker run -it --rm \
     -v /path/to/desktop_JSIN:/workspaces/rust/JSIN \
     -v /path/to/desktop_JSON:/workspaces/rust/JSON \
     parson
   ```

   Replace `/path/to/desktop_JSIN` and `/path/to/desktop_JSON` with the actual paths to the directories you created.

3. Place your input JSON files in the `desktop_JSIN` folder on your desktop.

4. Run Parson from within the Docker container. The processed files will appear in the `desktop_JSON` folder on your desktop.
