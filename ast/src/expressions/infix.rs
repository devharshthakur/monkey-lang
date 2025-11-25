use crate::{expressions::Expression, Node};
use lexer::token::Token;
use std::fmt::{Display, Formatter, Result};

/// Represents an infix expression in the Monkey language AST.
///
/// An infix expression consists of a left expression, an infix operator (like `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`),
/// and a right expression. Examples include `5 + 3`, `x == y`, `a < b`.
///
/// # Example
/// For source code:
/// ```monkey
/// 5 + 3
/// ```
/// or
/// ```monkey
/// x == y
/// ```
#[derive(Debug, Clone)]
pub struct InfixExpression {
    /// The token representing the infix operator (e.g., PLUS, MINUS, EQ, NOT_EQ)
    pub token: Token,
    /// The left-hand side expression
    pub left: Box<Expression>,
    /// The string representation of the operator (e.g., "+", "-", "==", "!=")
    pub operator: String,
    /// The right-hand side expression
    pub right: Box<Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for InfixExpression {
    /// Formats the infix expression as `(left operator right)`.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expressions::Identifier, literals::integer::IntegerLiteral};
    use lexer::token::{Token, TokenType};

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
    fn test_infix_expression_display_minus() {
        let left = IntegerLiteral {
            token: Token::new(TokenType::INT, "10".to_string()),
            value: 10,
        };
        let right = IntegerLiteral {
            token: Token::new(TokenType::INT, "2".to_string()),
            value: 2,
        };
        let infix = InfixExpression {
            token: Token::new(TokenType::MINUS, "-".to_string()),
            left: Box::new(Expression::IntegerLiteral(left)),
            operator: "-".to_string(),
            right: Box::new(Expression::IntegerLiteral(right)),
        };

        assert_eq!(infix.token_literal(), "-");
        assert_eq!(format!("{}", infix), "(10 - 2)");
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
