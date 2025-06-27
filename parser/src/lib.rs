use crate::ast::AsAny;
use crate::ast::Expression;
use crate::ast::Identifier;
use crate::ast::LetStatement;
use crate::ast::Program;
use crate::ast::Statement;
use lexer::token::Token;
use lexer::token::TokenType;
use lexer::Lexer;

pub mod ast;

pub struct Parser {
    l: Lexer,
    curr_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Parser {
            l,
            curr_token: Token::new(TokenType::EOF, "".to_string()),
            peek_token: Token::new(TokenType::EOF, "".to_string()),
            errors: Vec::new(),
        };
    }
}
