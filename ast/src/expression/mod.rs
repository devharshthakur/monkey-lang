//! Expression types in the Monkey language AST.
//!
//! Expressions represent values and computations that evaluate to a value.
//! Each variant wraps a specific expression type.

use crate::Node;
use lexer::token::Token;
use std::fmt::{Display, Formatter, Result};

pub mod identifier;
pub use identifier::Identifier;

/// Enum representing all expression types in the AST.
#[derive(Debug, Clone)]
pub struct Expression {
    pub token: Token,
    pub value: String,
}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for Expression {
    /// Formats the expression as its string value representation.
    ///
    /// # Example
    /// ```rust
    /// # use ast::expression::Expression;
    /// // For expression `5` → outputs: "5"
    /// // For expression `x + y` → outputs: "x + y"
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}
