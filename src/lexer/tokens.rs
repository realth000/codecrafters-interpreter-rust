use crate::errors::{AppError, AppResult};

pub(super) trait Tokened: Sized {
    /// Get the token info.
    fn info(&self) -> (&'static str, String, Option<String>);

    /// Try parse from character.
    fn from_char_slice(s: &[char], line: usize) -> AppResult<Option<Self>>;

    /// Get the characters count of current token.
    fn length(&self) -> usize;
}

pub enum Token {
    SingleCharacter(SingleCharToken),
    MultiCharToken(MultiCharToken),
    Ignored(IgnoredToken),
    String(StringToken),
}

impl Token {
    pub(super) fn info(&self) -> (&'static str, String, Option<String>) {
        match self {
            Token::SingleCharacter(t) => t.info(),
            Token::MultiCharToken(t) => t.info(),
            Token::Ignored(_) => unreachable!("no info provided on ignored tokens"),
            Token::String(t) => t.info(),
        }
    }

    pub(super) fn try_consume(s: &[char], line: usize) -> AppResult<Option<Self>> {
        if s.is_empty() {
            return Ok(None);
        }

        if let Some(v) = StringToken::from_char_slice(s, line)? {
            return Ok(Some(Self::String(v)));
        }

        // Multi characters
        if let Some(v) = MultiCharToken::from_char_slice(s, line)? {
            return Ok(Some(Self::MultiCharToken(v)));
        }

        if let Some(v) = IgnoredToken::from_char_slice(s, line)? {
            return Ok(Some(Self::Ignored(v)));
        }

        if let Some(v) = SingleCharToken::from_char_slice(s, line)? {
            return Ok(Some(Self::SingleCharacter(v)));
        }

        Ok(None)
    }

    /// Should ignore the character in AST or not.
    pub(super) fn ignored(&self) -> bool {
        match self {
            Token::SingleCharacter(..) => false,
            Token::MultiCharToken(..) => false,
            Token::Ignored(..) => true,
            Token::String(..) => false,
        }
    }

    pub(super) fn length(&self) -> usize {
        match self {
            Token::SingleCharacter(v) => v.length(),
            Token::MultiCharToken(v) => v.length(),
            Token::Ignored(v) => v.length(),
            Token::String(v) => v.length(),
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
    fn info(&self) -> (&'static str, String, Option<String>) {
        match self {
            SingleCharToken::LeftParen => ("LEFT_PAREN", "(".into(), None),
            SingleCharToken::RightParen => ("RIGHT_PAREN", ")".into(), None),
            SingleCharToken::LeftBrace => ("LEFT_BRACE", "{".into(), None),
            SingleCharToken::RightBrace => ("RIGHT_BRACE", "}".into(), None),
            SingleCharToken::Star => ("STAR", "*".into(), None),
            SingleCharToken::Dot => ("DOT", ".".into(), None),
            SingleCharToken::Comma => ("COMMA", ",".into(), None),
            SingleCharToken::Plus => ("PLUS", "+".into(), None),
            SingleCharToken::Minus => ("MINUS", "-".into(), None),
            SingleCharToken::Slash => ("SLASH", "/".into(), None),
            SingleCharToken::Semicolon => ("SEMICOLON", ";".into(), None),
            SingleCharToken::Assign => ("EQUAL", "=".into(), None),
            SingleCharToken::Bang => ("BANG", "!".into(), None),
            SingleCharToken::Less => ("LESS", "<".into(), None),
            SingleCharToken::Greater => ("GREATER", ">".into(), None),
        }
    }

    fn from_char_slice(s: &[char], _: usize) -> AppResult<Option<Self>> {
        let ch = match s.get(0) {
            Some(v) => v,
            None => return Ok(None),
        };
        let ret = match ch {
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
        };
        Ok(ret)
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
    fn info(&self) -> (&'static str, String, Option<String>) {
        unreachable!("no info provided on ignored tokens")
    }

    fn from_char_slice(s: &[char], _: usize) -> AppResult<Option<Self>> {
        let ret = match (s.get(0), s.get(1)) {
            (Some('\n'), _) => Some(Self::LineBreak),
            (Some('\t'), _) => Some(Self::Tab),
            (Some(' '), _) => Some(Self::Space),
            (Some('/'), Some('/')) => Some(Self::Comment(
                s.iter().position(|x| x == &'\n').unwrap_or_else(|| s.len()),
            )),
            _ => None,
        };

        Ok(ret)
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
    fn info(&self) -> (&'static str, String, Option<String>) {
        match self {
            MultiCharToken::EqualEqual => ("EQUAL_EQUAL", "==".into(), None),
            MultiCharToken::BangEqual => ("BANG_EQUAL", "!=".into(), None),
            MultiCharToken::LessEqual => ("LESS_EQUAL", "<=".into(), None),
            MultiCharToken::GreaterEqual => ("GREATER_EQUAL", ">=".into(), None),
        }
    }

    fn from_char_slice(s: &[char], _: usize) -> AppResult<Option<Self>> {
        let ret = match (s.get(0), s.get(1)) {
            (Some('='), Some('=')) => Some(Self::EqualEqual),
            (Some('!'), Some('=')) => Some(Self::BangEqual),
            (Some('<'), Some('=')) => Some(Self::LessEqual),
            (Some('>'), Some('=')) => Some(Self::GreaterEqual),
            _ => None,
        };

        Ok(ret)
    }

    fn length(&self) -> usize {
        // Till now, all multiple charaters token have 2 characters.
        2
    }
}

pub(super) struct StringToken(String);

impl Tokened for StringToken {
    fn info(&self) -> (&'static str, String, Option<String>) {
        ("STRING", format!(r#""{}""#, self.0), Some(self.0.clone()))
    }

    fn from_char_slice(s: &[char], line: usize) -> AppResult<Option<Self>> {
        match s.get(0) {
            Some(v) if v == &'"' => { /* Matched */ }
            Some(_) => return Ok(None),
            None => return Ok(None),
        }

        let end_pos = match s.iter().skip(1).position(|x| x == &'"') {
            Some(v) => v + 1,
            None => return Err(AppError::UnterminatedString { line }.into()),
        };

        // Lox does not support escapting `"` in strings.

        Ok(Some(StringToken(
            s[1..end_pos].iter().map(|x| x.to_owned()).collect(),
        )))
    }

    fn length(&self) -> usize {
        self.0.len() + 2
    }
}
