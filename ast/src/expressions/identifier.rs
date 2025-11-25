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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::token::{Token, TokenType};

    #[test]
    fn test_identifier_display() {
        let ident = Identifier {
            token: Token {
                token_type: TokenType::IDENT,
                literal: "foobar".to_string(),
            },
            value: "foobar".to_string(),
        };

        assert_eq!(ident.value, "foobar");
        assert_eq!(ident.token_literal(), "foobar");
        assert_eq!(format!("{}", ident), "foobar");
    }
}
