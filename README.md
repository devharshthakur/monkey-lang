# Monkey Language

![monkey_lang](https://github.com/user-attachments/assets/f1c7457b-8535-4d2c-a4fa-4d3ca16dda9a)

## What’s Monkey?

Monkey has a C-like syntax, supports **variable bindings**, **prefix** and **infix operators**, has **first-class** and **higher-order functions**, can handle **closures** with ease and has **integers**, **booleans**, **arrays** and **hashes** built-in.

This project is an interpreter and compiler for the Monkey programming language, inspired by the book ["Writing an Interpreter in Go" by Thorsten Ball](https://interpreterbook.com/).
Unlike the original implementation in Go, this project is written in **Rust** and aims to provide a modern, safe, and efficient version of the Monkey language tools.

## Project Goals

- [x] Faithfully implement the Monkey language as described in the book
- [x] Provide both an interpreter and a compiler
- [x] Leverage Rust's safety and performance features
- [x] Serve as a learning resource for Rust and language implementation

## Setup, Running & Development

To setup the project refer [SETUP](./md/setup.md) file.

## Documentation

- **[`agent.md`](./agent.md)** - Comprehensive project context for AI agents, including architecture, conventions, and development guidelines
- **[`md/setup.md`](./md/setup.md)** - Detailed setup instructions
- **[`md/checklist.md`](./md/checklist.md)** - Implementation progress tracker

## Progress Tracking

Project progress is now tracked in [`checklist.md`](./md/checklist.md), which contains a detailed, chapter-by-chapter checklist based on the book. This file is regularly updated to reflect completed and in-progress tasks. If you are contributing or following along, please refer to `checklist.md` to see the current status and next steps.

**How to use the checklist:**

- Each major implementation step is listed as a checkbox.
- Mark items as complete as you finish them.
- Use the checklist to guide your contributions or learning.

## References

1. [Writing an Interpreter in `Go`](https://interpreterbook.com/)
2. [Monkey Language Specification](https://github.com/miguelmota/monkey-lang)
3. [The original `Go` codebase](/go)

## Status

See [`checklist.md`](./md/checklist.md) for up-to-date progress and implementation status.

Contributions and feedback are welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md) to understand how and what to contribute.

## License

This project is licensed under the MIT License. See the [`LICENSE`](./LICENSE) file for details.
