pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;

use env_logger::Builder;
use repl::repl;
use std::io;

fn main() -> io::Result<()> {
    // Initialize logger (can be controlled via RUST_LOG environment variable)
    // Examples: RUST_LOG=debug, RUST_LOG=monkey_lang::parser=debug
    Builder::from_default_env().init();

    let stdin = io::stdin().lock();
    let stdout = io::stdout().lock();

    repl(stdin, stdout)?;
    Ok(())
}
