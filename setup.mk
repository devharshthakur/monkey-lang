# setup.mk for monkey-lang project

# Colors for output
CYAN := \033[0;36m
GREEN := \033[0;32m
RESET := \033[0m

.PHONY: help
help: ## Show this help message
	@echo "$(CYAN)Available setup targets:$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(GREEN)%-20s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(CYAN)Usage: make -f setup.mk <target>$(RESET)"

# =============================================================================
# SETUP TARGETS
# =============================================================================

.PHONY: check-rust
check-rust: ## Check if Rust and Cargo are installed
	@echo "$(CYAN)Checking for Rust and Cargo...$(RESET)"
	@command -v rustc >/dev/null 2>&1 || { echo >&2 "Rust is not installed. Please install it from https://rustup.rs/"; exit 1; }
	@command -v cargo >/dev/null 2>&1 || { echo >&2 "Cargo is not installed. Please install it from https://rustup.rs/"; exit 1; }
	@echo "$(GREEN)Rust and Cargo are installed.$(RESET)"

.PHONY: install-tools
install-tools: check-rust ## Install useful development tools
	@echo "$(CYAN)Installing development tools...$(RESET)"
	@echo "This will install: cargo-watch, cargo-audit, cargo-tree"
	@read -p "Continue? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		cargo install cargo-watch; \
		cargo install cargo-audit; \
		cargo install cargo-tree; \
		echo "$(GREEN)Tools installed successfully!$(RESET)"; \
	else \
		echo "Installation cancelled."; \
	fi

.PHONY: setup
setup: install-tools ## Run all setup steps
	@echo "$(GREEN)Project setup complete! You can now use the main Makefile.$(RESET)" 