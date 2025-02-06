use std::str::Chars;

use super::{is_whitespace, token::{LiteralKind, Token, TokenKind}, EOF_CHAR};

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    input: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let ch = match self.eat_next() {
            Some(ch) => ch,
            None => return Token::new(TokenKind::Eof),
        };

        let kind = match ch {
            '0'..'9' => self.eat_num(ch),
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            ch if is_whitespace(ch) => self.whitespace(),
            _ => TokenKind::Unknown,
        };

        Token::new(kind)
    }

    fn eat_num(&mut self, first_digit: char) -> TokenKind {
        let mut str_number = String::from(first_digit);
        // Eat next digits if there are any
        str_number.push_str(self.eat_next_digits().as_str());

        match self.first() {
            '.' => {
                // We check above that there is a point
                str_number.push(self.eat_next().expect("Error while processing point"));
                str_number.push_str(self.eat_next_digits().as_str());

                TokenKind::Lit {
                    kind: LiteralKind::Float { val: str_number },
                }
            }
            _ => TokenKind::Lit {
                kind: LiteralKind::Int { val: str_number },
            },
        }
    }

    fn eat_next_digits(&mut self) -> String {
        let mut str_number = String::new();

        loop {
            match self.first() {
                ch if ch.is_numeric() => {
                    self.eat_next();
                    str_number.push(ch)
                }
                _ => {
                    break;
                }
            }
        }

        str_number
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);

        TokenKind::Whitespace
    }

    fn eat_while(&mut self, condition: impl Fn(char) -> bool) {
        while condition(self.first()) && !self.is_eof() {
            self.eat_next();
        }
    }

    pub fn tokenize(&mut self) -> impl Iterator<Item = Token> + use<'a, '_> {
        std::iter::from_fn(|| {
            let token = self.next_token();
            if token.kind != TokenKind::Eof {
                Some(token)
            } else {
                None
            }
        })
    }

    pub fn is_eof(&self) -> bool {
        self.input.as_str().is_empty()
    }

    fn eat_next(&mut self) -> Option<char> {
        let ch = self.input.next()?;

        Some(ch)
    }

    fn first(&self) -> char {
        self.input.clone().next().unwrap_or(EOF_CHAR)
    }

    fn second(&self) -> char {
        let mut iter = self.input.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    fn third(&self) -> char {
        let mut iter = self.input.clone();
        iter.next();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }
}