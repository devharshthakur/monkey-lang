//! Expression types in the Monkey language AST.
//!
//! Expressions represent values and computations that evaluate to a value.
//! All expression types are consolidated in this module.

use crate::ast::{Node, Statement};
use crate::lexer::token::Token;
use std::fmt::{Display, Formatter, Result};

// ============ ENUM ============

/// Enum representing all expression types in the AST.
#[derive(Debug, Clone)]
pub enum Expression {
    /// An identifier expression (variable name, function name, etc.)
    Identifier(Identifier),
    /// An integer literal expression (e.g., `42`, `-10`)
    IntegerLiteral(IntegerLiteral),
    /// A boolean literal expression (e.g., `true`, `false`)
    BooleanLiteral(BooleanLiteral),
    /// A prefix expression (e.g., `!true`, `-5`)
    PrefixExpression(PrefixExpression),
    /// An infix expression (e.g., `5 + 3`, `x == y`)
    InfixExpression(InfixExpression),
    /// An if expression (e.g., `if (x < y) { x } else { y }`)
    IfExpression(IfExpression),
    /// A block statement (e.g., `{ <statements> }`)
    BlockStatement(BlockStatement),
    /// A function literal expression (e.g., `fn(x, y) { x + y }`)
    FunctionLiteral(FunctionLiteral),
}

// ============ STRUCTS ============

/// Represents an identifier expression in the Monkey language AST.
///
/// An identifier is a name that refers to a variable, function, or other named entity.
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

/// Represents an integer literal expression in the Monkey language AST.
#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

/// Represents a boolean literal expression in the Monkey language AST.
/// Boolean literal: true, false
#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

/// Represents a prefix expression (e.g., `!true`, `-5`).
#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<Expression>,
}

/// Represents an infix expression (e.g., `5 + 3`, `x == y`).
/// Infix expression: <left> <operator> <right>
#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

/// Represents an if expression in the Monkey language AST. Every if expression has a condition, a consequence, and an optional alternative.
/// if expression: if (<condition>) <consequence> else <alternative>
#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: Box<Expression>,
    pub alternative: Option<Box<Expression>>, // optional ie. else block statement is optional
}

/// Represents a block statement in the Monkey language AST. A block statement is a list of statements enclosed in curly braces.
/// block statement: { <statements> }
#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

/// Represents a function literal expression in the Monkey language AST.
/// The format of a function literal is: fn(<parameters>) <body>
#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

// ============ TRAIT IMPLEMENTATIONS ============

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.value)
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "fn(")?;
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param)?;
        }
        write!(f, ") {}", self.body)
    }
}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier(ident) => ident.token_literal(),
            Expression::IntegerLiteral(il) => il.token_literal(),
            Expression::BooleanLiteral(bl) => bl.token_literal(),
            Expression::PrefixExpression(pe) => pe.token_literal(),
            Expression::InfixExpression(infe) => infe.token_literal(),
            Expression::IfExpression(ife) => ife.token_literal(),
            Expression::BlockStatement(bs) => bs.token_literal(),
            Expression::FunctionLiteral(fl) => fl.token_literal(),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident),
            Expression::IntegerLiteral(il) => write!(f, "{}", il),
            Expression::BooleanLiteral(bl) => write!(f, "{}", bl),
            Expression::PrefixExpression(pe) => write!(f, "{}", pe),
            Expression::InfixExpression(ie) => write!(f, "{}", ie),
            Expression::IfExpression(ife) => write!(f, "{}", ife),
            Expression::BlockStatement(bs) => write!(f, "{}", bs),
            Expression::FunctionLiteral(fl) => write!(f, "{}", fl),
        }
    }
}

impl Node for IfExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{")?;
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "if")?;
        write!(f, "{}", self.condition)?;
        write!(f, "{}", self.consequence)?;
        if let Some(alternative) = self.alternative.as_ref() {
            write!(f, "else {}", alternative)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Token, TokenType};

    #[test]
    fn test_identifier_display() {
        let ident = Identifier {
            token: Token::new(TokenType::IDENT, "foobar".to_string()),
            value: "foobar".to_string(),
        };

        assert_eq!(ident.value, "foobar");
        assert_eq!(ident.token_literal(), "foobar");
        assert_eq!(format!("{}", ident), "foobar");
    }

    #[test]
    fn test_prefix_expression_display_bang() {
        let ident = Identifier {
            token: Token::new(TokenType::IDENT, "foobar".to_string()),
            value: "foobar".to_string(),
        };
        let prefix = PrefixExpression {
            token: Token::new(TokenType::BANG, "!".to_string()),
            operator: "!".to_string(),
            right: Box::new(Expression::Identifier(ident)),
        };

        assert_eq!(prefix.token_literal(), "!");
        assert_eq!(format!("{}", prefix), "(!foobar)");
    }

    #[test]
    fn test_prefix_expression_display_minus() {
        let int_lit = IntegerLiteral {
            token: Token::new(TokenType::INT, "5".to_string()),
            value: 5,
        };
        let prefix = PrefixExpression {
            token: Token::new(TokenType::MINUS, "-".to_string()),
            operator: "-".to_string(),
            right: Box::new(Expression::IntegerLiteral(int_lit)),
        };

        assert_eq!(prefix.token_literal(), "-");
        assert_eq!(format!("{}", prefix), "(-5)");
    }

    #[test]
    fn test_infix_expression_display_plus() {
        let left = IntegerLiteral {
            token: Token::new(TokenType::INT, "5".to_string()),
            value: 5,
        };
        let right = IntegerLiteral {
            token: Token::new(TokenType::INT, "3".to_string()),
            value: 3,
        };
        let infix = InfixExpression {
            token: Token::new(TokenType::PLUS, "+".to_string()),
            left: Box::new(Expression::IntegerLiteral(left)),
            operator: "+".to_string(),
            right: Box::new(Expression::IntegerLiteral(right)),
        };

        assert_eq!(infix.token_literal(), "+");
        assert_eq!(format!("{}", infix), "(5 + 3)");
    }

    #[test]
    fn test_infix_expression_display_eq() {
        let left = Identifier {
            token: Token::new(TokenType::IDENT, "x".to_string()),
            value: "x".to_string(),
        };
        let right = Identifier {
            token: Token::new(TokenType::IDENT, "y".to_string()),
            value: "y".to_string(),
        };
        let infix = InfixExpression {
            token: Token::new(TokenType::EQ, "==".to_string()),
            left: Box::new(Expression::Identifier(left)),
            operator: "==".to_string(),
            right: Box::new(Expression::Identifier(right)),
        };

        assert_eq!(infix.token_literal(), "==");
        assert_eq!(format!("{}", infix), "(x == y)");
    }
}
