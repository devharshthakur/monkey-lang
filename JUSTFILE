# List all available commands

# Run the project
run:
    cargo run

# Format code
format:
    cargo fmt
    prettier --write .

# Alias: fmt
fmt: format

# Check formatting
format-check:
    cargo fmt --check
    prettier --check .

# Alias: fmtc
fmtc: format-check

# Run tests
test:
    cargo test

# Build the project
build:
    cargo build

# Clean build artifacts
clean:
    cargo clean
