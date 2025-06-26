use lexer::token::TokenType;
use lexer::Lexer;
use std::io::BufRead;
use std::io::Write;
use std::io::{self};

const PROMPT: &str = ">>";

pub fn start<R: BufRead, W: Write>(input: R, mut output: W) -> io::Result<()> {
    let mut reader = input;
    let mut line = String::new();

    loop {
        write!(output, "{} ", PROMPT)?;
        output.flush()?;
        line.clear();

        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let trimmed_line = line.trim_end_matches(['\n', '\r']);
        if trimmed_line.is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(trimmed_line.to_string());

        loop {
            let token = lexer.next_token();
            writeln!(output, "{:?}", token)?;
            if token.token_type == TokenType::EOF {
                break;
            }
        }
    }
}
