use std::env;
use std::fs;
use std::io::{self, Write};

use anyhow::Context;
use anyhow::Ok;

use self::errors::AppResult;
use self::lexer::Lexer;

mod errors;
mod lexer;

fn main() -> AppResult<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return Ok(());
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let input = fs::read_to_string(filename).context("failed to read file")?;
            let mut lexer = Lexer::new(input);
            if let Err(e) = lexer.tokenize() {
                eprintln!("{}", e);
                lexer.print_tokens();
                std::process::exit(65);
            } else {
                lexer.print_tokens();
                if lexer.has_error() {
                    std::process::exit(65);
                }
            }
            return Ok(());
        }
        "tokenize-text" => {
            let mut lexer = Lexer::new(filename.to_string());
            if let Err(e) = lexer.tokenize() {
                eprintln!("{}", e);
                lexer.print_tokens();
                std::process::exit(65);
            } else {
                lexer.print_tokens();
                if lexer.has_error() {
                    std::process::exit(65);
                }
            }
            return Ok(());
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return Ok(());
        }
    }
}
