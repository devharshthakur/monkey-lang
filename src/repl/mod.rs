//! REPL for the Monkey programming language
//!
//! This module provides a REPL for the Monkey programming language. It allows
//! the user to enter expressions and statements, and it will print the result.
//!
//! # Examples
//!
//! ```
//! let x = 10;
//! println!("x is {}", x);
//! ```
use crate::{lexer::Lexer, parser::Parser};
pub use display::MONKEY_LOGO;
use display::{CYAN, GRAY, RESET, print_parser_errors, print_welcome};
use std::io::{self, BufRead, Write};
mod display;

const PROMPT: &str = ">>";

/// Starts the REPL
/// # Parameters
/// - `input`: The input reader to read the lines from
/// - `output`: The output writer to write the lines to
/// # Returns
/// - `Ok(())` if the REPL was started successfully
/// - `Err(e)` if an error occurred while starting the REPL
pub fn repl<R: BufRead, W: Write>(input: R, mut output: W) -> io::Result<()> {
    print_welcome();

    let mut reader = input;
    let mut line = String::new();

    loop {
        // Print prompt
        write!(output, "{}{}{} ", CYAN, PROMPT, RESET)?;
        output.flush()?;
        line.clear();
        // Read line from input
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            // If no bytes read, print goodbye message and exit
            println!("\n{}Goodbye!{}", GRAY, RESET);
            return Ok(());
        }
        // If line is empty, continue to next iteration
        let trimmed_line = line.trim_end_matches(['\n', '\r']);
        if trimmed_line.is_empty() {
            continue;
        }

        // Create lexer and parse tokens
        let lexer = Lexer::new(trimmed_line.to_string());
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        // If there are parser errors, print them and continue to next iteration
        if !parser.errors.is_empty() {
            print_parser_errors(&mut output, &parser.errors)?;
            continue;
        }

        println!("{}", program);
    }
}
