use anyhow::{bail, Context};

use crate::errors::AppResult;
use crate::lexer::{KeywordToken, SingleCharToken, Token};

#[derive(Debug, Clone)]
pub(super) enum Expr {
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Value(Value),
    Scope(Scope),
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
}

impl Expr {
    pub(super) fn new_binary<'a>(
        op: BinaryOp,
        lhs: Option<&'a Token>,
        rhs: Option<&'a Token>,
    ) -> AppResult<Self> {
        let lhs = lhs.context("lhs is null")?;
        let rhs = rhs.context("lhs is null")?;

        let expr = Expr::Binary {
            op,
            lhs: Box::new(Expr::new_value(lhs)?),
            rhs: Box::new(Expr::new_value(rhs)?),
        };
        Ok(expr)
    }

    pub(super) fn new_value<'a>(v: &'a Token) -> AppResult<Self> {
        Ok(Expr::Value(Value::try_from(v)?))
    }

    pub fn new_scope(scope_type: ScopeType, expr: Option<Expr>) -> AppResult<Self> {
        Ok(Expr::Scope(Scope {
            scope_type,
            expr: expr.map(Box::new),
        }))
    }

    pub fn new_unary<'a>(unary_type: UnaryOp, operand: Option<&'a Token>) -> AppResult<Self> {
        let operand = operand.context("operand is null")?;

        Ok(Expr::Unary {
            op: unary_type,
            operand: Box::new(Expr::new_value(operand)?),
        })
    }

    pub fn new_unary_from_expr(unary_type: UnaryOp, expr: Expr) -> Self {
        Expr::Unary {
            op: unary_type,
            operand: Box::new(expr),
        }
    }

    pub fn print_info(&self) {
        println!("{}", self.literal());
    }

    fn literal(&self) -> String {
        match self {
            Expr::Binary { op, lhs, rhs } => {
                format!("({} {} {})", op.literal(), lhs.literal(), rhs.literal())
            }
            Expr::Value(v) => v.literal(),
            Expr::Scope(s) => s.literal(),
            Expr::Unary { op, operand } => format!("({} {})", op.literal(), operand.literal()),
        }
    }
}

/// Operator that accepts two operands.
#[derive(Debug, Clone)]
pub(super) enum BinaryOp {
    /// `lhs + rhs`
    Plus,

    /// `lhs - rhs`
    Minus,

    /// `lhs * rhs`
    Multiply,

    /// `lhs / rhs`
    Divide,
}

impl BinaryOp {
    const fn literal(&self) -> &'static str {
        match self {
            BinaryOp::Plus => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
        }
    }
}

impl<'a> TryFrom<&'a Token> for BinaryOp {
    type Error = anyhow::Error;

    fn try_from(value: &'a Token) -> Result<Self, Self::Error> {
        match value {
            Token::SingleCharacter(s) => match s {
                SingleCharToken::LeftParen
                | SingleCharToken::RightParen
                | SingleCharToken::LeftBrace
                | SingleCharToken::Dot
                | SingleCharToken::Comma
                | SingleCharToken::Semicolon
                | SingleCharToken::Assign
                | SingleCharToken::Bang
                | SingleCharToken::RightBrace => unreachable!("check before convert"),
                SingleCharToken::Star => Ok(BinaryOp::Multiply),
                SingleCharToken::Plus => Ok(BinaryOp::Plus),
                SingleCharToken::Minus => Ok(BinaryOp::Minus),
                SingleCharToken::Slash => Ok(BinaryOp::Divide),
                SingleCharToken::Less => todo!(),
                SingleCharToken::Greater => todo!(),
            },
            Token::String(..)
            | Token::Number(..)
            | Token::Identifier(..)
            | Token::Keyword(..)
            | Token::Ignored(..) => unreachable!("check before convert"),
            Token::MultiCharToken(..) => todo!(),
        }
    }
}

/// The value evaluating on.
#[derive(Debug, Clone)]
pub(super) enum Value {
    /// Number.
    ///
    /// Int or float.
    Number { value: f64, info: String },

    /// String value.
    String { value: String, info: String },

    /// Bool value.
    Bool(bool),

    /// The nil value.
    Nil,
}

impl Value {
    fn literal(&self) -> String {
        match self {
            Value::Number { info, .. } => format!("{info}"),
            Value::String { info, .. } => info.clone(),
            Value::Bool(v) => v.to_string(),
            Value::Nil => "nil".into(),
        }
    }
}

impl<'a> TryFrom<&'a Token> for Value {
    type Error = anyhow::Error;

    fn try_from(value: &'a Token) -> Result<Self, Self::Error> {
        match value {
            Token::String(s) => Ok(Value::String {
                value: s.0.to_owned(),
                info: s.0.to_owned(),
            }),
            Token::Number(n) => Ok(Value::Number {
                value: n.as_f64(),
                info: n.info_string(),
            }),
            Token::Keyword(k) => match k {
                KeywordToken::KAnd => todo!(),
                KeywordToken::KClass => todo!(),
                KeywordToken::KElse => todo!(),
                KeywordToken::KFalse => Ok(Value::Bool(false)),
                KeywordToken::KFor => todo!(),
                KeywordToken::KFun => todo!(),
                KeywordToken::KIf => todo!(),
                KeywordToken::KNil => Ok(Value::Nil),
                KeywordToken::KOr => todo!(),
                KeywordToken::KPrint => todo!(),
                KeywordToken::KReturn => todo!(),
                KeywordToken::KSuper => todo!(),
                KeywordToken::KThis => todo!(),
                KeywordToken::KTrue => Ok(Value::Bool(true)),
                KeywordToken::KVar => todo!(),
                KeywordToken::KWhile => todo!(),
            },
            v => bail!("invalid value {v:?}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScopeType {
    Paren,
}

#[derive(Debug, Clone)]
pub struct Scope {
    scope_type: ScopeType,
    expr: Option<Box<Expr>>,
}

impl Scope {
    pub fn literal(&self) -> String {
        match self.scope_type {
            ScopeType::Paren => format!(
                "(group {})",
                self.expr
                    .as_ref()
                    .map(|x| x.as_ref().literal())
                    .unwrap_or_else(String::new)
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    /// `-`
    Negation,

    /// `!`
    LogicalNot,
}

impl UnaryOp {
    fn literal(&self) -> &'static str {
        match self {
            UnaryOp::Negation => "-",
            UnaryOp::LogicalNot => "!",
        }
    }
}

impl<'a> TryFrom<&'a Token> for UnaryOp {
    type Error = anyhow::Error;

    fn try_from(value: &'a Token) -> Result<Self, Self::Error> {
        match value {
            Token::SingleCharacter(s) => match s {
                SingleCharToken::LeftParen
                | SingleCharToken::RightParen
                | SingleCharToken::LeftBrace
                | SingleCharToken::RightBrace
                | SingleCharToken::Star
                | SingleCharToken::Dot
                | SingleCharToken::Comma
                | SingleCharToken::Plus
                | SingleCharToken::Slash
                | SingleCharToken::Semicolon
                | SingleCharToken::Assign
                | SingleCharToken::Less
                | SingleCharToken::Greater => unreachable!("check before convert"),
                SingleCharToken::Minus => Ok(Self::Negation),
                SingleCharToken::Bang => Ok(Self::LogicalNot),
            },
            Token::MultiCharToken(..)
            | Token::Ignored(..)
            | Token::String(..)
            | Token::Number(..)
            | Token::Identifier(..)
            | Token::Keyword(..) => unreachable!("check before convert"),
        }
    }
}
