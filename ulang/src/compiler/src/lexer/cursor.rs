use std::str::Chars;

// https://www.unicode.org/reports/tr31/
use unicode_xid::UnicodeXID;

use crate::span::{Pos, Span};

use super::token::{LiteralKind, Token, TokenKind};

const EOF_CHAR: char = '\0';
const NEW_LINE: char = '\u{000A}';

#[derive(Clone)]
pub struct Cursor<'src> {
    src: Chars<'src>,
    remaining: u16,
    pos: Pos,
}

impl<'src> Cursor<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src: src.chars(),
            remaining: src.len() as u16,
            pos: Pos::new(1, 1),
        }
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn next_token(&mut self) -> Token {
        let ch = match self.next_ch() {
            Some(ch) => ch,
            None => return Token::new(TokenKind::Eof, self.get_tok_span(false)),
        };
        let mut new_line = false;

        let kind = match ch {
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
            '/' => match self.next_ahead() {
                '/' => {
                    self.next_ch();
                    self.advance_while(|ch| ch != '\n');
                    TokenKind::Comment
                }
                _ => TokenKind::Slash,
            },
            '0'..='9' => self.handle_number(),
            '"' => self.handle_str(),
            '\'' => self.handle_char(),
            't' | 'f' => self.handle_bool(ch),
            // FIXME!: we should check that character is new line in the
            // `Cursor::handle_whitespaces()`, but by for some reason, when
            // this check is in the function, the cursor does not process newline characters.
            ch if ch == NEW_LINE => {
                new_line = true;
                self.handle_whitespaces()
            }
            ch if is_whitespace(ch) => self.handle_whitespaces(),
            ch if is_ident_start(ch) => self.handle_ident(),
            EOF_CHAR => TokenKind::Eof,
            _ => TokenKind::Unknown,
        };
        let span = self.get_tok_span(new_line);

        Token::new(kind, span)
    }

    fn handle_bool(&mut self, first: char) -> TokenKind {
        match first {
            'f' => self.bool_or_ident("alse"),
            't' => self.bool_or_ident("rue"),
            _ => unreachable!(),
        }
    }

    fn bool_or_ident(&mut self, maybe_bool: &str) -> TokenKind {
        if self.src.as_str().starts_with(maybe_bool) {
            self.advance_to(maybe_bool.len() as u32);
            if !is_ident_continue(self.next_ahead()) {
                TokenKind::Lit {
                    kind: LiteralKind::Bool,
                }
            } else {
                self.handle_ident()
            }
        } else {
            self.handle_ident()
        }
    }

    fn handle_whitespaces(&mut self) -> TokenKind {
        while is_whitespace(self.next_ahead()) && !self.is_eof() {
            let ch = self.next_ch().expect("Failed to get the next character");
            if ch == NEW_LINE {
                self.new_line();
            }
        }

        TokenKind::Whitespace
    }

    fn handle_ident(&mut self) -> TokenKind {
        self.advance_while(is_ident_continue);
        TokenKind::Ident
    }

    fn handle_number(&mut self) -> TokenKind {
        // eat next digits if there are any
        self.eat_next_digits();

        match self.next_ahead() {
            '.' => {
                // eat point
                self.next_ch();
                self.eat_next_digits();

                TokenKind::Lit {
                    kind: LiteralKind::Float,
                }
            }
            _ => TokenKind::Lit {
                kind: LiteralKind::Int,
            },
        }
    }

    fn eat_next_digits(&mut self) {
        loop {
            match self.next_ahead() {
                ch if ch.is_numeric() => {
                    self.next_ch();
                }
                _ => break,
            }
        }
    }

    fn handle_str(&mut self) -> TokenKind {
        TokenKind::Lit {
            kind: LiteralKind::Str {
                terminated: self.eat_str(),
            },
        }
    }

    /// Returns true if string is terminated and false otherwise.
    fn eat_str(&mut self) -> bool {
        loop {
            match self.next_ahead() {
                // skip `\"` characters
                '\\' if '"' == self.second_ahead() => {
                    self.advance_to(2);
                }
                '"' => {
                    self.next_ch();
                    return true;
                }
                EOF_CHAR => break,
                _ => {
                    self.next_ch();
                }
            }
        }
        false
    }

    /// It does not check the validity of the characters. It can only say
    /// that the character is not terminated. For example, incorrect characters
    /// such as `'\m'`, `'abc'` and others will simply be eaten. Checking for the
    /// correctness of the characters is at the parsing stage.
    fn handle_char(&mut self) -> TokenKind {
        TokenKind::Lit {
            kind: LiteralKind::Char {
                terminated: self.eat_char(),
            },
        }
    }

    /// Returns true if character is terminated and false otherwise.
    fn eat_char(&mut self) -> bool {
        // if true, it's just one characte like `'a'`
        if self.next_ahead() != '\\' && self.second_ahead() == '\'' {
            self.advance_to(2);
            return true;
        }

        loop {
            match self.next_ahead() {
                '\'' => {
                    self.next_ch();
                    return true;
                }
                '\\' => {
                    self.advance_to(2);
                }
                '\n' if self.second_ahead() != '\'' => break,
                EOF_CHAR => break,
                _ => {
                    self.next_ch();
                }
            }
        }
        false
    }

    fn advance_to(&mut self, n: u32) -> Option<char> {
        self.src.nth(n as usize - 1)
    }

    fn next_ch(&mut self) -> Option<char> {
        self.src.next()
    }

    /// Helps to look a character ahead. Does not advance `self.src`.
    fn next_ahead(&self) -> char {
        self.src.clone().next().unwrap_or(EOF_CHAR)
    }

    fn second_ahead(&self) -> char {
        let mut src = self.src.clone();
        src.next();
        src.next().unwrap_or(EOF_CHAR)
    }

    fn advance_while<F: Fn(char) -> bool>(&mut self, cond: F) {
        while cond(self.next_ahead()) && !self.is_eof() {
            self.next_ch();
        }
    }

    fn is_eof(&self) -> bool {
        self.src.as_str().is_empty()
    }

    fn get_tok_span(&mut self, new_line: bool) -> Span {
        let start = self.pos;
        if new_line {
            self.new_line();
            self.pos.col = self.pos.col + self.get_tok_len() - 1;
        } else {
            self.pos.col = self.pos.col + self.get_tok_len();
        }

        Span::new(start, self.pos)
    }

    fn get_tok_len(&mut self) -> u16 {
        let tok_len = self.remaining - self.src.as_str().len() as u16;
        self.remaining = self.src.as_str().len() as u16;
        tok_len
    }

    fn new_line(&mut self) {
        self.pos = Pos::new(self.pos.ln + 1, 1);
    }
}

/// # WARNING
/// Here is doesnt checked `\n`. See `Cursor::next_token()`
fn is_whitespace(ch: char) -> bool {
    matches!(
        ch,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        // | '\u{000A}' // \n
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

fn is_ident_start(ch: char) -> bool {
    UnicodeXID::is_xid_start(ch)
}

fn is_ident_continue(ch: char) -> bool {
    UnicodeXID::is_xid_continue(ch)
}
