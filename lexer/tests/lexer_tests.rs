use lexer::token::TokenType;
use lexer::Lexer;

#[test]
fn test_next_token() {
    let input = "=+(){},;".to_string();
    let tests = vec![
        (TokenType::ASSIGN, "=".to_string()),
        (TokenType::PLUS, "+".to_string()),
        (TokenType::LPAREN, "(".to_string()),
        (TokenType::RPAREN, ")".to_string()),
        (TokenType::LBRACE, "{".to_string()),
        (TokenType::RBRACE, "}".to_string()),
        (TokenType::COMMA, ",".to_string()),
        (TokenType::SEMICOLON, ";".to_string()),
        (TokenType::EOF, "".to_string()),
    ];
    let mut lex = Lexer::new(input);
    for (i, (expected_type, expected_literal)) in tests.into_iter().enumerate() {
        let token = lex.next_token();
        // Assert that token type matches the expected type
        assert_eq!(
            token.token_type, expected_type,
            "tests[{}] - tokentype wrong. expected={:?}, got={:?}",
            i, expected_type, token.token_type
        );
        // Assert that token literal matches the expected literal
        assert_eq!(
            token.literal, expected_literal,
            "tests[{}] - literaltype wrong. expected={:?}, got={:?}",
            i, expected_literal, token.literal
        )
    }
}
