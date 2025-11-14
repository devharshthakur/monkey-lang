use crate::{expression::Expression, Node};
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
    /// # use ast::statement::return_stmt::ReturnStatement;
    /// // For `return 5;` → outputs: "return 5;"
    /// // For `return;` (no value) → outputs: "return ;"
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ", self.token_literal())?;
        if let Some(ref value) = self.value {
            write!(f, "{}", value)?;
        }
        write!(f, ";")
    }
}
