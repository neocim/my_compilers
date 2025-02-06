mod tests;
pub mod token;
mod cursor;

use crate::ast::token::{BinOpKind, LiteralKind as AstLiteralKind, Token as AstToken};
use token::{
    LiteralKind,
    TokenKind,
};
use crate::lexer::cursor::Cursor;

pub const EOF_CHAR: char = '\0';

impl From<TokenKind> for AstToken {
    fn from(value: TokenKind) -> Self {
        loop {
            return match value {
                TokenKind::Lit { kind } => match kind {
                    LiteralKind::Int { val } => AstToken::Lit {
                        kind: AstLiteralKind::Int { val },
                    },
                    LiteralKind::Float { val } => AstToken::Lit {
                        kind: AstLiteralKind::Float { val },
                    },
                },
                TokenKind::Star => AstToken::BinOp(BinOpKind::Mul),
                TokenKind::Slash => AstToken::BinOp(BinOpKind::Div),
                TokenKind::Percent => AstToken::BinOp(BinOpKind::Mod),
                TokenKind::Plus => AstToken::BinOp(BinOpKind::Add),
                TokenKind::Minus => AstToken::BinOp(BinOpKind::Sub),
                TokenKind::OpenParen => AstToken::OpenParen,
                TokenKind::CloseParen => AstToken::CloseParen,
                TokenKind::Whitespace => continue,
                TokenKind::Eof => AstToken::Eof,
                TokenKind::Unknown => AstToken::Unknown,
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
                TokenKind::Lit { kind } => todo!(),
                TokenKind::Star => todo!(),
                TokenKind::Slash => todo!(),
                TokenKind::Percent => todo!(),
                TokenKind::Plus => todo!(),
                TokenKind::Minus => todo!(),
                TokenKind::OpenParen => todo!(),
                TokenKind::CloseParen => todo!(),
                TokenKind::Whitespace => todo!(),
                TokenKind::Eof => todo!(),
                TokenKind::Unknown => todo!(),
            };
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
