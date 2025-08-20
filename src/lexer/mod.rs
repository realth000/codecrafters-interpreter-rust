use crate::errors::AppResult;

use self::tokens::Tokens;

mod tokens;

pub struct Lexer {
    /// The input token.
    input: Vec<char>,

    /// Total count of chars in `input`.
    length: usize,

    /// Current scan position.
    pos: usize,

    /// Produced tokens.
    tokens: Vec<Tokens>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let input = input.chars().collect::<Vec<char>>();
        let length = input.len();

        Self {
            input,
            length,
            pos: 0,
            tokens: vec![],
        }
    }

    pub fn tokenize(&mut self) -> AppResult<()> {
        self.tokens.clear();

        while let Some(ch) = self.peek() {
            let token: Tokens = Tokens::try_from((ch, self.pos))?;
            self.tokens.push(token);
            self.advance();
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

    fn ended(&self) -> bool {
        self.pos >= self.length
    }

    fn peek(&self) -> Option<&'_ char> {
        if self.ended() {
            return None;
        }
        self.input.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}
