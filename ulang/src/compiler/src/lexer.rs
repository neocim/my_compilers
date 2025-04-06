mod cursor;
#[cfg(test)]
mod tests;
mod token;

use crate::span::Pos;
pub use {
    cursor::Cursor,
    token::{LiteralKind, Token, TokenKind},
};

pub struct Lexer<'src> {
    src: &'src str,
    pos: Pos,
    cursor: Cursor<'src>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            pos: Pos::new(1, 0),
            cursor: Cursor::new(src),
        }
    }
}
