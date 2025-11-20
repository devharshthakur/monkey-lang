//! Parser for the Monkey language producing an Abstract Syntax Tree (AST).
//!
//! This crate exposes:
//! - The `ast` module with core AST node types and traits.
//! - The `Parser` struct that turns tokens from the `lexer` crate into an AST.
//!
//! Parsing approach:
//! - Maintains a two-token lookahead (`curr_token`, `peek_token`).
//! - Provides helpers like `expect_peek`, `is_curr_token`, and `is_peek_token`.
//! - Reports user-friendly errors via the `errors` vector.

mod precedence;

use crate::precedence::Precedence;
use ast::{
    expression::{Expression, Identifier, PrefixExpression},
    literals::integer::IntegerLiteral,
    statement::{
        expr::ExpressionStatement, let_::LetStatement, return_::ReturnStatement, Statement,
    },
    Program,
};
use lexer::{
    token::{Token, TokenType},
    Lexer,
};
use std::collections::HashMap;

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
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;

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
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };
        p.register_prefix_parse_fn(TokenType::IDENT, Parser::parse_identifier);
        p.register_prefix_parse_fn(TokenType::INT, Parser::parse_integer_literal);
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
    fn is_curr_token(&self, token_type: TokenType) -> bool {
        self.curr_token.token_type == token_type
    }

    /// Checks if the peek token matches the expected token type.
    ///
    /// Returns true if the peek token's type matches the provided token type,
    /// false otherwise. Used for lookahead parsing decisions.
    fn is_peek_token(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    /// Expects the peek token to be of a specific type and advances if it matches.
    ///
    /// If the peek token matches the expected type, advances the token buffer
    /// and returns true. If it doesn't match, adds an error to the parser's
    /// error list and returns false. This is used for enforcing syntax rules.
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.is_peek_token(token_type) {
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

    fn register_prefix_parse_fn(&mut self, token_type: TokenType, parse_fn: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, parse_fn);
    }

    fn register_infix_parse_fn(&mut self, token_type: TokenType, parse_fn: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, parse_fn);
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
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
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
    /// Currently supports LET and RETURN statements. If an unsupported token type is
    /// encountered, adds an error and returns None. Returns a Statement enum
    /// variant for type-safe statement handling.
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token.token_type {
            TokenType::LET => Some(Statement::Let(self.parse_let_statement())),
            TokenType::RETURN => Some(Statement::Return(self.parse_return_statement())),
            _ => Some(Statement::Expression(self.parse_expression_statement())),
        }
    }

    /// Parses a let statement with the format: let <identifier> = <expression>;
    ///
    /// Expects the current token to be LET. Parses the identifier name and
    /// expects an equals sign. Currently doesn't parse the actual expression
    /// value (sets it to None). Returns a LetStatement with the token information.
    fn parse_let_statement(&mut self) -> LetStatement {
        let token = self.curr_token.clone();

        // Expect identifier after 'let'
        if !self.expect_peek(TokenType::IDENT) {
            return LetStatement {
                token,
                name: Identifier {
                    token: Token::new(TokenType::ILLEGAL, "".to_string()),
                    value: "".to_string(),
                },
                value: None,
            };
        }
        // Parse the identifier
        let name = Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        };

        // Expect '=' after identifier
        if !self.expect_peek(TokenType::ASSIGN) {
            return LetStatement {
                token,
                name,
                value: None,
            };
        }

        // Skip expression for now - just advance until semicolon
        while !self.is_peek_token(TokenType::SEMICOLON) && !self.is_peek_token(TokenType::EOF) {
            self.next_token();
        }

        // Expect semicolon
        if self.is_peek_token(TokenType::SEMICOLON) {
            self.next_token();
        }

        LetStatement {
            token,
            name,
            value: None,
        }
    }

    /// Parses a return statement with the format: return <expression>;
    ///
    /// Expects the current token to be RETURN. Parses the expression value until
    /// it finds a semicolon. Currently doesn't parse the actual expression
    /// value (sets it to None). Returns a ReturnStatement with the token information.
    fn parse_return_statement(&mut self) -> ReturnStatement {
        let token = self.curr_token.clone();

        // Skip expression for now - just advance until semicolon
        while !self.is_peek_token(TokenType::SEMICOLON) && !self.is_peek_token(TokenType::EOF) {
            self.next_token();
        }

        // Expect semicolon
        if self.is_peek_token(TokenType::SEMICOLON) {
            self.next_token();
        }

        ReturnStatement { token, value: None }
    }

    /// Parses an identifier expression.
    /// Expects the current token to be an identifier. Returns an Identifier expression.
    fn parse_identifier(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();
        let value = self.curr_token.literal.clone();
        Some(Expression::Identifier(Identifier { token, value }))
    }

    /// Parses an expression statement, which is an expression followed by an optional semicolon.
    ///
    /// An expression statement wraps an expression in a statement context, allowing
    /// expressions to be used as standalone statements. This is commonly used in REPL
    /// environments where users can type expressions directly without needing to wrap
    /// them in a let or return statement.
    ///
    /// The function parses the expression using the lowest precedence level and then
    /// optionally consumes a semicolon if present. The semicolon is optional to support
    /// REPL usage where semicolons may be omitted.
    ///
    /// # Returns
    /// An `ExpressionStatement` containing the parsed expression and its token information.
    fn parse_expression_statement(&mut self) -> ExpressionStatement {
        let stmt = ExpressionStatement {
            token: self.curr_token.clone(),
            value: self.parse_expression(Precedence::LOWEST).unwrap(),
        };

        // Optional semicolon for REPL
        if self.is_peek_token(TokenType::SEMICOLON) {
            self.next_token();
        }
        stmt
    }

    /// Parses an expression starting from the current token position.
    ///
    /// This is the main entry point for expression parsing. It uses the Pratt parsing
    /// algorithm approach, where expressions are parsed based on precedence levels.
    /// The function looks up a prefix parse function for the current token type and
    /// delegates to that function to parse the expression.
    ///
    /// # Parameters
    /// - `precedence`: The minimum precedence level required to continue parsing.
    ///
    /// # Returns
    /// - `Some(Expression)` if parsing succeeds
    /// - `None` if no parse function is registered for the current token type
    ///
    /// # Errors
    /// Adds an error to the parser's error list if no parse function is found for
    /// the current token type.
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix = self.prefix_parse_fns.get(&self.curr_token.token_type);

        if let Some(parse_fn) = prefix {
            let left_exp = parse_fn(self).unwrap();
            return Some(left_exp);
        } else {
            self.display_no_parse_function_error(self.curr_token.token_type);
            return None;
        }
    }

    /// Parses an integer literal expression from the current token.
    ///
    /// Expects the current token to be of type `INT`. Extracts the integer value
    /// from the token's literal string by parsing it as a 64-bit signed integer.
    /// Returns an `IntegerLiteral` expression node containing both the original token
    /// information and the parsed integer value.
    ///
    /// # Returns
    /// An `Option<Expression>` containing an `IntegerLiteral` variant if parsing succeeds.
    /// The function assumes the token literal is a valid integer string (parsing will
    /// panic if it's not, which should be caught during lexing).
    fn parse_integer_literal(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();
        let value = self.curr_token.literal.parse::<i64>().unwrap();
        Some(Expression::IntegerLiteral(IntegerLiteral { token, value }))
    }

    /// Parses a prefix expression (e.g., `!true`, `-5`).
    ///
    /// Expects the current token to be a prefix operator (BANG or MINUS).
    /// Extracts the operator, advances to the next token, and parses the
    /// right-hand expression with PREFIX precedence. Returns a PrefixExpression
    /// wrapped in an Expression variant.
    ///
    /// # Returns
    /// An `Option<Expression>` containing a `PrefixExpression` variant if parsing succeeds.
    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();
        let operator = self.curr_token.literal.clone();

        // Advance to the next token (the right-hand expression)
        self.next_token();

        // Parse the right-hand expression with PREFIX precedence
        let right = self.parse_expression(Precedence::PREFIX)?;

        Some(Expression::PrefixExpression(PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Node;

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
    fn check_parser_errors(p: &Parser) {
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

        // Check for any parser errors
        check_parser_errors(&p);

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

        // Test each statement to ensure it's a LetStatement with the correct identifier
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

        // Extract Let statement from Statement enum using pattern matching
        let let_stmt = match s {
            Statement::Let(stmt) => stmt,
            _ => panic!("s is not a LetStatement"),
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
    /// Tests parsing of multiple return statements.
    ///
    /// This test verifies that the parser correctly:
    /// 1. Parses multiple return statements from a single input string
    /// 2. Creates the correct number of statements in the AST
    /// 3. Each statement is correctly identified as a ReturnStatement
    /// 4. Each statement's token literal matches "return"
    ///
    /// The test follows the same structure as the Go implementation from
    /// "Writing an Interpreter in Go" by Thorsten Ball, ensuring compatibility
    /// with the reference implementation.
    ///
    /// # Test Structure
    /// - Creates a lexer and parser from input containing 3 return statements
    /// - Parses the program and verifies statement count
    /// - Iterates through each statement and validates its properties
    #[test]
    fn test_return_statements() {
        // Input containing three return statements with different return values
        let input = r#"
return 5;
return 10;
return 993322;
"#
        .to_string();

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        // Parse the program into an AST
        let program = p.parse_program();

        // Check for any parser errors
        check_parser_errors(&p);

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

        // Test each statement to ensure it's a ReturnStatement
        for (i, stmt) in program.statements.iter().enumerate() {
            assert!(
                is_return_statement(stmt),
                "is_return_statement failed for statement {}",
                i
            );
        }
    }

    /// Tests parsing a single return statement.
    ///
    /// This test verifies that a single return statement is correctly parsed
    /// and identified as a ReturnStatement in the AST.
    #[test]
    fn test_return_statement() {
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
    ///
    /// # Validations
    /// 1. Verifies the statement's token literal is "return"
    /// 2. Confirms the statement is actually a `ReturnStatement` (via pattern matching)
    fn is_return_statement(s: &Statement) -> bool {
        // Verify the statement's token literal is "return"
        assert_eq!(
            s.token_literal(),
            "return",
            "token_literal() is not 'return'. got={}",
            s.token_literal()
        );

        // Extract Return statement from Statement enum using pattern matching
        let return_stmt = match s {
            Statement::Return(stmt) => stmt,
            _ => panic!("s is not a ReturnStatement. got={:?}", s),
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

    /// Tests parsing of a single identifier expression.
    ///
    /// This test verifies that a single identifier expression is correctly parsed
    /// and identified as an IdentifierExpression in the AST.
    #[test]
    fn test_identifier_expression() {
        // Creates a lexer and parser from input containing a single identifier expression
        let input = "foobar;".to_string();
        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        // Parses the program and verifies statement count
        let program = p.parse_program();
        check_parser_errors(&p);
        assert_eq!(program.statements.len(), 1);

        // Iterates through each statement and validates its properties
        let stmt = &program.statements[0];

        // Verifies that the statement is an ExpressionStatement
        let expr_stmt = match stmt {
            Statement::Expression(expr_stmt) => expr_stmt,
            _ => panic!("s is not an ExpressionStatement. got={:?}", stmt),
        };

        // Verifies that the expression is an Identifier
        let expr = &expr_stmt.value;
        let ident = match expr {
            Expression::Identifier(ident) => ident,
            _ => panic!("expr is not an Identifier. got={:?}", expr),
        };

        // Verifies that the identifier's value matches the expected value
        assert_eq!(
            ident.value, "foobar",
            "ident.value is not foobar. got={}",
            ident.value
        );
        // Verifies that the identifier's token literal matches the expected value
        assert_eq!(
            ident.token_literal(),
            "foobar",
            "ident.token_literal() is not foobar. got={}",
            ident.token_literal()
        );
    }

    /// Tests parsing of a single integer literal expression.
    ///
    /// This test verifies that a single integer literal expression is correctly parsed
    /// and identified as an IntegerLiteralExpression in the AST.
    #[test]

    fn test_integer_literal_expression() {
        let input = "5;".to_string();
        // Creates a lexer and parser from input
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        // Checks for any parser errors
        check_parser_errors(&p);
        // Verifies that the program has exactly 1 statement
        assert_eq!(program.statements.len(), 1);
        // Iterates through each statement and validates its properties
        let stmt = &program.statements[0];
        // Verifies that the statement is an ExpressionStatement
        let expr_stmt = match stmt {
            Statement::Expression(expr_stmt) => expr_stmt,
            _ => panic!("s is not an ExpressionStatement. got={:?}", stmt),
        };
        let expr = &expr_stmt.value;
        // Verifies that the expression is an IntegerLiteral
        let int_lit = match expr {
            Expression::IntegerLiteral(int_lit) => int_lit,
            _ => panic!("expr is not an IntegerLiteral. got={:?}", expr),
        };
        // Verifies that the integer literal's value matches the expected value
        assert_eq!(
            int_lit.value, 5,
            "int_lit.value is not 5. got={}",
            int_lit.value
        );
        // Verifies that the integer literal's token literal matches the expected value
        assert_eq!(
            int_lit.token_literal(),
            "5",
            "int_lit.token_literal() is not 5. got={}",
            int_lit.token_literal()
        );
    }
}
