use crate::expression::Expression;
use crate::Node;
use lexer::token::Token;
use std::fmt::{Display, Formatter, Result};

/// Represents a `return` statement in the Monkey language AST.
///
/// A `return` statement terminates the execution of a function and returns a value to the caller.
/// The structure preserves the original token and the optional return value expression.
///
/// # Example
/// For source code:
/// ```monkey
/// return 5;
/// ```
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<Expression>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for ReturnStatement {
    /// Formats the return statement as `return <value>;`.
    ///
    /// If the value is `None`, only the keyword and semicolon are shown.
    ///
    /// # Example
    /// ```rust
    /// # use ast::statement::return_::ReturnStatement;
    /// // For `return 5;` → outputs: "return 5;"
    /// // For `return;` (no value) → outputs: "return ;"
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ", self.token_literal())?;
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
    fn test_return_statement_display_with_value() {
        use crate::expression::literals::IntegerLiteral;
        let token = Token::new(TokenType::RETURN, "return".to_string());
        let value_expr = IntegerLiteral {
            token: Token::new(TokenType::INT, "5".to_string()),
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
        let token = Token::new(TokenType::RETURN, "return".to_string());
        let stmt = ReturnStatement { token, value: None };
        assert_eq!(format!("{}", stmt), "return ;");
    }

    #[test]
    fn test_return_statement_display_with_expression() {
        let token = Token::new(TokenType::RETURN, "return".to_string());
        let value_expr = Identifier {
            token: Token::new(TokenType::IDENT, "x".to_string()),
            value: "x".to_string(),
        };
        let value = Expression::Identifier(value_expr);
        let stmt = ReturnStatement {
            token,
            value: Some(value),
        };

        assert_eq!(format!("{}", stmt), "return x;");
    }
}
