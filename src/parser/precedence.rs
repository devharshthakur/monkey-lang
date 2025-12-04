//! Operator precedence levels for parsing expressions.
//!
//! This module defines the precedence levels used in the Pratt parser
//! to correctly parse expressions with proper operator precedence.
//! Higher values indicate higher precedence.

use crate::lexer::token::TokenType;

/// Operator precedence levels for parsing expressions.
/// Higher values indicate higher precedence.
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    /// Lowest precedence (used as default)
    LOWEST = 1,
    /// Equality operators: ==, !=
    EQUALS = 2,
    /// Comparison operators: >, <
    LESSGREATER = 3,
    /// Addition and subtraction: +, -
    SUM = 4,
    /// Multiplication and division: *, /
    PRODUCT = 5,
    /// Prefix operators: -X, !X
    PREFIX = 6,
    /// Function calls: myFunction(X)
    CALL = 7,
}

impl Precedence {
    /// Returns the precedence level for a given token type as an i32.
    pub const fn from_token_type(token_type: &TokenType) -> i32 {
        match token_type {
            TokenType::EQ => Precedence::EQUALS as i32,
            TokenType::NOTEQ => Precedence::EQUALS as i32, // same precedence as EQUALS
            TokenType::LT => Precedence::LESSGREATER as i32,
            TokenType::GT => Precedence::LESSGREATER as i32,
            TokenType::PLUS => Precedence::SUM as i32,
            TokenType::MINUS => Precedence::SUM as i32,
            TokenType::SLASH => Precedence::PRODUCT as i32,
            TokenType::ASTERISK => Precedence::PRODUCT as i32,
            TokenType::LPAREN => Precedence::CALL as i32,
            _ => Precedence::LOWEST as i32,
        }
    }
}
