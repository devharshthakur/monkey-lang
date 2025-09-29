//! Abstract Syntax Tree (AST) types and traits for the Monkey language parser.
//!
//! This module defines the building blocks of the AST and their common
//! behavior:
//!
//! It also provides blanket/utility implementations to enable cloning of boxed
//! trait objects and to render nodes for debugging and tests.
//! Contributors should extend these definitions when adding new language forms.

use lexer::token::Token;
use std::any::Any;

/// Trait for converting types to `Any` for runtime type checking.
/// This allows downcasting trait objects to concrete types.
pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// `Node` is the basic building block of our Abstract Syntax Tree.
/// Every node in the AST must implement this trait.
///
/// This trait provides common functionality for all AST nodes including:
/// - Token literal representation
/// - String representation for debugging/display
/// - Cloning capability
pub trait Node: std::fmt::Debug + Any {
    /// Returns the literal string of the token that represents this node
    fn token_literal(&self) -> String;

    /// Returns a string representation of the node for debugging and display
    fn string(&self) -> String {
        self.token_literal()
    }

    /// Creates a boxed clone of this node
    fn clone_box(&self) -> Box<dyn Node>;
}

/// Represents a statement in the AST.
/// Statements are the top-level constructs in a program that perform actions
/// but don't produce values (unlike expressions).
pub trait Statement: Node + AsAny {
    /// Marks this node as a statement node.
    /// This is a marker method used for type safety and clarity.
    fn statement_node(&self);

    /// Creates a boxed clone of this statement
    fn clone_box(&self) -> Box<dyn Statement>;
}

/// Represents an expression in the AST.
/// Expressions are constructs that produce values and can be evaluated.
/// Unlike statements, expressions can be used in contexts where a value is expected.
pub trait Expression: Node {
    /// Marks this node as an expression node.
    /// This is a marker method used for type safety and clarity.
    fn expression_node(&self);

    /// Creates a boxed clone of this expression
    fn clone_box(&self) -> Box<dyn Expression>;
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>, // We use `Option` because we will skip parsing the actual expression for now.
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,                            // The first token of expression
    pub expression: Option<Box<dyn Expression>>, // The actual expression
}

// Implementations

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Box<dyn Statement> {
        Statement::clone_box(&**self)
    }
}

impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Self {
        Expression::clone_box(&**self)
    }
}

// Blanket implementation for any type that implements Any
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
    /// Provides a string representation of the entire program (concatination of statement strings)
    fn string(&self) -> String {
        self.statements
            .iter()
            .map(|s| s.string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.token_literal());
        s.push_str(" ");
        s.push_str(&self.name.string());
        s.push_str(" = ");
        if let Some(value_expr) = &self.value {
            s.push_str(&value_expr.string());
        }
        s.push_str(";");
        s
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(ReturnStatement {
            token: self.token.clone(),
            return_value: self.return_value.clone(),
        })
    }

    fn string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.token_literal());
        s.push_str(" ");
        if let Some(return_value) = &self.return_value {
            s.push_str(&return_value.string());
        }
        s.push_str(";");
        s
    }
}

impl Statement for ReturnStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ReturnStatement {
            token: self.token.clone(),
            return_value: self.return_value.clone(),
        })
    }
    fn statement_node(&self) {} // This is to statisfy the `Statement` trait for now
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}

    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl Node for ExpressionStatement {
    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn string(&self) -> String {
        self.expression
            .as_ref()
            .map(|exp| exp.string())
            .unwrap_or_else(|| "".to_string())
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
