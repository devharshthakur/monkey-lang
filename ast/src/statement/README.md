# Statement

This folder contains all statement types for the Monkey language AST. Statements are things that do something—they represent actions or declarations in the program.

**Examples:**

- `let x = 5;` (variable declaration)
- `return x + 1;` (return statement)
- `foobar(y, z);` (expression statement)

## Usage

Statements make up the body of programs. Each statement represents a complete action that can be executed. The parser builds a list of statements that form the program.
