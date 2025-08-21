use crate::errors::AppResult;
use crate::lexer::{KeywordToken, SingleCharToken, Token};

use anyhow::{bail, Ok};
use expr::Expr;

use self::expr::{BinaryOp, ScopeType, UnaryOp};

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
        let mut with_tokens: Option<Vec<&'a Token>> = None;
        while !self.finished() {
            let (t, step) = self.collapse(&self.input[self.begin..], with_tokens)?;
            with_tokens = t;
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

    fn collapse(
        &mut self,
        tokens: &'a [Token],
        with_tokens: Option<Vec<&'a Token>>,
    ) -> AppResult<(Option<Vec<&'a Token>>, usize)> {
        let mut parse_buf = with_tokens.unwrap_or_default();
        let (expr, step) = Self::parse_expr(tokens, &mut parse_buf)?;
        match expr {
            Some(v) => {
                self.output.push(v);
                Ok((None, step))
            }
            None => {
                if parse_buf.is_empty() {
                    bail!("no expr or token produced when parsing expr")
                } else {
                    Ok((Some(parse_buf), step))
                }
            }
        }
    }

    fn parse_expr(
        tokens: &'a [Token],
        parsed_tokens: &mut Vec<&'a Token>,
    ) -> AppResult<(Option<Expr>, usize)> {
        match &tokens[0] {
            Token::SingleCharacter(t) => match t {
                SingleCharToken::LeftParen => {
                    let mut parsed_tokens = vec![];
                    let (parsed_expr, step) = Self::parse_expr(&tokens[1..], &mut parsed_tokens)?;
                    if !parsed_tokens.is_empty() {
                        bail!("token left in paren")
                    }
                    match tokens.get(1 + step) {
                        Some(v) if v == &Token::SingleCharacter(SingleCharToken::RightParen) => {
                            return Ok((
                                Some(Expr::new_scope(ScopeType::Paren, parsed_expr)?),
                                step + 1 + 1,
                            ));
                        }
                        _ => bail!("paren not ended"),
                    }
                }
                SingleCharToken::RightParen => {
                    return Ok((None, 1));
                }
                SingleCharToken::LeftBrace => todo!(),
                SingleCharToken::RightBrace => todo!(),
                SingleCharToken::Star => todo!(),
                SingleCharToken::Dot => todo!(),
                SingleCharToken::Comma => todo!(),
                SingleCharToken::Plus => {
                    parsed_tokens.push(&tokens[0]);
                    return Ok((None, 1));
                }
                SingleCharToken::Minus => {
                    parsed_tokens.push(&tokens[0]);
                    return Ok((None, 1));
                }
                SingleCharToken::Slash => todo!(),
                SingleCharToken::Semicolon => todo!(),
                SingleCharToken::Assign => todo!(),
                SingleCharToken::Bang => {
                    parsed_tokens.push(&tokens[0]);
                    return Ok((None, 1));
                }
                SingleCharToken::Less => todo!(),
                SingleCharToken::Greater => todo!(),
            },
            Token::MultiCharToken(..) => {
                parsed_tokens.push(&tokens[0]);
                return Ok((None, 1));
            }
            Token::Ignored(..) => {
                /* Do nothing */
                return Ok((None, 1));
            }
            Token::String(..) | Token::Number(..) => {
                match parsed_tokens.last() {
                    Some(v) => {
                        let lhs = parsed_tokens.iter().rev().nth(1);
                        // Binary operator needs an valid lhs.
                        if v.is_binary_op() && lhs.is_some_and(|t| t.is_string_or_number()) {
                            let expr = Expr::new_binary(
                                BinaryOp::try_from(*v).unwrap(),
                                lhs.map(|x| *x),
                                Some(&tokens[0]),
                            )?;
                            parsed_tokens.pop();
                            parsed_tokens.pop();
                            return Ok((Some(expr), 1));
                        } else if v.is_unary_op() {
                            let expr =
                                Expr::new_unary(UnaryOp::try_from(*v).unwrap(), Some(&tokens[0]))?;
                            parsed_tokens.pop();
                            return Ok((Some(expr), 1));
                        } else {
                            bail!("adjacent token before string/number is not BinaryOp")
                        }
                    }
                    None => {
                        /* Do nothing */
                        return Ok((Some(Expr::new_value(&tokens[0])?), 1));
                    }
                }
            }
            Token::Identifier(identifier_token) => todo!(),
            Token::Keyword(v) => match v {
                KeywordToken::KAnd => todo!(),
                KeywordToken::KClass => todo!(),
                KeywordToken::KElse => todo!(),
                KeywordToken::KFalse | KeywordToken::KTrue => {
                    if parsed_tokens.last().is_some_and(|x| x.is_unary_op()) {
                        let op = parsed_tokens.pop().unwrap();
                        let expr =
                            Expr::new_unary(UnaryOp::try_from(op).unwrap(), Some(&tokens[0]))?;
                        return Ok((Some(expr), 1));
                    }
                    return Ok((Some(Expr::new_value(&tokens[0])?), 1));
                }
                KeywordToken::KFor => todo!(),
                KeywordToken::KFun => todo!(),
                KeywordToken::KIf => todo!(),
                KeywordToken::KNil => {
                    return Ok((Some(Expr::new_value(&tokens[0])?), 1));
                }
                KeywordToken::KOr => todo!(),
                KeywordToken::KPrint => todo!(),
                KeywordToken::KReturn => todo!(),
                KeywordToken::KSuper => todo!(),
                KeywordToken::KThis => todo!(),
                KeywordToken::KVar => todo!(),
                KeywordToken::KWhile => todo!(),
            },
        }
    }
}
