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
    LBRACKET,  // "["
    RBRACKET,  // "]"
    COLON,     // ":"
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

    /// Converts the token to a Monkey-Lang friendly string representation.
    ///
    /// ## Returns
    ///
    /// A string representation of the token in the Monkey-Lang friendly syntax.
    pub fn to_monkey_lang_token(&self) -> String {
        match self.token_type {
            TokenType::IDENT => self.literal.to_string(),
            TokenType::INT => self.literal.to_string(),
            TokenType::ASSIGN => "=".to_string(),
            TokenType::PLUS => "+".to_string(),
            TokenType::MINUS => "-".to_string(),
            TokenType::BANG => "!".to_string(),
            TokenType::SLASH => "/".to_string(),
            TokenType::ASTERISK => "*".to_string(),
            TokenType::LT => "<".to_string(),
            TokenType::GT => ">".to_string(),
            TokenType::NOTEQ => "!=".to_string(),
            TokenType::EQ => "==".to_string(),
            TokenType::COMMA => ",".to_string(),
            TokenType::SEMICOLON => ";".to_string(),
            TokenType::LPAREN => "(".to_string(),
            TokenType::RPAREN => ")".to_string(),
            TokenType::LBRACE => "{".to_string(),
            TokenType::RBRACE => "}".to_string(),
            TokenType::FUNCTION => "fn".to_string(),
            TokenType::LET => "let".to_string(),
            TokenType::IF => "if".to_string(),
            TokenType::ELSE => "else".to_string(),
            TokenType::RETURN => "return".to_string(),
            TokenType::TRUE => "true".to_string(),
            TokenType::FALSE => "false".to_string(),
            _ => "ILLEGAL".to_string(),
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

/// Looks up the token type for a given operator or delimiter string.
///
/// This function checks if the string matches any operator or delimiter
/// and returns the corresponding TokenType.
///
/// ## Arguments
/// * `op` - The operator or delimiter string to look up
/// ## Returns
/// The TokenType corresponding to the operator/delimiter, or TokenType::ILLEGAL if not found.
pub fn lookup_operator(op: &str) -> TokenType {
    match op {
        "==" => TokenType::EQ,
        "!=" => TokenType::NOTEQ,
        "<" => TokenType::LT,
        ">" => TokenType::GT,
        "(" => TokenType::LPAREN,
        ")" => TokenType::RPAREN,
        "{" => TokenType::LBRACE,
        "}" => TokenType::RBRACE,
        "," => TokenType::COMMA,
        ";" => TokenType::SEMICOLON,
        ":" => TokenType::COLON,
        "[" => TokenType::LBRACKET,
        "]" => TokenType::RBRACKET,
        _ => TokenType::ILLEGAL,
    }
}
