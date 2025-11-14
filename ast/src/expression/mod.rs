//! Expression types in the Monkey language AST.
//!
//! Expressions represent values and computations that evaluate to a value.
//! Each variant wraps a specific expression type.

use lexer::token::Token;

pub mod identifier;

pub use identifier::Identifier;

/// Enum representing all expression types in the AST.
#[derive(Debug, Clone)]
pub struct Expression {
    pub token: Token,
    pub value: String,
}
