use monkey_lang::{
    ast::{Expression, Statement},
    parser::test_helper::{
        check_parser_errors, test_infix_expression_str, test_literal_expression_str,
    },
    Lexer, Parser,
};

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
