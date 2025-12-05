//! Parser error types and handling for the Monkey language.
//!
//! This module provides structured error types that eliminate scattered format! strings
//! and enable proper error handling throughout the parser.

mod span;
use std::fmt;

use crate::lexer::token::Token;
pub use span::Span;

/// A parser error with location and kind.
#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub span: Span,
    pub message: String,
}

impl ParserError {
    /// Create a new parser error with the given message.
    /// # Parameters
    /// - `message`: The error message
    /// # Returns
    /// A new parser error with the given message.
    pub fn new(message: String) -> Self {
        Self {
            span: Span::new(0, 0),
            message,
        }
    }

    /// Create a new parser error at the given span.
    /// # Parameters
    /// - `span`: The span of the error
    /// - `message`: The error message
    /// # Returns
    /// A new parser error at the given span.
    pub fn at(span: Span, message: String) -> Self {
        Self { span, message }
    }

    /// Create a new parser error at the given token.
    /// # Parameters
    /// - `token`: The token at which the error occurred
    /// - `message`: The error message
    /// # Returns
    /// A new parser error at the given token.
    pub fn at_token(token: &Token, message: String) -> Self {
        Self {
            span: Span::from_token(token),
            message,
        }
    }

    /// Create a new parser error at the given line.
    /// # Parameters
    /// - `line`: The line at which the error occurred
    /// - `message`: The error message
    /// # Returns
    /// A new parser error at the given line.
    pub fn at_line(line: usize, message: String) -> Self {
        Self {
            span: Span::new(line, 0),
            message,
        }
    }

    /// Create a new parser error at the given column.
    /// # Parameters
    /// - `column`: The column at which the error occurred
    /// - `message`: The error message
    /// # Returns
    /// A new parser error at the given column.
    pub fn at_column(column: usize, message: String) -> Self {
        Self {
            span: Span::new(0, column),
            message,
        }
    }

    /// Create a new parser error at the given line and column.
    /// # Parameters
    /// - `line`: The line at which the error occurred
    /// - `column`: The column at which the error occurred
    /// - `message`: The error message
    /// # Returns
    /// A new parser error at the given line and column.
    pub fn at_line_column(line: usize, column: usize, message: String) -> Self {
        Self {
            span: Span::new(line, column),
            message,
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}:{}] {}",
            self.span.line, self.span.column, self.message
        )
    }
}
