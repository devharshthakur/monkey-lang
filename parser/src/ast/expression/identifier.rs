//! Identifier expression node.
//!
//! Represents a variable or function name in the AST.

use super::super::Node;
use lexer::token::Token;

/// An identifier (variable or function name) in the AST.
///
/// The `Identifier` structure holds both the token that produced
/// the identifier (i.e., the original lexical representation)
/// and the parsed name as a string. This allows the AST to retain
/// precise information about where the identifier appeared in the source code,
/// as well as provide convenient access to its value.
///
/// # Example
/// Given code: `let myVar = 5;`
/// The identifier `myVar` would be represented as:
/// ```
/// Identifier {
///     token: Token { token_type: Ident, literal: "myVar".to_string() },
///     value: "myVar".to_string(),
/// }
/// ```
///
/// Fields:
/// - `token`: The token corresponding to the identifier (`Ident` token, with its literal).
/// - `value`: The string name of the identifier, e.g., `"myVar"`.
#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,  // The token corresponding to the identifier, e.g., Ident("myVar")
    pub value: String, // The identifier's name as a string, e.g., "myVar"
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}
