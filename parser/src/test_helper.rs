//! This module contains helper functions for testing the parser.
//! It makes easier to test the parser.
use crate::Parser;
use ast::{
    expressions::Expression,
    literals::{BooleanLiteral, Literal},
    statements::Statement,
    Node,
};

/// Its a helper function which tests an integer literal expression.
///
/// This test verifies that an integer literal expression is correctly parsed
/// and identified as an IntegerLiteralExpression in the AST.
///
/// # Parameters
/// - `exp`: The expression to test
/// - `value`: The expected value of the integer literal
///
/// # Returns
/// - `true` if all assertions pass
/// - Panics if any assertion fails (standard Rust test behavior)
#[track_caller]
pub fn test_integer_literal(exp: Expression, value: i64) -> bool {
    // Verifies that the expression is an IntegerLiteral
    let int_lit = match exp {
        Expression::IntegerLiteral(il) => il,
        _ => {
            panic!("il not IntegerLiteral. got={:?}", exp);
        }
    };

    // Verifies that the integer literal's value matches the expected value
    if int_lit.value != value {
        panic!("integ.Value not {}. got={}", value, int_lit.value);
    }

    // Verifies that the integer literal's token literal matches the expected value
    let expected_token_literal = value.to_string();
    if int_lit.token_literal() != expected_token_literal {
        panic!(
            "integ.TokenLiteral not {}. got='{}'",
            value,
            int_lit.token_literal()
        );
    }

    true
}

/// Helper function to test a single let statement.
///
/// This function validates that a statement is a `LetStatement` and that
/// its identifier matches the expected name. It uses pattern matching to
/// extract the `LetStatement` from the `Statement` enum variant.
///
/// # Parameters
/// - `s`: A reference to a Statement enum to test
/// - `name`: The expected identifier name (e.g., "x", "y", "foobar")
///
/// # Returns
/// - `true` if all assertions pass
/// - Panics if any assertion fails (standard Rust test behavior)
#[track_caller]
pub fn test_let_statement(stmt: &Statement, name: &str) -> bool {
    // Verify the statement's token literal is "let"
    assert_eq!(
        stmt.token_literal(),
        "let",
        "s.token_literal() not 'let'. got={}",
        stmt.token_literal()
    );

    // Extract Let statement from Statement enum using pattern matching
    let let_stmt = match stmt {
        Statement::Let(let_stmt) => let_stmt,
        _ => panic!("stmt is not a LetStatement"),
    };

    // Verify the identifier's value matches the expected name
    assert_eq!(
        let_stmt.name.value, name,
        "letStmt.name.value not '{}'. got={}",
        name, let_stmt.name.value
    );

    // Verify the identifier's token literal also matches
    assert_eq!(
        let_stmt.name.token_literal(),
        name,
        "letStmt.name.token_literal() not '{}'. got={}",
        name,
        let_stmt.name.token_literal()
    );

    true
}

/// Checks for parser errors and prints them if any exist.
///
/// This function verifies the parser's error list and prints any errors
/// that were collected during parsing. If no errors are found, it returns
/// early. If errors are present, it prints each error message and then
/// panics with a summary of the error count. This is used to ensure that
/// the parser correctly handles invalid input and reports any issues
/// encountered during the parsing process.
///
/// # Parameters
/// - `p`: A reference to the Parser instance to check for errors
///
/// # Returns
/// - `None` if no errors are found
/// - Panics with a summary of the error count if errors are present
#[track_caller]
pub fn check_parser_errors(p: &Parser) {
    let errors = p.errors();

    if errors.is_empty() {
        return;
    }
    println!("parser errors:");

    for err in errors {
        println!("{}", err);
    }

    panic!("parser has {:?} errors", errors.len());
}

/// Helper function to test a single return statement.
///
/// This function validates that a statement is a `ReturnStatement` and that
/// its token literal is "return". It uses pattern matching to extract the
/// `ReturnStatement` from the `Statement` enum variant.
///
/// # Parameters
/// - `s`: A reference to a Statement enum to test
///
/// # Returns
/// - `true` if all assertions pass
/// - Panics if any assertion fails (standard Rust test behavior)
#[track_caller]
pub fn is_return_statement(stmt: &Statement) -> bool {
    // Verify the statement's token literal is "return"
    assert_eq!(
        stmt.token_literal(),
        "return",
        "token_literal() is not 'return'. got={}",
        stmt.token_literal()
    );

    // Extract Return statement from Statement enum using pattern matching
    let return_stmt = match stmt {
        Statement::Return(return_stmt) => return_stmt,
        _ => panic!("stmt is not a ReturnStatement. got={:?}", stmt),
    };

    // Verify the return statement's token literal matches
    assert_eq!(
        return_stmt.token_literal(),
        "return",
        "returnStmt.token_literal() not 'return'. got={}",
        return_stmt.token_literal()
    );

    true
}

/// Helper function to test an identifier expression.
///
/// This function validates that an expression is an `Identifier` expression and that
/// its value and token literal match the expected values. It uses pattern matching to
/// extract the `Identifier` from the `Expression` enum variant.
///
/// # Parameters
/// - `exp`: The expression to test
/// - `value`: The expected value of the identifier
///
/// # Returns
/// - `true` if all assertions pass
/// - Panics if any assertion fails (standard Rust test behavior)
#[track_caller]
pub fn test_identifier(exp: Expression, value: &str) -> bool {
    // Verify that the expression is an Identifier expression
    let ident = match exp {
        Expression::Identifier(ident) => ident,
        _ => panic!("exp is not an Identifier expression. got={:?}", exp),
    };

    // Verify that the identifier's value matches the expected value
    assert_eq!(
        ident.value, value,
        "ident.value is not {}. got={}",
        value, ident.value
    );

    // Verify that the identifier's token literal matches the expected value
    assert_eq!(
        ident.token_literal(),
        value,
        "ident.token_literal() is not {}. got={}",
        value,
        ident.token_literal()
    );
    // Returns true if all assertions pass
    true
}

/// Helper function to test a literal expression based on its expected type.
///
/// This function validates that an expression matches the expected literal type
/// and value. It dispatches to the appropriate test function based on the
/// expected literal type (integer or identifier).
///
/// # Parameters
/// - `exp`: The expression to test
/// - `expected`: The expected literal value (can be i32, i64, &str, or String)
///
/// # Returns
/// - `true` if all assertions pass
/// - Panics if any assertion fails (standard Rust test behavior)
///
/// # Example
/// ```ignore
/// // Test integer literal (i32 or i64)
/// test_literal_expression(expression, 5);
/// test_literal_expression(expression, 5i64);
///
/// // Test identifier (&str or String)
/// test_literal_expression(expression, "foobar");
/// test_literal_expression(expression, "foobar".to_string());
/// ```
#[track_caller]
pub fn test_literal_expression<E: Into<Literal>>(exp: Expression, expected: E) -> bool {
    let expected_literal = expected.into();
    match expected_literal {
        Literal::Integer(il) => test_integer_literal(exp, il.value),
        Literal::Identifier(ident) => test_identifier(exp, &ident.value),
        Literal::Boolean(bl) => test_boolean_literal(&bl, bl.value),
        _ => panic!(
            "expected literal type not handled. got={:?}",
            expected_literal
        ),
    }
}

#[track_caller]
pub fn test_boolean_literal(boolean_lit: &BooleanLiteral, value: bool) -> bool {
    // Verify that the boolean literal's value matches the expected value
    if boolean_lit.value != value {
        panic!("boolean.Value not {}. got={}", value, boolean_lit.value);
    } else {
        true
    }
}

#[track_caller]
pub fn test_infix_expression<L: Into<Literal>, R: Into<Literal>>(
    exp: Expression,
    left: L,
    operator: &str,
    right: R,
) -> bool {
    let infix_expr = match exp {
        Expression::InfixExpression(infix_expr) => infix_expr,
        _ => panic!("exp is not an InfixExpression. got={:?}", exp),
    };

    // Verify that the left expression matches the expected value
    test_literal_expression(*infix_expr.left, left);

    // Verify that the operator matches the expected operator
    assert_eq!(
        infix_expr.operator, operator,
        "infix_expr.operator is not the expected operator. got={}",
        infix_expr.operator
    );

    // Verify that the right expression matches the expected value
    test_literal_expression(*infix_expr.right, right);

    true
}

#[cfg(test)]
mod tests {
    use lexer::Lexer;

    use super::*;

    #[test]
    fn test_infix_expressions() {
        let tests: Vec<(&str, Literal, &str, Literal)> = vec![
            ("5 + 5;", 5.into(), "+", 5.into()),
            ("5 - 5;", 5.into(), "-", 5.into()),
            ("5 * 5;", 5.into(), "*", 5.into()),
            ("5 / 5;", 5.into(), "/", 5.into()),
            ("alice * bob;", "alice".into(), "*", "bob".into()),
        ];
        for (input, left, operator, right) in tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();

            check_parser_errors(&p);
            assert_eq!(program.statements.len(), 1);

            let stmt = &program.statements[0];
            let expr_stmt = match stmt {
                Statement::Expression(expr_stmt) => expr_stmt,
                _ => panic!("stmt is not an ExpressionStatement"),
            };

            test_infix_expression(expr_stmt.value.clone(), left, operator, right);
        }
    }
}
