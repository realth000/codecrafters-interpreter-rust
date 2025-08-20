use crate::errors::AppError;

#[derive(Debug, Clone)]
pub enum Tokens {
    LeftParen,
    RightParen,
}

impl Tokens {
    pub fn info(&self) -> (&'static str, &'static str, Option<String>) {
        match self {
            Tokens::LeftParen => ("LEFT_PAREN", "(", None),
            Tokens::RightParen => ("RIGHT_PAREN", ")", None),
        }
    }
}

/// Parse character into `Tokens`.
///
/// If token is invalid, return `AppError::UnknownToken` with given position at `usize`.
impl<'a> TryFrom<(&'a char, usize)> for Tokens {
    type Error = AppError;

    fn try_from(value: (&'a char, usize)) -> Result<Self, Self::Error> {
        match value.0 {
            '(' => Ok(Self::LeftParen),
            ')' => Ok(Self::RightParen),
            v => Err(AppError::UnknownToken {
                pos: value.1,
                token: v.to_string(),
            }),
        }
    }
}
