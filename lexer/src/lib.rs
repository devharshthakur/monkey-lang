use crate::token::{lookup_identifier, Token, TokenType};
pub mod token;

pub struct Lexer {
    input: String,
    curr_position: usize,
    curr_read_position: usize,
    curr_char: char, // We currently supports ASCII character only
}

impl Lexer {
    /// Creates a new Lexer instance with the given input string.
    /// # Arguments
    /// * `input` - The source code string to be tokenized
    /// # Returns
    /// A new Lexer instance initialized with the input string.
    pub fn new(input: String) -> Self {
        let l = Lexer {
            input,
            curr_position: 0,
            curr_read_position: 0,
            curr_char: '\0', // \0 => Null in ASCII
        };
        l
    }

    /// Reads the next character from the input and advances the position.
    ///
    /// This method updates the current character and advances both the current
    /// position and read position. If we've reached the end of the input,
    /// it sets the current character to null ('\0').
    fn read_char(&mut self) {
        if self.curr_position >= self.input.len() {
            self.curr_char = '\0'
        } else {
            let (index, character) = self
                .input
                .char_indices()
                .skip(self.curr_read_position)
                .next()
                .unwrap();
            self.curr_char = character;
            self.curr_position = index;
        }
        self.curr_read_position += self.curr_char.len_utf8();
    }

    /// Peeks at the next character without advancing the lexer's position.
    ///
    /// This method returns the next character in the input without modifying
    /// the current position. If we've reached the end of the input or if
    /// the read position is beyond the input length, it returns null ('\0').
    ///
    /// # Returns
    ///
    /// The next character in the input, or '\0' if at the end of input.
    fn peek_char(&self) -> char {
        if self.curr_read_position > self.input.len() {
            '\0'
        } else {
            self.input
                .char_indices()
                .skip(self.curr_position)
                .next()
                .map(|(_, c)| c)
                .unwrap_or('\0')
        }
    }

    /// Skips all whitespace characters from the current position.
    ///
    /// This method advances the lexer position past any whitespace characters
    /// (spaces, tabs, newlines, etc.) until it encounters a non-whitespace character.
    fn skip_white_space(&mut self) {
        while self.curr_char.is_ascii_whitespace() {
            self.read_char();
        }
    }

    /// Reads an identifier (variable name, function name, etc.) from the current position.
    ///
    /// This method reads consecutive alphabetic characters and underscores,
    /// starting from the current position. It stops when it encounters a character
    /// that is not alphabetic or an underscore.
    /// # Returns
    /// A String containing the identifier that was read.
    fn read_identifier(&mut self) -> String {
        let start_position = self.curr_position;
        while self.curr_char.is_ascii_alphabetic() || self.curr_char == '_' {
            self.read_char();
        }
        self.input[start_position..self.curr_position].to_string()
    }

    /// Reads a numeric literal from the current position.
    ///
    /// This method reads consecutive digit characters starting from the current position.
    /// It stops when it encounters a character that is not a digit.
    /// # Returns
    /// A String containing the numeric literal that was read.
    fn read_number(&mut self) -> String {
        let start_position = self.curr_position;
        while self.curr_char.is_ascii_digit() {
            self.read_char();
        }
        self.input[start_position..self.curr_position].to_string()
    }

    /// Returns the next token from the input stream.
    ///
    /// This method processes the current character and returns the appropriate token.
    /// It handles whitespace, identifiers, numbers, and various operators/delimiters.
    /// The lexer position is advanced as tokens are consumed.
    ///
    /// # Returns
    /// A Token representing the next lexical element in the input.
    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();

        let token = match self.curr_char {
            '=' => Token::new(TokenType::ASSIGN, self.curr_char.to_string()),
            '+' => Token::new(TokenType::PLUS, self.curr_char.to_string()),
            ',' => Token::new(TokenType::COMMA, self.curr_char.to_string()),
            ';' => Token::new(TokenType::SEMICOLON, self.curr_char.to_string()),
            '(' => Token::new(TokenType::LPAREN, self.curr_char.to_string()),
            ')' => Token::new(TokenType::RPAREN, self.curr_char.to_string()),
            '{' => Token::new(TokenType::LBRACE, self.curr_char.to_string()),
            '}' => Token::new(TokenType::RBRACE, self.curr_char.to_string()),
            '\0' => Token::new(TokenType::EOF, "".to_string()),
            _ => {
                if self.curr_char.is_ascii_alphabetic() || self.curr_char == '_' {
                    let literal = self.read_identifier();
                    let token_type = lookup_identifier(&literal);
                    Token::new(token_type, literal)
                } else if self.curr_char.is_ascii_digit() {
                    let literal = self.read_number();
                    Token::new(TokenType::INT, literal)
                } else {
                    Token::new(TokenType::ILLEGAL, self.curr_char.to_string())
                }
            }
        };

        self.read_char();
        token
    }
}
