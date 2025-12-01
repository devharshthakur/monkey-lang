use monkey_lang::repl::{print_welcome, start};
use std::io;

fn main() -> io::Result<()> {
    let username = users::get_current_username()
        .and_then(|name| name.into_string().ok())
        .unwrap_or_else(|| "unknownuser".to_string());

    print_welcome(&username);

    let stdin = io::stdin().lock();
    let stdout = io::stdout().lock();

    start(stdin, stdout)?;
    Ok(())
}
