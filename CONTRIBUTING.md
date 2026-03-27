# Contributing to Monkey-Lang

First off, thank you for considering contributing to this project! I really appreciate you taking the time to help out.

## Getting Started

Before you start contributing, please set up the project locally by ensuring you have Rust and Cargo installed. See the [setup guide](./md/setup.md) for detailed installation instructions. Use the standard Cargo commands described there to build, test, and run the project.

For an overview of the project's current implementation status, check the [progress checklist](./md/checklist.md).

## Project Context

This project is my personal journey of learning how to build an interpreter by following the book "Writing An Interpreter In Go" by Thorsten Ball. I'm implementing it in Rust to challenge myself and learn both Rust and interpreter design at the same time.

Because this is a learning project that follows a specific book, the scope of contributions is a bit different from a typical open-source project.

## How You Can Contribute

**I** welcome contributions that help me improve the existing code and my understanding. Here are the types of contributions I'm looking for:

- **Code Corrections**: If you spot any bugs or errors in my implementation, please let me know!
- **Code Sanitization & Refactoring**: Improvements to make the code cleaner, more readable, and more idiomatic to Rust.
- **Good Code Practices**: Suggestions on applying better software development practices.
- **Better Approaches**: If you have a better or more efficient way to implement something that's already in the code, I'm all ears! This is a great way for me to learn.

## What I'm Not Looking For

To keep the project aligned with the book's structure and my learning goals, I am **not** looking for:

- **New Features**: Please do not add new language features, parser functionalities, or anything that extends the interpreter beyond what's covered in the book chapters I've implemented.
- **Large-scale Architectural Changes**: While refactoring is welcome, please avoid proposing changes that dramatically alter the project's structure.

## How to Submit Contributions

**Open an Issue**: It's best to open an issue before creating a pull request and discuss the changes you'd like to make first. This helps ensure that your contribution is aligned with the project's goals.

## Issue Labels

This project uses a standardized labeling system to track issues and pull requests.

### Labels You Can Use

When creating issues or pull requests, you may only use these conventional commit type labels:

- **`feat`** - New feature or functionality
- **`fix`** - Bug fix
- **`chore`** - Maintenance tasks, tooling updates
- **`docs`** - Documentation improvements
- **`refactor`** - Code refactoring without behavior changes

### Labels Reserved for Maintainers

The following labels are **only** used by the project maintainer:

- **`issue:accepted`** - Issue has been accepted to be worked on
- **`issue:working`** - Issue is currently in development
- **`issue:external`** - Issue created by external contributor
- **`bug`** - Bug report
- **`advice`** - Advice or opinion on code/ideas
- **`priority:high`**, **priority:medium**, **priority:low** - Issue priority
- **`good first issue`** - Good for newcomers

> **Note**: Issues created by contributors using labels outside the conventional commit types (or using maintainer-only labels) may be closed automatically.

Thank you again for your interest in contributing! I'm excited to learn from your feedback.
