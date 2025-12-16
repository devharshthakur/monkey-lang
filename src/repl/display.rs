//! Display module for the Monkey programming language REPL
//!
//! This module provides functions to print the welcome message, parser errors,
//! and other display related functions for the Monkey programming language REPL.
//!
//! # Features
//!
//! - `print_welcome`: Prints the welcome message to the output
//! - `print_parser_errors`: Prints the parser errors to the output
//!
//! # Types
//!
//! - `MONKEY_LOGO`: The Monkey logo
//! - `RESET`: The reset color code
use crate::parser::error::ParserError;
use colored::Colorize;
use std::io::{Result, Write};

pub const MONKEY_LOGO: &str = r#"            
        .--.  .-"-----"-.  .--.
       / .. \/  .-. .-.  \/ .. \
      | |  '|  /   Y   \  |'  | |
      | \   \  \ 0 | 0 /  /   / |
       \ '- ,\.-"""""""-./, -' /
        ''-' /_   ^ ^   _\ '-''
            |  \._   _./  |
            \   \ '~' /   /
             '._ '-=-' _.'
                '-----'
"#;

// ANSI color codes
pub const RESET: &str = "\x1b[0m";
pub const CYAN: &str = "\x1b[36m";
pub const GRAY: &str = "\x1b[90m";

pub fn print_welcome() {
    // Get username for welcome message
    let username = users::get_current_username()
        .and_then(|name| name.into_string().ok())
        .unwrap_or_else(|| "unknownuser".to_string());

    // Print banner
    println!("\n{}", MONKEY_LOGO.cyan());
    println!("  {}\n", "Monkey Programming Language".cyan().bold());

    println!("{}", format!("Welcome, {}!", username).green().bold());
    println!(
        "{} {}",
        "Status:".red(),
        "🔧 Currently in development".yellow()
    );
    println!(
        "{} {}",
        "REPL:".bright_black(),
        "Currently Lexer and Parser is functional".green()
    );
    println!(
        "{} {}\n",
        "Note:".bright_black(),
        "📝 Not all features are implemented yet".blue()
    );

    println!(
        "{}\n",
        "Type your commands below. Press Ctrl+D or Ctrl+C to exit.".bright_black()
    );
}

/// Prints parser errors to the output
/// # Parameters
/// - `output`: The output writer to write the errors to
/// - `errors`: The vector of errors to print
/// # Returns
/// - `Ok(())` if the errors were printed successfully
/// - `Err(e)` if an error occurred while printing the errors
pub fn print_parser_errors<W: Write>(output: &mut W, errors: &Vec<ParserError>) -> Result<()> {
    writeln!(
        output,
        "{}",
        "Woops! We ran into some monkey business here!".red().bold()
    )?;
    writeln!(output, " parser errors:")?;
    for error in errors {
        writeln!(output, "  {}{}{}", GRAY, error, RESET)?;
    }
    Ok(())
}
