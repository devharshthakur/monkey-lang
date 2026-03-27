# Monkey-Lang Implementation Progress Checklist

This checklist tracks our progress through the implementation of the Monkey programming language, inspired by ["Writing an Interpreter in Go" by Thorsten Ball](https://interpreterbook.com/). Here you update the checklist after you implemented a feature . There is no predefined course of development. I am developing as per course of the book.

## Chapter 1: Lexing

- [x] Define token types and token structure
- [x] Implement the Lexer
  - [x] Lex identifiers and keywords
  - [x] Lex integer literals
  - [x] Lex operators and delimiters (`=`, `+`, `-`, `*`, `/`, `!`, `<`, `>`, `==`, `!=`)
  - [x] Lex parentheses, braces, commas, semicolons
  - [x] Lex keywords: `let`, `fn`, `true`, `false`, `if`, `else`, `return`
- [x] Write comprehensive lexer tests
- [x] Implement a REPL that tokenizes input and prints tokens

## Chapter 2: Parsing (Parser & AST)

- [x] Define AST node types and traits (`Node` trait, `Program` structure)
- [x] Implement Program structure
- [x] Parse let statements (basic structure - expression values not yet parsed)
- [x] Parse return statements (basic structure - expression values not yet parsed)
- [x] Parse identifiers
- [x] Parse expression statements
- [x] Parse integer literals
- [x] Parse prefix expressions (`!`, `-`)
- [x] Parse infix expressions (`+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`)
- [x] Parse boolean literals (`true`, `false`)
- [x] Parse grouped expressions (parentheses)
- [x] Parse if/else expressions
- [x] Parse function literals
- [x] Parse block statements
- [x] Write parser tests (tests for identifiers, integer literals, boolean literals, prefix expressions, infix expressions, operator precedence, if/else expressions, function literals, and function parameters)
- [ ] Parse call expressions (AST type exists, but parser function not implemented)
- [ ] Parse array literals
- [ ] Parse hash literals
- [ ] Parse index expressions
- [ ] Implement expression parsing in let statement values (currently skips until semicolon)
- [ ] Implement expression parsing in return statement values (currently skips until semicolon)

> **Important**: Update this file as you complete each step! Mark items as complete by changing `[ ]` to `[x]` and add any relevant implementation details or notes.
