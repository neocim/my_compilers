use cursor::{Cursor, EOF_CHAR};

pub mod cursor;
mod tests;

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
    Unknown,
    InvalidIdent,
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralKind {
    Integer { val: i32 },
    Float { val: f32 },
    Char { val: char },
}

impl<'a> Cursor<'a> {
    pub fn get_next_token(&mut self) -> Token {
        let first_char = match self.bump() {
            Some(c) => c,
            None => return Token::new(TokenKind::Eof),
        };

        let token_kind = match first_char {
            '/' => match self.first() {
                '/' => self.line_comment(),
                _ => TokenKind::Slash,
            },
            '0'..='9' => self.number(first_char),
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
            c if is_whitespace(c) => self.whitespace(),
            c if !c.is_ascii() => self.invalid_ident(),
            EOF_CHAR if self.is_eof() => TokenKind::Eof,
            _ => TokenKind::Unknown,
        };

        Token::new(token_kind)
    }

    fn number(&mut self, first_char: char) -> TokenKind {
        let mut num_str = first_char.to_string();
        let mut has_point = false;

        while self.first().is_digit(10) || self.first() == '.' {
            if self.first() == '.' {
                match has_point {
                    true => {
                        self.eat_while(|c| c.is_digit(10) || c == '.');
                        break;
                    }
                    false => has_point = true,
                }
            }

            match self.bump() {
                Some(ch) => num_str.push(ch),
                None => return TokenKind::Eof,
            }
        }

        match has_point {
            true => {
                let val = match num_str.parse() {
                    Ok(val) => val,
                    Err(err) => panic!("Error while parsing float number: {err:?}"),
                };
                TokenKind::Literal {
                    kind: LiteralKind::Float { val },
                }
            }
            false => {
                let val = match num_str.parse() {
                    Ok(val) => val,
                    Err(err) => panic!("Error while parsing integer number: {err:?}"),
                };
                TokenKind::Literal {
                    kind: LiteralKind::Integer { val },
                }
            }
        }
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
    c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

pub fn is_ident_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.get_next_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    })
}
