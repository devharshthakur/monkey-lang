//! Literal expression types in the Monkey language AST.
//!
//! Literals represent constant values in the source code, such as integers,
//! strings, booleans, etc. Each literal type stores both the token (for
//! position information) and the parsed value.

pub mod integer_literal;
pub use integer_literal::IntegerLiteral;
