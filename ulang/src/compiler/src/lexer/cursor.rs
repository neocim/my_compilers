use std::str::Chars;

use super::token::{Token, TokenKind};

const EOF_CHAR: char = '\0';

pub struct Cursor<'src> {
    src: Chars<'src>,
    remaining: u32,
}

impl<'src> Cursor<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src: src.chars(),
            remaining: src.len() as u32,
        }
    }

    pub fn next_token(&mut self) -> Token {
        let ch = match self.next_ch() {
            Some(ch) => ch,
            None => return Token::new(TokenKind::Eof, 0),
        };

        let kind = match ch {
            '/' => match self.next_ahead() {
                '/' => TokenKind::Comment,
                _ => TokenKind::Slash,
            },
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            '!' => TokenKind::Bang,
            '=' => TokenKind::Eq,
            '<' => TokenKind::LessThan,
            '>' => TokenKind::GreaterThan,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '%' => TokenKind::Percent,
            '*' => TokenKind::Star,
            ':' => TokenKind::Colon,
            ';' => TokenKind::SemiColon,
            ',' => TokenKind::Comma,
            '&' => TokenKind::And,
            '|' => TokenKind::Or,
            EOF_CHAR => TokenKind::Eof,
            ch if is_whitespace(ch) => {
                self.advance_while(is_whitespace);
                TokenKind::Whitespace
            }
            ch if is_ident_ch(ch) => {
                self.advance_while(is_ident_ch);

                if !is_ident_ch(self.next_ahead()) {
                    TokenKind::Unknown
                } else {
                    TokenKind::Ident
                }
            }
            _ => TokenKind::Unknown,
        };

        Token::new(kind, self.get_token_len())
    }

    fn next_ch(&mut self) -> Option<char> {
        self.src.next()
    }

    /// Helps to look a char ahead. Does not advance `self.src`.
    fn next_ahead(&mut self) -> char {
        self.src.clone().next().unwrap_or(EOF_CHAR)
    }

    fn advance_while<F: Fn(char) -> bool>(&mut self, cond: F) {
        while cond(self.next_ahead()) && !self.is_eof() {
            self.next_ch();
        }
    }

    fn is_eof(&self) -> bool {
        self.src.as_str().is_empty()
    }

    fn get_token_len(&mut self) -> u32 {
        let res = self.remaining - self.src.as_str().len() as u32;
        self.reset_pos();
        res
    }

    fn reset_pos(&mut self) {
        self.remaining = self.src.as_str().len() as u32
    }
}

pub fn is_whitespace(ch: char) -> bool {
    matches!(
        ch,
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

fn is_ident_ch(ch: char) -> bool {
    ch.is_ascii()
}
