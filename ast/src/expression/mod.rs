//! Expression types in the Monkey language AST.
//!
//! Expressions represent values and computations that evaluate to a value.
//! Each variant wraps a specific expression type.

use crate::{literals::integer::IntegerLiteral, Node};
use std::fmt::{Display, Formatter, Result};

pub mod identifier;
pub use identifier::Identifier;

pub mod prefix;
pub use prefix::PrefixExpression;

/// Enum representing all expression types in the AST.
///
/// This enum provides type-safe representation of all possible expressions
/// in the Monkey language. Each variant wraps a specific expression type,
/// allowing for pattern matching and type-specific operations.
///
/// # Example
/// ```rust
/// # use ast::expression::{Expression, Identifier};
/// # use lexer::token::{Token, TokenType};
/// // Create an identifier expression
/// let ident = Identifier
///     token: Token::new(TokenType::IDENT, "x".to_string()),
///     value: "x".to_string(),
/// };
/// let expr = Expression::Identifier(ident);
/// ```
#[derive(Debug, Clone)]
pub enum Expression {
    /// An identifier expression (variable name, function name, etc.)
    Identifier(Identifier),
    /// An integer literal expression (e.g., `42`, `-10`)
    IntegerLiteral(IntegerLiteral),
    /// A prefix expression (e.g., `!true`, `-5`)
    PrefixExpression(PrefixExpression),
}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier(ident) => ident.token_literal(),
            Expression::IntegerLiteral(il) => il.token_literal(),
            Expression::PrefixExpression(pe) => pe.token_literal(),
        }
    }
}

impl Display for Expression {
    /// Formats the expression as its string representation.
    ///
    /// Delegates to the specific expression type's Display implementation.
    ///
    /// # Example
    /// ```rust
    /// # use ast::expression::{Expression, Identifier};
    /// # use lexer::token::{Token, TokenType};
    /// let ident = Identifier {
    ///     token: Token::new(TokenType::IDENT, "x".to_string()),
    ///     value: "x".to_string(),
    /// };
    /// let expr = Expression::Identifier(ident);
    /// assert_eq!(format!("{}", expr), "x");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::IntegerLiteral(il) => write!(f, "{}", il),
            Expression::PrefixExpression(pe) => write!(f, "{}", pe),
        }
    }
}
