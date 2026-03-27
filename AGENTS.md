# AGENTS.md

This file is the quick-start guide for agentic coding work in this repo.
Read `agent.md` first; it is the deeper source of truth for architecture and status.

## Project Shape

- Primary implementation: Rust under `src/`.
- Reference implementation: Go under `go/`.
- Docs and task tracking: `md/`, `README.md`, `agent.md`.
- Utility scripts: `scripts/` plus `JUSTFILE` and `package.json`.
- Cursor rules live in `.cursor/rules/`; Copilot rules live in `.github/copilot-instructions.md`.

## Must-Read Context

- `agent.md` for structure, current status, and conventions.
- `md/checklist.md` before adding or changing language features.
- `.github/copilot-instructions.md` for cross-language guidance and workflow.
- `.cursor/rules/concise.mdc`: keep responses brief and direct.
- `.cursor/rules/use-agent-context.mdc`: consult `agent.md` before coding.
- `.cursor/rules/no-summary.mdc`: do not add recap sections unless asked.

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

## Repo Task Commands

- `just run` / `just rr` / `just go`
- `just build` / `just br`
- `just test`
- `just lint` / `just l`
- `just format` / `just fmt`
- `just format-check` / `just fmtc`
- `just pc` and `just pct`
- `just clean`

## JS / Formatting Helpers

- Install hooks and helper deps: `pnpm install`
- TypeScript setup helper: `pnpm run setup:rust`
- Check repo formatting helpers: `pnpm run format:check`
- Apply repo formatting helpers: `pnpm run format`

## Single-Test Guidance

- Prefer `cargo test <test_name>` for a single test function.
- Use `cargo test --test <file_stem>` for one integration test file.
- Add `-- --nocapture` when you need printed output while debugging.
- If a test is in `src/lexer/mod.rs` or another inline test module, run the matching `cargo test <name>`.

## Rust Style

- Follow `cargo fmt` output; do not hand-format around rustfmt.
- Keep imports grouped by source and let rustfmt normalize order.
- Prefer small, focused modules under `src/<area>/`.
- Use `pub mod` / `mod` declarations in `mod.rs`-style module roots.
- Use `snake_case` for functions, variables, modules, and test names.
- Use `PascalCase` for structs, enums, traits, and type aliases.
- Use `SCREAMING_SNAKE_CASE` for constants and enum variants only where the codebase already does so.
- Keep public APIs documented with `///` comments when they are part of the stable surface.
- Favor explicit types when they clarify parser/AST code, but avoid unnecessary annotations.

## Parsing / AST Conventions

- AST types are pure data plus `Display` / `Node` behavior.
- Parser logic belongs in `src/parser/mod.rs` and helpers in `src/parser/`.
- Precedence belongs in `src/parser/precedence.rs`.
- Test helpers belong in `src/parser/test_helper.rs`.
- Add or update parser tests when changing syntax, precedence, or AST shape.
- Mirror behavior with the Go reference when the feature exists in both implementations.

## Error Handling

- Do not panic in production parser/lexer code for recoverable problems.
- Return `Option`/`Result`-style control flow where the current design uses it.
- Add parser failures to `Parser.errors` and keep parsing when possible.
- Use `ParserError::at_token`, `ParserError::at`, or the span helpers for location-aware failures.
- Keep error messages precise and user-facing; tests may assert on them.
- Preserve line/column data on tokens and errors.

## Naming Conventions

- Name tests after behavior, not implementation details.
- Keep helper names descriptive and short, e.g. `check_parser_errors`.
- Prefer domain names from Monkey language concepts: `Statement`, `Expression`, `Program`, `TokenType`.
- Use enum variants and structs that match the AST vocabulary already in the repo.

## Comments / Docs

- Add comments only when a block is non-obvious or error-prone.
- Prefer module docs for file-level context and `///` for public items.
- Keep comments accurate; remove stale comments when code changes.

## Testing Style

- Write small tests that assert one language behavior at a time.
- Use the helpers in `src/parser/test_helper.rs` instead of duplicating assertions.
- Favor readable fixtures and literal Monkey source strings.
- Keep test output deterministic.

## Cross-Language Guidance

- When changing syntax or semantics, check both Rust and Go implementations.
- Update the Go reference under `go/` when parity matters.
- Use the Go code as a behavioral reference, not as a place to invent new rules.
- Keep naming and AST shape aligned across languages where practical.

## What To Avoid

- Avoid unrelated refactors in the same change.
- Avoid changing public behavior without adding tests.
- Avoid editing generated artifacts unless required.
- Avoid large comment blocks that restate the code.
- Avoid summaries in responses unless the user asks for them.

## Quick Workflow

- Read `agent.md` and `md/checklist.md`.
- Make the smallest change that solves the task.
- Run the narrowest useful test first.
- Expand to `cargo test`, `cargo fmt --check`, and `cargo clippy` when appropriate.
- If the change touches parser/AST syntax, also validate the Go reference path when relevant.

## Handy Paths

- Rust entry point: `src/main.rs`
- Library root: `src/lib.rs`
- Lexer: `src/lexer/`
- AST: `src/ast/`
- Parser: `src/parser/`
- Parser tests: `src/tests/`
- REPL: `src/repl/`
- Go reference: `go/`

## Final Reminder

- Keep changes focused.
- Keep tests updated.
- Keep the Rust and Go sides aligned when the feature spans both.
