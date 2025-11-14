use crate::{expression::Expression, Node};
use lexer::token::Token;

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
