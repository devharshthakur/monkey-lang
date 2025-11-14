use crate::Node;
use lexer::token::Token;

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
