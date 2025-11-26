use monkey_lang::lexer::Lexer;
use monkey_lang::parser::{test_helper::*, Parser};

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
        !program.statements.is_empty(),
        "ParseProgram() returned empty program"
    );
    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    let tests = vec!["x", "y", "foobar"];
    for (i, expected_identifier) in tests.iter().enumerate() {
        let stmt = &program.statements[i];
        assert!(
            test_let_statement(stmt, expected_identifier),
            "test_let_statement failed for statement {}",
            i
        );
    }
}
