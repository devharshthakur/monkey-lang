//! Operator precedence levels for parsing expressions.
//!
//! This module defines the precedence levels used in the Pratt parser
//! to correctly parse expressions with proper operator precedence.
//! Higher values indicate higher precedence.

/// Operator precedence levels for parsing expressions.
/// Higher values indicate higher precedence.

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    /// Lowest precedence (used as default)
    LOWEST = 1,
    /// Equality operators: ==, !=
    EQUALS = 2,
    /// Comparison operators: >, <
    LESSGREATER = 3,
    /// Addition and subtraction: +, -
    SUM = 4,
    /// Multiplication and division: *, /
    PRODUCT = 5,
    /// Prefix operators: -X, !X
    PREFIX = 6,
    /// Function calls: myFunction(X)
    CALL = 7,
}

impl Precedence {
    /// Returns the lowest precedence level as a u8.
    pub const fn lowest() -> u8 {
        Precedence::LOWEST as u8
    }
}
