# Oxystyl
Oxystyl is a fast compiler for Stylus CSS implemented in Rust. It uses Logos for tokenization and LALRPOP for parsing, ensuring a clear separation between lexing and parsing phases. The lexer focuses solely on token extraction, delegating high-level semantic processing (like functions or mixins) to the parser.

## Features

- High performance: Built with Rust and a strict separation of concerns.
- Modularity: The lexer extracts only tokens without handling high-level logic.
- Compatibility: Supports most tokens and regex patterns from the original Stylus CSS compiler:
  - Literals and special sequences (strings, numbers with units, colors, Unicode, etc.).
  - Arithmetic and logical operators, delimiters, and grouping symbols.
  - Handling of comments, spaces, newlines, and carriage returns.
  - Specific tokens for Stylus constructs such as \@import, \@css literals, and anonymous functions.
- Flexible grammar: Includes definitions for non-terminal symbols while keeping lexing and parsing logic separate.

## Project Structure

- **lexer.rs**: Contains token definitions using Logos. This file defines regular expressions for each token—from primitive values to operators, literals, and Stylus-specific constructs.
- **parser.rs**: Uses LALRPOP to build the abstract syntax tree (AST) from the tokens produced.
- **main.rs**: Demonstrates example usage, iterating over input to generate tokens.

## Requirements

- [Rust](https://www.rust-lang.org/) (latest stable version recommended)
- Cargo for dependency management

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your_username/Oxystyl.git
   cd Oxystyl
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

After building, you can run the compiler by providing a Stylus file as input:

```bash
cargo run --release -- path/to/your_file.styl
```

The compiler processes the input, generating tokens while deferring semantic interpretation to the parsing phase.

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch for your feature:
   ```bash
   git checkout -b feature/new-feature
   ```
3. Make your changes and commit.
4. Open a pull request describing your changes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

With Oxystyl, we aim to provide an efficient and modular tool for compiling Stylus CSS, simplifying maintenance and future extension as the language evolves.
