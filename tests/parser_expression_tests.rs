use monkey_lang::ast::{expression::Expression, statement::Statement, Node};
use monkey_lang::lexer::Lexer;
use monkey_lang::parser::{test_helper::*, Parser};

// =============================================================================
// Identifier & Integer Literal Tests
// =============================================================================

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

// =============================================================================
// Prefix & Infix Expression Tests
// =============================================================================

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

// =============================================================================
// Operator Precedence Tests
// =============================================================================

/// Tests operator precedence parsing to ensure expressions are parsed correctly
/// according to operator precedence rules.
#[test]
fn test_operator_precedence_parsing() {
    let tests: Vec<(&str, &str)> = vec![
        // Prefix operators with infix operators
        ("-a * b;", "((-a) * b)"),
        ("!-a;", "(!(-a))"),
        // Left-associative operators
        ("a + b + c;", "((a + b) + c)"),
        ("a + b - c;", "((a + b) - c)"),
        ("a * b * c;", "((a * b) * c)"),
        ("a * b / c;", "((a * b) / c)"),
        // Precedence: multiplication/division higher than addition/subtraction
        ("a + b / c;", "(a + (b / c))"),
        ("a + b * c + d / e - f;", "(((a + (b * c)) + (d / e)) - f)"),
        // Multiple statements
        ("3 + 4; -5 * 5;", "(3 + 4)((-5) * 5)"),
        // Comparison operators
        ("5 > 4 == 3 < 4;", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4;", "((5 < 4) != (3 > 4))"),
        // Mixed precedence
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5;",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        // Boolean operators
        ("true;", "true"),
        ("false;", "false"),
        ("3 > 5 == false;", "((3 > 5) == false)"),
        ("3 < 5 == true;", "((3 < 5) == true)"),
        ("!(true == true);", "(!(true == true))"),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();

        check_parser_errors(&p);

        let actual = format!("{}", program);
        assert_eq!(
            actual, expected,
            "expected={:?}, got={:?}",
            expected, actual
        );
    }
}

// =============================================================================
// If Expression Tests
// =============================================================================

/// Tests parsing of if expressions: if (<condition>) <consequence>
#[test]
fn test_parsing_if_expression() {
    let input = "if (x < y) { x }";
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

    let if_expr = match &expr_stmt.value {
        Expression::IfExpression(if_expr) => if_expr,
        _ => panic!("expr is not an IfExpression. got={:?}", expr_stmt.value),
    };

    if !test_infix_expression_str(*if_expr.condition.clone(), "x", "<", "y") {
        return;
    }

    let consequence_block_stmt = match &*if_expr.consequence {
        Expression::BlockStatement(bs) => bs,
        _ => panic!(
            "consequence is not a BlockStatement. got={:?}",
            if_expr.consequence
        ),
    };

    if consequence_block_stmt.statements.len() != 1 {
        panic!(
            "consequence is not 1 statement. got={}",
            consequence_block_stmt.statements.len()
        );
    }

    let consequence_stmt = &consequence_block_stmt.statements[0];
    let consequence_expr = match consequence_stmt {
        Statement::Expression(expr_stmt) => &expr_stmt.value,
        _ => panic!(
            "consequence_stmt is not an ExpressionStatement. got={:?}",
            consequence_stmt
        ),
    };

    if !test_identifier(consequence_expr.clone(), "x") {
        return;
    }

    if if_expr.alternative.is_some() {
        panic!("alternative is not nil. got={:?}", if_expr.alternative);
    }
}

/// Tests parsing of if-else expressions: if (<condition>) <consequence> else <alternative>
#[test]
fn test_parsing_if_else_expression() {
    let input = "if (x < y) { x } else { y }";
    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();

    check_parser_errors(&p);
    assert_eq!(program.statements.len(), 1);
    let stmt = &program.statements[0];
    // Check if the statement is an ExpressionStatement
    let expr_stmt = match stmt {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!("stmt is not an ExpressionStatement. got={:?}", stmt),
    };
    // Check if the expression is an IfExpression
    let if_expr = match &expr_stmt.value {
        Expression::IfExpression(if_expr) => if_expr,
        _ => panic!("expr is not an IfExpression. got={:?}", expr_stmt.value),
    };
    // Check if the condition is an infix expression
    if !test_infix_expression_str(*if_expr.condition.clone(), "x", "<", "y") {
        return;
    }

    // Extract consequence block statement
    let consequence_block = match &*if_expr.consequence {
        Expression::BlockStatement(bs) => bs,
        _ => panic!(
            "consequence is not a BlockStatement. got={:?}",
            if_expr.consequence
        ),
    };

    // Check if the consequence is 1 statement
    if consequence_block.statements.len() != 1 {
        panic!(
            "consequence is not 1 statement. got={}",
            consequence_block.statements.len()
        );
    }

    // Check if the consequence statement is an ExpressionStatement
    let consequence_stmt = &consequence_block.statements[0];
    let consequence_expr = match consequence_stmt {
        Statement::Expression(expr_stmt) => &expr_stmt.value,
        _ => panic!(
            "consequence_stmt is not an ExpressionStatement. got={:?}",
            consequence_stmt
        ),
    };
    // Check if the consequence statement is an Identifier
    if !test_identifier(consequence_expr.clone(), "x") {
        return;
    }

    // Extract alternative block statement
    let alternative_block = match if_expr.alternative.as_deref() {
        Some(Expression::BlockStatement(bs)) => bs,
        Some(other) => panic!("alternative is not a BlockStatement. got={:?}", other),
        None => panic!("alternative is None"),
    };

    // Check if the alternative is 1 statement
    if alternative_block.statements.len() != 1 {
        panic!(
            "alternative is not 1 statement. got={}",
            alternative_block.statements.len()
        );
    }

    // Check if the alternative statement is an ExpressionStatement
    let alternative_stmt = match &alternative_block.statements[0] {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!(
            "alternative_stmt is not an ExpressionStatement. got={:?}",
            alternative_block.statements[0]
        ),
    };
    // Check if the alternative statement is an Identifier
    if !test_identifier(alternative_stmt.value.clone(), "y") {
        return;
    }
}

// =============================================================================
// Function Literal Tests
// =============================================================================

#[test]
fn test_parsing_function_literal() {
    let input = "fn(x, y) { x + y; }";
    // Create a new lexer and parser
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    // Check that the program has 1 statement
    assert_eq!(
        program.statements.len(),
        1,
        "program.statements does not contain 1 statement. got={}",
        program.statements.len()
    );
    // Check that the statement is an ExpressionStatement
    let stmt = program.statements[0].clone();
    // Check that the statement is an ExpressionStatement
    let expr_stmt = match stmt {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!("stmt is not ExpressionStatement. got={:?}", stmt),
    };

    // Check that the expression is a FunctionLiteral
    let func_lit = match expr_stmt.value {
        Expression::FunctionLiteral(func_lit) => func_lit,
        _ => panic!(
            "expr_stmt.value is not FunctionLiteral. got={:?}",
            expr_stmt.value
        ),
    };

    // Check that the function literal has 2 parameters
    assert_eq!(
        func_lit.parameters.len(),
        2,
        "func_lit.parameters does not contain 2 parameters. got={}",
        func_lit.parameters.len()
    );
    // Check that the function literal has the correct parameters
    test_literal_expression_str(Expression::Identifier(func_lit.parameters[0].clone()), "x");
    test_literal_expression_str(Expression::Identifier(func_lit.parameters[1].clone()), "y");

    // Check that the function literal has 1 body statement
    assert_eq!(
        func_lit.body.statements.len(),
        1,
        "function body does not contain 1 statement. got={}",
        func_lit.body.statements.len()
    );

    // Check that the body statement is an ExpressionStatement
    let body_stmt = func_lit.body.statements[0].clone();
    let body_expr = match body_stmt {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!("body_stmt is not ExpressionStatement. got={:?}", body_stmt),
    };

    test_infix_expression_str(body_expr.value, "x", "+", "y");
}

#[test]
fn test_parsing_function_parameter() {
    let input = "fn(x, y) { x + y; }";
    // Create a new lexer and parser
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&parser);
    // Check that the program has 1 statement
    assert_eq!(
        program.statements.len(),
        1,
        "program.statements does not contain 1 statement. got={}",
        program.statements.len()
    );
    // Check that the statement is an ExpressionStatement
    let stmt = program.statements[0].clone();
    let expr_stmt = match stmt {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!("stmt is not ExpressionStatement. got={:?}", stmt),
    };
    // Check that the expression is a FunctionLiteral
    let func_lit = match expr_stmt.value {
        Expression::FunctionLiteral(func_lit) => func_lit,
        _ => panic!(
            "expr_stmt.value is not FunctionLiteral. got={:?}",
            expr_stmt.value
        ),
    };
    // Check that the function literal has 2 parameters
    assert_eq!(
        func_lit.parameters.len(),
        2,
        "func_lit.parameters does not contain 2 parameters. got={}",
        func_lit.parameters.len()
    );
    // Check that the function literal has the correct parameters
    test_literal_expression_str(Expression::Identifier(func_lit.parameters[0].clone()), "x");
    test_literal_expression_str(Expression::Identifier(func_lit.parameters[1].clone()), "y");
}

// =============================================================================
// Call Expression Tests
// =============================================================================

#[test]
fn test_parsing_call_expression() {
    let input = "add(1, 2 * 3, 4 + 5);";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);
    // Check that the program has 1 statement
    assert_eq!(program.statements.len(), 1);

    // Check that the statement is an ExpressionStatement
    let stmt = &program.statements[0];
    // Check that the expression is a CallExpression
    let expr_stmt = match stmt {
        Statement::Expression(expr_stmt) => expr_stmt,
        _ => panic!("stmt is not an ExpressionStatement. got={:?}", stmt),
    };

    // Check that the expression is a CallExpression
    let call_expr = match &expr_stmt.value {
        Expression::CallExpression(ce) => ce,
        _ => panic!(
            "expr_stmt.value is not a CallExpression. got={:?}",
            expr_stmt.value
        ),
    };

    // Check that the function is an Identifier
    assert!(
        test_identifier(*call_expr.function.clone(), "add"),
        "call_expr.function is not 'add'. got={}",
        call_expr.function
    );
    assert_eq!(
        call_expr.arguments.len(),
        3,
        "call_expr.arguments does not contain 3 arguments. got={}",
        call_expr.arguments.len()
    );
    // Check that the arguments are correct
    test_literal_expression_str(call_expr.arguments[0].clone(), "1");
    test_infix_expression_str(call_expr.arguments[1].clone(), "2", "*", "3");
    test_infix_expression_str(call_expr.arguments[2].clone(), "4", "+", "5");
}
