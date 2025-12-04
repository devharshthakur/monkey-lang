# Parser

## Overview

The parser transforms tokens from the lexer into an Abstract Syntax Tree (AST). It analyzes the token stream according to the language's grammar rules, building a tree structure that represents the program's syntactic structure.

The parser handles various language constructs including expressions, statements, and control flow. It maintains error reporting capabilities to provide meaningful feedback when encountering invalid syntax, ensuring developers receive clear information about parsing issues.

## Purpose in the Project

The parser is the second stage of the language processing pipeline, sitting between the lexer (which produces tokens) and the evaluator (which will execute the AST). Its key responsibilities include:

- **Syntax Analysis**: Verifies that the token sequence conforms to Monkey language grammar rules
- **AST Construction**: Builds a hierarchical tree structure representing the program's structure
- **Precedence Handling**: Correctly parses expressions according to operator precedence rules
- **Error Collection**: Gathers multiple parsing errors for comprehensive feedback
- **Source Position Preservation**: Maintains position information from tokens in AST nodes

## Core Concepts

### Pratt Parsing Algorithm

The parser uses the Pratt parsing algorithm (also known as top-down operator precedence parsing). This approach is particularly well-suited for parsing expressions with complex precedence hierarchies.

Key characteristics of Pratt parsing:

- **Precedence-Driven**: Uses precedence levels to determine parsing order
- **Prefix and Infix Functions**: Registers separate parsing functions for prefix operators (like `-5`) and infix operators (like `5 + 3`)
- **Recursive Descent**: Recursively parses nested expressions based on precedence
- **Efficient**: Makes a single pass through the token stream

### Two-Token Lookahead

The parser maintains a two-token lookahead buffer (`curr_token` and `peek_token`). This allows it to:

- Make parsing decisions based on upcoming tokens
- Handle optional constructs (like `else` clauses)
- Detect syntax errors early
- Parse complex constructs that require lookahead

### Precedence Levels

Operators have different precedence levels that determine evaluation order:

- Lowest: Default precedence for expressions
- Equals: Equality operators (`==`, `!=`)
- LessGreater: Comparison operators (`<`, `>`)
- Sum: Addition and subtraction (`+`, `-`)
- Product: Multiplication and division (`*`, `/`)
- Prefix: Unary operators (`-`, `!`)
- Call: Function calls (highest precedence)

### Error Recovery

The parser employs error recovery strategies:

- **Continue on Error**: When encountering a syntax error, it attempts to recover and continue parsing
- **Error Collection**: Collects multiple errors rather than stopping at the first one
- **Position-Aware Errors**: All error messages include source position information
- **Graceful Degradation**: Produces partial ASTs even when errors occur

## Parsing Process

### Statement Parsing

The parser recognizes three types of statements:

1. **Let Statements**: Variable declarations (`let x = 5;`)
2. **Return Statements**: Function returns (`return 10;`)
3. **Expression Statements**: Standalone expressions (`x + y;`)

### Expression Parsing

Expressions are parsed recursively using precedence:

1. **Prefix Parsing**: Handles unary operators and literals
2. **Infix Parsing**: Handles binary operators based on precedence
3. **Grouping**: Parentheses override normal precedence
4. **Function Calls**: Highest precedence operations

### Control Flow Parsing

The parser handles:

- **If Expressions**: Conditional expressions with optional else clauses
- **Block Statements**: Sequences of statements in braces
- **Function Literals**: Function definitions with parameters and bodies
- **Call Expressions**: Function invocations with arguments

## Design Principles

### Extensibility

The parser uses a registration system for prefix and infix parsing functions. This makes it easy to add new operators or expression types without modifying core parsing logic.

### Error Reporting

All parser errors include:

- Source position (line and column)
- Expected token type
- Actual token found
- Contextual information about what was being parsed

### Debugging Support

The parser includes optional debug tracing (via the `log` crate) that can be enabled to understand parsing decisions and precedence handling.

## Relationship to Other Modules

- **Lexer**: Consumes token stream produced by the lexer
- **AST**: Produces AST nodes that represent the parsed program structure
- **Precedence**: Uses precedence definitions to correctly parse expressions
- **REPL**: Used by the REPL to parse user input

## Parsing Phases

The parsing process involves several phases:

1. **Initialization**: Sets up the parser with a lexer and registers parsing functions
2. **Token Buffer Setup**: Reads initial tokens to establish lookahead
3. **Program Parsing**: Iterates through tokens, parsing statements
4. **Statement Parsing**: Determines statement type and delegates to appropriate parser
5. **Expression Parsing**: Recursively parses expressions using precedence
6. **Error Collection**: Gathers errors encountered during parsing

## Error Handling Strategy

The parser's error handling approach:

- **Non-Fatal Errors**: Continues parsing after errors to find additional issues
- **Error Messages**: Provides clear, actionable error messages with position information
- **Partial ASTs**: May produce partial ASTs even when errors occur
- **Error Recovery**: Attempts to recover from errors by skipping problematic tokens

## Future Considerations (--ai)

Potential parser enhancements:

- Support for array and hash literals
- Index expressions (`array[0]`, `hash["key"]`)
- More sophisticated error recovery
- Parse tree visualization
- Incremental parsing for editor support

The modular design and registration system ensure that new language features can be added without major architectural changes.
