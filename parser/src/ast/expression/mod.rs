//! Expression types in the Monkey language AST.
//!
//! Expressions represent values and computations that evaluate to a value.
//! Each variant wraps a specific expression type.

use super::Node;

pub mod identifier;

// Re-export for convenience
pub use identifier::Identifier;

/// Enum representing all expression types in the AST.
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(identifier::Identifier),
}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier(ident) => ident.token_literal(),
        }
    }
}
