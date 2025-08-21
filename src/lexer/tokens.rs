use crate::errors::{AppError, AppResult};

pub(super) trait Tokened: Sized {
    /// Get the token info.
    fn info(&self) -> (&'static str, String, Option<String>);

    /// Try parse from character.
    fn from_char_slice(s: &[char], line: usize) -> AppResult<Option<Self>>;

    /// Get the characters count of current token.
    fn length(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    SingleCharacter(SingleCharToken),
    MultiCharToken(MultiCharToken),
    Ignored(IgnoredToken),
    String(StringToken),
    Number(NumberToken),
    Identifier(IdentifierToken),
    Keyword(KeywordToken),
}

impl Token {
    pub(super) fn info(&self) -> (&'static str, String, Option<String>) {
        match self {
            Token::SingleCharacter(t) => t.info(),
            Token::MultiCharToken(t) => t.info(),
            Token::Ignored(_) => unreachable!("no info provided on ignored tokens"),
            Token::String(t) => t.info(),
            Token::Number(t) => t.info(),
            Token::Identifier(t) => t.info(),
            Token::Keyword(t) => t.info(),
        }
    }

    pub(super) fn try_consume(s: &[char], line: usize) -> AppResult<Option<Self>> {
        if s.is_empty() {
            return Ok(None);
        }

        if let Some(v) = KeywordToken::from_char_slice(s, line)? {
            return Ok(Some(Self::Keyword(v)));
        }

        if let Some(v) = StringToken::from_char_slice(s, line)? {
            return Ok(Some(Self::String(v)));
        }

        if let Some(v) = NumberToken::from_char_slice(s, line)? {
            return Ok(Some(Self::Number(v)));
        }

        if let Some(v) = IdentifierToken::from_char_slice(s, line)? {
            return Ok(Some(Self::Identifier(v)));
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
            Token::Number(..) => false,
            Token::Identifier(..) => false,
            Token::Keyword(..) => false,
        }
    }

    pub(super) fn length(&self) -> usize {
        match self {
            Token::SingleCharacter(v) => v.length(),
            Token::MultiCharToken(v) => v.length(),
            Token::Ignored(v) => v.length(),
            Token::String(v) => v.length(),
            Token::Number(v) => v.length(),
            Token::Identifier(v) => v.length(),
            Token::Keyword(v) => v.length(),
        }
    }

    pub fn is_string(&self) -> bool {
        if let Token::String(..) = &self {
            true
        } else {
            false
        }
    }

    pub fn is_number(&self) -> bool {
        if let Token::Number(..) = &self {
            true
        } else {
            false
        }
    }

    pub fn is_string_or_number(&self) -> bool {
        self.is_string() || self.is_number()
    }

    pub fn is_binary_op(&self) -> bool {
        match self {
            Token::SingleCharacter(v) => match v {
                SingleCharToken::LeftParen => false,
                SingleCharToken::RightParen => false,
                SingleCharToken::LeftBrace => false,
                SingleCharToken::RightBrace => false,
                SingleCharToken::Star => true,
                SingleCharToken::Dot => false,
                SingleCharToken::Comma => false,
                SingleCharToken::Plus => true,
                SingleCharToken::Minus => true,
                SingleCharToken::Slash => true,
                SingleCharToken::Semicolon => false,
                SingleCharToken::Assign => true,
                SingleCharToken::Bang => false,
                SingleCharToken::Less => true,
                SingleCharToken::Greater => true,
            },
            Token::MultiCharToken(..) => true,
            Token::Ignored(..) => false,
            Token::String(..) => false,
            Token::Number(..) => false,
            Token::Identifier(..) => false,
            Token::Keyword(..) => false,
        }
    }

    pub fn is_unary_op(&self) -> bool {
        match self {
            Token::SingleCharacter(v) => match v {
                SingleCharToken::LeftParen => false,
                SingleCharToken::RightParen => false,
                SingleCharToken::LeftBrace => false,
                SingleCharToken::RightBrace => false,
                SingleCharToken::Star => false,
                SingleCharToken::Dot => false,
                SingleCharToken::Comma => false,
                SingleCharToken::Plus => false,
                SingleCharToken::Minus => true,
                SingleCharToken::Slash => false,
                SingleCharToken::Semicolon => false,
                SingleCharToken::Assign => false,
                SingleCharToken::Bang => true,
                SingleCharToken::Less => false,
                SingleCharToken::Greater => false,
            },
            Token::MultiCharToken(..) => false,
            Token::Ignored(..) => false,
            Token::String(..) => false,
            Token::Number(..) => false,
            Token::Identifier(..) => false,
            Token::Keyword(..) => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleCharToken {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IgnoredToken {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiCharToken {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringToken(pub String);

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

/// Number in lox.
///
/// Valid formats:
///
/// * 123
/// * 123.0
///
/// Invalid formats:
///
/// * .123 => parse started after the `.`
/// * 123. => parse finshed before the `.`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberToken {
    integer: u32,

    /// The length of integer part.
    integer_length: usize,

    /// The deciaml part contains of the original value `string` and calculated number value `usize`.
    decimal: Option<(String, u32)>,
}

impl NumberToken {
    pub fn as_f64(&self) -> f64 {
        match &self.decimal {
            Some((l, f)) => self.integer as f64 + (f.to_owned() as f64 / l.len() as f64),
            None => self.integer as f64,
        }
    }

    pub fn info_string(&self) -> String {
        self.info().2.unwrap()
    }
}

impl Tokened for NumberToken {
    fn info(&self) -> (&'static str, String, Option<String>) {
        let mut value = self.integer.to_string();
        match &self.decimal {
            Some(v) => value.push_str(format!(".{}", v.0).as_str()),
            None => { /* Do nothing*/ }
        }

        let r = match &self.decimal {
            Some(v) => format!("{}.{}", self.integer, v.1),
            None => format!("{}.0", self.integer),
        };

        ("NUMBER", value, Some(r))
    }

    fn from_char_slice(s: &[char], _: usize) -> AppResult<Option<Self>> {
        if s.is_empty() || !s[0].is_digit(10) {
            return Ok(None);
        }

        // The first character is integer.
        // Parse the integer part.
        let raw_integer_chars = s
            .iter()
            .take_while(|x| x.is_digit(10))
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        let integer = raw_integer_chars
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, i)| i * 10_u32.pow(idx as u32))
            .fold(0, |acc, x| acc + x);
        let mut decimal_it = s.iter().skip(raw_integer_chars.len());
        match decimal_it.next() {
            Some(v) => {
                if v == &'.' {
                    // May have decimal parts.
                    let raw_decimal_chars = decimal_it
                        .take_while(|x| x.is_digit(10))
                        .map(|x| x.to_digit(10).unwrap())
                        .collect::<Vec<_>>();
                    if raw_decimal_chars.is_empty() {
                        // No decimal part, the `.` after integer part is another token.
                        return Ok(Some(NumberToken {
                            integer,
                            integer_length: raw_integer_chars.len(),
                            decimal: None,
                        }));
                    } else {
                        // Have deciaml part. the `.` after integer part is part of number.
                        let decimal_value = raw_decimal_chars
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(idx, i)| i * 10_u32.pow(idx as u32))
                            .fold(0, |acc, x| acc + x);
                        let decimal_string = raw_decimal_chars
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<String>();

                        return Ok(Some(NumberToken {
                            integer,
                            integer_length: raw_integer_chars.len(),
                            decimal: Some((decimal_string, decimal_value)),
                        }));
                    }
                } else {
                    // Only have integer part.
                    return Ok(Some(NumberToken {
                        integer,
                        integer_length: raw_integer_chars.len(),
                        decimal: None,
                    }));
                }
            }
            None => {
                // Reach the end of input.
                // No decimal part, the `.` after integer part is another token.
                return Ok(Some(NumberToken {
                    integer,
                    integer_length: raw_integer_chars.len(),
                    decimal: None,
                }));
            }
        }
    }

    fn length(&self) -> usize {
        match &self.decimal {
            Some((l, _)) => self.integer_length + 1 + l.len(),
            None => self.integer_length,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifierToken(String);

impl Tokened for IdentifierToken {
    fn info(&self) -> (&'static str, String, Option<String>) {
        ("IDENTIFIER", self.0.clone(), None)
    }

    fn from_char_slice(s: &[char], _: usize) -> AppResult<Option<Self>> {
        if s.is_empty() {
            return Ok(None);
        }

        let chs = s
            .iter()
            .take_while(|x| {
                ((&'a' <= x) && (x <= &&'z'))
                    || ((&'A' <= x) && (x <= &&'Z'))
                    || ((&'0' <= x) && (x <= &&'9'))
                    || x == &&'_'
            })
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();

        // TODO: Check the first character is number or not before moving forward.
        if !chs.is_empty() && !chs[0].is_digit(10) {
            Ok(Some(Self(chs.into_iter().collect())))
        } else {
            Ok(None)
        }
    }

    fn length(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordToken {
    /// `and`
    KAnd,

    /// `class`
    KClass,

    /// `else`
    KElse,

    /// `false`
    KFalse,

    /// `for`
    KFor,

    /// `fun`
    KFun,

    /// `if`
    KIf,

    /// `nil`
    KNil,

    /// `or`
    KOr,

    /// `print`
    KPrint,

    /// `return`
    KReturn,

    /// `super`
    KSuper,

    /// `this`
    KThis,

    /// `true`
    KTrue,

    /// `var`
    KVar,

    /// `While`
    KWhile,
}

impl Tokened for KeywordToken {
    fn info(&self) -> (&'static str, String, Option<String>) {
        match self {
            KeywordToken::KAnd => ("AND", "and".into(), None),
            KeywordToken::KClass => ("CLASS", "class".into(), None),
            KeywordToken::KElse => ("ELSE", "else".into(), None),
            KeywordToken::KFalse => ("FALSE", "false".into(), None),
            KeywordToken::KFor => ("FOR", "for".into(), None),
            KeywordToken::KFun => ("FUN", "fun".into(), None),
            KeywordToken::KIf => ("IF", "if".into(), None),
            KeywordToken::KNil => ("NIL", "nil".into(), None),
            KeywordToken::KOr => ("OR", "or".into(), None),
            KeywordToken::KPrint => ("PRINT", "print".into(), None),
            KeywordToken::KReturn => ("RETURN", "return".into(), None),
            KeywordToken::KSuper => ("SUPER", "super".into(), None),
            KeywordToken::KThis => ("THIS", "this".into(), None),
            KeywordToken::KTrue => ("TRUE", "true".into(), None),
            KeywordToken::KVar => ("VAR", "var".into(), None),
            KeywordToken::KWhile => ("WHILE", "while".into(), None),
        }
    }

    fn from_char_slice(s: &[char], _: usize) -> AppResult<Option<Self>> {
        // If better to build a huffman tree, but it is fast enough, I think.
        let mut it = s.iter();
        let s0 = it.next();
        let s1 = it.next();
        let s2 = it.next();
        let s3 = it.next();
        let s4 = it.next();
        let s5 = it.next();

        let ss = [s0, s1, s2, s3, s4, s5];

        if ss[..3] == [Some(&'a'), Some(&'n'), Some(&'d')] {
            Ok(Some(Self::KAnd))
        } else if ss[..5] == [Some(&'c'), Some(&'l'), Some(&'a'), Some(&'s'), Some(&'s')] {
            Ok(Some(Self::KClass))
        } else if ss[..4] == [Some(&'e'), Some(&'l'), Some(&'s'), Some(&'e')] {
            Ok(Some(Self::KElse))
        } else if ss[..5] == [Some(&'f'), Some(&'a'), Some(&'l'), Some(&'s'), Some(&'e')] {
            Ok(Some(Self::KFalse))
        } else if ss[..3] == [Some(&'f'), Some(&'o'), Some(&'r')] {
            Ok(Some(Self::KFor))
        } else if ss[..3] == [Some(&'f'), Some(&'u'), Some(&'n')] {
            Ok(Some(Self::KFun))
        } else if ss[..2] == [Some(&'i'), Some(&'f')] {
            Ok(Some(Self::KIf))
        } else if ss[..3] == [Some(&'n'), Some(&'i'), Some(&'l')] {
            Ok(Some(Self::KNil))
        } else if ss[..2] == [Some(&'o'), Some(&'r')] {
            Ok(Some(Self::KOr))
        } else if ss[..5] == [Some(&'p'), Some(&'r'), Some(&'i'), Some(&'n'), Some(&'t')] {
            Ok(Some(Self::KPrint))
        } else if ss[..6]
            == [
                Some(&'r'),
                Some(&'e'),
                Some(&'t'),
                Some(&'u'),
                Some(&'r'),
                Some(&'n'),
            ]
        {
            Ok(Some(Self::KReturn))
        } else if ss[..5] == [Some(&'s'), Some(&'u'), Some(&'p'), Some(&'e'), Some(&'r')] {
            Ok(Some(Self::KSuper))
        } else if ss[..4] == [Some(&'t'), Some(&'h'), Some(&'i'), Some(&'s')] {
            Ok(Some(Self::KThis))
        } else if ss[..4] == [Some(&'t'), Some(&'r'), Some(&'u'), Some(&'e')] {
            Ok(Some(Self::KTrue))
        } else if ss[..3] == [Some(&'v'), Some(&'a'), Some(&'r')] {
            Ok(Some(Self::KVar))
        } else if ss[..5] == [Some(&'w'), Some(&'h'), Some(&'i'), Some(&'l'), Some(&'e')] {
            Ok(Some(Self::KWhile))
        } else {
            Ok(None)
        }
    }

    fn length(&self) -> usize {
        match self {
            KeywordToken::KAnd => 3,
            KeywordToken::KClass => 5,
            KeywordToken::KElse => 4,
            KeywordToken::KFalse => 5,
            KeywordToken::KFor => 3,
            KeywordToken::KFun => 3,
            KeywordToken::KIf => 2,
            KeywordToken::KNil => 3,
            KeywordToken::KOr => 2,
            KeywordToken::KPrint => 5,
            KeywordToken::KReturn => 6,
            KeywordToken::KSuper => 5,
            KeywordToken::KThis => 4,
            KeywordToken::KTrue => 4,
            KeywordToken::KVar => 3,
            KeywordToken::KWhile => 5,
        }
    }
}
