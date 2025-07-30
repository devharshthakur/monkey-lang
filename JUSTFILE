# List all available commands
default:
    @just --list

# Run the project
run:
    cargo run

# Format code
format:
    cargo fmt
    pnpm format

# Check formatting
format-check:
    cargo fmt --check
    pnpm format --check

# Run tests
test:
    cargo test

# Build the project
build:
    cargo build

# Clean build artifacts
clean:
    cargo clean
