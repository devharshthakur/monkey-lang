# Makefile for monkey-lang Rust project
# A comprehensive Makefile with targets for basic Cargo commands

# Variables
CARGO := cargo
RUSTC := rustc
CARGO_TOML := Cargo.toml
LEXER_CRATE := lexer
TARGET_DIR := target
RELEASE_DIR := target/release
DEBUG_DIR := target/debug

# Default target
.DEFAULT_GOAL := help

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
PURPLE := \033[0;35m
CYAN := \033[0;36m
WHITE := \033[0;37m
RESET := \033[0m

# Help target
.PHONY: help
help: ## Show this help message
	@echo "$(CYAN)Available targets:$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(GREEN)%-20s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Usage: make <target>$(RESET)"

# =============================================================================
# BUILD TARGETS
# =============================================================================

.PHONY: build
build: ## Build all crates in debug mode
	@echo "$(BLUE)Building all crates in debug mode...$(RESET)"
	$(CARGO) build

.PHONY: build-release
build-release: ## Build all crates in release mode
	@echo "$(BLUE)Building all crates in release mode...$(RESET)"
	$(CARGO) build --release

.PHONY: build-lexer
build-lexer: ## Build only the lexer crate in debug mode
	@echo "$(BLUE)Building lexer crate in debug mode...$(RESET)"
	$(CARGO) build -p $(LEXER_CRATE)

.PHONY: build-lexer-release
build-lexer-release: ## Build only the lexer crate in release mode
	@echo "$(BLUE)Building lexer crate in release mode...$(RESET)"
	$(CARGO) build --release -p $(LEXER_CRATE)

# =============================================================================
# CLEAN TARGETS
# =============================================================================

.PHONY: clean
clean: ## Clean all build artifacts
	@echo "$(YELLOW)Cleaning all build artifacts...$(RESET)"
	$(CARGO) clean

.PHONY: clean-target
clean-target: ## Remove target directory completely
	@echo "$(YELLOW)Removing target directory...$(RESET)"
	rm -rf $(TARGET_DIR)

# =============================================================================
# TEST TARGETS
# =============================================================================

.PHONY: test
test: ## Run all tests
	@echo "$(GREEN)Running all tests...$(RESET)"
	$(CARGO) test

.PHONY: test-verbose
test-verbose: ## Run all tests with verbose output
	@echo "$(GREEN)Running all tests with verbose output...$(RESET)"
	$(CARGO) test -- --nocapture

.PHONY: test-lexer
test-lexer: ## Run tests for lexer crate only
	@echo "$(GREEN)Running lexer tests...$(RESET)"
	$(CARGO) test -p $(LEXER_CRATE)

.PHONY: test-lexer-verbose
test-lexer-verbose: ## Run lexer tests with verbose output
	@echo "$(GREEN)Running lexer tests with verbose output...$(RESET)"
	$(CARGO) test -p $(LEXER_CRATE) -- --nocapture

.PHONY: test-watch
test-watch: ## Run tests in watch mode (requires cargo-watch)
	@echo "$(GREEN)Running tests in watch mode...$(RESET)"
	cargo watch -x test

# =============================================================================
# CHECK TARGETS
# =============================================================================

.PHONY: check
check: ## Check if code compiles without producing output
	@echo "$(BLUE)Checking if code compiles...$(RESET)"
	$(CARGO) check

.PHONY: check-release
check-release: ## Check if code compiles in release mode
	@echo "$(BLUE)Checking if code compiles in release mode...$(RESET)"
	$(CARGO) check --release

.PHONY: check-lexer
check-lexer: ## Check lexer crate only
	@echo "$(BLUE)Checking lexer crate...$(RESET)"
	$(CARGO) check -p $(LEXER_CRATE)

# =============================================================================
# CLIPPY TARGETS
# =============================================================================

.PHONY: clippy
clippy: ## Run clippy linter
	@echo "$(PURPLE)Running clippy linter...$(RESET)"
	$(CARGO) clippy

.PHONY: clippy-fix
clippy-fix: ## Run clippy with automatic fixes
	@echo "$(PURPLE)Running clippy with automatic fixes...$(RESET)"
	$(CARGO) clippy --fix

.PHONY: clippy-lexer
clippy-lexer: ## Run clippy on lexer crate only
	@echo "$(PURPLE)Running clippy on lexer crate...$(RESET)"
	$(CARGO) clippy -p $(LEXER_CRATE)

# =============================================================================
# FORMAT TARGETS
# =============================================================================

.PHONY: fmt
fmt: ## Format all code with rustfmt
	@echo "$(CYAN)Formatting code with rustfmt...$(RESET)"
	$(CARGO) fmt

.PHONY: fmt-check
fmt-check: ## Check if code is properly formatted
	@echo "$(CYAN)Checking code formatting...$(RESET)"
	$(CARGO) fmt -- --check

# =============================================================================
# DOCUMENTATION TARGETS
# =============================================================================

.PHONY: doc
doc: ## Generate documentation
	@echo "$(BLUE)Generating documentation...$(RESET)"
	$(CARGO) doc

.PHONY: doc-open
doc-open: ## Generate and open documentation
	@echo "$(BLUE)Generating and opening documentation...$(RESET)"
	$(CARGO) doc --open

.PHONY: doc-lexer
doc-lexer: ## Generate documentation for lexer crate
	@echo "$(BLUE)Generating lexer documentation...$(RESET)"
	$(CARGO) doc -p $(LEXER_CRATE)

# =============================================================================
# RUN TARGETS
# =============================================================================

.PHONY: run
run: ## Run the main binary (if exists)
	@echo "$(GREEN)Running main binary...$(RESET)"
	$(CARGO) run

.PHONY: run-release
run-release: ## Run the main binary in release mode
	@echo "$(GREEN)Running main binary in release mode...$(RESET)"
	$(CARGO) run --release

# =============================================================================
# DEVELOPMENT TARGETS
# =============================================================================

.PHONY: dev
dev: ## Development workflow: check, test, build
	@echo "$(CYAN)Running development workflow...$(RESET)"
	$(MAKE) check
	$(MAKE) test
	$(MAKE) build

.PHONY: ci
ci: ## CI workflow: check, test, clippy, fmt-check
	@echo "$(CYAN)Running CI workflow...$(RESET)"
	$(MAKE) check
	$(MAKE) test
	$(MAKE) clippy
	$(MAKE) fmt-check

.PHONY: pre-commit
pre-commit: ## Pre-commit checks: fmt, clippy, test
	@echo "$(CYAN)Running pre-commit checks...$(RESET)"
	$(MAKE) fmt
	$(MAKE) clippy
	$(MAKE) test

# =============================================================================
# UTILITY TARGETS
# =============================================================================

.PHONY: update
update: ## Update dependencies
	@echo "$(YELLOW)Updating dependencies...$(RESET)"
	$(CARGO) update

.PHONY: tree
tree: ## Show dependency tree
	@echo "$(BLUE)Showing dependency tree...$(RESET)"
	$(CARGO) tree

.PHONY: audit
audit: ## Audit dependencies for security vulnerabilities
	@echo "$(RED)Auditing dependencies for security vulnerabilities...$(RESET)"
	$(CARGO) audit

.PHONY: install-tools
install-tools: ## Install useful development tools
	@echo "$(CYAN)Installing development tools...$(RESET)"
	cargo install cargo-watch
	cargo install cargo-audit
	cargo install cargo-tree

# =============================================================================
# WORKSPACE SPECIFIC TARGETS
# =============================================================================

.PHONY: workspace-check
workspace-check: ## Check all workspace members
	@echo "$(BLUE)Checking all workspace members...$(RESET)"
	$(CARGO) check --workspace

.PHONY: workspace-test
workspace-test: ## Test all workspace members
	@echo "$(GREEN)Testing all workspace members...$(RESET)"
	$(CARGO) test --workspace

.PHONY: workspace-build
workspace-build: ## Build all workspace members
	@echo "$(BLUE)Building all workspace members...$(RESET)"
	$(CARGO) build --workspace

# =============================================================================
# DEBUG TARGETS
# =============================================================================

.PHONY: debug-info
debug-info: ## Show debug information about the project
	@echo "$(CYAN)Project Debug Information:$(RESET)"
	@echo "Cargo version: $(shell cargo --version)"
	@echo "Rustc version: $(shell rustc --version)"
	@echo "Target directory: $(TARGET_DIR)"
	@echo "Workspace members: $(shell cargo metadata --format-version=1 --no-deps | jq -r '.workspace_members[]' 2>/dev/null || echo "jq not available")"

# =============================================================================
# CLEANUP TARGETS
# =============================================================================

.PHONY: distclean
distclean: clean ## Deep clean: remove all generated files
	@echo "$(RED)Performing deep clean...$(RESET)"
	rm -rf $(TARGET_DIR)
	rm -rf node_modules
	rm -f pnpm-lock.yaml
	@echo "$(GREEN)Deep clean completed!$(RESET)"

# =============================================================================
# ALIASES
# =============================================================================

.PHONY: b
b: build ## Alias for build

.PHONY: t
t: test ## Alias for test

.PHONY: c
c: clean ## Alias for clean

.PHONY: f
f: fmt ## Alias for fmt

.PHONY: k
k: clippy ## Alias for clippy

.PHONY: d
d: doc ## Alias for doc

# =============================================================================
# SPECIAL TARGETS
# =============================================================================

.PHONY: all
all: clean build test ## Clean, build, and test everything

.PHONY: release
release: clean build-release test ## Create a release build with tests

.PHONY: install
install: build-release ## Install the project (builds release version)

# =============================================================================
# NOTES
# =============================================================================
# 
# Common usage:
#   make help          - Show all available targets
#   make build         - Build in debug mode
#   make test          - Run tests
#   make clippy        - Run linter
#   make fmt           - Format code
#   make dev           - Development workflow
#   make ci            - CI workflow
#   make pre-commit    - Pre-commit checks
#
# For workspace-specific commands:
#   make workspace-build  - Build all workspace members
#   make workspace-test   - Test all workspace members
#
# For the lexer crate specifically:
#   make build-lexer      - Build lexer crate
#   make test-lexer       - Test lexer crate
#   make clippy-lexer     - Lint lexer crate 