# AST (Abstract Syntax Tree)

## Overview

The AST module defines the Abstract Syntax Tree structures for the Monkey language. It provides the foundational data types that represent parsed Monkey code in a structured, hierarchical format.

The AST serves as an intermediate representation between the source code and the execution engine, allowing for efficient manipulation and analysis of program structure. It captures the syntactic structure of Monkey programs, including expressions, statements, and their relationships, enabling downstream components to process and evaluate code.

## Purpose in the Project

The AST module is the central data structure that bridges the gap between the parser (which produces it) and the evaluator (which will consume it). It provides a tree-like representation of the program's structure that:

- **Preserves Structure**: Maintains the hierarchical relationships between different parts of the program
- **Enables Evaluation**: Provides a structured format that can be traversed to execute the program
- **Facilitates Analysis**: Allows static analysis, optimization, and transformation of code
- **Supports Error Reporting**: Contains source position information for meaningful error messages

## Core Concepts

### Node Trait

All AST nodes implement the `Node` trait, which provides a common interface for accessing token information. This allows uniform access to the original source token that created each node, enabling better error reporting and debugging.

### Program Structure

The `Program` struct is the root of the AST. It contains a collection of top-level statements that make up a complete Monkey program. Each program is essentially a sequence of statements executed in order.

### Statements vs Expressions

The AST distinguishes between two fundamental concepts:

- **Statements**: Represent actions or declarations that don't necessarily produce values. Examples include variable declarations (`let`) and return statements (`return`).
- **Expressions**: Represent computations that evaluate to a value. Examples include arithmetic operations, function calls, and literals.

This distinction is important because statements control program flow and side effects, while expressions compute values.

## Key AST Node Types

### Statements

- **LetStatement**: Variable declarations that bind names to values
- **ReturnStatement**: Function return statements that exit with a value
- **ExpressionStatement**: Standalone expressions used as statements

### Expressions

- **Identifier**: Variable and function names
- **IntegerLiteral**: Numeric constant values
- **BooleanLiteral**: Boolean constant values (`true`, `false`)
- **PrefixExpression**: Unary operators (e.g., `!`, `-`)
- **InfixExpression**: Binary operators (e.g., `+`, `-`, `==`, `!=`)
- **IfExpression**: Conditional expressions with optional else branches
- **BlockStatement**: Sequences of statements enclosed in braces
- **FunctionLiteral**: Function definitions with parameters and body
- **CallExpression**: Function invocations with arguments

## Design Principles

### Immutability

AST nodes are designed to be immutable once created. This ensures that the tree structure remains stable during analysis and evaluation phases.

### Recursive Structure

The AST uses recursive structures (expressions containing expressions, statements containing expressions) to represent the nested nature of programming language syntax. This allows complex expressions like `(5 + 3) * (10 - 2)` to be naturally represented.

### Source Position Tracking

Each AST node retains information about its source position (line and column), enabling precise error reporting and debugging. This information flows from tokens through the parsing process into the AST.

## Relationship to Other Modules

- **Lexer**: Produces tokens that contain the raw source information
- **Parser**: Consumes tokens and produces AST nodes
- **Evaluator** (future): Will traverse the AST to execute the program
- **REPL**: Uses the AST to display parsed program structure

## Future Considerations

As the project evolves, the AST may be extended to support:

- Additional expression types (arrays, hashes, index operations)
- More statement types
- Metadata for optimization passes
- Transformation utilities for code generation

The modular design ensures that new node types can be added without disrupting existing functionality.
