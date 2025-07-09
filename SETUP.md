# Project Setup 🚀

To get started with the project, you'll need to set up your development environment. This guide will walk you through the process.

## Prerequisites

Before you begin, please make sure you have the following installed:

-   [Rust](https://www.rust-lang.org/tools/install) (which includes `cargo`)
-   `make` (optional, for easier setup)

---

## Manual Setup

If you prefer not to use the Makefile, you can set up the project manually with the following steps:

1. **Clone the repository:**
    ```bash
    git clone https://github.com/devharshthakur/monkey-lang.git
    cd monkey-lang
    ```

2. **Install Rust (if not already installed):**
    - Check if Rust is installed:
      ```bash
      rustc --version
      ```
    - If you see a version, you're good to go! If not, install Rust with:
      ```bash
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
    - Follow the on-screen instructions to complete the installation, then restart your terminal.


3. **Run the project:**
    ```bash
    cargo run
    ```

---

## Optional: Use the Makefile

We provide a `Makefile` in the project root to automate and simplify the setup process. This is purely optional.

1. **Run the setup script:**
    ```bash
    make setup
    ```
    - This will check for Rust, prompt to install it if missing, build the project, and run it—all with clear prompts and your confirmation at each step.

2. **Explore other commands:**
    See all available commands and their descriptions:
    ```bash
    make help
    ```
    Or run individual steps:
    - `make check-rust` — Check for Rust and install if missing
    - `make install-deps` — Build the project and install dependencies
    - `make run` — Run the project (from cli/)

---

Happy coding! 😊 