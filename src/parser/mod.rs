use std::thread::current;

use crate::errors::AppResult;
use crate::lexer::{KeywordToken, SingleCharToken, Token};

use anyhow::{bail, Ok};
use expr::Expr;

use self::expr::BinaryOp;

mod expr;

pub struct Parser<'a> {
    /// Tokens to parse.
    input: &'a Vec<Token>,

    /// Count of tokens in `input`.
    length: usize,

    /// Current parsing postion.
    pos: usize,

    /// The start position of current parsing statement.
    begin: usize,

    /// The output.
    output: Vec<Expr>,

    /// Current parsed tokens that waiting to form an expr.
    curr: Vec<&'a Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a Vec<Token>) -> Self {
        let length = input.len();
        Self {
            input,
            length,
            pos: 0,
            begin: 0,
            output: vec![],
            curr: vec![],
        }
    }

    pub fn parse(&mut self) -> AppResult<()> {
        while !self.finished() {
            let step = self.collapse()?;
            if step > 0 {
                self.begin += step;
            } else if step == 0 {
                bail!("?????")
            }
            self.advance_pos(step);
        }
        for t in self.curr.iter() {
            self.output.push(Expr::new_value(t)?);
        }

        Ok(())
    }

    pub fn print_info(&self) {
        for expr in self.output.iter() {
            expr.print_info();
        }
    }

    fn finished(&self) -> bool {
        self.begin > self.length - 1 && self.pos > self.length - 1
    }

    fn advance_pos(&mut self, step: usize) {
        self.pos += step;
    }

    fn collapse(&mut self) -> AppResult<usize> {
        let tokens = &self.input[self.begin..];
        match &tokens[0] {
            Token::SingleCharacter(t) => match t {
                SingleCharToken::LeftParen => todo!(),
                SingleCharToken::RightParen => todo!(),
                SingleCharToken::LeftBrace => todo!(),
                SingleCharToken::RightBrace => todo!(),
                SingleCharToken::Star => todo!(),
                SingleCharToken::Dot => todo!(),
                SingleCharToken::Comma => todo!(),
                SingleCharToken::Plus => {
                    self.curr.push(&tokens[0]);
                    return Ok(1);
                }
                SingleCharToken::Minus => {
                    self.curr.push(&tokens[0]);
                    return Ok(1);
                }
                SingleCharToken::Slash => todo!(),
                SingleCharToken::Semicolon => todo!(),
                SingleCharToken::Assign => todo!(),
                SingleCharToken::Bang => todo!(),
                SingleCharToken::Less => todo!(),
                SingleCharToken::Greater => todo!(),
            },
            Token::MultiCharToken(..) => {
                self.curr.push(&tokens[0]);
                return Ok(1);
            }
            Token::Ignored(..) => {
                /* Do nothing */
                return Ok(1);
            }
            Token::String(..) | Token::Number(..) => {
                match self.curr.last() {
                    Some(v) => {
                        if v.is_binary_op() {
                            self.output.push(Expr::new_binary(
                                BinaryOp::Plus,
                                self.curr.get(self.curr.len() - 2).map(|x| *x),
                                Some(&tokens[0]),
                            )?);
                            self.curr.pop();
                            self.curr.pop();
                        }
                    }
                    None => {
                        /* Do nothing */
                        self.curr.push(&tokens[0]);
                    }
                }
                return Ok(1);
            }
            Token::Identifier(identifier_token) => todo!(),
            Token::Keyword(v) => match v {
                KeywordToken::KAnd => todo!(),
                KeywordToken::KClass => todo!(),
                KeywordToken::KElse => todo!(),
                KeywordToken::KFalse => {
                    self.curr.push(&tokens[0]);
                    return Ok(1);
                }
                KeywordToken::KFor => todo!(),
                KeywordToken::KFun => todo!(),
                KeywordToken::KIf => todo!(),
                KeywordToken::KNil => {
                    self.curr.push(&tokens[0]);
                    return Ok(1);
                }
                KeywordToken::KOr => todo!(),
                KeywordToken::KPrint => todo!(),
                KeywordToken::KReturn => todo!(),
                KeywordToken::KSuper => todo!(),
                KeywordToken::KThis => todo!(),
                KeywordToken::KTrue => {
                    self.curr.push(&tokens[0]);
                    return Ok(1);
                }
                KeywordToken::KVar => todo!(),
                KeywordToken::KWhile => todo!(),
            },
        }
    }
}
