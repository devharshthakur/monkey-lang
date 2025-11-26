use monkey_lang::lexer::Lexer;
use monkey_lang::parser::{test_helper::*, Parser};

/// Tests parsing of multiple return statements.
#[test]
fn test_parsing_return_statements() {
    let input = r#"
return 5;
return 10;
return 993322;
"#
    .to_string();

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    check_parser_errors(&p);

    assert!(
        !program.statements.is_empty(),
        "ParseProgram() returned empty program"
    );
    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    for (i, stmt) in program.statements.iter().enumerate() {
        assert!(
            is_return_statement(stmt),
            "is_return_statement failed for statement {}",
            i
        );
    }
}

/// Tests parsing a single return statement.
#[test]
fn test_parsing_return_statement() {
    let input = "return 5;".to_string();

    let l = Lexer::new(input);
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
    assert!(
        is_return_statement(stmt),
        "statement is not a ReturnStatement"
    );
}
