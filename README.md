# JSON Parser CLI

This is a Rust CLI application that uses serde_json for JSON parsing and querying. The output is formatted to be compatible with Google Cloud AI processing.

## Usage

To use the JSON Parser CLI, run the following command:

```
json_parser_cli --file <path_to_json_file> --key <json_key_to_query>
```

For example:

```
json_parser_cli --file sample.json --key example
```

This will parse the JSON file, query the specified key, and output the result in a Google Cloud AI compatible format.

## Features

- Fast JSON parsing using serde_json
- Simple key-based JSON querying
- Output formatting compatible with Google Cloud AI
- Error handling with informative messages

## Building the Project

To build the project, make sure you have Rust installed, then run:

```
cargo build --release
```

The built executable will be available in the `target/release` directory.
