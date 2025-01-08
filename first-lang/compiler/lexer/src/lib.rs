use std::num::{ParseFloatError, ParseIntError};

use cursor::{Cursor, EOF_CHAR};
use errors::{ParseErrors, ParseLiteralError};

pub mod cursor;
pub mod errors;
pub mod tests;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    fn new(kind: TokenKind) -> Token {
        Token { kind }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    /// `//`
    LineComment,
    /// Any literal
    Literal {
        kind: LiteralKind,
    },
    /// Any whitespace character
    Whitespace,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `=`
    Eq,
    /// `<`
    Less,
    /// `>`
    Greater,
    /// `!`
    Bang,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `;`
    Semicolon,
    /// `.`
    Dot,
    Ident,
    InvalidIdent,
    Unknown,
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralKind {
    Integer { val: i32 },
    Float { val: f32 },
    Char { val: char },
}

impl<'a> Cursor<'a> {
    pub fn get_next_token(&mut self) -> Result<Token, ParseErrors> {
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Ok(Token::new(TokenKind::Eof)),
        };

        let token_kind = match first_char {
            c if is_ident_start(c) => self.ident_or_unknown(),
            '/' => match self.first() {
                '/' => self.line_comment(),
                _ => TokenKind::Slash,
            },
            '0'..='9' => self.number(first_char)?,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '=' => TokenKind::Eq,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '<' => TokenKind::Less,
            '>' => TokenKind::Greater,
            '!' => TokenKind::Bang,
            '%' => TokenKind::Percent,
            ';' => TokenKind::Semicolon,
            '.' => TokenKind::Dot,
            c if is_whitespace(c) => self.whitespace(),
            c if !c.is_ascii() => self.invalid_ident(),
            EOF_CHAR if self.is_eof() => TokenKind::Eof,
            _ => TokenKind::Unknown,
        };

        Ok(Token::new(token_kind))
    }

    fn number(&mut self, first_digit: char) -> Result<TokenKind, ParseLiteralError> {
        let mut num_str = first_digit.to_string();

        // all `expect()`s here are safe because we check existing of digits with `is_digit()`
        match self.first() {
            '.' => {
                // eat point
                num_str.push(self.bump().expect("Error while processing point in number"));

                if self.first().is_digit(10) {
                    num_str += self
                        .eat_next_digits()
                        .expect("Error while processing next digits after point")
                        .as_str();
                }
                let val = self.parse_float_num(num_str.as_str())?;
                Ok(TokenKind::Literal {
                    kind: LiteralKind::Float { val },
                })
            }
            _ => {
                if self.first().is_digit(10) {
                    num_str += self
                        .eat_next_digits()
                        .expect("Error while processing next digits after point")
                        .as_str();
                }

                let val = self.parse_integer_num(num_str.as_str())?;
                Ok(TokenKind::Literal {
                    kind: LiteralKind::Integer { val },
                })
            }
        }
    }

    /// Eat all next digits and return string with this numbers
    fn eat_next_digits(&mut self) -> Option<String> {
        let mut num_str = String::new();

        // `expect()` is safe because we are check existing of digits with `Cursor::first()` and `match '0'..='9'`
        loop {
            match self.first() {
                '0'..='9' => num_str.push(self.bump().expect("")),
                _ => break,
            }
        }

        if num_str.is_empty() {
            Some(num_str)
        } else {
            None
        }
    }

    fn parse_integer_num(&self, num_str: &str) -> Result<i32, ParseIntError> {
        num_str.parse()
    }

    fn parse_float_num(&self, num_str: &str) -> Result<f32, ParseFloatError> {
        num_str.parse()
    }

    fn line_comment(&mut self) -> TokenKind {
        self.bump();

        self.eat_while(|c| c != '\n');
        TokenKind::LineComment
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);

        TokenKind::Whitespace
    }

    fn ident_or_unknown(&mut self) -> TokenKind {
        self.eat_while(is_ident_continue);

        match self.first() {
            c if !c.is_ascii() => self.invalid_ident(),
            _ => TokenKind::Ident,
        }
    }

    fn invalid_ident(&mut self) -> TokenKind {
        self.eat_while(|c| {
            const ZERO_WIDTH_JOINER: char = '\u{200d}';
            !c.is_ascii() || c == ZERO_WIDTH_JOINER
        });

        TokenKind::InvalidIdent
    }
}

pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

pub fn is_ident_start(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_start(c)
}

pub fn is_ident_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

pub fn tokenize(input: &str) -> Result<impl Iterator<Item = Token> + '_, ParseErrors> {
    let mut cursor = Cursor::new(input);
    Ok(std::iter::from_fn(move || {
        let token = cursor.get_next_token().ok()?;
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    }))
}
