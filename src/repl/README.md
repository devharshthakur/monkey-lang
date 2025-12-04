# REPL (Read-Eval-Print Loop)

## Overview

The REPL module provides an interactive command-line interface for the Monkey programming language. It allows users to enter expressions and statements interactively, see the results of parsing, and experiment with the language in real-time.

A REPL (Read-Eval-Print Loop) is a fundamental tool for language development and learning. It provides immediate feedback, making it easy to test language features, debug issues, and understand how the language works.

## Purpose in the Project

The REPL serves multiple important roles in the Monkey language project:

- **Interactive Development**: Enables developers to test language features without writing full programs
- **User Experience**: Provides a friendly, welcoming interface for users to interact with the language
- **Debugging Tool**: Helps identify parsing issues and test edge cases
- **Learning Platform**: Allows users to experiment and learn the language interactively
- **Development Feedback**: Shows the current state of language implementation

## Core Concepts

### Read-Eval-Print Loop Cycle

The REPL follows a classic interactive loop pattern:

1. **Read**: Reads a line of input from the user
2. **Eval**: Processes the input (currently: lexes and parses it)
3. **Print**: Displays the result (currently: prints the AST representation)
4. **Loop**: Repeats the cycle until the user exits

### Input Processing Pipeline

Each user input goes through a processing pipeline:

1. **Input Reading**: Reads a line from standard input
2. **Lexing**: Converts the input string into tokens using the lexer
3. **Parsing**: Converts tokens into an AST using the parser
4. **Error Handling**: Displays parsing errors if any occur
5. **Output**: Prints the AST representation or error messages

### User Experience Features

The REPL provides several UX enhancements:

- **Welcome Message**: Greets users with a friendly message and project status
- **Colored Output**: Uses terminal colors for better readability
- **Error Display**: Shows parsing errors in a clear, formatted way
- **Prompt**: Provides a visual prompt (`>>`) to indicate readiness for input
- **Graceful Exit**: Handles exit signals (Ctrl+D, Ctrl+C) gracefully

## Module Organization

The REPL module consists of:

- **`mod.rs`**: Contains the main REPL loop logic and input/output handling
- **`display.rs`**: Provides display utilities including welcome messages, error formatting, and visual elements

### Error Tolerance

The REPL continues operating even when errors occur:

- Parsing errors are displayed but don't terminate the REPL
- Empty input is ignored gracefully
- Invalid syntax is reported but doesn't crash the session

### State Management

The REPL maintains minimal state:

- No persistent variable bindings (yet - will be added with evaluator)
- Each input is processed independently
- No history tracking (could be added in the future)

## Relationship to Other Modules

- **Lexer**: Used to tokenize user input
- **Parser**: Used to parse tokenized input into AST
- **Display**: Uses display utilities for formatted output
- **Main**: Entry point that initializes and starts the REPL

## Current Functionality

### Implemented Features

- **Interactive Input**: Reads and processes user input line by line
- **Lexing**: Tokenizes input using the lexer
- **Parsing**: Parses tokens into AST structures
- **AST Display**: Shows the parsed AST structure
- **Error Reporting**: Displays parser errors with formatting
- **Welcome Message**: Shows project information and status
- **Exit Handling**: Gracefully handles end-of-input signals

### Future Functionality (--ai)

As the project evolves, the REPL will gain:

- **Evaluation**: Execute parsed AST and return results
- **Variable Binding**: Maintain state across REPL sessions
- **Function Definitions**: Allow defining and calling functions
- **Result Display**: Show evaluated values instead of just AST
- **Error Evaluation**: Display runtime errors from evaluation
- **History**: Command history for better UX
- **Syntax Highlighting**: Color-code input based on tokens

## User Interaction Flow

1. **Startup**: REPL displays welcome message and status
2. **Prompt**: Shows `>>` prompt and waits for input
3. **Input**: User types Monkey code and presses Enter
4. **Processing**: REPL lexes and parses the input
5. **Output**: REPL displays AST or error messages
6. **Repeat**: Returns to prompt for next input
7. **Exit**: User presses Ctrl+D or Ctrl+C to exit

## Error Handling

The REPL handles various error scenarios:

- **Parsing Errors**: Displays formatted error messages with position information
- **Empty Input**: Silently ignores empty lines
- **IO Errors**: Handles input/output errors gracefully
- **Exit Signals**: Detects end-of-input and exits cleanly

## Display Module

The `display` module provides:

- **Welcome Message**: Formatted welcome with project status
- **Error Formatting**: Consistent error message display
- **Visual Elements**: Logo and color schemes
- **Status Information**: Current implementation status

## Future Considerations

Potential REPL enhancements:

- **Multi-line Input**: Support for entering multi-line programs
- **Tab Completion**: Auto-complete for keywords and identifiers
- **Syntax Validation**: Real-time syntax checking
- **Pretty Printing**: Better formatted AST output
- **Export/Import**: Save and load REPL sessions
- **Debug Mode**: Toggle verbose output for debugging

The REPL's modular design allows these features to be added incrementally without disrupting core functionality.
