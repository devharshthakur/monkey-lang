#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
    EQ,       // "=="

    // Delimiters
    COMMA,     // ","
    SEMICOLON, // ";"
    LPAREN,    // "("
    RPAREN,    // ")"
    LBRACE,    // "{"
    RBRACE,    // "}"
    // Keywords
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
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Creates a new Token with the specified token type, literal value, and position.
    ///
    /// ## Arguments
    ///
    /// * `token_type` - The type of the token (e.g., IDENT, INT, PLUS, etc.)
    /// * `literal` - The actual string value of the token
    /// * `line` - The line number where the token starts (1-indexed)
    /// * `column` - The column number where the token starts (1-indexed)
    ///
    /// ## Returns
    ///
    /// A new Token instance with the provided type, literal, and position.
    pub fn new(token_type: TokenType, literal: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            literal,
            line,
            column,
        }
    }
}

/// Looks up the token type for a given identifier string.
///
/// This function checks if the identifier matches any of the predefined keywords
/// and returns the corresponding TokenType. If the identifier is not a keyword,
/// it returns TokenType::IDENT.
///
/// ## Arguments
/// * `ident` - The identifier string to look up
/// ## Returns
/// The TokenType corresponding to the identifier, or TokenType::IDENT if not found.
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
