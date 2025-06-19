#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers and literals
    IDENT,
    INT,

    // Opreators
    ASSIGN,
    PLUS,

    // Delimeters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        "=" => TokenType::ASSIGN,
        "+" => TokenType::PLUS,
        "," => TokenType::COMMA,
        ";" => TokenType::SEMICOLON,
        "(" => TokenType::LPAREN,
        ")" => TokenType::RPAREN,
        "{" => TokenType::LBRACE,
        "}" => TokenType::RBRACE,
        _ => TokenType::IDENT,
    }
}
