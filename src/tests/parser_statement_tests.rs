use crate::ast::Statement;
use crate::lexer::Lexer;
use crate::parser::{Parser, test_helper::*};

// =============================================================================
// Let Statement Tests
// =============================================================================

/// Tests parsing of multiple let statements.
#[test]
fn test_parsing_let_statements() {
    let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#
    .to_string();
    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    check_parser_errors(&p);

    assert!(
        program.statements.len() == 3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    let expected: [(&str, i64); 3] = [("x", 5), ("y", 10), ("foobar", 838383)];
    for (i, (identifier, expected_value)) in expected.iter().enumerate() {
        let stmt = &program.statements[i];
        assert!(
            test_let_statement(stmt, *identifier),
            "test_let_statement failed at index {}",
            i
        );

        let value = match stmt {
            Statement::Let(let_stmt) => let_stmt.value.clone(),
            _ => panic!("stmt is not a LetStatement"),
        };

        if let Some(value) = value {
            assert!(
                test_literal_expression(value, *expected_value),
                "test_literal_expression failed at index {}",
                i
            );
        }
    }
}

// =============================================================================
// Return Statement Tests
// =============================================================================

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
