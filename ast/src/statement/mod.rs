//! Statement types in the Monkey language AST.
//!
//! Statements represent actions or declarations in the program.
//! Each variant wraps a specific statement type.

use crate::Node;
pub mod let_stmt;
pub mod return_stmt;

/// Enum representing all statement types in the AST.
#[derive(Debug, Clone)]
pub enum Statement {
    Let(let_stmt::LetStatement),
    Return(return_stmt::ReturnStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Statement::Let(stmt) => stmt.token_literal(),
            Statement::Return(stmt) => stmt.token_literal(),
        }
    }
}
