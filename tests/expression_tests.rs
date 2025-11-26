use monkey_lang::ast::{expression::Expression, statement::Statement, Node};
use monkey_lang::lexer::Lexer;
use monkey_lang::parser::{test_helper::*, Parser};

/// Tests parsing of a single identifier expression.
#[test]
fn test_parsing_identifier_expression() {
    let input = "foobar;".to_string();
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    check_parser_errors(&p);
    assert_eq!(program.statements.len(), 1);

    let stmt = &program.statements[0];

    let expr_stmt = match stmt {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!("s is not an ExpressionStatement. got={:?}", stmt),
    };

    let expr = &expr_stmt.value;
    let ident = match expr {
        Expression::Identifier(ident) => ident,
        _ => panic!("expr is not an Identifier. got={:?}", expr),
    };

    assert_eq!(
        ident.value, "foobar",
        "ident.value is not foobar. got={}",
        ident.value
    );
    assert_eq!(
        ident.token_literal(),
        "foobar",
        "ident.token_literal() is not foobar. got={}",
        ident.token_literal()
    );
}

/// Tests parsing of a single integer literal expression.
#[test]
fn test_parsing_integer_literal_expression() {
    let input = "5;".to_string();
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);
    assert_eq!(program.statements.len(), 1);

    let stmt = &program.statements[0];
    let expr_stmt = match stmt {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!("s is not an ExpressionStatement. got={:?}", stmt),
    };
    let expr = &expr_stmt.value;
    let int_lit = match expr {
        Expression::IntegerLiteral(int_lit) => int_lit,
        _ => panic!("expr is not an IntegerLiteral. got={:?}", expr),
    };
    assert_eq!(
        int_lit.value, 5,
        "int_lit.value is not 5. got={}",
        int_lit.value
    );
    assert_eq!(
        int_lit.token_literal(),
        "5",
        "int_lit.token_literal() is not 5. got={}",
        int_lit.token_literal()
    );
}

/// Tests parsing of prefix expressions (e.g., `!5`, `-15`, `!foobar`, `-foobar`).
#[test]
fn test_parsing_prefix_expressions() {
    let prefix_tests: Vec<(&str, &str, &str)> = vec![
        ("!5;", "!", "5"),
        ("-15;", "-", "15"),
        ("!foobar;", "!", "foobar"),
        ("-foobar;", "-", "foobar"),
    ];

    for (input, expected_operator, expected_right_value) in prefix_tests {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);

        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain 1 statement. got={}",
            program.statements.len()
        );

        let stmt = &program.statements[0];

        let expr_stmt = match stmt {
            Statement::Expression(expr_stmt) => expr_stmt,
            _ => panic!("stmt is not an ExpressionStatement. got={:?}", stmt),
        };

        let prefix_expr = match &expr_stmt.value {
            Expression::PrefixExpression(pe) => pe,
            _ => panic!("expr is not a PrefixExpression. got={:?}", expr_stmt.value),
        };

        assert_eq!(
            prefix_expr.operator, expected_operator,
            "prefix_expr.operator is not '{}'. got={}",
            expected_operator, prefix_expr.operator
        );

        match &*prefix_expr.right {
            Expression::IntegerLiteral(int_lit) => {
                let expected_int = expected_right_value.parse::<i64>().unwrap();
                assert_eq!(
                    int_lit.value, expected_int,
                    "int_lit.value is not {}. got={}",
                    expected_int, int_lit.value
                );
                assert_eq!(
                    int_lit.token_literal(),
                    expected_right_value,
                    "int_lit.token_literal() is not '{}'. got='{}'",
                    expected_right_value,
                    int_lit.token_literal()
                );
            }
            Expression::Identifier(ident) => {
                assert_eq!(
                    ident.value, expected_right_value,
                    "ident.value is not '{}'. got='{}'",
                    expected_right_value, ident.value
                );
                assert_eq!(
                    ident.token_literal(),
                    expected_right_value,
                    "ident.token_literal() is not '{}'. got='{}'",
                    expected_right_value,
                    ident.token_literal()
                );
            }
            _ => panic!(
                "prefix_expr.right is not IntegerLiteral or Identifier. got={:?}",
                prefix_expr.right
            ),
        }
    }
}

/// Tests parsing of infix expressions.
#[test]
fn test_parsing_infix_expression() {
    let infix_tests: Vec<(&str, i32, &str, i32)> = vec![
        ("5 + 5;", 5, "+", 5),
        ("5 - 5;", 5, "-", 5),
        ("5 * 5;", 5, "*", 5),
        ("5 / 5;", 5, "/", 5),
        ("5 > 5;", 5, ">", 5),
        ("5 < 5;", 5, "<", 5),
        ("5 == 5;", 5, "==", 5),
        ("5 != 5;", 5, "!=", 5),
    ];

    for (input, expected_left_value, expected_operator, expected_right_value) in infix_tests {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();

        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 1);

        let stmt = &program.statements[0];
        let expr_stmt = match stmt {
            Statement::Expression(expr_stmt) => expr_stmt,
            _ => panic!("stmt is not an ExpressionStatement. got={:?}", stmt),
        };

        let infix_expr = match &expr_stmt.value {
            Expression::InfixExpression(ie) => ie,
            _ => panic!("expr is not an InfixExpression. got={:?}", expr_stmt.value),
        };

        let left_val = match &*infix_expr.left {
            Expression::IntegerLiteral(int_lit) => int_lit.value as i32,
            _ => panic!(
                "infix_expr.left is not the expected value. got={:?}",
                infix_expr.left
            ),
        };
        assert_eq!(
            left_val, expected_left_value,
            "left value mismatch. expected={}, got={}",
            expected_left_value, left_val
        );
        assert_eq!(
            infix_expr.operator, expected_operator,
            "operator mismatch. expected='{}', got='{}'",
            expected_operator, infix_expr.operator
        );

        let right_val = match &*infix_expr.right {
            Expression::IntegerLiteral(int_lit) => int_lit.value as i32,
            _ => panic!(
                "infix_expr.right is not the expected value. got={:?}",
                infix_expr.right
            ),
        };
        assert_eq!(
            right_val, expected_right_value,
            "right value mismatch. expected={}, got={}",
            expected_right_value, right_val
        );
    }
}
