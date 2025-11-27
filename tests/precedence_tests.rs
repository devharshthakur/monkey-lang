use monkey_lang::lexer::Lexer;
use monkey_lang::parser::{test_helper::*, Parser};

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
