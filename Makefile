# Makefile for monkey-lang Rust project

# Variables
CARGO := cargo

# Colors for output
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
PURPLE := \033[0;35m
CYAN := \033[0;36m
RESET := \033[0m

# Default target
.DEFAULT_GOAL := help

# Help target
.PHONY: help
help: ## Show this help message
	@echo "$(CYAN)Available targets:$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(GREEN)%-20s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Usage: make <target>$(RESET)"

# =============================================================================
# BASIC COMMANDS
# =============================================================================

.PHONY: build
build: ## Build all crates in debug mode
	@echo "$(BLUE)Building...$(RESET)"
	$(CARGO) build

.PHONY: build-release
build-release: ## Build all crates in release mode
	@echo "$(BLUE)Building for release...$(RESET)"
	$(CARGO) build --release

.PHONY: test
test: ## Run all tests
	@echo "$(GREEN)Running tests...$(RESET)"
	$(CARGO) test

.PHONY: run
run: ## Run the main binary
	@echo "$(GREEN)Running main binary...$(RESET)"
	$(CARGO) run

.PHONY: clean
clean: ## Clean all build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(RESET)"
	$(CARGO) clean

# =============================================================================
# LINTING & FORMATTING
# =============================================================================

.PHONY: clippy
clippy: ## Run clippy linter
	@echo "$(PURPLE)Running clippy linter...$(RESET)"
	$(CARGO) clippy --all-targets -- -D warnings

.PHONY: fmt
fmt: ## Format all code with rustfmt
	@echo "$(CYAN)Formatting code...$(RESET)"
	$(CARGO) fmt

.PHONY: check
check: ## Check if code compiles without producing output
	@echo "$(BLUE)Checking if code compiles...$(RESET)"
	$(CARGO) check

# =============================================================================
# CI / WORKFLOWS
# =============================================================================

.PHONY: ci
ci: fmt check clippy test ## Run all checks for CI 