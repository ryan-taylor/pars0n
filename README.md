# JSON Parser CLI

This is a Rust CLI application that uses serde_json for JSON parsing and querying. The output can be formatted in multiple ways, including a format compatible with Google Cloud AI processing.

## Usage

To use the JSON Parser CLI, run the following command:

```
json_parser_cli --files <path_to_json_file1> <path_to_json_file2> ... --query <json_query> [--format <output_format>]
```

For example:

```
json_parser_cli --files sample1.json sample2.json --query "example.name" --format pretty_json
```

This will parse the JSON files, execute the specified query on each file, and output the results in the specified format.

## Features

- Fast JSON parsing using serde_json
- Advanced JSON querying with support for nested keys and array indexing
- Support for processing multiple JSON files in batch
- Multiple output format options:
  - Raw JSON
  - Google Cloud AI compatible format
  - Pretty-printed JSON
- Error handling with informative messages

## Query Syntax

The query syntax supports nested key access and array indexing:

- Use dot notation for nested keys: `parent.child.grandchild`
- Use numeric indices for array access: `array.0.key`

Examples:
- `example.name` will retrieve the "name" field inside the "example" object
- `data.users.0.email` will retrieve the email of the first user in the "users" array inside the "data" object

## Output Formats

The CLI supports three output formats:

1. `raw`: Outputs the raw JSON result of the query
2. `google_cloud_ai`: Formats the output to be compatible with Google Cloud AI processing (default)
3. `pretty_json`: Outputs the result as pretty-printed JSON

To specify the output format, use the `--format` option followed by one of the above format names.

## Building the Project

To build the project, make sure you have Rust installed, then run:

```
cargo build --release
```

The built executable will be available in the `target/release` directory.
