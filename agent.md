# Project Context for AI Agents

<!-- Last generated: 2025-12-17 -->

## Project Overview

**Monkey-Lang** is a Rust implementation of the Monkey programming language interpreter and compiler, based on the book ["Writing an Interpreter in Go" by Thorsten Ball](https://interpreterbook.com/).

Monkey features C-like syntax with variable bindings, prefix/infix operators, first-class and higher-order functions, closures, integers, booleans, arrays, and hashes.

**Author:** Harsh Thakur  
**Repository:** <https://github.com/devharshthakur/monkey-lang>

## Technology Stack

- **Language:** Rust (Edition 2024)
- **Package Manager:** Cargo (Rust) + pnpm@10.26.0
- **Build Tool:** Cargo
- **Task Runner:** [just](https://github.com/casey/just)
- **Formatting:** `cargo fmt` + Prettier + shfmt
- **Linting:** Clippy
- **Git Hooks:** Husky

## Project Structure

```text
monkey-lang/
├── src/                  # Rust source code
│   ├── main.rs           # Binary entry point (REPL startup)
│   ├── lexer/
│   │   ├── mod.rs        # Lexer implementation
│   │   └── token.rs      # Token types and definitions
│   ├── ast/
│   │   ├── mod.rs        # AST root, Node trait, Program struct
│   │   ├── expression.rs # Expression types (Identifier, Literals, etc.)
│   │   └── statement.rs  # Statement types (Let, Return, Expression)
│   ├── parser/
│   │   ├── mod.rs        # Pratt parser implementation
│   │   ├── precedence.rs # Operator precedence definitions
│   │   ├── error/        # Structured error handling
│   │   │   ├── mod.rs    # ParserErrorType enum
│   │   │   ├── parser_error.rs # ParserError struct with span
│   │   │   └── span.rs   # Span struct for source location
│   │   └── test_helper.rs # Test utilities for parser tests
│   ├── repl/
│   │   ├── mod.rs        # REPL implementation (tokenizes and parses input)
│   │   └── display.rs    # REPL display utilities (welcome message, error printing)
│   └── tests/
│       ├── parser_expression_tests.rs
│       └── parser_statement_tests.rs
├── assets/               # Project assets (images, documentation)
├── go/                   # Original Go implementation (reference)
├── scripts/
│   ├── ts/setup-rust.ts  # Rust setup helper (TypeScript)
│   └── bash/setup-rust.sh
├── md/
│   ├── checklist.md      # Implementation progress tracker
│   └── setup.md          # Setup instructions
├── Cargo.toml            # Rust dependencies and project config
├── package.json          # Node.js dev dependencies
├── tsconfig.json         # TypeScript configuration for scripts
└── JUSTFILE              # Task runner commands
```

## Key Dependencies

### Rust (Cargo.toml)

| Dependency   | Purpose                                         |
| ------------ | ----------------------------------------------- |
| `colored`    | Terminal output coloring for REPL               |
| `users`      | Get current username for REPL welcome           |
| `log`        | Logging facade for debug tracing                |
| `env_logger` | Logger implementation (controlled via RUST_LOG) |

### Dev Dependencies (package.json)

| Dependency    | Version  | Purpose                                |
| ------------- | -------- | -------------------------------------- |
| `@types/node` | ^24.10.4 | TypeScript definitions for Node        |
| `husky`       | ^9.1.7   | Git hooks management                   |
| `prettier`    | ^3.7.4   | Code formatting (Markdown, JSON, etc.) |
| `shfmt`       | ^0.0.1   | Bash script formatting                 |
| `ts-node`     | ^10.9.2  | TypeScript execution environment       |
| `typescript`  | ^5.9.3   | TypeScript compiler and tooling        |

## Development Setup

```bash
# Prerequisites: Rust, just (optional), pnpm (optional)

# Clone and enter project
git clone https://github.com/devharshthakur/monkey-lang.git
cd monkey-lang

# Run the interpreter
cargo run

# Or with just
just run
```

### Git Hooks (Optional)

```bash
pnpm install  # Sets up Husky pre-commit hooks
```

## Important Commands

| Command                           | Description                               |
| --------------------------------- | ----------------------------------------- |
| `just run` / `cargo run`          | Run the REPL (with debug logging)         |
| `just run-release` / `just rr`    | Run the REPL in release mode              |
| `just run-go` / `just go`         | Run the Go reference implementation       |
| `just test` / `cargo test`        | Run all tests                             |
| `just lint` / `just l`            | Lint code                                 |
| `just format` / `just fmt`        | Format all code (Rust + Prettier + shfmt) |
| `just format-check` / `just fmtc` | Check formatting without applying         |
| `just build` / `cargo build`      | Build the project (debug mode)            |
| `just build-release` / `just br`  | Build the project (release mode)          |
| `just clean` / `cargo clean`      | Clean build artifacts                     |
| `just pc`                         | Pre-commit check (format + lint)          |
| `just pct`                        | Pre-commit with tests                     |

## Architecture Overview

### Module Dependencies

```text
src/main.rs → repl → lexer, parser
parser → lexer, ast, parser::error
parser::error → lexer::token
repl → lexer, parser, display
```

**Note:** The project uses a binary-only structure with `src/main.rs` as the entry point. All modules are organized in their respective subdirectories under `src/` and declared using standard `pub mod` declarations.

### Key Types

- **`Lexer`** - Tokenizes input string into tokens with position tracking
- **`Token`** / **`TokenType`** - Token representation with line/column position
- **`Parser`** - Pratt parser producing AST with structured error reporting
- **`ParserError`** - Structured error type with span (line/column) and error kind
- **`ParserErrorType`** - Exhaustive enum of all possible parser errors
- **`Span`** - Source location tracking (line and column)
- **`Program`** - Root AST node containing statements
- **`Statement`** - Let, Return, Expression statements
- **`Expression`** - Identifier, Literals, Prefix/Infix, If, Function, Call

### Parsing Approach

Uses **Pratt parsing** (top-down operator precedence):

- Two-token lookahead (`curr_token`, `peek_token`)
- Prefix/infix parse function registrations via HashMaps
- Precedence levels defined in `src/parser/precedence.rs`
- **Structured error handling** via `parser::error` module:
  - `ParserError` struct combines error type with source location (`Span`)
  - `ParserErrorType` enum provides exhaustive error categorization
  - All errors include `[line X:Y]` format for precise error reporting
  - Errors collected in `Parser.errors` vector instead of panicking
- Optional debug tracing via `log` crate (enable with `RUST_LOG=debug`)

## Current Implementation Status

**Completed:**

- Full lexer with all tokens and source position tracking
- Let and Return statement parsing (with expression values)
- Expression parsing: identifiers, integers, booleans, prefix, infix
- If/else expressions
- Function literals
- Grouped expressions (parentheses)
- Block statements
- Call expressions
- Structured parser error handling (ParserError, ParserErrorType, Span)
- Parser debugging improvements (source position in errors, debug tracing)

**In Progress:**

- (None currently)

**Pending:**

- Array literals, hash literals, index expressions
- Evaluator and object system

See `md/checklist.md` for detailed progress.

## Testing

Tests are located in:

- `src/lexer/mod.rs` (inline tests)
- `src/tests/parser_expression_tests.rs`
- `src/tests/parser_statement_tests.rs`

```bash
cargo test                    # Run all tests
cargo test --test parser_expression_tests  # Specific test file
```

### Test Helpers

`src/parser/test_helper.rs` provides utilities:

- `check_parser_errors()` - Assert no parser errors
- `test_identifier()`, `test_integer_literal()`, etc.
- `test_infix_expression()`, `test_literal_expression()`

## Coding Conventions

- Follow Rust idioms and clippy suggestions
- Use `cargo fmt` for formatting
- Modules use `mod.rs` pattern (not `module_name.rs`)
- Test functions prefixed with `test_`
- Doc comments (`///`) for public APIs
- AST types implement `Node` trait and `Display`

## Reference Implementation

The `go/` directory contains the original Go implementation from the book. Use it as reference when implementing features.

## Contributing Notes

This is a learning project following a specific book structure:

- ✅ Bug fixes, code improvements, better Rust idioms welcome
- ❌ New features beyond the book scope not accepted
- ❌ Major architectural changes not accepted

See `CONTRIBUTING.md` for full guidelines.

## Notes for AI Agents

1. **Check `md/checklist.md`** before implementing features to see what's done
2. **Reference `go/` directory** for expected behavior
3. **Parser uses Pratt parsing** - understand precedence when adding operators
4. **Tests are critical** - add tests for any new parsing functionality
5. **Branch naming**: Feature branches follow `devharshthakur/issue{N}` pattern
6. **Run `just pc`** before commits to ensure formatting and linting pass
7. **Parser debugging**: All parser errors include `[line X:Y]` format. Enable debug tracing with `RUST_LOG=debug cargo run`
8. **Token structure**: Token now includes `line` and `column` fields - always provide position when creating tokens
9. **Error handling**: Use `ParserError::at_token()` or `ParserError::at()` to create errors with proper span information. Never panic - always add errors to `Parser.errors` vector
