# Parser

The parser converts tokens from the lexer into an Abstract Syntax Tree (AST). It uses a Pratt parser approach with prefix and infix parse functions registered for different token types.

## Features

- Parses let statements (`let x = 5;`)
- Parses return statements (`return 5;`)
- Parses expression statements
- Parses identifiers
- Parses integer literals
- Parses prefix expressions (`!true`, `-5`)
- Maintains a two-token lookahead buffer for parsing decisions
- Collects parsing errors for user-friendly error reporting

## Dependencies

- `lexer` - For token input
- `ast` - For AST node types

## Usage

```rust
use lexer::Lexer;
use parser::Parser;

let input = "let x = 5;".to_string();
let lexer = Lexer::new(input);
let mut parser = Parser::new(lexer);

let program = parser.parse_program();

if !parser.errors().is_empty() {
    for error in parser.errors() {
        eprintln!("{}", error);
    }
}
```
