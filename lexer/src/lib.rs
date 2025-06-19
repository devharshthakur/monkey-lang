use crate::token::{lookup_ident, Token, TokenType};
pub mod token;

pub struct Lexer {
    input: String,
    curr_position: usize,
    curr_read_position: usize,
    curr_char: char, // We currently supports ASCII character only
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            curr_position: 0,
            curr_read_position: 0,
            curr_char: '\0', // \0 => Null in ASCII
        };
        l
    }

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

    fn next_token(token_type: TokenType, ch: char) -> Token {
        Token {
            token_type,
            literal: ch.to_string(),
        }
    }
}
