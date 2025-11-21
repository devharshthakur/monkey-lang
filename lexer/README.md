# Lexer

The lexer (tokenizer) converts Monkey source code into a stream of tokens. It reads characters sequentially and identifies keywords, identifiers, integers, operators, and delimiters.

## Features

- Tokenizes identifiers and keywords (`let`, `fn`, `if`, `else`, `return`, `true`, `false`)
- Tokenizes integer literals
- Tokenizes operators (`+`, `-`, `*`, `/`, `!`, `==`, `!=`, `<`, `>`, `=`)
- Tokenizes delimiters (parentheses, braces, commas, semicolons)
- Handles multi-character operators with lookahead (`==`, `!=`)

## Usage

```rust
use lexer::Lexer;

let input = "let x = 5;".to_string();
let mut lexer = Lexer::new(input);

loop {
    let token = lexer.next_token();
    println!("{:?}", token);
    if token.token_type == TokenType::EOF {
        break;
    }
}
```
