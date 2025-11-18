# Monkey Language Interpreter (Go)

This is the implementation of the Monkey programming language interpreter in Go, based on ["Writing an Interpreter in Go"](https://interpreterbook.com/) written by Thorsten Ball, of which we are making the rust implementation

## Features

The interpreter consists of four main components: **Lexer** for tokenizing Monkey source code, **Parser** for building Abstract Syntax Tree (AST) from tokens, **Evaluator** for executing AST with environment-based evaluation, and **REPL** for interactive read-eval-print loop.

## Language Support

Supports **Data Types** including Integers, Booleans, Strings, Arrays, and Hashes. **Expressions** include arithmetic, comparison, and prefix/infix operators. **Statements** include `let` bindings and `return` statements. **Control Flow** supports `if/else` expressions. **Functions** are first-class with closures. **Built-ins** include `len`, `puts`, `first`, `last`, `rest`, and `push`.

## Usage

```bash
go run main.go
```

## Project Structure

```text
go/
├── lexer/      # Tokenization
├── parser/     # AST construction
├── evaluator/  # Expression evaluation
├── object/     # Runtime objects and environment
├── repl/       # Interactive REPL
└── token/      # Token definitions
```

## Testing

Run tests for each module:

```bash
go test ./...
```
