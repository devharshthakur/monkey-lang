use lexer::{token::TokenType, Lexer};
use std::io::{self, BufRead, Write};

// ANSI color codes
const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const GRAY: &str = "\x1b[90m";

const PROMPT: &str = ">>";

pub fn start<R: BufRead, W: Write>(input: R, mut output: W) -> io::Result<()> {
    let mut reader = input;
    let mut line = String::new();

    loop {
        write!(output, "{}{}{} ", CYAN, PROMPT, RESET)?;
        output.flush()?;
        line.clear();

        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            println!("\n{}Goodbye!{}", GRAY, RESET);
            return Ok(());
        }

        let trimmed_line = line.trim_end_matches(['\n', '\r']);
        if trimmed_line.is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(trimmed_line.to_string());

        loop {
            let token = lexer.next_token();
            writeln!(output, "  {}{:?}{}", GRAY, token, RESET)?;
            if token.token_type == TokenType::EOF {
                break;
            }
        }
    }
}
