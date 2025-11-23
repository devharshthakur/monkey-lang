# Project Setup

> **Note:** The content of this file may change as the project progresses and the setup process evolves.

## Prerequisites

Before you begin, please ensure you have the following installed:

- [Rust (includes Cargo)](https://www.rust-lang.org/tools/install)
- (Optional) [just](https://github.com/casey/just) for developer convenience commands
- (Optional) [pnpm](https://pnpm.io/installation) for formatting and git hooks (Node.js tooling)

## Setting Up Rust

1. **Check if Rust is installed:**

   ```bash
   rustc --version
   cargo --version
   ```

   If you see version numbers, Rust and Cargo are installed.

2. **If not installed, install Rust:**
   - Visit the [official Rust installation page](https://www.rust-lang.org/tools/install) or run:

     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

   - Follow the on-screen instructions and restart your terminal after installation.

## Running the Project

1. **Clone the repository:**

   ```bash
   git clone https://github.com/devharshthakur/monkey-lang.git
   cd monkey-lang
   ```

2. **Run the project:**

   ```bash
   cargo run
   ```

   This will build and run the Monkey interpreter/compiler from the default entry point.

## Developer Convenience (Optional)

A `JUSTFILE` is provided for common tasks (like running, formatting, testing, and linting). You can use it if you have `just` installed:

- To run the project:

  ```bash
  just run
  ```

- To format the codebase:

  ```bash
  just format
  ```

- To run tests:

  ```bash
  just test
  ```

- To lint the code:

  ```bash
  just lint
  ```

- To see all available commands:

  ```bash
  just --list
  ```

### Git Hooks

This project uses [Husky](https://typicode.github.io/husky/) for git hooks. After cloning, run:

```bash
pnpm install
```

This will set up pre-commit hooks that automatically format code and run clippy before commits.
