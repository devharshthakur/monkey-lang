# Monkey-Lang Interpreter & Compiler in Rust

This project is an interpreter and compiler for the Monkey programming language, inspired by the book ["Writing an Interpreter in Go" by Thorsten Ball](https://interpreterbook.com/). 

Unlike the original implementation in Go, this project is written in **Rust** and aims to provide a modern, safe, and efficient version of the Monkey language tools.

## Project Goals
- Faithfully implement the Monkey language as described in the book
- Provide both an interpreter and a compiler
- Leverage Rust's safety and performance features
- Serve as a learning resource for Rust and language implementation

## Setup

To get the project set up, you can use the `setup.mk` file which is designed to help contributors get started.

```bash
make -f setup.mk setup
```

Please note that since the project is in its early stages, the `setup.mk` file is a work in progress and will evolve as the project progresses.

## References
- [Writing an Interpreter in Go](https://interpreterbook.com/)
- [Monkey Language Specification](https://github.com/miguelmota/monkey-lang)

## Status
Work in progress. Contributions and feedback are welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md).
