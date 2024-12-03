use cursor::{Cursor, EOF_CHAR};

pub mod cursor;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    fn new(kind: TokenKind) -> Token {
        Token { kind }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    Unknown,
    InvalidIdent,
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int,
    Float,
    Char,
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
            '0'..='9' => self.number(),
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '=' => TokenKind::Eq,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '<' => TokenKind::Less,
            '>' => TokenKind::Greater,
            '!' => TokenKind::Bang,
            '%' => TokenKind::Percent,
            c if is_whitespace(c) => self.whitespace(),
            c if !c.is_ascii() => self.invalid_ident(),
            EOF_CHAR if self.is_eof() => TokenKind::Eof,
            _ => TokenKind::Unknown,
        };

        Token::new(token_kind)
    }

    /// TODO! fix problem with whitespaces and `self.bump()` at the begin of the function
    /// and update float parsing
    fn number(&mut self) -> TokenKind {
        // self.bump();

        let mut is_float = false;
        while self.first().is_digit(10) || self.first() == '.' {
            let ch = match self.bump() {
                Some(c) => c,
                None => break,
            };

            if ch == '.' {
                is_float = true;
            }
        }

        match is_float {
            true => TokenKind::Literal {
                kind: LiteralKind::Float,
            },
            false => TokenKind::Literal {
                kind: LiteralKind::Int,
            },
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

#[cfg(test)]
mod tests {
    use super::{Cursor, LiteralKind, Token, TokenKind};

    #[test]
    fn get_next_token_test() {
        let mut cursor = Cursor::new(
            "// hello \n\
            101-100 \
            4/2 \
            5%2 \
            2.1+2.2 \
            (2!=3)",
        );

        // --- `//hello\n` ---
        assert_eq!(Token::new(TokenKind::LineComment), cursor.get_next_token());
        assert_eq!(Token::new(TokenKind::Whitespace), cursor.get_next_token());

        // --- `101-100` ---
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Int
            }),
            cursor.get_next_token()
        );
        assert_eq!(Token::new(TokenKind::Minus), cursor.get_next_token());
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Int
            }),
            cursor.get_next_token()
        );

        cursor.get_next_token();

        // --- `4/2` ---
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Int
            }),
            cursor.get_next_token()
        );
        assert_eq!(Token::new(TokenKind::Slash), cursor.get_next_token());
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Int
            }),
            cursor.get_next_token()
        );

        cursor.get_next_token();

        // --- `5%2` ---
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Int
            }),
            cursor.get_next_token()
        );
        assert_eq!(Token::new(TokenKind::Percent), cursor.get_next_token());
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Int
            }),
            cursor.get_next_token()
        );

        cursor.get_next_token();

        // --- `2.1+2.2` ---
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Float
            }),
            cursor.get_next_token()
        );
        assert_eq!(Token::new(TokenKind::Plus), cursor.get_next_token());
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Float
            }),
            cursor.get_next_token()
        );

        cursor.get_next_token();

        // --- `(2!=1)` ---
        assert_eq!(Token::new(TokenKind::OpenParen), cursor.get_next_token());
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Int
            }),
            cursor.get_next_token()
        );
        assert_eq!(Token::new(TokenKind::Bang), cursor.get_next_token());
        assert_eq!(Token::new(TokenKind::Eq), cursor.get_next_token());
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Int
            }),
            cursor.get_next_token()
        );
        assert_eq!(Token::new(TokenKind::CloseParen), cursor.get_next_token());
    }

    #[test]
    fn cursor_test() {
        let mut cursor = Cursor::new("abc");
        assert_eq!(Some('a'), cursor.bump());
        assert_eq!('b', cursor.first());
        assert_eq!('c', cursor.second());
        assert_eq!('\0', cursor.third());
    }
}
