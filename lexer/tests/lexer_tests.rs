use lexer::token::TokenType;
use lexer::Lexer;

#[test]
fn test_next_token() {
    let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
return true;
} else {
return false;
}
!=
"
    .to_string();
    let tests = vec![
        (TokenType::LET, "let".to_string()),
        (TokenType::IDENT, "five".to_string()),
        (TokenType::ASSIGN, "=".to_string()),
        (TokenType::INT, "5".to_string()),
        (TokenType::SEMICOLON, ";".to_string()),
        (TokenType::LET, "let".to_string()),
        (TokenType::IDENT, "ten".to_string()),
        (TokenType::ASSIGN, "=".to_string()),
        (TokenType::INT, "10".to_string()),
        (TokenType::SEMICOLON, ";".to_string()),
        (TokenType::LET, "let".to_string()),
        (TokenType::IDENT, "add".to_string()),
        (TokenType::ASSIGN, "=".to_string()),
        (TokenType::FUNCTION, "fn".to_string()),
        (TokenType::LPAREN, "(".to_string()),
        (TokenType::IDENT, "x".to_string()),
        (TokenType::COMMA, ",".to_string()),
        (TokenType::IDENT, "y".to_string()),
        (TokenType::RPAREN, ")".to_string()),
        (TokenType::LBRACE, "{".to_string()),
        (TokenType::IDENT, "x".to_string()),
        (TokenType::PLUS, "+".to_string()),
        (TokenType::IDENT, "y".to_string()),
        (TokenType::SEMICOLON, ";".to_string()),
        (TokenType::RBRACE, "}".to_string()),
        (TokenType::SEMICOLON, ";".to_string()),
        (TokenType::LET, "let".to_string()),
        (TokenType::IDENT, "result".to_string()),
        (TokenType::ASSIGN, "=".to_string()),
        (TokenType::IDENT, "add".to_string()),
        (TokenType::LPAREN, "(".to_string()),
        (TokenType::IDENT, "five".to_string()),
        (TokenType::COMMA, ",".to_string()),
        (TokenType::IDENT, "ten".to_string()),
        (TokenType::RPAREN, ")".to_string()),
        (TokenType::SEMICOLON, ";".to_string()),
        (TokenType::BANG, "!".to_string()),
        (TokenType::MINUS, "-".to_string()),
        (TokenType::SLASH, "/".to_string()),
        (TokenType::ASTERISK, "*".to_string()),
        (TokenType::INT, "5".to_string()),
        (TokenType::SEMICOLON, ";".to_string()),
        (TokenType::INT, "5".to_string()),
        (TokenType::LT, "<".to_string()),
        (TokenType::INT, "10".to_string()),
        (TokenType::GT, ">".to_string()),
        (TokenType::INT, "5".to_string()),
        (TokenType::SEMICOLON, ";".to_string()),
        (TokenType::IF, "if".to_string()),
        (TokenType::LPAREN, "(".to_string()),
        (TokenType::INT, "5".to_string()),
        (TokenType::LT, "<".to_string()),
        (TokenType::INT, "10".to_string()),
        (TokenType::RPAREN, ")".to_string()),
        (TokenType::LBRACE, "{".to_string()),
        (TokenType::RETURN, "return".to_string()),
        (TokenType::TRUE, "true".to_string()),
        (TokenType::SEMICOLON, ";".to_string()),
        (TokenType::RBRACE, "}".to_string()),
        (TokenType::ELSE, "else".to_string()),
        (TokenType::LBRACE, "{".to_string()),
        (TokenType::RETURN, "return".to_string()),
        (TokenType::FALSE, "false".to_string()),
        (TokenType::SEMICOLON, ";".to_string()),
        (TokenType::RBRACE, "}".to_string()),
        (TokenType::NOTEQ, "!=".to_string()),
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
