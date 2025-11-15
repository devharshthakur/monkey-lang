use crate::Node;
use lexer::token::Token;
use std::fmt::{Display, Formatter, Result};

/// Represents an identifier expression in the Monkey language AST.
///
/// An identifier is a name that refers to a variable, function, or other named entity.
/// It consists of the token that represents it and its string value.
///
/// # Example
/// For source code:
/// ```monkey
/// myVar
/// ```
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Display for Identifier {
    /// Formats the identifier as its string value.
    ///
    /// # Example
    /// ```rust
    /// # use ast::expression::Identifier;
    /// // For identifier `myVar` → outputs: "myVar"
    /// // For identifier `x` → outputs: "x"
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.value)?;
        Ok(())
    }
}
