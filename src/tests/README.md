# Tests

## Overview

The tests module contains comprehensive test suites for verifying the correctness of the Monkey language parser. These tests ensure that the parser correctly handles various language constructs, edge cases, and error conditions.

Testing is a critical component of language implementation, as it provides confidence that the parser correctly interprets Monkey code according to the language specification.

## Purpose in the Project

The tests module serves several important purposes:

- **Correctness Verification**: Ensures the parser produces correct AST structures for valid Monkey code
- **Regression Prevention**: Catches bugs introduced during development or refactoring
- **Specification Documentation**: Tests serve as executable documentation of language behavior
- **Development Guide**: Provides examples of how different language constructs should be parsed
- **Quality Assurance**: Maintains code quality by requiring tests for new features

## Core Concepts

### Test Organization

Tests are organized by the type of language construct they verify:

- **Expression Tests**: Verify parsing of various expression types (identifiers, literals, operators, function calls, etc.)
- **Statement Tests**: Verify parsing of statement types (let statements, return statements, expression statements)

### Test Structure

Each test follows a consistent pattern:

1. **Input**: Monkey source code to be parsed
2. **Parsing**: Lex and parse the input
3. **Verification**: Check that the resulting AST matches expected structure
4. **Error Checking**: Verify no parsing errors occurred

### Test Helpers

The parser module provides test helper utilities that simplify writing tests:

- **Error Checking**: Verify no parser errors occurred
- **Node Verification**: Check specific AST node types and values
- **Expression Testing**: Verify expression structures match expectations
- **Statement Testing**: Verify statement structures match expectations

## Module Organization

The tests module consists of:

- **`parser_expression_tests.rs`**: Comprehensive tests for expression parsing
- **`parser_statement_tests.rs`**: Comprehensive tests for statement parsing

Additional tests are located inline within the modules they test (e.g., lexer tests in `lexer/mod.rs`).

## Test Coverage Areas

### Expression Tests

The expression test suite covers:

- **Identifiers**: Variable and function names
- **Literals**: Integer and boolean literals
- **Prefix Expressions**: Unary operators (`!`, `-`)
- **Infix Expressions**: Binary operators (`+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`)
- **Operator Precedence**: Correct parsing order for complex expressions
- **Grouped Expressions**: Parentheses and precedence override
- **If Expressions**: Conditional expressions with and without else clauses
- **Function Literals**: Function definitions with various parameter counts
- **Call Expressions**: Function calls with various argument counts
- **Nested Expressions**: Complex nested expression structures

### Statement Tests

The statement test suite covers:

- **Let Statements**: Variable declarations with various expression types
- **Return Statements**: Return statements with various expression types
- **Expression Statements**: Standalone expressions used as statements
- **Block Statements**: Sequences of statements in blocks
- **Nested Statements**: Statements containing other statements

## Design Principles

### Comprehensive Coverage

Tests aim to cover:

- **Happy Paths**: Normal, expected usage patterns
- **Edge Cases**: Boundary conditions and unusual but valid inputs
- **Error Cases**: Invalid syntax that should produce appropriate errors
- **Complex Cases**: Nested and complex language constructs

### Maintainability

Tests are designed to be:

- **Readable**: Clear test names and structure
- **Isolated**: Each test is independent
- **Fast**: Quick execution for rapid feedback
- **Deterministic**: Same input always produces same result

### Documentation Value

Tests serve as:

- **Examples**: Show how to use language features
- **Specification**: Define expected behavior
- **Reference**: Demonstrate correct AST structures

## Relationship to Other Modules

- **Parser**: Tests the parser's functionality
- **AST**: Verifies correct AST construction
- **Lexer**: Tests may indirectly verify lexer behavior
- **Test Helpers**: Uses parser test utilities for convenience

## Running Tests

Tests can be run using:

```bash
cargo test                    # Run all tests
cargo test --test parser_expression_tests  # Run specific test file
cargo test --lib              # Run library tests
```

## Test Development Guidelines

When adding new language features:

1. **Write Tests First**: Define expected behavior through tests
2. **Cover Edge Cases**: Test boundary conditions and error cases
3. **Use Helpers**: Leverage test helper functions for consistency
4. **Verify AST Structure**: Check that AST nodes have correct structure
5. **Check Error Handling**: Verify appropriate errors for invalid input

## Future Test Areas

As the project evolves, additional test areas will include:

- **Evaluator Tests**: Verify correct execution of AST
- **Object System Tests**: Test runtime object behavior
- **Built-in Function Tests**: Verify built-in function behavior
- **Integration Tests**: End-to-end REPL and program execution tests
- **Performance Tests**: Ensure parsing performance meets requirements
- **Error Recovery Tests**: Verify parser error recovery behavior

## Best Practices

- **Descriptive Names**: Test names clearly describe what is being tested
- **One Concept Per Test**: Each test verifies a single aspect of behavior
- **Arrange-Act-Assert**: Follow clear test structure
- **Test Independence**: Tests don't depend on execution order
- **Clean Setup**: Minimal setup required for each test

The test suite is an essential part of maintaining code quality and ensuring the Monkey language implementation behaves correctly.
