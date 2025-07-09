use crate::ast::AsAny;
use crate::ast::Expression;
use crate::ast::Identifier;
use crate::ast::LetStatement;
use crate::ast::Program;
use crate::ast::Statement;
use lexer::token::Token;
use lexer::token::TokenType;
use lexer::Lexer;

pub mod ast;

/// A parser that converts tokens from a lexer into an Abstract Syntax Tree (AST).
///
/// The parser maintains a two-token lookahead buffer (current and peek tokens)
/// to make parsing decisions. It processes tokens sequentially and builds
/// the AST by parsing different statement types.
pub struct Parser {
    l: Lexer,
    curr_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    /// Creates a new parser instance with the given lexer.
    ///
    /// Initializes the parser with empty tokens and then reads the first two tokens
    /// to set up the lookahead buffer. This ensures the parser always has
    /// both current and peek tokens available for parsing decisions.
    pub fn new(l: Lexer) -> Self {
        let mut p = Parser {
            l,
            curr_token: Token::new(TokenType::EOF, "".to_string()),
            peek_token: Token::new(TokenType::EOF, "".to_string()),
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    /// Advances the token buffer by one position.
    ///
    /// Moves the peek token to the current token position and reads
    /// the next token from the lexer into the peek position. This maintains
    /// the two-token lookahead buffer/window used for parsing decisions.
    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    /// Checks if the current token matches the expected token type.
    ///
    /// Returns true if the current token's type matches the provided token type,
    /// false otherwise. Used for conditional parsing logic.
    fn curr_token_is(&self, token_type: TokenType) -> bool {
        self.curr_token.token_type == token_type
    }

    /// Checks if the peek token matches the expected token type.
    ///
    /// Returns true if the peek token's type matches the provided token type,
    /// false otherwise. Used for lookahead parsing decisions.
    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    /// Expects the peek token to be of a specific type and advances if it matches.
    ///
    /// If the peek token matches the expected type, advances the token buffer
    /// and returns true. If it doesn't match, adds an error to the parser's
    /// error list and returns false. This is used for enforcing syntax rules.
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    /// Adds a peek error to the parser's error list.
    ///
    /// Creates a descriptive error message indicating what token type was expected
    /// versus what was actually found in the peek position. This helps with
    /// debugging parsing issues.
    fn peek_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "Expected next token be {:?}, got {:?} instead",
            token_type, self.peek_token.token_type
        );
        self.errors.push(msg);
    }

    /// Adds a "no parse function" error to the parser's error list.
    ///
    /// Creates an error message when the parser encounters a token type
    /// that it doesn't know how to handle. This indicates that the parser
    /// needs to be extended to support new token types.
    fn no_parse_function_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "No parse funtion for token type {:?} found for token `{}`",
            token_type, self.curr_token.literal
        );
        self.errors.push(msg);
    }

    /// Returns a reference to the parser's error list.
    ///
    /// Allows external code to check if any parsing errors occurred
    /// during the parsing process.
    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    /// Parses the entire program and returns the root AST node.
    ///
    /// Iterates through all tokens until EOF is reached, parsing each
    /// statement encountered. Collects all successfully parsed statements
    /// into a Program node. If parsing of a statement fails, it continues
    /// with the next statement rather than stopping the entire parse.
    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program {
            statements: Vec::new(),
        };

        while self.curr_token.token_type != TokenType::EOF {
            let statement = self.parse_statement();
            if let Some(stmt) = statement {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    /// Parses a single statement based on the current token type.
    ///
    /// Uses the current token to determine what type of statement to parse.
    /// Currently supports LET statements. If an unsupported token type is
    /// encountered, adds an error and returns None. Returns a boxed Statement
    /// trait object for polymorphic statement handling.
    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.curr_token.token_type {
            TokenType::LET => Some(Box::new(self.parse_let_statement())),
            _ => {
                self.no_parse_function_error(self.curr_token.token_type);
                None
            }
        }
    }

    /// Parses a let statement with the format: let <identifier> = <expression>;
    ///
    /// Expects the current token to be LET. Parses the identifier name,
    /// expects an equals sign, then skips over the expression value until
    /// it finds a semicolon. Currently doesn't parse the actual expression
    /// value (sets it to None). Returns a LetStatement with the parsed
    /// identifier and token information.
    fn parse_let_statement(&mut self) -> LetStatement {
        let mut stmt = LetStatement {
            token: self.curr_token.clone(),
            name: Identifier {
                token: Token::new(TokenType::ILLEGAL, "".to_string()),
                value: "".to_string(),
            },
            value: None,
        };
        if !self.expect_peek(TokenType::IDENT) {
            return stmt;
        }

        stmt.name = Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return stmt;
        }

        // Skip until we encounter a semicolon
        while !self.curr_token_is(TokenType::SEMICOLON) && !self.curr_token_is(TokenType::EOF) {
            self.next_token();
        }

        stmt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;
    use crate::ast::Node;

    /// Helper function to check for parser errors and fail the test if any are found.
    /// This makes test failures more informative.
    fn check_parser_errors(parser: &Parser, test_name: &str) {
        if !parser.errors.is_empty() {
            eprintln!(
                "Parser has {} errors for test `{}`:",
                parser.errors.len(),
                test_name
            );
            for err in parser.errors.iter() {
                eprintln!("parser error: {}", err);
            }
            // `panic!` aborts the test with the given message.
            panic!("Parser errors encountered in test `{}`", test_name);
        }
    }

    /// Helper function to test the structure of a `LetStatement`.
    /// This function verifies the token literal, type, name value, and name's token literal.
    /// It corresponds directly to `testLetStatement` in the Go example.
    ///
    /// # Arguments
    /// - `s`: A reference to a `Box<dyn Statement>` which is the statement to be tested.
    /// - `expected_name`: The expected string value of the identifier (variable name).
    ///
    /// # Returns
    /// `true` if all assertions pass, `false` if any fail.
    fn test_let_statement_structure(s: &Box<dyn Statement>, expected_name: &str) -> bool {
        // 1. Check the statement's `TokenLiteral()`. It should be "let".
        if s.token_literal() != "let" {
            eprintln!("s.token_literal not 'let'. got={}", s.token_literal());
            return false;
        }

        // 2. Downcast the `Statement` trait object to a concrete `LetStatement` type.
        // `s.as_any().downcast_ref::<ast::LetStatement>()` attempts to cast the trait object
        // back to its concrete type. It returns `Some(&LetStatement)` if successful, `None` otherwise.
        let let_stmt = match s.as_any().downcast_ref::<ast::LetStatement>() {
            Some(ls) => ls,
            None => {
                eprintln!("s not ast::LetStatement. got={:?}", s); // Print debug info for statement
                return false;
            }
        };

        // 3. Check the `Name.Value` field of the `LetStatement`. This is the actual string name.
        if let_stmt.name.value != expected_name {
            eprintln!(
                "let_stmt.name.value not '{}'. got={}",
                expected_name, let_stmt.name.value
            );
            return false;
        }

        // 4. Check the `Name.TokenLiteral()` field. This should also match the expected name.
        if let_stmt.name.token_literal() != expected_name {
            eprintln!(
                "let_stmt.name.token_literal() not '{}'. got={}",
                expected_name,
                let_stmt.name.token_literal()
            );
            return false;
        }

        true // All checks passed for this specific `let` statement.
    }

    /// The main test function for `let` statements.
    /// `#[test]` attribute marks this as a unit test to be run by `cargo test`.
    #[test]
    fn test_let_statements() {
        // Input Monkey source code containing multiple `let` statements.
        // `r#"`..."#` is a raw string literal, useful for multi-line strings without needing to escape newlines.
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#
        .to_string(); // Convert `&str` to `String` because `Lexer::new` expects owned `String`.

        // Create a lexer and then a parser instance with the input.
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        // Parse the program to get the AST root node.
        let program = parser.parse_program();

        // Immediately check for any errors reported by the parser during the parsing process.
        check_parser_errors(&parser, "test_let_statements");

        // Assert that the program contains the expected number of statements.
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        // Define the expected identifiers (variable names) for each `let` statement.
        let expected_identifiers = vec!["x", "y", "foobar"];

        // Iterate through the statements in the parsed program and verify each one.
        for (i, expected_ident) in expected_identifiers.iter().enumerate() {
            let stmt = &program.statements[i]; // Get a reference to the current statement (a `Box<dyn Statement>`).
                                               // Call the helper function `test_let_statement` to perform detailed checks on the statement.
                                               // If the helper returns `false`, `assert!` will panic, failing the test.
            assert!(
                test_let_statement_structure(stmt, expected_ident),
                "test[{}] failed for identifier {}",
                i,
                expected_ident
            );
        }
    }
}
