# Lexer

## Overview

The lexer (also known as a tokenizer or scanner) is the first stage of the Monkey language processing pipeline. It transforms raw Monkey source code into a stream of tokens—discrete, categorized units that represent the fundamental building blocks of the language.

The lexer processes the raw character input sequentially, identifying and categorizing language elements such as keywords, identifiers, literals, operators, and delimiters. This transformation from unstructured text to structured tokens is essential for efficient parsing.

## Purpose in the Project

The lexer serves as the foundation of the language processing system. Its primary responsibilities include:

- **Character-Level Processing**: Reads source code character by character to identify language elements
- **Token Classification**: Categorizes sequences of characters into meaningful token types
- **Whitespace Handling**: Skips irrelevant whitespace while preserving structural information
- **Position Tracking**: Maintains accurate line and column information for error reporting
- **Keyword Recognition**: Distinguishes between user-defined identifiers and language keywords

## Core Concepts

### Tokenization Process

The lexer operates as a state machine that reads characters sequentially and groups them into tokens. It recognizes patterns such as:

- Sequences of letters and underscores forming identifiers
- Sequences of digits forming numeric literals
- Single or multi-character operators (e.g., `=`, `==`, `!=`)
- Delimiters that structure the code (parentheses, braces, semicolons)

### Lookahead Mechanism

The lexer uses a peek-ahead mechanism to handle multi-character tokens correctly. For example, when encountering `=`, it must check the next character to determine if it's an assignment operator (`=`) or an equality operator (`==`). This lookahead prevents premature tokenization.

### Position Tracking

Every token includes source position information (line and column numbers). This is crucial for:

- Error reporting that points to exact locations in source code
- Debugging and development tools
- Source mapping for execution traces

### Token Types

Tokens are categorized into several groups:

- **Keywords**: Reserved words with special meaning (`let`, `fn`, `if`, `return`, etc.)
- **Identifiers**: User-defined names for variables, functions, etc.
- **Literals**: Constant values (integers, booleans)
- **Operators**: Symbols for operations (`+`, `-`, `*`, `/`, `==`, `!=`, etc.)
- **Delimiters**: Structural markers (`(`, `)`, `{`, `}`, `;`, `,`)
- **Special Tokens**: End-of-file marker, illegal character markers

## Design Principles

### Sequential Processing

The lexer processes input sequentially from start to finish, making a single pass through the source code. This ensures efficiency and simplicity.

### Deterministic Behavior

Given the same input, the lexer always produces the same token stream. This determinism is essential for reliable parsing and debugging.

### Error Tolerance

When encountering illegal characters, the lexer produces `ILLEGAL` tokens rather than stopping. This allows the parser to collect multiple errors and provide comprehensive feedback.

### ASCII-First Approach

The current implementation focuses on ASCII characters, which covers the core Monkey language syntax. This can be extended to support Unicode identifiers in the future.
The lexer's design allows these enhancements without disrupting the existing tokenization logic.
