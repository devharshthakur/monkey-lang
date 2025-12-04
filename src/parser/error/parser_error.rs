//! Parser error types and error collection.

use super::span::Span;
use crate::{lexer::token::Token, parser::error::ParserErrorType};
use std::fmt;

/// A parser error with location and kind.
#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub kind: ParserErrorType,
    pub span: Span,
}

impl ParserError {
    /// Create a new parser error.
    pub fn new(kind: ParserErrorType, span: Span) -> Self {
        Self { kind, span }
    }

    /// Create error from token position.
    pub fn at_token(kind: ParserErrorType, token: &Token) -> Self {
        Self {
            kind,
            span: Span::from_token(token),
        }
    }

    /// Create error at specific position.
    pub fn at(kind: ParserErrorType, line: usize, column: usize) -> Self {
        Self {
            kind,
            span: Span::new(line, column),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.span, self.kind)
    }
}

impl std::error::Error for ParserError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::TokenType;

    #[test]
    fn test_error_display() {
        let error = ParserError::at(
            ParserErrorType::ExpectedToken {
                expected: TokenType::SEMICOLON,
                got: TokenType::RBRACE,
                literal: "}".to_string(),
            },
            1,
            10,
        );
        assert_eq!(
            error.to_string(),
            "[line 1:10] expected SEMICOLON, got RBRACE ('}')"
        );
    }
}
