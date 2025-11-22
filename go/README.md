# Monkey Language Interpreter (Go)

This folder contains the original Go implementation of the Monkey programming language interpreter from ["Writing an Interpreter in Go"](https://interpreterbook.com/) by Thorsten Ball.
This is provided by the author himself to refer while reading the code.

I'm using this as a reference while reimplementing the interpreter in Rust. The Rust implementation progress is tracked in [`docs/checklist.md`](../docs/checklist.md).

## Project Structure

- **`ast/`** - Abstract Syntax Tree node definitions
- **`lexer/`** - Tokenizes Monkey source code into tokens
- **`parser/`** - Builds AST from tokens using recursive descent parsing
- **`evaluator/`** - Executes AST nodes with environment-based evaluation
- **`object/`** - Runtime object representations and environment management
- **`repl/`** - Interactive read-eval-print loop
- **`token/`** - Token type definitions
- **`main.go`** - Entry point for running the interpreter

## Usage

```bash
go run main.go
```

## Testing

```bash
go test ./...
```
