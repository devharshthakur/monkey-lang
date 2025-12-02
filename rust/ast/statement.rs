//! Statement types in the Monkey language AST.
//!
//! Statements represent actions or declarations in the program.
//! All statement types are consolidated in this module.

use crate::ast::{
    expression::{Expression, Identifier},
    Node,
};
use crate::lexer::token::Token;
use std::fmt::{Display, Formatter, Result};

// ============ STRUCTS ============

/// Represents a `let` statement in the Monkey language AST.
#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}

/// Represents a `return` statement in the Monkey language AST.
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<Expression>,
}

/// Represents an expression statement (standalone expression).
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub value: Expression,
}

// ============ ENUM ============

/// Enum representing all statement types in the AST.
#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

// ============ TRAIT IMPLEMENTATIONS ============

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ", self.token_literal())?;
        write!(f, "{}", self.name)?;
        write!(f, " = ")?;
        if let Some(value) = &self.value {
            write!(f, "{}", value)?;
        }
        write!(f, ";")
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ", self.token_literal())?;
        if let Some(value) = &self.value {
            write!(f, "{}", value)?;
        }
        write!(f, ";")
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Statement::Let(stmt) => write!(f, "{}", stmt),
            Statement::Return(stmt) => write!(f, "{}", stmt),
            Statement::Expression(stmt) => write!(f, "{}", stmt),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expression::{Identifier, IntegerLiteral};
    use crate::lexer::token::{Token, TokenType};

    #[test]
    fn test_let_statement_display_with_value() {
        let token = Token::new(TokenType::LET, "let".to_string(), 1, 1);
        let name = Identifier {
            token: Token::new(TokenType::IDENT, "x".to_string(), 1, 5),
            value: "x".to_string(),
        };
        let value_expr = IntegerLiteral {
            token: Token::new(TokenType::INT, "5".to_string(), 1, 9),
            value: 5,
        };
        let value = Expression::IntegerLiteral(value_expr);
        let stmt = LetStatement {
            token,
            name,
            value: Some(value),
        };

        assert_eq!(format!("{}", stmt), "let x = 5;");
    }

    #[test]
    fn test_let_statement_display_without_value() {
        let token = Token::new(TokenType::LET, "let".to_string(), 1, 1);
        let name = Identifier {
            token: Token::new(TokenType::IDENT, "y".to_string(), 1, 5),
            value: "y".to_string(),
        };
        let stmt = LetStatement {
            token,
            name,
            value: None,
        };

        assert_eq!(format!("{}", stmt), "let y = ;");
    }

    #[test]
    fn test_return_statement_display_with_value() {
        let token = Token::new(TokenType::RETURN, "return".to_string(), 1, 1);
        let value_expr = IntegerLiteral {
            token: Token::new(TokenType::INT, "5".to_string(), 1, 8),
            value: 5,
        };
        let value = Expression::IntegerLiteral(value_expr);
        let stmt = ReturnStatement {
            token,
            value: Some(value),
        };

        assert_eq!(format!("{}", stmt), "return 5;");
    }

    #[test]
    fn test_return_statement_display_without_value() {
        let token = Token::new(TokenType::RETURN, "return".to_string(), 1, 1);
        let stmt = ReturnStatement { token, value: None };
        assert_eq!(format!("{}", stmt), "return ;");
    }
}
