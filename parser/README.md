# Parser

The parser transforms tokens from the lexer into an Abstract Syntax Tree (AST). It analyzes the token stream according to the language's grammar rules, building a tree structure that represents the program's syntactic structure.

The parser handles various language constructs and maintains error reporting capabilities to provide meaningful feedback when encountering invalid syntax.

## Dependencies

- `lexer` - For token input
- `ast` - For AST node types
