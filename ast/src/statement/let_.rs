//! Let statement node.
//!
//! Represents a variable declaration: `let <identifier> = <expression>;`

use crate::expression::{Expression, Identifier};
use crate::Node;
use lexer::token::Token;
use std::fmt::{Display, Formatter, Result};

/// Represents a `let` statement in the Monkey language AST.
///
/// A `let` statement binds an identifier to a value (expression).
/// The structure preserves the original token, the identifier being declared,
/// and the right-hand side expression (if present) assigned to it.
///
/// # Example
/// For source code:
/// ```monkey
/// let myVar = 5;
/// ```
/// The node would contain:
/// - `token`: The `let` token
/// - `name`: Identifier node for `myVar`
/// - `value`: Expression representing `5`
///
/// # Fields
/// - `token`: Token corresponding to the `let` keyword
/// - `name`: The identifier being declared
/// - `value`: The expression assigned to the identifier (optional; may be `None` during parsing)
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

impl Display for LetStatement {
    /// Formats the let statement as `let <name> = <value>;`.
    ///
    /// If the value is `None`, only the identifier is shown.
    ///
    /// # Example
    /// ```rust
    /// # use ast::statement::let_::LetStatement;
    /// // For `let x = 5;` → outputs: "let x = 5;"
    /// // For `let y;` (no value) → outputs: "let y ;"
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Identifier;
    use lexer::token::{Token, TokenType};

    #[test]
    fn test_let_statement_display_with_value() {
        use crate::expression::literals::IntegerLiteral;
        let token = Token::new(TokenType::LET, "let".to_string());
        let name = Identifier {
            token: Token::new(TokenType::IDENT, "x".to_string()),
            value: "x".to_string(),
        };
        let value_expr = IntegerLiteral {
            token: Token::new(TokenType::INT, "5".to_string()),
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
        let token = Token::new(TokenType::LET, "let".to_string());
        let name = Identifier {
            token: Token::new(TokenType::IDENT, "y".to_string()),
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
    fn test_let_statement_display_with_identifier_value() {
        let token = Token::new(TokenType::LET, "let".to_string());
        let name = Identifier {
            token: Token::new(TokenType::IDENT, "myVar".to_string()),
            value: "myVar".to_string(),
        };
        let value_expr = Identifier {
            token: Token::new(TokenType::IDENT, "anotherVar".to_string()),
            value: "anotherVar".to_string(),
        };
        let value = Expression::Identifier(value_expr);
        let stmt = LetStatement {
            token,
            name,
            value: Some(value),
        };

        assert_eq!(format!("{}", stmt), "let myVar = anotherVar;");
    }
}
