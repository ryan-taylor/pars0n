# JSON Parser CLI

This is a Rust CLI application that uses serde_json for JSON parsing and querying. The output is formatted to be compatible with Google Cloud AI processing.

## Usage

To use the JSON Parser CLI, run the following command:

```
json_parser_cli --files <path_to_json_file1> <path_to_json_file2> ... --query <json_query>
```

For example:

```
json_parser_cli --files sample1.json sample2.json --query "example.name"
```

This will parse the JSON files, execute the specified query on each file, and output the results in a Google Cloud AI compatible format.

## Features

- Fast JSON parsing using serde_json
- Advanced JSON querying with support for nested keys and array indexing
- Support for processing multiple JSON files in batch
- Output formatting compatible with Google Cloud AI
- Error handling with informative messages

## Query Syntax

The query syntax supports nested key access and array indexing:

- Use dot notation for nested keys: `parent.child.grandchild`
- Use numeric indices for array access: `array.0.key`

Examples:
- `example.name` will retrieve the "name" field inside the "example" object
- `data.users.0.email` will retrieve the email of the first user in the "users" array inside the "data" object

## Building the Project

To build the project, make sure you have Rust installed, then run:

```
cargo build --release
```

The built executable will be available in the `target/release` directory.
