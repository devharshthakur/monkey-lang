//! Expression statement node.
//!
//! Represents a standalone expression as a statement: `<expression>;`

use crate::{expressions::Expression, Node};
use lexer::token::Token;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub value: Expression,
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
