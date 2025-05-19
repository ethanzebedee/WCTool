# WCTool

A Rust implementation of the classic Unix `wc` (word count) utility. This command-line tool counts lines, words, bytes, and characters in text files.

## Features

WCTool provides the following counting functionality:

- `-l, --lines`: Count the number of lines in each file
- `-w, --words`: Count the number of words in each file
- `-c, --bytes`: Count the number of bytes in each file
- `-m, --chars`: Count the number of characters in each file (UTF-8 support)

If no options are specified, WCTool displays all counts. You can also combine multiple options.

## Usage

Basic usage pattern:

```
wc_tool [OPTIONS] [FILE]...
```

Where:
- `[OPTIONS]` can be any combination of `-l`, `-w`, `-c`, and `-m`
- `[FILE]...` is one or more input files. Use `-` for stdin.

### Examples

Count all metrics in a file:

```bash
wc_tool file.txt
```

Count only lines and words:

```bash
wc_tool -l -w file.txt
```

Count characters in multiple files:

```bash
wc_tool -m file1.txt file2.txt
```

Process input from stdin:

```bash
cat file.txt | wc_tool -
```

## Implementation Details

This project was created to practice Rust programming and gain experience with:

- Command-line argument parsing using `clap`
- File I/O and handling standard input
- Error handling with the Result type
- UTF-8 string processing
- Memory management with ownership and borrowing
- Rust's type system and struct implementation
- Formatting output and string manipulation

The implementation reads files into memory to provide accurate counts, with special handling for UTF-8 encoded text when counting characters vs bytes.

## Installation

### Prerequisites

- Rust toolchain (rustc, cargo)

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/your-username/WCTool.git
cd WCTool
```

2. Build the project:
```bash
cargo build --release
```

3. The executable will be available at `target/release/wc_tool`

Alternatively, you can run it directly using cargo:

```bash
cargo run -- [OPTIONS] [FILE]...
```

Note: When using `cargo run`, you need to separate the wc_tool arguments with `--` to pass them through to the application.

## What I Learned With This Project

Through this project, I gained practical experience with:

- Rust's strong type system and ownership model
- Working with command-line arguments in Rust
- Error handling patterns in Rust
- Efficient text processing
- The difference between byte and character counts in UTF-8 text
- Using Rust's standard library for file and console I/O
- Structuring a small but complete Rust application

This project helped solidify my understanding of Rust's core concepts and gave me confidence to tackle more complex projects in the future.

