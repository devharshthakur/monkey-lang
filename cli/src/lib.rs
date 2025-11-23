use colored::Colorize;

pub const MONKEY_FACE: &str = r#"            
        .--.  .-"__,__"-.  .--.
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

pub fn print_welcome(username: &str) {
    // Print banner
    println!("\n{}", MONKEY_FACE.cyan());
    println!("  {}\n", "Monkey Programming Language".cyan().bold());

    println!("{}", format!("Welcome, {}!", username).green().bold());
    println!(
        "{} {}",
        "Status:".bright_black(),
        "Currently in development".yellow()
    );
    println!(
        "{} {}",
        "REPL:".bright_black(),
        "Lexer is functional".green()
    );
    println!(
        "{} {}\n",
        "Note:".bright_black(),
        "Not all features are implemented yet".blue()
    );

    println!(
        "{}\n",
        "Type your commands below. Press Ctrl+D or Ctrl+C to exit.".bright_black()
    );
}
