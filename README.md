# Dada Poetry Generator

A simple Rust command-line tool that reads a Markdown file, shuffles all its words randomly, and writes them into a new Markdown file with a specified number of words per line. Inspired by Dadaist cut-up techniques.

## Features

*   Reads text content from a specified Markdown (`.md`) file.
*   Splits the content into individual words (based on whitespace).
*   Converts all words to lowercase.
*   Randomly shuffles the order of all words.
*   Groups the shuffled words into lines, with a configurable number of words per line.
*   Writes the resulting randomized text to a new Markdown file named `output_randomized.md` in the same directory as the input file.
*   Uses command-line arguments to specify the input file.
*   (Optional: Reads configuration from `files/config.toml` for default input file and words per line - *Currently primarily driven by CLI args and default config values*).

## Requirements

*   [Rust programming language](https://www.rust-lang.org/tools/install) (including Cargo)

## Installation

1.  Clone this repository (or ensure you are in the project directory).
2.  Build the project:
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/dada-poetry`.

## Configuration

The tool can read default settings from `files/config.toml`.

```toml
# Default input file path (relative to project root or absolute)
# This is typically overridden by the command-line argument.
input_filename = "files/foods.md"

# Number of words per line in the output file
words_per_line = 5
```

*   `input_filename`: The default Markdown file to process if none is provided via the command line.
*   `words_per_line`: How many shuffled words to put on each line in the output file.

## Usage

The primary way to run the tool is by providing the input Markdown file as a command-line argument.

**Using `cargo run`:**

```bash
# Replace <path_to_your_file.md> with the actual path to your input file
cargo run -- <path_to_your_file.md>
```

**Example:**

```bash
cargo run -- files/foods.md
```

This command will:
1.  Read the content from `files/foods.md`.
2.  Read the `words_per_line` setting from `files/config.toml` (defaults to 5 if not set).
3.  Shuffle the words from `files/foods.md`.
4.  Create a new file named `files/output_randomized.md`.
5.  Write the shuffled words into `files/output_randomized.md`, with 5 words on each line.

**Using the compiled binary:**

After building with `cargo build --release`:

```bash
./target/release/dada-poetry <path_to_your_file.md>
```

## Example Workflow

1.  Create an input file, e.g., `input.md`:
    ```markdown
    This is a simple example text file. We will shuffle these words around.
    ```
2.  Ensure `files/config.toml` exists and sets `words_per_line = 3`.
3.  Run the tool:
    ```bash
    cargo run -- input.md
    ```
4.  Check for the output file `output_randomized.md`. Its content might look something like this (randomness will vary):
    ```markdown
    example file we
    text these around
    will is simple
    a shuffle words
    ```

## License

(Optional: Add your license information here, e.g., This project is licensed under the MIT License.)

## Contributing

(Optional: Add contribution guidelines if applicable.)
