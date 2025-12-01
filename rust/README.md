# Monkey Language Interpreter (Rust)

This directory contains the Rust implementation of the Monkey programming language interpreter, based on ["Writing an Interpreter in Go"](https://interpreterbook.com/) by Thorsten Ball.

## Project Structure

```text
rust/
├── cli/      # Command-line interface and library root
├── lexer/    # Tokenization of source code
├── ast/      # Abstract Syntax Tree definitions
├── parser/   # Pratt parser implementation
└── repl/     # Interactive Read-Eval-Print Loop
```

## Usage

### Running the REPL

```bash
cargo run
# Or: just run
```

### Building

```bash
cargo build
cargo build --release
```

## Testing

Tests are organized in two locations: inline tests within module files and dedicated test files in the `tests/` directory. Run all tests with `cargo test`.

## Reference Implementation

The [`go`](../go/) directory contains the original Go implementation from the book, used as a reference for this Rust implementation.

## Contributing

This is a learning project following a specific book structure. See [`../CONTRIBUTING.md`](../CONTRIBUTING.md) for guidelines.
