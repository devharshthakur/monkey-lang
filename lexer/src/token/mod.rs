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
    /// Creates a new Token with the specified token type and literal value.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of the token (e.g., IDENT, INT, PLUS, etc.)
    /// * `literal` - The actual string value of the token
    ///
    /// # Returns
    ///
    /// A new Token instance with the provided type and literal.
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

pub fn lookup_identifier(ident: &str) -> TokenType {
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
