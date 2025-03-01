mod cursor;
#[cfg(test)]
mod tests;
pub mod token;

use std::collections::VecDeque;

use crate::ast::{token as ast, TokenStream};
use crate::lexer::cursor::Cursor;
use token::{LiteralKind, Token};

pub const EOF_CHAR: char = '\0';

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

    fn next_token(&mut self) -> ast::Token {
        loop {
            let token = self.cursor.next_token();

            let token = match token {
                Token::Lit { kind } => match kind {
                    LiteralKind::Int { val } => ast::Token::Lit {
                        kind: ast::LiteralKind::Int { val },
                    },
                    LiteralKind::Float { val } => ast::Token::Lit {
                        kind: ast::LiteralKind::Float { val },
                    },
                },
                Token::Star => ast::Token::BinOp(ast::BinOpKind::Mul),
                Token::Slash => ast::Token::BinOp(ast::BinOpKind::Div),
                Token::Percent => ast::Token::BinOp(ast::BinOpKind::Mod),
                Token::Plus => ast::Token::BinOp(ast::BinOpKind::Add),
                Token::Minus => ast::Token::BinOp(ast::BinOpKind::Sub),
                Token::OpenParen => ast::Token::OpenParen,
                Token::CloseParen => ast::Token::CloseParen,
                // Skip all whitespaces
                Token::Whitespace => continue,
                Token::Eof => ast::Token::Eof,
                Token::Unknown { content } => ast::Token::Unknown { content },
            };

            break token;
        }
    }

    pub fn token_stream(&mut self) -> TokenStream {
        let mut buf = Vec::new();

        loop {
            match self.next_token() {
                ast::Token::Eof => return TokenStream::new(VecDeque::from(buf)),
                token => {
                    buf.push(token);
                }
            }
        }
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
