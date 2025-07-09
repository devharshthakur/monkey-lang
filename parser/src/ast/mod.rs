use lexer::token::Token;
use lexer::token::TokenType;
use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// `Node` is the basic building block of our Abstract Syntax Tree.
/// Every node in the AST must implement this trait.
pub trait Node: std::fmt::Debug + Any {
    fn token_literal(&self) -> String;

    fn string(&self) -> String {
        self.token_literal()
    }

    fn clone_box(&self) -> Box<dyn Node>;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn clone_box(&self) -> Box<dyn Statement>;
}

impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Box<dyn Statement> {
        Statement::clone_box(&**self)
    }
}

pub trait Expression: Node {
    fn expression_node(&self);
    fn clone_box(&self) -> Box<dyn Expression>;
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

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
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

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
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

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
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
