mod cursor;
#[cfg(test)]
mod tests;
mod token;

pub use {
    cursor::Cursor,
    token::{LiteralKind, Token, TokenKind},
};

pub struct Lexer {}
