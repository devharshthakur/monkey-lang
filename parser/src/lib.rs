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
mod test_helper;

use crate::precedence::Precedence;
use ast::{
    expressions::{infix, Expression, Identifier, PrefixExpression},
    literals::{integer::IntegerLiteral, BooleanLiteral},
    statements::{
        expr::ExpressionStatement, let_::LetStatement, return_::ReturnStatement, Statement,
    },
    Program,
};
use lexer::{
    token::{Token, TokenType},
    Lexer,
};
use std::collections::HashMap;
use test_helper::{
    check_parser_errors, is_return_statement, test_integer_literal, test_let_statement,
};

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
        // Register prefix parse functions
        p.register_prefix_parse_fn(TokenType::IDENT, Parser::parse_identifier);
        p.register_prefix_parse_fn(TokenType::INT, Parser::parse_integer_literal);
        p.register_prefix_parse_fn(TokenType::BANG, Parser::parse_prefix_expression);
        p.register_prefix_parse_fn(TokenType::MINUS, Parser::parse_prefix_expression);
        p.register_prefix_parse_fn(TokenType::TRUE, Parser::parse_boolean_literal);
        p.register_prefix_parse_fn(TokenType::FALSE, Parser::parse_boolean_literal);
        // Register Infix parse functions
        p.register_infix_parse_fn(TokenType::PLUS, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::MINUS, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::SLASH, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::ASTERISK, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::EQ, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::NOTEQ, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::LT, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::GT, Parser::parse_infix_expression);
        // Advance the token buffer to have a two-token lookahead
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
    fn display_no_parse_function_error(&mut self, token_type: &TokenType) {
        let msg = format!(
            "No parse function for token type {:?} found for token `{}`",
            token_type, self.curr_token.literal
        );
        self.errors.push(msg);
    }

    fn no_prefix_parse_function_error(&mut self, token_type: TokenType) {
        let msg = format!(
            "No prefix parse function found for token type {:?}",
            token_type
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
            value: self.parse_expression(Precedence::LOWEST as i32).unwrap(),
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
    fn parse_expression(&mut self, precedence: i32) -> Option<Expression> {
        let token_type = self.curr_token.token_type.clone();
        let prefix = self.prefix_parse_fns.get(&token_type);
        // If the prefix parse function is found, parse the left-hand side expression and returns an Expression
        let mut left = if let Some(prefix_parse_fn) = prefix {
            let left_exp = prefix_parse_fn(self)?;
            left_exp
        } else {
            return None;
        };

        // If the precedence is less than the peek precedence, parse the infix expression
        while !self.is_peek_token(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            // Extract token type first to end the borrow before mutating self
            let peek_token_type = self.peek_token.token_type.clone();
            let infix = self.infix_parse_fns.get(&peek_token_type).copied();
            // If the infix parse function is not found, return the left-hand side expression
            if infix.is_none() {
                return Some(left);
            }
            self.next_token();
            // If the infix parse function is found, parse the right-hand side expression with the precedence level and returns an Expression
            if let Some(infix_parse_fn) = infix {
                left = infix_parse_fn(self, left)?;
            } else {
                return Some(left);
            }
        }

        Some(left)
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
        let value = token.literal.parse::<i64>().unwrap();
        Some(Expression::IntegerLiteral(IntegerLiteral { token, value }))
    }
    /// Parses a boolean literal expression from the current token.
    ///
    /// Expects the current token to be of type `TRUE` or `FALSE`. Extracts the boolean value
    /// from the token's literal string. Returns a `BooleanLiteral` expression node containing
    /// both the original token information and the parsed boolean value.
    ///
    /// # Returns
    /// An `Option<Expression>` containing a `BooleanLiteral` variant if parsing succeeds.
    /// The function assumes the token literal is a valid boolean string (parsing will
    /// panic if it's not, which should be caught during lexing).
    ///
    /// # Errors
    /// Adds an error to the parser's error list if the token literal is not a valid boolean string.
    fn parse_boolean_literal(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();
        let value = self.curr_token.literal.parse::<bool>().unwrap();
        Some(Expression::BooleanLiteral(BooleanLiteral { token, value }))
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
        let right = self.parse_expression(Precedence::PREFIX as i32)?;

        Some(Expression::PrefixExpression(PrefixExpression {
            token,
            operator,
            right: Box::new(right),
        }))
    }
    /// Returns the precedence level for the next token
    /// If no precedence is found, returns the lowest precedence
    fn peek_precedence(&self) -> i32 {
        let token_type = &self.peek_token.token_type;
        let precedence = Precedence::from_token_type(token_type);
        precedence
    }

    /// Returns the precedence level for the current token
    /// If no precedence is found, returns the lowest precedence
    fn curr_precedence(&self) -> i32 {
        let token_type = &self.curr_token.token_type.clone();
        let precedence = Precedence::from_token_type(token_type);
        precedence
    }
    /// Parses an infix expression (e.g., `5 + 5`, `x == y`).
    ///
    /// Expects the current token to be an infix operator (PLUS, MINUS, SLASH, ASTERISK, EQ, NOTEQ, LT, GT).
    /// Parses the left-hand side expression, the operator, and the right-hand side expression.
    /// Returns an InfixExpression wrapped in an Expression variant.
    ///
    /// # Parameters
    /// - `left`: The left-hand side expression
    /// - `operator`: The infix operator
    /// - `right`: The right-hand side expression
    ///
    /// # Returns
    /// An `Option<Expression>` containing an `InfixExpression` variant if parsing succeeds.
    /// The function assumes the token literal is a valid infix operator string (parsing will
    /// panic if it's not, which should be caught during lexing).
    ///
    /// # Errors
    /// Adds an error to the parser's error list if no parse function is found for
    /// the current token type.
    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        // Expects the current token to be an infix operator.
        let token = self.curr_token.clone();
        let operator = self.curr_token.literal.clone();

        // Get the precedence level for the current token which is the operator of the infix expression and advances the token
        let precedence = self.curr_precedence();

        // Advance to the next token to point to the right operand
        self.next_token();

        // Parse the right-hand side expression with the precedence level and returns an Expression
        let right = self.parse_expression(precedence)?;
        Some(Expression::InfixExpression(infix::InfixExpression {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Node;

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
    fn test_parsing_let_statements() {
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
    fn test_parsing_return_statements() {
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

    /// Tests parsing of a single identifier expression.
    ///
    /// This test verifies that a single identifier expression is correctly parsed
    /// and identified as an IdentifierExpression in the AST.
    #[test]
    fn test_parsing_identifier_expression() {
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
    fn test_parsing_integer_literal_expression() {
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

    /// Tests parsing of prefix expressions (e.g., `!5`, `-15`, `!foobar`, `-foobar`).
    ///
    /// This test verifies that prefix expressions with BANG (!) and MINUS (-) operators
    /// are correctly parsed and identified as PrefixExpression in the AST. It tests
    /// both operators with integer literals and identifiers.
    #[test]
    fn test_parsing_prefix_expressions() {
        // Test cases: (input, expected_operator, expected_right_value)
        let prefix_tests: Vec<(&str, &str, &str)> = vec![
            ("!5;", "!", "5"),
            ("-15;", "-", "15"),
            ("!foobar;", "!", "foobar"),
            ("-foobar;", "-", "foobar"),
        ];

        for (input, expected_operator, expected_right_value) in prefix_tests {
            // Creates a lexer and parser from input containing a prefix expression
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);

            // Parses the program
            let program = p.parse_program();

            // Checks for any parser errors
            check_parser_errors(&p);

            // Verifies that the program has exactly 1 statement
            assert_eq!(
                program.statements.len(),
                1,
                "program.statements does not contain 1 statement. got={}",
                program.statements.len()
            );

            // Extracts the first statement from the program
            let stmt = &program.statements[0];

            // Verifies that the statement is an ExpressionStatement
            let expr_stmt = match stmt {
                Statement::Expression(expr_stmt) => expr_stmt,
                _ => panic!("stmt is not an ExpressionStatement. got={:?}", stmt),
            };

            // Verifies that the expression is a PrefixExpression
            let prefix_expr = match &expr_stmt.value {
                Expression::PrefixExpression(pe) => pe,
                _ => panic!("expr is not a PrefixExpression. got={:?}", expr_stmt.value),
            };

            // Verifies that the prefix operator matches the expected operator
            assert_eq!(
                prefix_expr.operator, expected_operator,
                "prefix_expr.operator is not '{}'. got={}",
                expected_operator, prefix_expr.operator
            );

            // Tests the right-hand expression based on its type
            match &*prefix_expr.right {
                Expression::IntegerLiteral(int_lit) => {
                    // Verifies that the integer literal's value matches the expected value
                    let expected_int = expected_right_value.parse::<i64>().unwrap();
                    assert_eq!(
                        int_lit.value, expected_int,
                        "int_lit.value is not {}. got={}",
                        expected_int, int_lit.value
                    );
                    // Verifies that the integer literal's token literal matches the expected value
                    assert_eq!(
                        int_lit.token_literal(),
                        expected_right_value,
                        "int_lit.token_literal() is not '{}'. got='{}'",
                        expected_right_value,
                        int_lit.token_literal()
                    );
                }
                Expression::Identifier(ident) => {
                    // Verifies that the identifier's value matches the expected value
                    assert_eq!(
                        ident.value, expected_right_value,
                        "ident.value is not '{}'. got='{}'",
                        expected_right_value, ident.value
                    );
                    // Verifies that the identifier's token literal matches the expected value
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

    /// Tests parsing of infix expressions (e.g., `5 + 5`, `5 - 5`, `5 * 5`, `5 / 5`, `5 > 5`, `5 < 5`, `5 == 5`, `5 != 5`).
    ///
    /// This test verifies that infix expressions with the operators +, -, *, /, >, <, ==, != are correctly parsed
    /// and identified as InfixExpression in the AST.
    ///
    /// # Parameters
    /// - `input`: The input string containing the infix expression
    /// - `left_value`: The expected value of the left operand
    /// - `operator`: The infix operator
    /// - `right_value`: The expected value of the right operand
    ///
    /// # Returns
    /// - `true` if all assertions pass
    /// - Panics if any assertion fails.
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
            // Creates a lexer and parser from input
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program();

            // Checks for any parser errors and verifies that the program has exactly 1 statement
            check_parser_errors(&p);
            assert_eq!(program.statements.len(), 1);

            // Verify that the statement is an ExpressionStatement
            let stmt = &program.statements[0];
            let expr_stmt = match stmt {
                Statement::Expression(expr_stmt) => expr_stmt,
                _ => panic!("stmt is not an ExpressionStatement. got={:?}", stmt),
            };

            // Verify that the expression is an InfixExpression
            let infix_expr = match &expr_stmt.value {
                Expression::InfixExpression(ie) => ie,
                _ => panic!("expr is not an InfixExpression. got={:?}", expr_stmt.value),
            };

            // Verify that the left expression is an IntegerLiteral
            let left_val = match &*infix_expr.left {
                Expression::IntegerLiteral(int_lit) => int_lit.value as i32,
                _ => panic!(
                    "infix_expr.left is not the expected value. got={:?}",
                    infix_expr.left
                ),
            };
            // Verify that the left value matches the expected value
            assert_eq!(
                left_val, expected_left_value,
                "left value mismatch. expected={}, got={}",
                expected_left_value, left_val
            );
            // Verify that the operator matches the expected operator
            assert_eq!(
                infix_expr.operator, expected_operator,
                "operator mismatch. expected='{}', got='{}'",
                expected_operator, infix_expr.operator
            );

            // Verify that the right expression's value matches the expected value and
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

    /// Tests operator precedence parsing to ensure expressions are parsed correctly
    /// according to operator precedence rules.
    ///
    /// This test verifies that:
    /// 1. Prefix operators have higher precedence than infix operators
    /// 2. Multiplication/division have higher precedence than addition/subtraction
    /// 3. Comparison operators have lower precedence than arithmetic operators
    /// 4. Equality operators have lower precedence than comparison operators
    /// 5. Left-associative operators are grouped correctly
    /// 6. Complex expressions with multiple precedence levels are parsed correctly
    /// 7. Boolean operators are parsed correctly
    ///
    /// # Parameters
    /// - `input`: The input string containing the expression
    /// - `expected`: The expected string containing the parsed expression
    ///
    /// # Returns
    /// - `true` if all assertions pass
    /// - Panics if any assertion fails.
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
}
