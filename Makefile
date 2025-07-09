# Makefile for monkey-lang project setup and run

CYAN := \033[0;36m
GREEN := \033[0;32m
YELLOW := \033[0;33m
RESET := \033[0m

.PHONY: help
help: ## Show this help message
	@echo "$(CYAN)Available targets:$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(GREEN)%-20s$(RESET) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(CYAN)Usage: make <target>$(RESET)"

.PHONY: check-rust
check-rust: ## Check if Rust and Cargo are installed, prompt to install if missing
	@echo "$(CYAN)Checking for Rust and Cargo...$(RESET)"
	@if ! command -v rustc >/dev/null 2>&1; then \
		echo "$(YELLOW)Rust is not installed.$(RESET)"; \
		echo "You need Rust to build and run this project."; \
		read -p "Would you like to install Rust now using rustup? [y/N] " -n 1 -r; echo; \
		if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
			curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; \
			echo "$(GREEN)Rust installed successfully!$(RESET)"; \
		else \
			echo "$(YELLOW)Rust installation cancelled. Please install Rust manually from https://rustup.rs/$(RESET)"; \
			exit 1; \
		fi; \
	else \
		echo "$(GREEN)Rust and Cargo are already installed.$(RESET)"; \
	fi


.PHONY: run
run: check-rust ## Run the project (cargo run in cli/)
	@echo "$(CYAN)Ready to run the project (cargo run --manifest-path cli/Cargo.toml).$(RESET)"
	@read -p "Proceed to run the project? [y/N] " -n 1 -r; echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		cargo run --manifest-path cli/Cargo.toml; \
	else \
		echo "$(YELLOW)Run cancelled.$(RESET)"; \
	fi

.PHONY: setup
setup: check-rust run ## Full setup: check Rust, install dependencies, and run the project
	@echo "$(GREEN)Setup complete!$(RESET)" 