use anyhow::{bail, Context};

use crate::errors::AppResult;
use crate::lexer::{KeywordToken, Token};

pub(super) enum Expr {
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Value(Value),
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

    pub fn print_info(&self) {
        println!("{}", self.literal());
    }

    fn literal(&self) -> String {
        match self {
            Expr::Binary { op, lhs, rhs } => {
                format!("({} {} {})", op.literal(), lhs.literal(), rhs.literal())
            }
            Expr::Value(v) => v.literal(),
        }
    }
}

/// Operator that accepts two operands.
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

/// The value evaluating on.
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
