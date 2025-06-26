use repl::start;
use std::io;

fn main() -> io::Result<()> {
    let username = users::get_current_username()
        .and_then(|name| name.into_string().ok())
        .unwrap_or_else(|| "unknownuser".to_string());

    println!(
        "Hello {} this is monkey programming language \n Feel free to type in commands",
        username
    );

    let stdin = io::stdin().lock();
    let stdout = io::stdout().lock();

    start(stdin, stdout)?;
    Ok(())
}
