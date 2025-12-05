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

pub mod error;
mod precedence;
pub mod test_helper;

use crate::ast::{
    Program,
    expression::{
        BlockStatement, BooleanLiteral, CallExpression, Expression, FunctionLiteral, Identifier,
        IfExpression, InfixExpression, IntegerLiteral, PrefixExpression,
    },
    statement::{ExpressionStatement, LetStatement, ReturnStatement, Statement},
};
use crate::lexer::{
    Lexer,
    token::{Token, TokenType},
};
use crate::parser::error::ParserError;
use precedence::Precedence;
use std::collections::HashMap;

/// A parser that converts tokens from a lexer into an Abstract Syntax Tree (AST).
///
/// The parser maintains a two-token lookahead buffer (current and peek tokens)
/// to make parsing decisions. It processes tokens sequentially and builds
/// the AST by parsing different statement types.
#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    l: Lexer,
    curr_token: Token,
    peek_token: Token,
    pub errors: Vec<ParserError>,
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
            curr_token: Token::new(TokenType::EOF, "".to_string(), 0, 0),
            peek_token: Token::new(TokenType::EOF, "".to_string(), 0, 0),
            errors: Vec::<ParserError>::new(),
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
        p.register_prefix_parse_fn(TokenType::LPAREN, Parser::parse_grouped_expression);
        p.register_prefix_parse_fn(TokenType::IF, Parser::parse_if_expression);
        p.register_prefix_parse_fn(TokenType::LBRACE, Parser::parse_block_statement);
        p.register_prefix_parse_fn(TokenType::ELSE, Parser::parse_if_expression);
        p.register_prefix_parse_fn(TokenType::FUNCTION, Parser::parse_function_literal);
        // Register Infix parse functions
        p.register_infix_parse_fn(TokenType::PLUS, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::MINUS, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::SLASH, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::ASTERISK, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::EQ, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::NOTEQ, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::LT, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::GT, Parser::parse_infix_expression);
        p.register_infix_parse_fn(TokenType::LPAREN, Parser::parse_call_expression);
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

    /// Adds a peek error to the parser's error list and displays it.
    ///
    /// Creates a descriptive error message indicating what token type was expected
    /// versus what was actually found in the peek position. Includes source position
    /// information for easier debugging.
    fn display_peek_error(&mut self, expected: TokenType) {
        let error = ParserError::at_token(
            &self.peek_token,
            format!(
                "expected token to be {:?}, got {:?}",
                expected, self.peek_token.token_type
            ),
        );
        log::error!("{}", error);
        self.errors.push(error);
    }

    fn no_prefix_parse_function_error(&mut self) {
        let error = ParserError::at_token(
            &self.curr_token,
            format!(
                "no prefix parse function for {:?}",
                self.curr_token.token_type
            ),
        );
        log::error!("{}", error);
        self.errors.push(error);
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
    pub fn errors(&self) -> &Vec<ParserError> {
        &self.errors
    }

    /// Parses the entire program and returns the root AST node.
    ///
    /// Iterates through all tokens until EOF is reached, parsing each
    /// statement encountered. Collects all successfully parsed statements
    /// into a Program node. If parsing of a statement fails, it continues
    /// with the next statement rather than stopping the entire parse.
    pub fn parse_program(&mut self) -> Program {
        log::error!("Starting to parse program");
        let mut program = Program {
            statements: Vec::new(),
        };
        // Loop until EOF is reached
        while self.curr_token.token_type != TokenType::EOF {
            log::error!(
                "[{}:{}] Parsing statement, curr_token={:?}, peek_token={:?}",
                self.curr_token.line,
                self.curr_token.column,
                self.curr_token.token_type,
                self.peek_token.token_type
            );
            let statement = self.parse_statement();
            if let Some(stmt) = statement {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        log::error!(
            "Finished parsing program, {} statements parsed",
            program.statements.len()
        );
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
            TokenType::LET => self.parse_let_statement().map(Statement::Let),
            TokenType::RETURN => self.parse_return_statement().map(Statement::Return),
            _ => self.parse_expression_statement().map(Statement::Expression),
        }
    }

    /// Parses a let statement with the format: let <identifier> = <expression>;
    ///
    /// Expects the current token to be LET. Parses the identifier name and
    /// expects an equals sign. Returns Some(LetStatement) if parsing succeeds,
    /// None if parsing fails.
    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let token = self.curr_token.clone();

        // Expect identifier after 'let'
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }
        // Parse the identifier
        let name = Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        };

        let mut stmt = LetStatement {
            token,
            name,
            value: None,
        };

        // Expect '=' after identifier
        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }
        // Advance to the next token to point to the value and parse the expression
        self.next_token();
        stmt.value = self.parse_expression(Precedence::LOWEST as i32);

        // Require semicolon
        if !self.is_peek_token(TokenType::SEMICOLON) {
            let error = ParserError::at_token(
                &self.peek_token,
                format!("missing semicolon, got {:?}", self.peek_token.token_type),
            );
            log::error!("{}", error);
            self.errors.push(error);
            return None;
        }
        self.next_token();

        Some(stmt)
    }

    /// Parses a return statement with the format: return <expression>;
    ///
    /// Expects the current token to be RETURN. Parses the expression value until
    /// it finds a semicolon. Currently doesn't parse the actual expression
    /// value (sets it to None). Returns a ReturnStatement with the token information.
    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let token = self.curr_token.clone();

        if self.curr_token.token_type != TokenType::RETURN {
            return None;
        }

        // Advance to the next token to point to the value and parse the expression
        self.next_token();
        let value = self.parse_expression(Precedence::LOWEST as i32);

        // Require semicolon
        if !self.is_peek_token(TokenType::SEMICOLON) {
            let error = ParserError::at_token(
                &self.peek_token,
                format!("missing semicolon, got {:?}", self.peek_token.token_type),
            );
            log::error!("{}", error);
            self.errors.push(error);
            return None;
        }
        self.next_token();

        Some(ReturnStatement { token, value })
    }

    /// Parses an identifier expression.
    /// Expects the current token to be an identifier. Returns an Identifier expression.
    fn parse_identifier(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();
        let value = self.curr_token.literal.clone();
        Some(Expression::Identifier(Identifier { token, value }))
    }

    /// Parses an expression statement, which is an expression followed by a semicolon.
    ///
    /// An expression statement wraps an expression in a statement context, allowing
    /// expressions to be used as standalone statements.
    ///
    /// The function parses the expression using the lowest precedence level and then
    /// requires a semicolon.
    ///
    /// # Returns
    /// An `ExpressionStatement` containing the parsed expression and its token information.
    fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let expr = self.parse_expression(Precedence::LOWEST as i32)?;
        let stmt = ExpressionStatement {
            token: self.curr_token.clone(),
            value: expr,
        };

        // Require semicolon
        if !self.is_peek_token(TokenType::SEMICOLON) {
            let error = ParserError::at_token(
                &self.peek_token,
                format!("missing semicolon, got {:?}", self.peek_token.token_type),
            );
            log::error!("{}", error);
            self.errors.push(error);
            return None;
        }
        self.next_token();
        Some(stmt)
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
        log::error!(
            "[{}:{}] parse_expression called with precedence={}, curr_token={:?}",
            self.curr_token.line,
            self.curr_token.column,
            precedence,
            self.curr_token.token_type
        );
        let token_type = self.curr_token.token_type;
        let prefix = self.prefix_parse_fns.get(&token_type);
        // If the prefix parse function is found, parse the left-hand side expression and returns an Expression
        let mut left = if let Some(prefix_parse_fn) = prefix {
            log::error!("Found prefix parse function for {:?}", token_type);
            let left_exp = prefix_parse_fn(self)?;
            left_exp
        } else {
            self.no_prefix_parse_function_error();
            return None;
        };

        // If the precedence is less than the peek precedence, parse the infix expression
        while !self.is_peek_token(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            // Extract token type first to end the borrow before mutating self
            let peek_token_type = self.peek_token.token_type;
            let peek_precedence = self.peek_precedence();
            log::error!(
                "[{}:{}] Continuing infix parsing, peek_token={:?}, peek_precedence={}",
                self.peek_token.line,
                self.peek_token.column,
                peek_token_type,
                peek_precedence
            );
            let infix = self.infix_parse_fns.get(&peek_token_type).copied();
            // If the infix parse function is not found, return the left-hand side expression
            if infix.is_none() {
                log::error!(
                    "No infix parse function for {:?}, returning left expression",
                    peek_token_type
                );
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

        log::error!("Finished parsing expression");
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
    /// Returns `None` and adds an error if the token literal cannot be parsed as an integer.
    fn parse_integer_literal(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();
        match token.literal.parse::<i64>() {
            Ok(value) => Some(Expression::IntegerLiteral(IntegerLiteral { token, value })),
            Err(_) => {
                let error = ParserError::at_token(
                    &token,
                    format!("invalid integer literal: {}", token.literal),
                );
                log::error!("{}", error);
                self.errors.push(error);
                None
            }
        }
    }
    /// Parses a boolean literal expression from the current token.
    ///
    /// Expects the current token to be of type `TRUE` or `FALSE`. Determines the boolean value
    /// by checking the token type (matching Go implementation). Returns a `BooleanLiteral`
    /// expression node containing both the original token information and the parsed boolean value.
    ///
    /// # Returns
    /// An `Option<Expression>` containing a `BooleanLiteral` variant if parsing succeeds.
    ///
    /// # Errors
    /// Adds an error to the parser's error list if the token type is not TRUE or FALSE.
    fn parse_boolean_literal(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();
        let value = self.is_curr_token(TokenType::TRUE);
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
        log::error!(
            "Parsing prefix expression with operator {:?}",
            self.curr_token.token_type
        );
        let token = self.curr_token.clone();
        let operator = self.curr_token.literal.clone();

        // Advance to the next token (the right-hand expression)
        self.next_token();

        // Parse the right-hand expression with PREFIX precedence
        let right = match self.parse_expression(Precedence::PREFIX as i32) {
            Some(expr) => expr,
            None => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!("failed to parse prefix rhs: {}", operator),
                );
                log::error!("{}", error);
                self.errors.push(error);
                return None;
            }
        };

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
        let token_type = &self.curr_token.token_type;
        Precedence::from_token_type(token_type)
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
    ///
    /// # Errors
    /// Adds an error to the parser's error list if parsing the right-hand expression fails.
    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        log::error!(
            "Parsing infix expression with operator {:?}",
            self.curr_token.token_type
        );
        // Expects the current token to be an infix operator.
        let token = self.curr_token.clone();
        let operator = self.curr_token.literal.clone();

        // Get the precedence level for the current token which is the operator of the infix expression and advances the token
        let precedence = self.curr_precedence();

        // Advance to the next token to point to the right operand
        self.next_token();

        // Parse the right-hand side expression with the precedence level and returns an Expression
        let right = match self.parse_expression(precedence) {
            Some(expr) => expr,
            None => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!(
                        "failed to parse right-hand side of infix expression: {}",
                        operator
                    ),
                );
                log::error!("{}", error);
                self.errors.push(error);
                return None;
            }
        };
        Some(Expression::InfixExpression(InfixExpression {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }
    /// Parses a grouped expression (e.g., `(5 + 5)`).
    ///
    /// Expects the current token to be a left parenthesis. Parses the expression inside the parentheses.
    /// Returns an Expression wrapped in an Expression variant.
    ///
    /// # Returns
    /// An `Option<Expression>` containing an `Expression` variant if parsing succeeds.
    ///
    /// # Errors
    /// Adds an error to the parser's error list if the right parenthesis is not found or expression parsing fails.
    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        log::error!("Parsing grouped expression");
        // Expects the current token to be a left parenthesis. skips it and advances the token
        self.next_token();

        let expr = match self.parse_expression(Precedence::LOWEST as i32) {
            Some(e) => e,
            None => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!("failed to parse grouped expression"),
                );
                log::error!("{}", error);
                self.errors.push(error);
                return None;
            }
        };

        if !self.expect_peek(TokenType::RPAREN) {
            // Error already added by expect_peek
            return None;
        }

        Some(expr)
    }
    /// Parses an if expression (e.g., `if (<condition>) <consequence> else <alternative>`).
    ///
    /// Expects the current token to be an if keyword. Parses the condition, consequence, and alternative.
    /// Returns an IfExpression wrapped in an Expression variant.
    ///
    /// # Returns
    /// An `Option<Expression>` containing an `IfExpression` variant if parsing succeeds.
    fn parse_if_expression(&mut self) -> Option<Expression> {
        log::error!("Parsing if expression");
        let token = self.curr_token.clone();

        if !self.expect_peek(TokenType::LPAREN) {
            // Error already added by expect_peek
            return None;
        }
        // Advance to the next token to point to the condition and parse it with the lowest precedence
        self.next_token();
        let condition = match self.parse_expression(Precedence::LOWEST as i32) {
            Some(e) => e,
            None => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!("failed to parse if condition"),
                );
                log::error!("{}", error);
                self.errors.push(error);
                return None;
            }
        };
        // Expects the next token to be a right parenthesis
        if !self.expect_peek(TokenType::RPAREN) {
            // Error already added by expect_peek
            return None;
        }
        // Expects the next token to be a left brace
        if !self.expect_peek(TokenType::LBRACE) {
            // Error already added by expect_peek
            return None;
        }
        // Parse the consequence block statement and returns a BlockStatement
        let consequence = match self.parse_block_statement() {
            Some(Expression::BlockStatement(bs)) => bs,
            Some(_) => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!("expected block statement for if consequence"),
                );
                log::error!("{}", error);
                self.errors.push(error);
                return None;
            }
            None => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!("failed to parse if block for consequence"),
                );
                log::error!("{}", error);
                self.errors.push(error);
                return None;
            }
        };
        // If the next token is an else keyword, parse the alternative block statement
        let alternative = if self.is_peek_token(TokenType::ELSE) {
            self.next_token();

            if !self.expect_peek(TokenType::LBRACE) {
                // Error already added by expect_peek
                return None;
            }
            // Parse the alternative block statement and returns a BlockStatement
            match self.parse_block_statement() {
                Some(Expression::BlockStatement(bs)) => Some(Box::new(bs)),
                Some(_) => {
                    let error = ParserError::at_token(
                        &self.curr_token,
                        format!("expected block statement for if alternative"),
                    );
                    log::error!("{}", error);
                    self.errors.push(error);
                    return None;
                }
                None => {
                    let error = ParserError::at_token(
                        &self.curr_token,
                        format!("failed to parse if block for alternative"),
                    );
                    log::error!("{}", error);
                    self.errors.push(error);
                    return None;
                }
            }
        } else {
            None
        };
        Some(Expression::IfExpression(IfExpression {
            token,
            condition: Box::new(condition),
            consequence: Box::new(Expression::BlockStatement(consequence)),
            alternative: alternative.map(|bs| Box::new(Expression::BlockStatement(*bs))),
        }))
    }
    /// Parses a block statement (e.g., `{ <statements> }`).
    ///
    /// Expects the current token to be a left brace. Parses the statements in the block until the right brace is found or EOF is reached.
    /// Returns a BlockStatement containing the parsed statements and their token information.
    ///
    /// # Returns
    /// An `Option<BlockStatement>` containing a `BlockStatement` variant if parsing succeeds.
    ///
    /// # Errors
    /// Adds an error to the parser's error list if the right brace is not found or statement parsing fails.
    fn parse_block_statement(&mut self) -> Option<Expression> {
        log::error!("Parsing block statement");
        let token = self.curr_token.clone();
        let mut statements = Vec::new();
        // Parse the statements in the block until the right brace is found or EOF is reached
        while !self.is_peek_token(TokenType::RBRACE) && !self.is_peek_token(TokenType::EOF) {
            self.next_token();
            match self.parse_statement() {
                Some(stmt) => statements.push(stmt),
                None => {
                    let error = ParserError::at_token(
                        &self.curr_token,
                        format!("failed to parse statement in block"),
                    );
                    log::error!("{}", error);
                    self.errors.push(error);
                    // Continue parsing to collect more errors
                }
            }
        }
        if !self.expect_peek(TokenType::RBRACE) {
            // Error already added by expect_peek
            return None;
        }
        Some(Expression::BlockStatement(BlockStatement {
            token,
            statements,
        }))
    }

    /// Parses a function literal expression (e.g., `fn(<parameters>) <body>`).
    ///
    /// Expects the current token to be a function keyword. Parses the parameters and body.
    /// Returns a FunctionLiteral wrapped in an Expression variant.
    ///
    /// # Returns
    /// An `Option<Expression>` containing a `FunctionLiteral` variant if parsing succeeds.
    fn parse_function_literal(&mut self) -> Option<Expression> {
        log::error!("Parsing function literal");
        let token = self.curr_token.clone();
        if !self.expect_peek(TokenType::LPAREN) {
            // Error already added by expect_peek
            return None;
        }
        let parameters = match self.parse_function_parameters() {
            Some(p) => p,
            None => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!("failed to parse function parameters"),
                );
                log::error!("{}", error);
                self.errors.push(error);
                return None;
            }
        };
        if !self.expect_peek(TokenType::LBRACE) {
            // Error already added by expect_peek
            return None;
        }
        let body = match self.parse_block_statement() {
            Some(Expression::BlockStatement(block_stmt)) => block_stmt,
            Some(_) => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!("expected block statement for function body"),
                );
                log::error!("{}", error);
                self.errors.push(error);
                return None;
            }
            None => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!("failed to parse function body"),
                );
                log::error!("{}", error);
                self.errors.push(error);
                return None;
            }
        };
        Some(Expression::FunctionLiteral(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    /// Parses the function parameters (e.g., `x, y`).
    ///
    /// Expects the current token to be a left parenthesis. Parses the parameters until the right parenthesis is found.
    /// Returns a Vec<Identifier> containing the parsed parameters.
    ///
    /// # Returns
    /// An `Option<Vec<Identifier>>` containing a `Vec<Identifier>` variant if parsing succeeds.
    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        log::error!("Parsing function parameters");
        let mut parameters = Vec::new();
        if self.is_peek_token(TokenType::RPAREN) {
            self.next_token();
            return Some(parameters);
        }
        // Advance to the next token to point to the first parameter
        self.next_token();

        // Parse first parameter
        let first_param = match self.parse_identifier() {
            Some(Expression::Identifier(ident)) => ident,
            Some(_) => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!(
                        "expected parameter to be an identifier, got {:?}",
                        self.curr_token.token_type
                    ),
                );
                log::error!("Expected parameter to be an identifier but got: {}", error);
                self.errors.push(error);
                return None;
            }
            None => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!("failed to parse first parameter"),
                );
                log::error!("Failed to parse first parameter: {}", error);
                self.errors.push(error);
                return None;
            }
        };
        parameters.push(first_param);

        // Parse remaining parameters
        while self.is_peek_token(TokenType::COMMA) {
            self.next_token();
            self.next_token();

            let identifier = match self.parse_identifier() {
                Some(Expression::Identifier(ident)) => ident,
                Some(_) => {
                    let error = ParserError::at_token(
                        &self.curr_token,
                        format!(
                            "expected parameter to be an identifier, got {:?}",
                            self.curr_token.token_type
                        ),
                    );
                    log::error!("{}", error);
                    self.errors.push(error);
                    return None;
                }
                None => {
                    let error = ParserError::at_token(
                        &self.curr_token,
                        format!("failed to parse parameter after comma"),
                    );
                    log::error!("{}", error);
                    self.errors.push(error);
                    return None;
                }
            };
            parameters.push(identifier);
        }

        if !self.expect_peek(TokenType::RPAREN) {
            // Error already added by expect_peek
            return None;
        }

        Some(parameters)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        log::error!("Parsing call expression");
        let token = self.curr_token.clone();
        let arguments = match self.parse_call_arguments() {
            Some(args) => args,
            None => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!("failed to parse call arguments"),
                );
                log::error!("{}", error);
                self.errors.push(error);
                return None;
            }
        };
        Some(Expression::CallExpression(CallExpression {
            token,
            function: Box::new(function),
            arguments,
        }))
    }
    fn parse_call_arguments(&mut self) -> Option<Vec<Expression>> {
        log::error!("Parsing call arguments");
        let mut arguments = Vec::new();

        if self.is_peek_token(TokenType::RPAREN) {
            self.next_token();
            return Some(arguments);
        }

        self.next_token();
        let first_arg = match self.parse_expression(Precedence::LOWEST as i32) {
            Some(arg) => arg,
            None => {
                let error = ParserError::at_token(
                    &self.curr_token,
                    format!("failed to parse call argument"),
                );
                log::error!("{}", error);
                self.errors.push(error);
                return None;
            }
        };
        arguments.push(first_arg);

        while self.is_peek_token(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            let arg = match self.parse_expression(Precedence::LOWEST as i32) {
                Some(arg) => arg,
                None => {
                    let error = ParserError::at_token(
                        &self.curr_token,
                        format!("failed to parse call argument after comma"),
                    );
                    log::error!("{}", error);
                    self.errors.push(error);
                    return None;
                }
            };
            arguments.push(arg);
        }

        if !self.expect_peek(TokenType::RPAREN) {
            // Error already added by expect_peek, but add additional context
            let error = ParserError::at_token(
                &self.peek_token,
                format!(
                    "unclosed call arguments, got {:?}",
                    self.peek_token.token_type
                ),
            );
            log::error!("{}", error);
            self.errors.push(error);
            return None;
        }

        Some(arguments)
    }
}
