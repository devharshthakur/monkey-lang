//! Abstract Syntax Tree (AST) types and traits for the Monkey language parser.
//!
//! This module defines the building blocks of the AST and their common
//! behavior:
//!
//! It also provides blanket/utility implementations to enable cloning of boxed
//! trait objects and to render nodes for debugging and tests.
//! Contributors should extend these definitions when adding new language forms.

use crate::ast::statement::Statement;
pub mod expression;
pub mod statement;

pub trait Node {
    /// Returns the literal string representation of the token that
    /// this node represents.
    fn token_literal(&self) -> &str;
}
/// The root node of the AST, containing all top-level statements.
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            ""
        }
    }
}
