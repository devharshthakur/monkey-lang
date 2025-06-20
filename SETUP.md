# Project Setup 🚀

To get started with the project, you'll need to set up your development environment. This guide will walk you through the process.

## Prerequisites

Before you begin, please make sure you have the following installed:

-   [Rust](https://www.rust-lang.org/tools/install) (which includes `cargo`)
-   `make`

## Setup Steps

We have a `Makefile` script to help you get everything set up quickly.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/devharshthakur/monkey-lang.git
    cd monkey-lang
    ```

2.  **Run the setup script:**
    This command will check for necessary prerequisites and install some helpful `cargo` tools for development.

    ```bash
    make -f setup.mk setup
    ```

    The script will prompt you for confirmation before installing the tools.

3.  **You're ready to go!**
    Once the setup is complete, you can use the main `Makefile` for common tasks like building, testing, and running the project. Check out the available commands with:
    ```bash
    make help
    ```

Happy coding! 😊 