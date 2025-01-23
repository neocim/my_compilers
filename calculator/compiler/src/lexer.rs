mod tests;
pub mod token;

use crate::ast::token::{BinOpKind, LiteralKind as AstLiteralKind, Token as AstToken};
use token::{
    LiteralKind, Token,
    TokenKind::{self, *},
};

use std::str::Chars;

pub const EOF_CHAR: char = '\0';

impl From<TokenKind> for AstToken {
    fn from(value: TokenKind) -> Self {
        loop {
            return match value {
                Lit { kind } => match kind {
                    LiteralKind::Int { val } => AstToken::Lit {
                        kind: AstLiteralKind::Int { val },
                    },
                    LiteralKind::Float { val } => AstToken::Lit {
                        kind: AstLiteralKind::Float { val },
                    },
                },
                Star => AstToken::BinOp(BinOpKind::Mul),
                Slash => AstToken::BinOp(BinOpKind::Div),
                Percent => AstToken::BinOp(BinOpKind::Mod),
                Plus => AstToken::BinOp(BinOpKind::Add),
                Minus => AstToken::BinOp(BinOpKind::Sub),
                OpenParen => AstToken::OpenParen,
                CloseParen => AstToken::CloseParen,
                Whitespace => continue,
                Eof => AstToken::Eof,
                Unknown => AstToken::Unknown,
            };
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn next_token(&mut self) -> AstToken {
        loop {
            let token = self.cursor.next_token();

            let kind = match token.kind {
                Lit { kind } => todo!(),
                Star => todo!(),
                Slash => todo!(),
                Percent => todo!(),
                Plus => todo!(),
                Minus => todo!(),
                OpenParen => todo!(),
                CloseParen => todo!(),
                Whitespace => todo!(),
                Eof => todo!(),
                Unknown => todo!(),
            };
        }
    }
}

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
            None => return Token::new(Eof),
        };

        let kind = match ch {
            '0'..'9' => self.eat_num(ch),
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' => Slash,
            '%' => Percent,
            '(' => OpenParen,
            ')' => CloseParen,
            ch if is_whitespace(ch) => self.whitespace(),
            _ => Unknown,
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

                Lit {
                    kind: LiteralKind::Float { val: str_number },
                }
            }
            _ => Lit {
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

        Whitespace
    }

    fn eat_while(&mut self, condition: impl Fn(char) -> bool) {
        while condition(self.first()) && !self.is_eof() {
            self.eat_next();
        }
    }

    pub fn tokenize(&mut self) -> impl Iterator<Item = Token> + use<'a, '_> {
        std::iter::from_fn(|| {
            let token = self.next_token();
            if token.kind != Eof {
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
