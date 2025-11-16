use crate::Node;
use lexer::token::Token;
use std::fmt::{Display, Formatter, Result};

/// Represents an integer literal expression in the Monkey language AST.
///
/// An integer literal represents a numeric value in the source code.
/// It stores both the token (for position information) and the parsed integer value.
///
/// # Example
/// For source code:
/// ```monkey
/// 42
/// ```
#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for IntegerLiteral {
    /// Formats the integer literal as its string representation.
    ///
    /// # Example
    /// ```rust
    /// # use ast::expression::literals::IntegerLiteral;
    /// # use lexer::token::{Token, TokenType};
    /// let int_lit = IntegerLiteral {
    ///     token: Token::new(TokenType::INT, "42".to_string()),
    ///     value: 42,
    /// };
    /// assert_eq!(format!("{}", int_lit), "42");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}
