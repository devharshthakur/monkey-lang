//! Statement types in the Monkey language AST.
//!
//! Statements represent actions or declarations in the program.
//! Each variant wraps a specific statement type.

use crate::Node;
use std::fmt::{self, Display};
pub mod expr;
pub mod let_;
pub mod return_;

/// Enum representing all statement types in the AST.
#[derive(Debug, Clone)]
pub enum Statement {
    Let(let_::LetStatement),
    Return(return_::ReturnStatement),
    Expression(expr::ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Statement::Let(stmt) => stmt.token_literal(),
            Statement::Return(stmt) => stmt.token_literal(),
            Statement::Expression(stmt) => stmt.token_literal(),
        }
    }
}

impl Display for Statement {
    /// Formats the statement as a string by delegating to the specific statement type.
    ///
    /// # Example
    /// ```rust
    /// # use ast::statement::Statement;
    /// // LetStatement: "let x = 5;"
    /// // ReturnStatement: "return 10;"
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(stmt) => write!(f, "{}", stmt),
            Statement::Return(stmt) => write!(f, "{}", stmt),
            Statement::Expression(stmt) => write!(f, "{}", stmt),
        }
    }
}
