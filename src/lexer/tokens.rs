pub(super) trait Tokened: Sized {
    /// Get the token info.
    fn info(&self) -> (&'static str, &'static str, Option<String>);

    /// Try parse from character.
    fn from_char_slice(s: &[char]) -> Option<Self>;

    /// Get the characters count of current token.
    fn length(&self) -> usize;
}

pub enum Token {
    SingleCharacter(SingleCharToken),
    MultiCharToken(MultiCharToken),
    Ignored(IgnoredToken),
}

impl Token {
    pub(super) fn info(&self) -> (&'static str, &'static str, Option<String>) {
        match self {
            Token::SingleCharacter(t) => t.info(),
            Token::MultiCharToken(t) => t.info(),
            Token::Ignored(_) => unreachable!("no info provided on ignored tokens"),
        }
    }

    pub(super) fn try_consume(s: &[char]) -> Option<Self> {
        if s.is_empty() {
            return None;
        }

        // Multi characters
        if let Some(v) = MultiCharToken::from_char_slice(s) {
            return Some(Self::MultiCharToken(v));
        }

        if let Some(v) = IgnoredToken::from_char_slice(s) {
            return Some(Self::Ignored(v));
        }

        if let Some(v) = SingleCharToken::from_char_slice(s) {
            return Some(Self::SingleCharacter(v));
        }

        None
    }

    /// Should ignore the character in AST or not.
    pub(super) fn ignored(&self) -> bool {
        match self {
            Token::SingleCharacter(..) => false,
            Token::MultiCharToken(..) => false,
            Token::Ignored(..) => true,
        }
    }

    pub(super) fn length(&self) -> usize {
        match self {
            Token::SingleCharacter(v) => v.length(),
            Token::MultiCharToken(v) => v.length(),
            Token::Ignored(v) => v.length(),
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

    /// `=`
    Assign,

    /// `!`
    Bang,

    /// `<`
    Less,

    /// `>`
    Greater,
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
            SingleCharToken::Assign => ("EQUAL", "=", None),
            SingleCharToken::Bang => ("BANG", "!", None),
            SingleCharToken::Less => ("LESS", "<", None),
            SingleCharToken::Greater => ("GREATER", ">", None),
        }
    }

    fn from_char_slice(s: &[char]) -> Option<Self> {
        let s = s.get(0)?;
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
            '=' => Some(Self::Assign),
            '!' => Some(Self::Bang),
            '<' => Some(Self::Less),
            '>' => Some(Self::Greater),
            _ => None,
        }
    }

    fn length(&self) -> usize {
        // Single character token always only have 1 character.
        1
    }
}

pub(super) enum IgnoredToken {
    /// `\n`
    LineBreak,

    /// `//`
    ///
    ///
    /// Holding the comment length till the end of current line.
    Comment(usize),

    /// `\t`
    Tab,

    /// ' '
    Space,
}

impl Tokened for IgnoredToken {
    fn info(&self) -> (&'static str, &'static str, Option<String>) {
        unreachable!("no info provided on ignored tokens")
    }

    fn from_char_slice(s: &[char]) -> Option<Self> {
        match (s.get(0), s.get(1)) {
            (Some('\n'), _) => Some(Self::LineBreak),
            (Some('\t'), _) => Some(Self::Tab),
            (Some(' '), _) => Some(Self::Space),
            (Some('/'), Some('/')) => Some(Self::Comment(
                s.iter().position(|x| x == &'\n').unwrap_or_else(|| s.len()),
            )),
            _ => None,
        }
    }

    fn length(&self) -> usize {
        match self {
            IgnoredToken::LineBreak => 1,
            IgnoredToken::Comment(len) => len.to_owned(),
            IgnoredToken::Tab => 1,
            IgnoredToken::Space => 1,
        }
    }
}

pub(super) enum MultiCharToken {
    /// `==`
    EqualEqual,

    /// `!=`
    BangEqual,

    /// `<=`
    LessEqual,

    /// `>=`
    GreaterEqual,
}

impl Tokened for MultiCharToken {
    fn info(&self) -> (&'static str, &'static str, Option<String>) {
        match self {
            MultiCharToken::EqualEqual => ("EQUAL_EQUAL", "==", None),
            MultiCharToken::BangEqual => ("BANG_EQUAL", "!=", None),
            MultiCharToken::LessEqual => ("LESS_EQUAL", "<=", None),
            MultiCharToken::GreaterEqual => ("GREATER_EQUAL", ">=", None),
        }
    }

    fn from_char_slice(s: &[char]) -> Option<Self> {
        match (s.get(0), s.get(1)) {
            (Some('='), Some('=')) => Some(Self::EqualEqual),
            (Some('!'), Some('=')) => Some(Self::BangEqual),
            (Some('<'), Some('=')) => Some(Self::LessEqual),
            (Some('>'), Some('=')) => Some(Self::GreaterEqual),
            _ => None,
        }
    }

    fn length(&self) -> usize {
        // Till now, all multiple charaters token have 2 characters.
        2
    }
}
