pub(super) trait Tokened: Sized {
    /// Get the token info.
    fn info(&self) -> (&'static str, &'static str, Option<String>);

    /// Try parse from character.
    fn from_char(s: &char) -> Option<Self>;
}

pub enum Token {
    SingleCharacter(SingleCharToken),
    Ignored(IgnoredToken),
}

impl Token {
    pub(super) fn info(&self) -> (&'static str, &'static str, Option<String>) {
        match self {
            Token::SingleCharacter(t) => t.info(),
            Token::Ignored(_) => unreachable!("no info provided on ignored tokens"),
        }
    }

    pub(super) fn from_char(s: &char) -> Option<Self> {
        if let Some(v) = SingleCharToken::from_char(s) {
            return Some(Self::SingleCharacter(v));
        }

        if let Some(v) = IgnoredToken::from_char(s) {
            return Some(Self::Ignored(v));
        }

        None
    }

    /// Should ignore the character in AST or not.
    pub(super) fn ignored(&self) -> bool {
        match self {
            Token::SingleCharacter(..) => false,
            Token::Ignored(..) => true,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum SingleCharToken {
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

impl Tokened for SingleCharToken {
    fn info(&self) -> (&'static str, &'static str, Option<String>) {
        match self {
            SingleCharToken::LeftParen => ("LEFT_PAREN", "(", None),
            SingleCharToken::RightParen => ("RIGHT_PAREN", ")", None),
            SingleCharToken::LeftBrace => ("LEFT_BRACE", "{", None),
            SingleCharToken::RightBrace => ("RIGHT_BRACE", "}", None),
            SingleCharToken::Star => ("STAR", "*", None),
            SingleCharToken::Dot => ("DOT", ".", None),
            SingleCharToken::Comma => ("COMMA", ",", None),
            SingleCharToken::Plus => ("PLUS", "+", None),
            SingleCharToken::Minus => ("MINUS", "-", None),
            SingleCharToken::Slash => ("SLASH", "/", None),
            SingleCharToken::Semicolon => ("SEMICOLON", ";", None),
        }
    }

    fn from_char(s: &char) -> Option<Self> {
        match s {
            '(' => Some(Self::LeftParen),
            ')' => Some(Self::RightParen),
            '{' => Some(Self::LeftBrace),
            '}' => Some(Self::RightBrace),
            '*' => Some(Self::Star),
            '.' => Some(Self::Dot),
            ',' => Some(Self::Comma),
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '/' => Some(Self::Slash),
            ';' => Some(Self::Semicolon),
            _ => None,
        }
    }
}

pub(super) enum IgnoredToken {
    /// `\n`
    LineBreak,
}

impl Tokened for IgnoredToken {
    fn info(&self) -> (&'static str, &'static str, Option<String>) {
        unreachable!("no info provided on ignored tokens")
    }

    fn from_char(s: &char) -> Option<Self> {
        match s {
            '\n' => Some(Self::LineBreak),
            _ => None,
        }
    }
}
