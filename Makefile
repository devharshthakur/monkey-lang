# Simple Makefile for monkey-lang

CYAN := \033[0;36m
GREEN := \033[0;32m
RESET := \033[0m

.PHONY: help run format

# Default target
.DEFAULT_GOAL := run

help: ## Show this help message
	@echo "$(CYAN)Available targets:$(RESET)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(GREEN)%-15s$(RESET) %s\n", $$1, $$2}'

run: check-rust ## Run the project (cargo run in cli/)
	@cargo run 

format: ## Format code using Prettier (JS/TS) and cargo fmt (Rust)
	cargo fmt 
	pnpm format