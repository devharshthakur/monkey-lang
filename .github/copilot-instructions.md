# Copilot / AI Agent Instructions for monkey-lang

Purpose

- Provide concise, actionable guidance to an AI coding agent to be productive in this repository.

Big picture

- This repository implements the "monkey" programming language in multiple languages; the primary active implementation lives in Rust under `src/` (see `src/main.rs`).
- Other language versions and reference implementations live under `go/` and the top-level `evaluator/`, `parser/`, `lexer/` folders used by the Go and reference tests.
- The project is organized so language-independent concepts (AST, evaluator, lexer, parser, REPL) are implemented in parallel across languages; keep changes consistent across implementations when adding language features.

Key directories and files (start here)

- Rust implementation: `src/` — `src/ast/`, `src/lexer/`, `src/parser/`, `src/repl/`, `src/main.rs`.
- Parser specifics: `src/parser/mod.rs`, `src/parser/precedence.rs`, `src/parser/test_helper.rs` and `src/tests/` for parser tests.
- Go reference: `go/` — `go/ast/ast.go`, `go/parser/parser.go`, `go/evaluator/evaluator.go` and associated `_test.go` files.
- REPL: `src/repl/` and `repl/` in the Go tree; used to exercise interactive behavior.
- Scripts and environment: `scripts/` contains setup helpers (see `scripts/bash/setup-rust.sh` and `scripts/README.md`).
- Docs and examples: top-level `README.md` and many `README.md` files in subfolders.

Build & test workflows

- Rust (primary): from repository root
  - Build: `cargo build` or `cargo build --release`
  - Test: `cargo test` (runs Rust unit/integration tests in `src/` and `tests/`)
  - Format: `cargo fmt`
  - Run: `cargo run` (runs the binary built from `src/main.rs`)
- Go: from repository root
  - Test: `go test ./go/...`
  - Run: `go run go/main.go` (if you need to run the Go implementation)
- JS/TS tooling: see `package.json` for NPM scripts (used for helper scripts, not the language core)

Project conventions & patterns

- Cross-language parity: When adding language features (new AST nodes, token kinds, parser precedence), update both the Rust `src/` and the Go `go/` implementations where appropriate and add tests in each language's test directories.
- AST/Parser separation: AST types are pure data in `src/ast/*`. Parser logic lives in `src/parser/*`. Prefer adding AST constructors before parser changes.
- Parser tests: Look at `src/tests/parser_*_tests.rs` to see examples of test-driven parser adjustments; add small, focused tests mirroring the Go tests when possible.
- Precedence table: `src/parser/precedence.rs` centralizes operator precedence — update it rather than spreading precedence logic.
- Error handling: parser and lexer errors are surfaced via `src/parser/error/` and tests expect specific behaviors — update tests when changing error messages.

Examples of common tasks (where to change code)

- Add a new expression node:
  - Add Rust AST type in `src/ast/` (e.g., `expression.rs`), update `src/parser/mod.rs` to parse it, and add tests under `src/tests/`.
  - Mirror in Go: `go/ast/ast.go`, `go/parser/` and `go/*_test.go`.
- Change operator precedence:
  - Update `src/parser/precedence.rs` and adjust parser code in `src/parser/mod.rs`. Run `cargo test` to validate.

Integration points & external deps

- No external network services; most dependencies are crates (see top-level `Cargo.toml`) and standard Go modules (see `go/go.mod`).
- Use the workspace `Cargo.toml` in the repo root; run Rust commands from the repo root.

Editing & PR guidance for AI agents

- Keep changes minimal and focused. Update tests alongside behavior changes.
- When touching parser/lexer/AST, run `cargo test` and `go test ./go/...` to catch regressions early.
- Preserve naming and structure parity across implementations (Rust ↔ Go). Cite example files in PR description (e.g., `src/parser/mod.rs`, `go/parser/parser.go`).

References (examples to inspect)

- Parser: `src/parser/mod.rs`, `src/parser/precedence.rs`, `src/tests/parser_expression_tests.rs`
- AST: `src/ast/expression.rs`, `go/ast/ast.go`
- Evaluator: `src/` (Rust evaluator equivalents under `src` and `go/evaluator` for the Go version)
- REPL: `src/repl/mod.rs`, `repl/` in Go tree

If uncertain

- Run the test suites first (`cargo test`, `go test ./go/...`) and read failing tests to discover intended behavior.
- Ask the repo owner for desired target implementations to change (Rust only, Go only, or both) when introducing language features.

Feedback

- If any of these pointers are unclear or you'd like more examples, tell me which area (parser, AST, evaluator, build) to expand.
