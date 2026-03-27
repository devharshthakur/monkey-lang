# Project Setup

> [!NOTE]
> The content of this file may change as the project progresses and the setup process evolves.

## Prerequisites

Before you begin, please ensure you have the following installed:

- [Rust (includes Cargo)](https://www.rust-lang.org/tools/install)
- (Optional) [pnpm](https://pnpm.io/) for developer convenience commands

## Setting Up Rust

1. **Check if Rust is installed:**

   ```
   rustc --version
   cargo --version
   ```

   If you see version numbers, Rust and Cargo are installed.

2. **If not installed, install Rust:**
   - Visit the [official Rust installation page](https://www.rust-lang.org/tools/install) for installation instructions

## Running the Project

1. **Clone the repository:**

   ```
   git clone https://github.com/devharshthakur/monkey-lang.git
   cd monkey-lang
   ```

2. **Run the project:**

   ```
   cargo run
   ```

   This will build and run the Monkey interpreter/compiler from the default entry point.

## Developer Convenience (Optional)

Since this is a rust project , you can run cargo commands as usual from root, but for convenience i have provided `pnpm` scripts you can find them in [here](../package.json).

> [!IMPORTANT]
> This project uses [Husky](https://typicode.github.io/husky/) for git hooks. After cloning, run:
>  ```pnpm install ```
> This will set up pre-commit hooks that automatically format code and run clippy before commits.
