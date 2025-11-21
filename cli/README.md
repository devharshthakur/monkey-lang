# CLI (Command-Line Interface)

The command-line entry point for the Monkey language interpreter. Provides a user-friendly interface to launch the REPL.

## Features

- Greets the user with a personalized welcome message
- Launches the interactive REPL
- Handles standard input/output streams

## Dependencies

- `repl` - For the REPL functionality
- `users` - For getting the current username

## Usage

Run the interpreter:

```bash
cargo run --bin monkey
```

Or build and run:

```bash
cargo build --release
./target/release/monkey
```
