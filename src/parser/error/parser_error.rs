//! Parser error types and error collection.

use super::span::Span;
use crate::{
    lexer::token::{Token, TokenType},
    parser::error::ParserErrorType,
};
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

    // ============================================================================
    // Convenience constructors for common errors
    // ============================================================================

    /// Expected token error.
    pub fn expected_token(expected: TokenType, token: &Token) -> Self {
        Self::at_token(
            ParserErrorType::ExpectedToken {
                expected,
                got: token.token_type,
                literal: token.literal.clone(),
            },
            token,
        )
    }

    /// Missing semicolon error.
    pub fn missing_semicolon(token: &Token) -> Self {
        Self::at_token(
            ParserErrorType::MissingSemicolon {
                got: token.token_type,
                literal: token.literal.clone(),
            },
            token,
        )
    }

    /// No prefix parse function error.
    pub fn no_prefix_fn(token: &Token) -> Self {
        Self::at_token(
            ParserErrorType::NoPrefixParseFunction {
                token_type: token.token_type,
                literal: token.literal.clone(),
            },
            token,
        )
    }

    /// Invalid integer literal error.
    pub fn invalid_integer(token: &Token) -> Self {
        Self::at_token(
            ParserErrorType::InvalidIntegerLiteral {
                literal: token.literal.clone(),
            },
            token,
        )
    }

    /// Failed to parse prefix RHS.
    pub fn prefix_rhs_failed(operator: &str, token: &Token) -> Self {
        Self::at_token(
            ParserErrorType::FailedToParsePrefixRHS {
                operator: operator.to_string(),
            },
            token,
        )
    }

    /// Failed to parse infix RHS.
    pub fn infix_rhs_failed(operator: &str, token: &Token) -> Self {
        Self::at_token(
            ParserErrorType::FailedToParseInfixRHS {
                operator: operator.to_string(),
            },
            token,
        )
    }

    /// Expected parameter identifier.
    pub fn expected_param_ident(token: &Token) -> Self {
        Self::at_token(
            ParserErrorType::ExpectedParameterIdentifier {
                got: token.token_type,
                literal: token.literal.clone(),
            },
            token,
        )
    }

    /// Unclosed call arguments.
    pub fn unclosed_call(token: &Token) -> Self {
        Self::at_token(
            ParserErrorType::UnclosedCallArguments {
                got: token.token_type,
                literal: token.literal.clone(),
            },
            token,
        )
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.span, self.kind)
    }
}

impl std::error::Error for ParserError {}

/// Collection of parser errors with helper methods.
#[derive(Debug, Clone, Default)]
pub struct ParserErrors {
    errors: Vec<ParserError>,
}

impl ParserErrors {
    /// Create a new empty error collection.
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Push an error into the collection.
    pub fn push(&mut self, error: ParserError) {
        self.errors.push(error);
    }

    /// Add error at token position.
    pub fn add(&mut self, kind: ParserErrorType, token: &Token) {
        self.errors.push(ParserError::at_token(kind, token));
    }

    /// Add error at specific position.
    pub fn add_at(&mut self, kind: ParserErrorType, line: usize, column: usize) {
        self.errors.push(ParserError::at(kind, line, column));
    }

    /// Check if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the number of errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Get an iterator over errors.
    pub fn iter(&self) -> impl Iterator<Item = &ParserError> {
        self.errors.iter()
    }

    /// Get all errors as formatted strings (for backwards compatibility).
    pub fn messages(&self) -> Vec<String> {
        self.errors.iter().map(|e| e.to_string()).collect()
    }

    /// Clear all errors.
    pub fn clear(&mut self) {
        self.errors.clear();
    }
}

impl IntoIterator for ParserErrors {
    type Item = ParserError;
    type IntoIter = std::vec::IntoIter<ParserError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<'a> IntoIterator for &'a ParserErrors {
    type Item = &'a ParserError;
    type IntoIter = std::slice::Iter<'a, ParserError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.iter()
    }
}

impl fmt::Display for ParserErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, error) in self.errors.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", error)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_parser_errors_collection() {
        let mut errors = ParserErrors::new();
        assert!(errors.is_empty());

        errors.add_at(ParserErrorType::UnexpectedEOF, 5, 1);
        assert_eq!(errors.len(), 1);

        let messages = errors.messages();
        assert_eq!(messages[0], "[line 5:1] unexpected end of file");
    }
}
