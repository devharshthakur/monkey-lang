//! Let statement node.
//!
//! Represents a variable declaration: `let <identifier> = <expression>;`

use crate::expression::{Expression, Identifier};
use crate::Node;
use lexer::token::Token;

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
