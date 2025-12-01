//! Abstract Syntax Tree (AST) types and traits for the Monkey language parser.
//!
//! This module defines the building blocks of the AST and their common
//! behavior.

use std::fmt::{Display, Formatter, Result};

pub mod expression;
pub mod statement;

pub use expression::{
    BooleanLiteral, Expression, Identifier, InfixExpression, IntegerLiteral, PrefixExpression,
};
pub use statement::{ExpressionStatement, LetStatement, ReturnStatement, Statement};

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

impl Display for Program {
    /// Formats the program as a string by concatenating all statements.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}
