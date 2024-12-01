use cursor::Cursor;

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
    LineComment,
    Func,
    Ident,
    Literal { kind: LiteralKind },
    Whitespace,
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

        match first_char {
            '/' => self.line_comment(),
            '0'..='9' => self.number(),
            c if is_whitespace(c) => self.whitespace(),
            _ => todo!(),
        }
    }

    fn number(&mut self) -> Token {
        self.bump();

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
            true => Token::new(TokenKind::Literal {
                kind: LiteralKind::Float,
            }),
            false => Token::new(TokenKind::Literal {
                kind: LiteralKind::Int,
            }),
        }
    }

    fn line_comment(&mut self) -> Token {
        self.bump();

        self.eat_while(|c| c != '\n');
        Token::new(TokenKind::LineComment)
    }

    fn whitespace(&mut self) -> Token {
        self.eat_while(is_whitespace);

        Token::new(TokenKind::Whitespace)
    }
}

pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

#[cfg(test)]
mod tests {
    use super::{Cursor, LiteralKind, Token, TokenKind};

    #[test]
    fn get_next_token_test() {
        let mut cursor = Cursor::new("// hello\n123.45 100");

        assert_eq!(Token::new(TokenKind::LineComment), cursor.get_next_token());
        assert_eq!(Token::new(TokenKind::Whitespace), cursor.get_next_token());
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Float
            }),
            cursor.get_next_token()
        );
        assert_eq!(Token::new(TokenKind::Whitespace), cursor.get_next_token());
        assert_eq!(
            Token::new(TokenKind::Literal {
                kind: LiteralKind::Int
            }),
            cursor.get_next_token()
        );
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
