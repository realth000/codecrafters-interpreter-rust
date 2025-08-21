use crate::errors::{AppError, AppResult};

mod tokens;
pub use tokens::*;

pub struct Lexer {
    /// The input token.
    input: Vec<char>,

    /// Total count of chars in `input`.
    length: usize,

    /// Current scan position.
    pos: usize,

    /// Current line index.
    ///
    /// I want to remember the column position at the same time, but leave it now.
    line_idx: usize,

    /// Produced tokens.
    tokens: Vec<Token>,

    /// Has tokenize error or not.
    has_error: bool,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.chars().collect::<Vec<char>>();
        let length = input.len();

        Self {
            input,
            length,
            pos: 0,
            line_idx: 1,
            tokens: vec![],
            has_error: false,
        }
    }

    pub fn tokenize(&mut self) -> AppResult<()> {
        self.tokens.clear();

        while let Some(ch) = self.peek() {
            match Token::try_consume(&self.input[self.pos..], self.line_idx)? {
                Some(t) => {
                    if let Token::Ignored(IgnoredToken::LineBreak) = t {
                        self.line_idx += 1;
                    }
                    self.advance(t.length());
                    if !t.ignored() {
                        self.tokens.push(t);
                    }
                    continue;
                }
                None => {
                    // Unknown token.
                    self.has_error = true;
                    eprintln!(
                        "{}",
                        AppError::UnexpectedChar {
                            line: self.line_idx,
                            token: ch.to_string(),
                        }
                    );
                    self.advance(1);
                }
            }
        }

        Ok(())
    }

    pub fn print_tokens(&self) {
        for token in self.tokens.iter() {
            let (name, literal, value) = token.info();
            println!("{} {} {}", name, literal, value.unwrap_or("null".into()))
        }
        println!("EOF  null");
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    fn ended(&self) -> bool {
        self.pos >= self.length
    }

    fn peek(&self) -> Option<char> {
        if self.ended() {
            return None;
        }
        self.input.get(self.pos).map(|x| x.to_owned())
    }

    fn advance(&mut self, step: usize) {
        self.pos += step;
    }
}
