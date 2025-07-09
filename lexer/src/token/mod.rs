#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    ILLEGAL, // Invalid token
    EOF,     // End of file

    // Identifiers and literals
    IDENT, // Variable names, function names, etc.
    INT,   // Integer literals

    // Operators
    ASSIGN,   // "="
    PLUS,     // "+"
    MINUS,    // "-"
    BANG,     // "!"
    SLASH,    // "/"
    ASTERISK, // "*"
    LT,       // "<"
    GT,       // ">"
    NOTEQ,    // "!="

    // Delimiters
    COMMA,     // ","
    SEMICOLON, // ";"
    LPAREN,    // "("
    RPAREN,    // ")"
    LBRACE,    // "{"
    RBRACE,    // "}"

    FUNCTION, // "fn"
    LET,      // "let"
    IF,       // "if"
    ELSE,     // "else"
    RETURN,   // "return"
    TRUE,     // "true"
    FALSE,    // "false"
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    /// Creates a new Token with the specified token type and literal value.
    ///
    /// ## Arguments
    ///
    /// * `token_type` - The type of the token (e.g., IDENT, INT, PLUS, etc.)
    /// * `literal` - The actual string value of the token
    ///
    /// ## Returns
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
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "return" => TokenType::RETURN,
        "true" => TokenType::TRUE,
        "false" => TokenType::FALSE,
        _ => TokenType::IDENT,
    }
}
