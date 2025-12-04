# List all available commands

# Run the project
run:
    cargo run 2>/dev/null

# Format code
format:
    cargo fmt
    prettier --log-level silent --write .
    shfmt -w scripts/bash

# Alias: fmt
fmt: format

# Check formatting
format-check:
    cargo fmt --check
    prettier --ignore-path .gitignore --check .
    shfmt -d scripts/bash

# Alias: fmtc
fmtc: format-check

# Run tests
test:
    cargo test

# Lint code
lint:
    cargo clippy

# Alias: l
l: lint

# Build the project
build:
    cargo build

# Pre-commit with test
pre-commit-with-test:
    just format
    just lint
    just test

# Alias: pc
pct: pre-commit-with-test

# Pre-commit without test
pre-commit-without-test:
    just format
    just lint

# Alias: pc
pc: pre-commit-without-test

# Clean build artifacts
clean:
    cargo clean
