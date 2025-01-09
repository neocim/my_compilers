mod tests;
mod token;

use token::{LiteralKind, OpKind, Token};

use std::str::Chars;

pub struct Cursor<'a> {
    input: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
        }
    }
    pub fn next_token(&self) -> Token {
        todo!()
    }
    pub fn parse_num(&self) -> LiteralKind {
        todo!()
    }
    pub fn parse_op(&self) -> OpKind {
        todo!()
    }
}
