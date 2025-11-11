//! Statement types in the Monkey language AST.
//!
//! Statements represent actions or declarations in the program.
//! Each variant wraps a specific statement type.

use super::Node;

pub mod let_statement;

// Re-export for convenience
pub use let_statement::LetStatement;

/// Enum representing all statement types in the AST.
#[derive(Debug, Clone)]
pub enum Statement {
    Let(let_statement::LetStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Statement::Let(stmt) => stmt.token_literal(),
        }
    }
}
