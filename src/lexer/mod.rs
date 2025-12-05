pub mod token;

use token::{Token, TokenType, lookup_identifier};

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    input: String,
    curr_position: usize,
    next_read_position: usize,
    curr_char: char, // We currently supports ASCII character only
    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new Lexer instance with the given input string.
    /// ## Arguments
    /// * `input` - The source code string to be tokenized
    /// ## Returns
    /// A new Lexer instance initialized with the input string.
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            curr_position: 0,
            next_read_position: 0,
            curr_char: '\0', // \0 => Null
            line: 1,
            column: 0, // Will be 1 after first read_char
        };
        l.read_char();
        l
    }

    /// Reads the next character from the input and advances the position.
    ///
    /// This method updates the current character and advances both the current
    /// position and read position. If we've reached the end of the input,
    /// it sets the current character to null ('\0'). Also tracks line and column
    /// position for error reporting.
    fn read_char(&mut self) {
        // Update line/column based on current character before advancing
        if self.curr_char == '\n' {
            self.line += 1;
            self.column = 0; // Reset to 0, will be 1 after increment below
        }

        if self.next_read_position >= self.input.len() {
            self.curr_char = '\0';
        } else {
            let (index, character) = self
                .input
                .char_indices()
                .find(|(idx, _)| *idx == self.next_read_position)
                .unwrap();

            self.curr_char = character;
            self.curr_position = index;
        }

        // Increment column for the character we just read (or EOF)
        self.column += 1;
        self.next_read_position += self.curr_char.len_utf8();
    }

    /// Peeks at the next character without advancing the lexer's position.
    ///
    /// This method returns the next character in the input without modifying
    /// the current position. If we've reached the end of the input or if
    /// the read position is beyond the input length, it returns null ('\0').
    ///
    /// ## Returns
    ///
    /// The next character in the input, or '\0' if at the end of input.
    fn peek_char(&self) -> char {
        if self.next_read_position >= self.input.len() {
            '\0'
        } else {
            self.input
                .char_indices()
                .find(|(idx, _)| *idx == self.next_read_position)
                .map(|(_, ch)| ch)
                .unwrap()
        }
    }

    /// Skips all whitespace characters from the current position.
    ///
    /// This method advances the lexer position past any whitespace characters
    /// (spaces, tabs, newlines, etc.) until it encounters a non-whitespace character.
    /// Line/column tracking is handled by read_char.
    fn skip_white_space(&mut self) {
        while self.curr_char.is_ascii_whitespace() {
            self.read_char();
        }
    }

    /// Checks if the current character is a letter (alphabetic or underscore).
    ///
    /// This method returns true if the current character is an alphabetic character
    /// or an underscore, which are valid characters for identifiers.
    /// ## Returns
    /// True if the current character is a letter, false otherwise.
    fn is_letter(&self) -> bool {
        self.curr_char.is_ascii_alphabetic() || self.curr_char == '_'
    }

    /// Checks if the current character is a digit.
    ///
    /// This method returns true if the current character is a digit (0-9).
    /// ## Returns
    /// True if the current character is a digit, false otherwise.
    fn is_digit(&self) -> bool {
        self.curr_char.is_ascii_digit()
    }

    /// Reads an identifier (variable name, function name, etc.) from the current position.
    ///
    /// This method reads consecutive alphabetic characters and underscores,
    /// starting from the current position. It stops when it encounters a character
    /// that is not alphabetic or an underscore.
    /// ## Returns
    /// A String containing the identifier that was read.
    fn read_identifier(&mut self) -> String {
        let start_position = self.curr_position;
        while self.is_letter() {
            self.read_char();
        }
        let end_position = if self.curr_char == '\0' {
            self.input.len()
        } else {
            self.curr_position
        };
        self.input[start_position..end_position].to_string()
    }

    /// Gets the current column position.
    /// This is used to capture the starting column for multi-character tokens.
    fn get_column(&self) -> usize {
        self.column
    }

    /// Reads a numeric literal from the current position.
    ///
    /// This method reads consecutive digit characters starting from the current position.
    /// It stops when it encounters a character that is not a digit.
    /// ## Returns
    /// A String containing the numeric literal that was read.
    fn read_number(&mut self) -> String {
        let start_position = self.curr_position;
        while self.is_digit() {
            self.read_char();
        }
        let end_position = if self.curr_char == '\0' {
            self.input.len()
        } else {
            self.curr_position
        };
        self.input[start_position..end_position].to_string()
    }

    /// Returns the next token from the input stream.
    ///
    /// This method processes the current character and returns the appropriate token.
    /// It handles whitespace, identifiers, numbers, and various operators/delimiters.
    /// The lexer position is advanced as tokens are consumed.
    ///
    /// ## Returns
    /// A Token representing the next lexical element in the input.
    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();

        // Capture position before reading token
        let line = self.line;
        let column = self.column;

        let token = match self.curr_char {
            '=' => {
                // Handling '==' case
                if self.peek_char() == '=' {
                    let ch = self.curr_char;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.curr_char);
                    Token::new(TokenType::EQ, literal, line, column)
                } else {
                    // Handling '=' case : Its a assingment operator
                    Token::new(TokenType::ASSIGN, self.curr_char.to_string(), line, column)
                }
            }
            '-' => Token::new(TokenType::MINUS, self.curr_char.to_string(), line, column),
            '!' => {
                // Here we have two cases '!' or '!=' they both are separate tokens so need to check
                if self.peek_char() == '=' {
                    let ch = self.curr_char;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.curr_char);
                    Token::new(TokenType::NOTEQ, literal, line, column)
                } else {
                    Token::new(TokenType::BANG, self.curr_char.to_string(), line, column)
                }
            }
            '/' => Token::new(TokenType::SLASH, self.curr_char.to_string(), line, column),
            '*' => Token::new(
                TokenType::ASTERISK,
                self.curr_char.to_string(),
                line,
                column,
            ),
            '<' => Token::new(TokenType::LT, self.curr_char.to_string(), line, column),
            '>' => Token::new(TokenType::GT, self.curr_char.to_string(), line, column),
            '+' => Token::new(TokenType::PLUS, self.curr_char.to_string(), line, column),
            ',' => Token::new(TokenType::COMMA, self.curr_char.to_string(), line, column),
            ';' => Token::new(
                TokenType::SEMICOLON,
                self.curr_char.to_string(),
                line,
                column,
            ),
            '(' => Token::new(TokenType::LPAREN, self.curr_char.to_string(), line, column),
            ')' => Token::new(TokenType::RPAREN, self.curr_char.to_string(), line, column),
            '{' => Token::new(TokenType::LBRACE, self.curr_char.to_string(), line, column),
            '}' => Token::new(TokenType::RBRACE, self.curr_char.to_string(), line, column),
            '[' => Token::new(
                TokenType::LBRACKET,
                self.curr_char.to_string(),
                line,
                column,
            ),
            ']' => Token::new(
                TokenType::RBRACKET,
                self.curr_char.to_string(),
                line,
                column,
            ),
            ':' => Token::new(TokenType::COLON, self.curr_char.to_string(), line, column),
            '\0' => Token::new(TokenType::EOF, "".to_string(), line, column),
            _ => {
                // Handling identifiers and numbers
                if self.is_letter() {
                    let start_col = self.get_column();
                    let literal = self.read_identifier();
                    let token_type = lookup_identifier(&literal);
                    return Token::new(token_type, literal, line, start_col);
                } else if self.is_digit() {
                    let start_col = self.get_column();
                    let literal = self.read_number();
                    return Token::new(TokenType::INT, literal, line, start_col);
                } else {
                    Token::new(TokenType::ILLEGAL, self.curr_char.to_string(), line, column)
                }
            }
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;

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
}
