//! Source location tracking for parser errors.

use crate::lexer::token::Token;
use std::fmt;

/// Source location for error reporting.
///
/// Represents where an error occurred in the source code,
/// specifically the line and column position.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    /// Create a new span at the given line and column.
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Create a span from a token's position.
    pub fn from_token(token: &Token) -> Self {
        Self {
            line: token.line,
            column: token.column,
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}:{}]", self.line, self.column)
    }
}
