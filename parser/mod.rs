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
pub mod test_helper;

use crate::ast::{
    expression::{
        BlockStatement, BooleanLiteral, Expression, FunctionLiteral, Identifier, IfExpression,
        InfixExpression, IntegerLiteral, PrefixExpression,
    },
    statement::{ExpressionStatement, LetStatement, ReturnStatement, Statement},
    Program,
};
use crate::lexer::{
    token::{Token, TokenType},
    Lexer,
};
use precedence::Precedence;
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
            _ => self.parse_expression_statement().map(Statement::Expression),
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
    fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let expr = self.parse_expression(Precedence::LOWEST as i32)?;
        let stmt = ExpressionStatement {
            token: self.curr_token.clone(),
            value: expr,
        };

        // Optional semicolon for REPL
        if self.is_peek_token(TokenType::SEMICOLON) {
            self.next_token();
        }
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
        let token_type = self.curr_token.token_type.clone();
        let prefix = self.prefix_parse_fns.get(&token_type);
        // If the prefix parse function is found, parse the left-hand side expression and returns an Expression
        let mut left = if let Some(prefix_parse_fn) = prefix {
            let left_exp = prefix_parse_fn(self)?;
            left_exp
        } else {
            self.no_prefix_parse_function_error(token_type);
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
    /// Adds an error to the parser's error list if the right parenthesis is not found.
    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        // Expects the current token to be a left parenthesis. skips it and advances the token
        self.next_token();

        let expr = self.parse_expression(Precedence::LOWEST as i32)?;

        if !self.expect_peek(TokenType::RPAREN) {
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
        let token = self.curr_token.clone();

        if !self.expect_peek(TokenType::LPAREN) {
            return None;
        }
        // Advance to the next token to point to the condition and parse it with the lowest precedence
        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST as i32)?;
        // Expects the next token to be a right parenthesis
        if !self.expect_peek(TokenType::RPAREN) {
            return None;
        }
        // Expects the next token to be a left brace
        if !self.expect_peek(TokenType::LBRACE) {
            return None;
        }
        // Parse the consequence block statement and returns a BlockStatement
        let consequence = self.parse_block_statement()?;
        // If the next token is an else keyword, parse the alternative block statement
        let alternative = if self.is_peek_token(TokenType::ELSE) {
            self.next_token();

            if !self.expect_peek(TokenType::LBRACE) {
                return None;
            }
            // Parse the alternative block statement and returns a BlockStatement
            let alternative = self.parse_block_statement()?;
            Some(Box::new(alternative))
        } else {
            None
        };
        Some(Expression::IfExpression(IfExpression {
            token,
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative,
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
    /// Adds an error to the parser's error list if the right brace is not found.
    fn parse_block_statement(&mut self) -> Option<Expression> {
        let token = self.curr_token.clone();
        let mut statements = Vec::new();
        // Parse the statements in the block until the right brace is found or EOF is reached
        while !self.is_peek_token(TokenType::RBRACE) && !self.is_peek_token(TokenType::EOF) {
            self.next_token();
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        if !self.expect_peek(TokenType::RBRACE) {
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
        let token = self.curr_token.clone();
        if !self.expect_peek(TokenType::LPAREN) {
            return None;
        }
        let parameters = self.parse_function_parameters()?;
        if !self.expect_peek(TokenType::LBRACE) {
            return None;
        }
        let body = match self.parse_block_statement()? {
            Expression::BlockStatement(block_stmt) => block_stmt,
            _ => return None,
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
        let mut parameters = Vec::new();
        if self.is_peek_token(TokenType::RPAREN) {
            self.next_token();
            return Some(parameters);
        }
        // Advance to the next token to point to the first parameter
        self.next_token();

        // Parse first parameter
        let first_param = match self.parse_identifier()? {
            Expression::Identifier(ident) => ident,
            _ => return None,
        };
        parameters.push(first_param);

        // Parse remaining parameters
        while self.is_peek_token(TokenType::COMMA) {
            self.next_token();
            self.next_token();

            let identifier = match self.parse_identifier()? {
                Expression::Identifier(ident) => ident,
                _ => return None,
            };
            parameters.push(identifier);
        }

        if !self.expect_peek(TokenType::RPAREN) {
            return None;
        }

        Some(parameters)
    }
}
