//! Parser error types and handling for the Monkey language.
//!
//! This module provides structured error types that eliminate scattered format! strings
//! and enable proper error handling throughout the parser.

mod parser_error;
mod span;

use crate::lexer::token::TokenType;
pub use parser_error::ParserError;
pub use span::Span;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

/// All parser error types - centralized and exhaustive.
///
/// This enum represents every possible error that can occur during parsing.
/// Each variant contains the necessary context to generate a helpful error message.
#[derive(Debug, Clone, PartialEq)]
pub enum ParserErrorType {
    // === Token expectation errors ===
    ExpectedToken {
        expected: TokenType,
        got: TokenType,
        literal: String,
    },
    UnexpectedEOF,
    MissingSemicolon {
        got: TokenType,
        literal: String,
    },

    // === Parse function errors ===
    NoPrefixParseFunction {
        token_type: TokenType,
        literal: String,
    },
    NoInfixParseFunction {
        token_type: TokenType,
    },

    // === Literal parsing errors ===
    InvalidIntegerLiteral {
        literal: String,
    },

    // === Expression parsing errors ===
    FailedToParseExpression {
        context: &'static str,
    },
    FailedToParsePrefixRHS {
        operator: String,
    },
    FailedToParseInfixRHS {
        operator: String,
    },
    FailedToParseGroupedExpression,

    // === If expression errors ===
    FailedToParseIfCondition,
    ExpectedBlockStatement {
        context: &'static str,
    },
    FailedToParseIfBlock {
        context: &'static str,
    },

    // === Block statement errors ===
    FailedToParseStatementInBlock,

    // === Function errors ===
    FailedToParseFunctionParameters,
    FailedToParseFunctionBody,
    ExpectedParameterIdentifier {
        got: TokenType,
        literal: String,
    },
    FailedToParseParameter {
        context: &'static str,
    },

    // === Call expression errors ===
    FailedToParseCallArguments,
    FailedToParseCallArgument {
        context: &'static str,
    },
    UnclosedCallArguments {
        got: TokenType,
        literal: String,
    },
}

impl Display for ParserErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::ExpectedToken {
                expected,
                got,
                literal,
            } => {
                write!(f, "expected {:?}, got {:?} ('{}')", expected, got, literal)
            }
            Self::UnexpectedEOF => write!(f, "unexpected end of file"),
            Self::MissingSemicolon { got, literal } => {
                write!(
                    f,
                    "expected ';' after statement, got {:?} ('{}')",
                    got, literal
                )
            }
            Self::NoPrefixParseFunction {
                token_type,
                literal,
            } => {
                write!(
                    f,
                    "no prefix parse function for {:?} ('{}')",
                    token_type, literal
                )
            }
            Self::NoInfixParseFunction { token_type } => {
                write!(f, "no infix parse function for {:?}", token_type)
            }
            Self::InvalidIntegerLiteral { literal } => {
                write!(f, "could not parse '{}' as integer", literal)
            }
            Self::FailedToParseExpression { context } => {
                write!(f, "failed to parse expression {}", context)
            }
            Self::FailedToParsePrefixRHS { operator } => {
                write!(
                    f,
                    "failed to parse expression after prefix operator '{}'",
                    operator
                )
            }
            Self::FailedToParseInfixRHS { operator } => {
                write!(
                    f,
                    "failed to parse expression after infix operator '{}'",
                    operator
                )
            }
            Self::FailedToParseGroupedExpression => {
                write!(f, "failed to parse expression inside parentheses")
            }
            Self::FailedToParseIfCondition => {
                write!(f, "failed to parse condition in if expression")
            }
            Self::ExpectedBlockStatement { context } => {
                write!(f, "expected block statement for {}", context)
            }
            Self::FailedToParseIfBlock { context } => {
                write!(f, "failed to parse {} block in if expression", context)
            }
            Self::FailedToParseStatementInBlock => {
                write!(f, "failed to parse statement in block")
            }
            Self::FailedToParseFunctionParameters => {
                write!(f, "failed to parse function parameters")
            }
            Self::FailedToParseFunctionBody => {
                write!(f, "failed to parse function body")
            }
            Self::ExpectedParameterIdentifier { got, literal } => {
                write!(
                    f,
                    "expected identifier for parameter, got {:?} ('{}')",
                    got, literal
                )
            }
            Self::FailedToParseParameter { context } => {
                write!(f, "failed to parse {} parameter", context)
            }
            Self::FailedToParseCallArguments => {
                write!(f, "failed to parse call arguments")
            }
            Self::FailedToParseCallArgument { context } => {
                write!(f, "failed to parse {} argument in function call", context)
            }
            Self::UnclosedCallArguments { got, literal } => {
                write!(
                    f,
                    "expected ')' to close arguments, got {:?} ('{}')",
                    got, literal
                )
            }
        }
    }
}
