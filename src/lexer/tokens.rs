use crate::errors::AppError;

#[derive(Debug, Clone)]
pub enum Tokens {
    /// `(`
    LeftParen,

    /// `)`
    RightParen,

    /// `{`
    LeftBrace,

    /// `}`
    RightBrace,

    /// `*`
    Star,

    /// `.`
    Dot,

    /// `,`
    Comma,

    /// `+`
    Plus,

    /// `-`
    Minus,

    /// `/`
    Slash,

    /// `;`
    Semicolon,
}

impl Tokens {
    pub fn info(&self) -> (&'static str, &'static str, Option<String>) {
        match self {
            Tokens::LeftParen => ("LEFT_PAREN", "(", None),
            Tokens::RightParen => ("RIGHT_PAREN", ")", None),
            Tokens::LeftBrace => ("LEFT_BRACE", "{", None),
            Tokens::RightBrace => ("RIGHT_BRACE", "}", None),
            Tokens::Star => ("STAR", "*", None),
            Tokens::Dot => ("DOT", ".", None),
            Tokens::Comma => ("COMMA", ",", None),
            Tokens::Plus => ("PLUS", "+", None),
            Tokens::Minus => ("MINUS", "-", None),
            Tokens::Slash => ("SLASH", "/", None),
            Tokens::Semicolon => ("SEMICOLON", ";", None),
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
            '{' => Ok(Self::LeftBrace),
            '}' => Ok(Self::RightBrace),
            '*' => Ok(Self::Star),
            '.' => Ok(Self::Dot),
            ',' => Ok(Self::Comma),
            '+' => Ok(Self::Plus),
            '-' => Ok(Self::Minus),
            '/' => Ok(Self::Slash),
            ';' => Ok(Self::Semicolon),
            v => Err(AppError::UnknownToken {
                pos: value.1,
                token: v.to_string(),
            }),
        }
    }
}
