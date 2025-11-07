//! Abstract Syntax Tree (AST) types and traits for the Monkey language parser.
//!
//! This module defines the building blocks of the AST and their common
//! behavior:
//!
//! It also provides blanket/utility implementations to enable cloning of boxed
//! trait objects and to render nodes for debugging and tests.
//! Contributors should extend these definitions when adding new language forms.

use lexer::token::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
}

/// Enum representing all statement types in the AST.
#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Statement::Let(stmt) => stmt.token_literal(),
        }
    }
}

/// Enum representing all expression types in the AST.
#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier(ident) => ident.token_literal(),
        }
    }
}

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

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}
