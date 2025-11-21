# AST (Abstract Syntax Tree)

Defines the Abstract Syntax Tree node types and traits for the Monkey language. This crate provides the data structures that represent parsed Monkey code.

## Core Types

- **`Node`** - Trait that all AST nodes implement, providing `token_literal()` method
- **`Program`** - Root AST node containing all top-level statements
- **`Expression`** - Enum of all expression types
- **`Statement`** - Enum of all statement types

## Dependencies

- `lexer` - For token types used in AST nodes

## Usage

```rust
use ast::{Program, Statement, Expression};

let program = Program {
    statements: vec![
        // ... statements
    ],
};
```
