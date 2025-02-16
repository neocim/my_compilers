mod cursor;
mod tests;
pub mod token;

use std::collections::VecDeque;

use crate::ast::{
    token::{BinOpKind, LiteralKind as AstLiteralKind, Token as AstToken},
    TokenStream,
};
use crate::lexer::cursor::Cursor;
use token::{LiteralKind, Token};

pub const EOF_CHAR: char = '\0';

impl From<Token> for AstToken {
    fn from(value: Token) -> Self {
        loop {
            return match value {
                Token::Lit { kind } => match kind {
                    LiteralKind::Int { val } => AstToken::Lit {
                        kind: AstLiteralKind::Int { val },
                    },
                    LiteralKind::Float { val } => AstToken::Lit {
                        kind: AstLiteralKind::Float { val },
                    },
                },
                Token::Star => AstToken::BinOp(BinOpKind::Mul),
                Token::Slash => AstToken::BinOp(BinOpKind::Div),
                Token::Percent => AstToken::BinOp(BinOpKind::Mod),
                Token::Plus => AstToken::BinOp(BinOpKind::Add),
                Token::Minus => AstToken::BinOp(BinOpKind::Sub),
                Token::OpenParen => AstToken::OpenParen,
                Token::CloseParen => AstToken::CloseParen,
                Token::Whitespace => continue,
                Token::Eof => AstToken::Eof,
                Token::Unknown => AstToken::Unknown,
            };
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }

    fn next_token(&mut self) -> AstToken {
        loop {
            let token = self.cursor.next_token();

            let token = match token {
                Token::Lit { kind } => AstToken::from(Token::Lit { kind }),
                Token::Star => AstToken::from(Token::Star),
                Token::Slash => AstToken::from(Token::Slash),
                Token::Percent => AstToken::from(Token::Percent),
                Token::Plus => AstToken::from(Token::Plus),
                Token::Minus => AstToken::from(Token::Minus),
                Token::OpenParen => AstToken::from(Token::OpenParen),
                Token::CloseParen => AstToken::from(Token::CloseParen),
                // Skip all whitespaces
                Token::Whitespace => continue,
                Token::Eof => AstToken::from(Token::Eof),
                Token::Unknown => AstToken::from(Token::Unknown),
            };

            break token;
        }
    }

    pub fn token_stream(&mut self) -> TokenStream {
        let mut buf = Vec::new();

        loop {
            match self.next_token() {
                AstToken::Eof => return TokenStream::new(VecDeque::from(buf)),
                token => {
                    buf.push(token);
                }
            }
        }
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
