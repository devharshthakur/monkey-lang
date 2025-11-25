//! Literal expression types in the Monkey language AST.
//!
//! Literals represent constant values in the source code, such as integers,
//! strings, booleans, etc. Each literal type stores both the token (for
//! position information) and the parsed value.

pub mod boolean;
pub mod integer;
pub use boolean::BooleanLiteral;
pub use integer::IntegerLiteral;

use crate::{expressions::Identifier, Node};
use lexer::token::{Token, TokenType};
use std::fmt::{Display, Formatter, Result};

/// Enum representing all literal types in the AST.
///
/// This enum provides type-safe representation of all possible literals
/// in the Monkey language. Each variant wraps a specific literal type,
/// allowing for pattern matching and type-specific operations.
///
/// # Example
/// ```rust
/// # use ast::literals::{Literal, IntegerLiteral};
/// # use lexer::token::{Token, TokenType};
/// // Create an integer literal
/// let int_lit = IntegerLiteral {
///     token: Token::new(TokenType::INT, "42".to_string()),
///     value: 42,
/// };
/// let literal = Literal::Integer(int_lit);
/// ```
#[derive(Debug, Clone)]
pub enum Literal {
    /// An integer literal (e.g., `42`, `-10`)
    Integer(IntegerLiteral),
    /// An identifier literal (e.g., `x`, `foobar`)
    Identifier(Identifier),
    /// A boolean literal (e.g., `true`, `false`)
    Boolean(BooleanLiteral),
}

impl Node for Literal {
    fn token_literal(&self) -> &str {
        match self {
            Literal::Integer(il) => il.token_literal(),
            Literal::Identifier(ident) => ident.token_literal(),
            Literal::Boolean(bl) => bl.token_literal(),
        }
    }
}

impl Display for Literal {
    /// Formats the literal as its string representation.
    ///
    /// Delegates to the specific literal type's Display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Literal::Integer(il) => write!(f, "{}", il),
            Literal::Identifier(ident) => write!(f, "{}", ident),
            Literal::Boolean(bl) => write!(f, "{}", bl),
        }
    }
}

impl From<i64> for Literal {
    fn from(value: i64) -> Self {
        let literal_str = value.to_string();
        Literal::Integer(IntegerLiteral {
            token: Token::new(TokenType::INT, literal_str.clone()),
            value,
        })
    }
}

impl From<i32> for Literal {
    fn from(value: i32) -> Self {
        Literal::from(value as i64)
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Literal::Identifier(Identifier {
            token: Token::new(TokenType::IDENT, value.to_string()),
            value: value.to_string(),
        })
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Literal::Identifier(Identifier {
            token: Token::new(TokenType::IDENT, value.clone()),
            value,
        })
    }
}
