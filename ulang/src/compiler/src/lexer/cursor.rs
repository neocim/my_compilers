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
            _ => TokenKind::Unknown,
        };

        Token::new(kind, self.get_token_size())
    }

    fn next_ch(&mut self) -> Option<char> {
        self.src.next()
    }

    // does not advance `self.src`
    fn next_ahead(&mut self) -> char {
        self.src.clone().next().unwrap_or(EOF_CHAR)
    }

    fn get_token_size(&mut self) -> u32 {
        let res = self.remaining - self.src.as_str().len() as u32;
        self.reset_pos();
        res
    }

    fn reset_pos(&mut self) {
        self.remaining = self.src.as_str().len() as u32
    }
}

fn is_whitespace(c: char) -> bool {
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
