use crate::{expression::Expression, Node};
use lexer::token::Token;
use std::fmt::{Display, Formatter, Result};

/// Represents a prefix expression in the Monkey language AST.
///
/// A prefix expression consists of a prefix operator (like `!` or `-`) followed
/// by an expression. Examples include `!true`, `-5`, `!x`.
///
/// # Example
/// For source code:
/// ```monkey
/// !true
/// ```
/// or
/// ```monkey
/// -5
/// ```
#[derive(Debug, Clone)]
pub struct PrefixExpression {
    /// The token representing the prefix operator (e.g., BANG, MINUS)
    pub token: Token,
    /// The string representation of the operator (e.g., "!", "-")
    pub operator: String,
    /// The expression that the operator is applied to
    pub right: Box<Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for PrefixExpression {
    /// Formats the prefix expression as `(operator right)`.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}{})", self.operator, self.right)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expression::Identifier, literals::integer::IntegerLiteral};
    use lexer::token::{Token, TokenType};

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
}
