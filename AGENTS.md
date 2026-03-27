# AGENTS.md

This file is the quick-start guide and rules for this repo.

## Project Shape

- Primary implementation: Rust under `src/`.
- Reference implementation: Go under `go/`.
- Docs and task tracking: `md/`, `README.md`, `agent.md`.
- Utility scripts: `scripts/` plus `JUSTFILE` and `package.json`.
- Cursor rules live in `.cursor/rules/`; Copilot rules live in `.github/copilot-instructions.md`.

## GitHub Tasks

- Use GitHub MCP tools first for any GitHub-related task.
- If MCP cannot complete the task, fall back to `gh` via Bash.
- Prefer MCP for issues, pull requests, comments, branches, checks, and repository metadata whenever possible.

## Build / Run / Test

- Run the app: `cargo run`
- Run with debug logging: `RUST_LOG=debug cargo run`
- Release run: `cargo run --release`
- Build: `cargo build`
- Release build: `cargo build --release`
- Test all Rust code: `cargo test`
- Test one Rust test file: `cargo test --test parser_statement_tests`
- Test one specific test: `cargo test test_parsing_let_statements`
- Run one test with output: `cargo test test_parsing_let_statements -- --nocapture`
- Format Rust: `cargo fmt`
- Check Rust formatting: `cargo fmt --check`
- Lint Rust: `cargo clippy`
