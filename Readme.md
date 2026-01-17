# Minigrep-rs

A lightweight and a simple command-line search utility written in Rust, inspired by the classic `grep` command. This project demonstrates core Rust concepts including error handling, command-line argument parsing, and file I/O operations.

## Repository

[GitHub Repository: https://github.com/JustUzair/minigrep-rs.git](https://github.com/JustUzair/minigrep-rs.git)

## Overview

Minigrep-rs allows you to search for a specific query string within a text file and displays all matching lines with their line numbers.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system

### Building the Project

```bash
cargo build
```

## Usage

Run the program with the following syntax:

```bash
cargo run <query> <file_path>
```

Where:

- `<query>` - The string you want to search for
- `<file_path>` - The path to the file to search in

### Example

```bash
cargo run on data.txt
```

Output:

```bash
Searching for 'on' in file 'data.txt'
Line 8: The birds sing songs of joy among the trees,
Line 13: The dew still glistens on the tender leaves,
Line 19: The morning light brings harmony and peace.
```

## Testing

Run the test suite with:

```bash
cargo test
```

### Test Output

```bash
0xjustuzair@Mac minigrep-rs % cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/minigrep_rs-f50fe07112bc33d2)

running 2 tests
test tests::test_search ... ok
test tests::test_search_case_insensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/minigrep_rs-dd36a5fa8cb481e4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep_rs

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Features

- **Simple Query Search** - Search for exact string matches in files
- **Case-Sensitive Search** - Default behavior is case-sensitive
- **Case-Insensitive Search** - Support for case-insensitive search operations
- **Line Number Display** - Shows the line number where each match is found
- **Error Handling** - Graceful error handling for missing files and invalid arguments

## Project Structure

```
minigrep-rs/
├── Cargo.toml
├── Readme.md
├── data.txt
└── src/
    ├── main.rs
    └── lib.rs
```

## Author

[JustUzair](https://github.com/JustUzair)
