# Project Setup

> **Note:** The content of this file may change as the project progresses and the setup process evolves.

## Prerequisites

Before you begin, please ensure you have the following installed:

- [Rust (includes Cargo)](https://www.rust-lang.org/tools/install)
- (Optional) [Make](https://www.gnu.org/software/make/) for developer convenience

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

A `Makefile` is provided for common tasks (like running or formatting the project). You can use it if you have `make` and `pnpm` installed:

- To run the project:
  ```bash
  make
  ```
- To format the codebase:
  ```bash
  make format
  ```
- To see all available commands:
  ```bash
  make help
  ```

---

Happy coding! 🚀
