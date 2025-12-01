use monkey_lang::repl::repl;
use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin().lock();
    let stdout = io::stdout().lock();

    repl(stdin, stdout)?;
    Ok(())
}
