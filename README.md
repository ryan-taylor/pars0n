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
- Interactive output format selection menu
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
./target/release/parson --files <path_to_json_file1> <path_to_json_file2> ... --query <json_query> [--format <output_format>]
```

For example:

```bash
./target/release/parson --files sample.json sample2.json --query "example.name" --format pretty_json
```

If you don't specify the `--format` option, Parson will present an interactive menu to select the output format.

This will parse the JSON files, execute the specified query on each file, and output the results in the specified format.

### Command-line Options

- `--files`: Paths to the JSON files (required, multiple files can be specified)
- `--query`: JSON query string (required)
- `--format`: Output format (optional, if not provided, an interactive menu will be shown)

## Running the Application

To run the Parson application on Replit, follow these steps:

1. Place your JSON files in the 'JSON go here' folder. You can do this by clicking on the folder icon in the Replit file explorer, navigating to the 'JSON go here' folder, and uploading your JSON files.

2. To run the application, use the following command in the Replit shell:

   ```bash
   run parson --files "JSON go here"/* --query <your_query>
   ```

   Replace `<your_query>` with the JSON query you want to execute on your files.

3. The application will process all JSON files in the 'JSON go here' folder. Each file will be parsed, queried, and the results will be displayed in the console.

4. If you haven't specified an output format using the `--format` option, you'll be prompted to select an output format interactively. You'll see a menu like this:

   ```
   Select output format:
   1. Raw JSON
   2. Google Cloud AI compatible
   3. Pretty JSON
   Enter your choice (1-3):
   ```

   Type the number corresponding to your desired output format and press Enter.

5. The processed results will be displayed in the console. If you want to save the output, you can redirect it to a file in the 'JSON fresh here' folder. For example:

   ```bash
   run parson --files "JSON go here"/* --query <your_query> > "JSON fresh here/output.json"
   ```

   This will save the output to a file named `output.json` in the 'JSON fresh here' folder.

Remember, you can always refer to the Usage section above for more detailed information on the command-line options and query syntax.

## Query Syntax

The query syntax supports nested key access and array indexing:

- Use dot notation for nested keys: `parent.child.grandchild`
- Use numeric indices for array access: `array.0.key`

Examples:
- `example.name` will retrieve the "name" field inside the "example" object
- `data.users.0.email` will retrieve the email of the first user in the "users" array inside the "data" object

## Output Formats

Parson supports three output formats:

1. `raw`: Outputs the raw JSON result of the query
2. `google_cloud_ai`: Formats the output to be compatible with Google Cloud AI processing
3. `pretty_json`: Outputs the result as pretty-printed JSON

To specify the output format, use the `--format` option followed by one of the above format names. If you don't specify a format, you'll be prompted to choose one interactively.

## JSON File Locations

Parson expects JSON files to be placed in two specific folders:

- `JSON go here`: Place your input JSON files in this folder.
- `JSON fresh here`: This folder is used for any output or processed JSON files.

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
