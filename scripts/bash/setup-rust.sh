#!/usr/bin/env bash

# Harden script execution: exit on error, undefined var, or pipe failure.
set -euo pipefail

# Uniform prefix for all script logging.
log() {
	printf '[setup-rust] %s\n' "$1"
}

ensure_homebrew() {
	# Short-circuit if Homebrew is missing; the rest of the flow depends on it.
	if ! command -v brew >/dev/null 2>&1; then
		log "Homebrew is required but not found. Install it from https://brew.sh/"
		exit 1
	fi
}

install_rustup() {
	# Install rustup-init only if it isn't already managed by Homebrew.
	if brew list rustup-init >/dev/null 2>&1; then
		log "rustup-init already installed"
	else
		log "Installing rustup-init via Homebrew"
		brew install rustup-init
	fi
}

initialize_rust() {
	# Initialize rustup once, then enforce the stable toolchain and required components.
	if command -v rustup >/dev/null 2>&1 && rustup show active-toolchain >/dev/null 2>&1; then
		log "Rust toolchain already configured"
	else
		log "Bootstrapping Rust toolchain with rustup-init"
		rustup-init -y --no-modify-path
	fi

	log "Ensuring stable toolchain and common components"
	rustup default stable
	rustup component add rustfmt clippy
}

verify_setup() {
	# Surface final tool versions for quick verification.
	log "Installed tool versions:"
	rustc --version
	cargo --version
	rustup --version
}

main() {
	ensure_homebrew
	log "Updating Homebrew formulae"
	brew update
	install_rustup
	initialize_rust
	verify_setup
	log "Rust setup via Homebrew completed"
}

main "$@"
