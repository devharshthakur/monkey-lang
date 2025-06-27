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

## Progress Tracking

Project progress is now tracked in [`checklist.md`](./checklist.md), which contains a detailed, chapter-by-chapter checklist based on the book. This file is regularly updated to reflect completed and in-progress tasks. If you are contributing or following along, please refer to `checklist.md` to see the current status and next steps.

**How to use the checklist:**
- Each major implementation step is listed as a checkbox.
- Mark items as complete as you finish them.
- Use the checklist to guide your contributions or learning.

## References
- [Writing an Interpreter in Go](https://interpreterbook.com/)
- [Monkey Language Specification](https://github.com/miguelmota/monkey-lang)

## Status
See [`checklist.md`](./checklist.md) for up-to-date progress and implementation status.

Contributions and feedback are welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

This project is licensed under the MIT License. See the [`LICENSE`](./LICENSE) file for details.

