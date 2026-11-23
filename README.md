![Rust](https://img.shields.io/badge/Rust-orange?logo=rust&style=for-the-badge)
![GitHub License](https://img.shields.io/github/license/NamelessProj/word_count_rs?style=for-the-badge)
![GitHub repo size](https://img.shields.io/github/repo-size/NamelessProj/word_count_rs?style=for-the-badge)


# Word Count Rust

A lightweight command-line tool written in Rust for counting words, lines, and characters in text files.


## Features

- Count whitespace-separated words
- Count lines
- Count Unicode characters
- Print only the final count for easy use in scripts


## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with Cargo
- Windows, macOS, or Linux


## Installation

clone this repository:
```
git clone https://github.com/NamelessProj/word_count_rs.git
cd word_count_rs
```

Build the release executable:
```
cargo build --release
```

The executable is created in `target/release`.


## Usage

```
cargo run -- --file <PATH> --type <TYPE>
```

Or, after building:
```
./target/release/word_count_rs --file <PATH> --type <TYPE>
```

On Windows, run:
```
./target/release/word_count_rs.exe --file <PATH> --type <TYPE>
```


### Arguments

| Argument | Values | Description |
|----|----|----|
| `--file` | File path | The text file to analyse |
| `--type` | `word`, `line`, or `character` | The value to count |


### Examples

Count words:
```
cargo run -- --file tests/test1.txt --type word
```

Count lines:
```
cargo run -- --file tests/test2.txt --type line
```

Count characters:
```
cargo run -- --file tests/test2.txt --type charachter
```

The command prints a single number. For example:
```
1345
```


## How It Works

- **Word count**: Counts text separated by whitespace.
- **Line count**: Counts lines using Rust's built-in `lines()` method.
- **Character count**: Counts Unicode characters using Rust's `chars()` method.


## Test Files

The `tests` directory contains example text files: [tests/test1.txt](./tests/test1.txt), [tests/test2.txt](./tests/test2.txt).


## Dependencies

- [Clap](https://crates.io/crates/clap): Provides command-line argument parsing.


## License

This project is licensed under the [MIT License](./LICENSE).