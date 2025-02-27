use std::str::Chars;

use super::{
    is_whitespace,
    token::{LiteralKind, Token},
    EOF_CHAR,
};

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
            None => return Token::Eof,
        };

        let token = match ch {
            '0'..'9' => self.eat_num(ch),
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '%' => Token::Percent,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            EOF_CHAR => Token::Eof,
            ch if is_whitespace(ch) => self.whitespace(),
            _ => self.unknown(ch),
        };

        token
    }

    fn eat_num(&mut self, first_digit: char) -> Token {
        let mut str_number = String::from(first_digit);
        // Eat next digits if there are any
        str_number.push_str(self.eat_next_digits().as_str());

        match self.first() {
            '.' => {
                // We check above that there is a point
                str_number.push(self.eat_next().expect("Error while processing point"));
                str_number.push_str(self.eat_next_digits().as_str());

                Token::Lit {
                    kind: LiteralKind::Float { val: str_number },
                }
            }
            _ => Token::Lit {
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

    fn unknown(&mut self, first_ch: char) -> Token {
        let mut content = String::from(first_ch);

        while !is_whitespace(self.first()) {
            let ch = match self.eat_next() {
                Some(ch) => ch,
                None => return Token::Unknown { content },
            };
            content.push(ch);
        }

        Token::Unknown { content }
    }

    fn whitespace(&mut self) -> Token {
        self.eat_while(is_whitespace);

        Token::Whitespace
    }

    fn eat_while(&mut self, condition: impl Fn(char) -> bool) {
        while condition(self.first()) && !self.is_eof() {
            self.eat_next();
        }
    }

    pub fn is_eof(&self) -> bool {
        self.input.as_str().is_empty()
    }

    fn eat_next(&mut self) -> Option<char> {
        self.input.next()
    }

    fn first(&self) -> char {
        self.input.clone().next().unwrap_or(EOF_CHAR)
    }
}
