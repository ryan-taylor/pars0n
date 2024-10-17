# JSON Parser CLI

This is a Rust CLI application that uses serde_json for JSON parsing and querying. The output is formatted to be compatible with Google Cloud AI processing.

## Usage

To use the JSON Parser CLI, run the following command:

```
json_parser_cli --files <path_to_json_file1> <path_to_json_file2> ... --key <json_key_to_query>
```

For example:

```
json_parser_cli --files sample1.json sample2.json --key example
```

This will parse the JSON files, query the specified key in each file, and output the results in a Google Cloud AI compatible format.

## Features

- Fast JSON parsing using serde_json
- Simple key-based JSON querying
- Support for processing multiple JSON files in batch
- Output formatting compatible with Google Cloud AI
- Error handling with informative messages

## Building the Project

To build the project, make sure you have Rust installed, then run:

```
cargo build --release
```

The built executable will be available in the `target/release` directory.
