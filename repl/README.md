# REPL (Read-Eval-Print Loop)

Provides an interactive REPL for the Monkey language. Currently tokenizes user input and prints tokens, with plans to extend to full parsing and evaluation.

## Features

- Interactive prompt (`>>`)
- Reads input line by line
- Tokenizes input and displays tokens
- Handles empty lines gracefully
- Supports EOF (Ctrl+D) to exit

## Dependencies

- `lexer` - For tokenizing user input

## Usage

```rust
use repl::start;
use std::io;

let stdin = io::stdin().lock();
let stdout = io::stdout().lock();
start(stdin, stdout)?;
```
