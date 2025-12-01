# List all available commands

# Run the project
run:
    cd rust && cargo run

# Format code
format:
    cd rust && cargo fmt
    prettier --log-level silent --write .
    shfmt -w scripts/bash

# Alias: fmt
fmt: format

# Check formatting
format-check:
    cd rust && cargo fmt --check
    prettier --ignore-path .gitignore --check .
    shfmt -d scripts/bash

# Alias: fmtc
fmtc: format-check

# Run tests
test:
    cd rust && cargo test

# Lint code
lint:
    cd rust && cargo clippy

# Alias: l
l: lint

# Build the project
build:
    cd rust && cargo build

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
    cd rust && cargo clean
