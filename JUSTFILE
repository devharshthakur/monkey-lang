# List all available commands

# Run the project
run:
    cargo run 2>/dev/null

# Format code
format:
    cargo fmt
    prettier --log-level silent --write .
    shfmt -w scripts/bash

# Alias for format
fmt: format

# Check formatting
format-check:
    cargo fmt --check
    prettier --ignore-path .gitignore --check .
    shfmt -d scripts/bash

# Alias for format-check
fmtc: format-check

# Run tests
test:
    cargo test

# Lint code
lint:
    cargo clippy

# Alias for lint
l: lint

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode
build-release:
    cargo build --release

# Alias for build-release
br: build-release

# Pre-commit with test
pre-commit-with-test:
    just format
    just lint
    just test

# Alias for pre-commit-with-test
pct: pre-commit-with-test

# Pre-commit without test
pre-commit-without-test:
    just format
    just lint

# Alias for pre-commit-without-test
pc: pre-commit-without-test

# Clean build artifacts
clean:
    cargo clean
