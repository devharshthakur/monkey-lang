# Monkey-Lang Interpreter & Compiler in Rust

This project is an interpreter and compiler for the Monkey programming language, inspired by the book ["Writing an Interpreter in Go" by Thorsten Ball](https://interpreterbook.com/).

Unlike the original implementation in Go, this project is written in **Rust** and aims to provide a modern, safe, and efficient version of the Monkey language tools.

## Project Goals

- [x] Faithfully implement the Monkey language as described in the book
- [x] Provide both an interpreter and a compiler
- [x] Leverage Rust's safety and performance features
- [x] Serve as a learning resource for Rust and language implementation

## Running & Development

A `Justfile` is provided for developer convenience. You can use it to run the project and perform other common tasks. As the project progresses, more commands may be added to the Justfile to help with development and automation.

To run the project:

```bash
just
```

To see all available commands:

```bash
just --list
```

To format the codebase:

```bash
just format
```

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
