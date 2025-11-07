//! Parser for the Monkey language producing an Abstract Syntax Tree (AST).
//!
//! This crate exposes:
//! - The `ast` module with core AST node types and traits.
//! - The `Parser` struct that turns tokens from the `lexer` crate into an AST.
//!
//! Parsing approach:
//! - Maintains a two-token lookahead (`curr_token`, `peek_token`).
//! - Provides helpers like `expect_peek`, `curr_token_is`, and `peek_token_is`.
//! - Reports user-friendly errors via the `errors` vector.
//! - Currently supports parsing `let` statements and collects them in `Program`.

use crate::ast::Identifier;
use crate::ast::LetStatement;
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
            self.display_peek_error(token_type);
            false
        }
    }

    /// Adds a peek error to the parser's error list.
    ///
    /// Creates a descriptive error message indicating what token type was expected
    /// versus what was actually found in the peek position. This helps with
    /// debugging parsing issues.
    fn display_peek_error(&mut self, token_type: TokenType) {
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
    fn display_no_parse_function_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "No parse funtion for token type {:?} found for token `{}`",
            token_type, self.curr_token.literal
        );
        self.errors.push(msg);
    }

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
        // Loop until EOF is reached
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
    /// encountered, adds an error and returns None. Returns a Statement enum
    /// variant for type-safe statement handling.
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token.token_type {
            TokenType::LET => Some(Statement::Let(self.parse_let_statement())),
            _ => {
                self.display_no_parse_function_error(self.curr_token.token_type);
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
    use crate::ast::Node;

    /// Tests parsing of multiple let statements.
    ///
    /// This test verifies that the parser correctly:
    /// 1. Parses multiple let statements from a single input string
    /// 2. Creates the correct number of statements in the AST
    /// 3. Each statement is correctly identified as a LetStatement
    /// 4. Each statement's identifier name matches the expected value
    ///
    /// The test follows the same structure as the Go implementation from
    /// "Writing an Interpreter in Go" by Thorsten Ball, ensuring compatibility
    /// with the reference implementation.
    ///
    /// # Test Structure
    /// - Creates a lexer and parser from input containing 3 let statements
    /// - Parses the program and verifies statement count
    /// - Iterates through each statement and validates its properties
    #[test]
    fn test_let_statements() {
        // Input containing three let statements with different identifiers
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#
        .to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        // Parse the program into an AST
        let program = p.parse_program();

        // Verify that parsing succeeded (program is not empty)
        assert!(
            !program.statements.is_empty(),
            "ParseProgram() returned empty program"
        );
        // Verify that exactly 3 statements were parsed
        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        // Expected identifier names for each statement (in order)
        let tests = vec!["x", "y", "foobar"];

        // Test each statement to ensure it's a LetStatement with the correct identifier
        for (i, expected_identifier) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            assert!(
                test_let_statement(stmt, expected_identifier),
                "test_let_statement failed for statement {}",
                i
            );
        }
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
    ///
    /// # Validations
    /// 1. Verifies the statement's token literal is "let"
    /// 2. Confirms the statement is actually a `LetStatement` (via pattern matching)
    /// 3. Checks that the identifier's value matches the expected name
    /// 4. Verifies the identifier's token literal matches the expected name
    fn test_let_statement(s: &Statement, name: &str) -> bool {
        // Verify the statement's token literal is "let"
        assert_eq!(
            s.token_literal(),
            "let",
            "s.token_literal() not 'let'. got={}",
            s.token_literal()
        );

        // Extract LetStatement from Statement enum using pattern matching
        let let_stmt = match s {
            Statement::Let(stmt) => stmt,
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
}
