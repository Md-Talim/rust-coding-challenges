# Word Count (`wc`) Implementation

A Rust implementation of the Unix `wc` command that counts lines, words, characters, and bytes in files.

## Features

- **Line counting** (`-l`): Count newlines in the input
- **Word counting** (`-w`): Count words (sequences of non-whitespace characters)
- **Character counting** (`-m`): Count UTF-8 characters
- **Byte counting** (`-c`): Count bytes
- **Multiple files**: Process multiple files and show totals
- **Standard input**: Read from stdin when no files are specified

## Usage

```bash
# Count all stats (lines, words, bytes) - default behavior
cargo run -- filename.txt

# Count specific stats with flags
cargo run -- -l filename.txt          # Lines only
cargo run -- -w filename.txt          # Words only
cargo run -- -c filename.txt          # Bytes only
cargo run -- -m filename.txt          # Characters only
cargo run -- -lwc filename.txt        # Multiple flags

# Multiple files (shows totals)
cargo run -- file1.txt file2.txt

# Read from standard input
cat filename.txt | cargo run
```

## Build and run

```bash
cargo build --release
cargo run -- test.txt
```
