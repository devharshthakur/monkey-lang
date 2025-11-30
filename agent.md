# Project Context for AI Agents

<!-- Last generated: 2025-11-30 -->

## Project Overview

**Monkey-Lang** is a Rust implementation of the Monkey programming language interpreter and compiler, based on the book ["Writing an Interpreter in Go" by Thorsten Ball](https://interpreterbook.com/).

Monkey features C-like syntax with variable bindings, prefix/infix operators, first-class and higher-order functions, closures, integers, booleans, arrays, and hashes.

**Author:** Harsh Thakur  
**Repository:** <https://github.com/devharshthakur/monkey-lang>

## Technology Stack

- **Language:** Rust (Edition 2021)
- **Package Manager:** Cargo (Rust) + pnpm (Node.js tooling)
- **Build Tool:** Cargo
- **Task Runner:** [just](https://github.com/casey/just)
- **Formatting:** `cargo fmt` + Prettier + shfmt
- **Linting:** Clippy
- **Git Hooks:** Husky

## Project Structure

```text
monkey-lang/
├── src/
│   ├── main.rs          # Binary entry point (REPL startup)
│   └── lib.rs           # Library root, re-exports modules
├── lexer/
│   ├── mod.rs           # Lexer implementation
│   └── token.rs         # Token types and definitions
├── ast/
│   ├── mod.rs           # AST root, Node trait, Program struct
│   ├── expression.rs    # Expression types (Identifier, Literals, etc.)
│   └── statement.rs     # Statement types (Let, Return, Expression)
├── parser/
│   ├── mod.rs           # Pratt parser implementation
│   ├── precedence.rs    # Operator precedence definitions
│   └── test_helper.rs   # Test utilities for parser tests
├── repl/
│   └── mod.rs           # REPL implementation (tokenizes input)
├── tests/
│   ├── parser_expression_tests.rs
│   └── parser_statement_tests.rs
├── go/                  # Original Go implementation (reference)
├── scripts/
│   ├── ts/setup-rust.ts # Rust setup helper (TypeScript)
│   └── bash/setup-rust.sh
├── md/
│   ├── checklist.md     # Implementation progress tracker
│   └── setup.md         # Setup instructions
├── Cargo.toml           # Rust dependencies
├── package.json         # Node.js dev dependencies
└── JUSTFILE             # Task runner commands
```

## Key Dependencies

### Rust (Cargo.toml)

| Dependency | Purpose                               |
| ---------- | ------------------------------------- |
| `colored`  | Terminal output coloring for REPL     |
| `users`    | Get current username for REPL welcome |

### Dev Dependencies (package.json)

| Dependency               | Purpose                                |
| ------------------------ | -------------------------------------- |
| `prettier`               | Code formatting (Markdown, JSON, etc.) |
| `husky`                  | Git hooks management                   |
| `typescript` / `ts-node` | TypeScript scripts                     |
| `shfmt`                  | Bash script formatting                 |

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

| Command                      | Description                               |
| ---------------------------- | ----------------------------------------- |
| `just run` / `cargo run`     | Run the REPL                              |
| `just test` / `cargo test`   | Run all tests                             |
| `just lint` / `cargo clippy` | Lint code                                 |
| `just format`                | Format all code (Rust + Prettier + shfmt) |
| `just build` / `cargo build` | Build the project                         |
| `just clean` / `cargo clean` | Clean build artifacts                     |
| `just pc`                    | Pre-commit check (format + lint)          |
| `just pct`                   | Pre-commit with tests                     |

## Architecture Overview

### Module Dependencies

```text
main.rs → repl → lexer
lib.rs → lexer, ast, parser, repl
parser → lexer, ast
```

### Key Types

- **`Lexer`** - Tokenizes input string into tokens
- **`Token`** / **`TokenType`** - Token representation
- **`Parser`** - Pratt parser producing AST
- **`Program`** - Root AST node containing statements
- **`Statement`** - Let, Return, Expression statements
- **`Expression`** - Identifier, Literals, Prefix/Infix, If, Function, Call

### Parsing Approach

Uses **Pratt parsing** (top-down operator precedence):

- Two-token lookahead (`curr_token`, `peek_token`)
- Prefix/infix parse function registrations via HashMaps
- Precedence levels defined in `parser/precedence.rs`

## Current Implementation Status

**Completed:**

- Full lexer with all tokens
- Let and Return statement parsing (basic)
- Expression parsing: identifiers, integers, booleans, prefix, infix
- If/else expressions
- Function literals
- Grouped expressions (parentheses)
- Block statements

**In Progress:**

- Call expressions

**Pending:**

- Array literals, hash literals, index expressions
- Expression values in let/return statements
- Evaluator and object system

See `md/checklist.md` for detailed progress.

## Testing

Tests are located in:

- `lexer/mod.rs` (inline tests)
- `tests/parser_expression_tests.rs`
- `tests/parser_statement_tests.rs`

```bash
cargo test                    # Run all tests
cargo test --test parser_expression_tests  # Specific test file
```

### Test Helpers

`parser/test_helper.rs` provides utilities:

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
