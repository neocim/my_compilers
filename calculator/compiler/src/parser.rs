use crate::ast::{token::Token, TokenStream};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenCursor {
    token_stream: TokenStream,
    cur_tok: Token,
}

impl TokenCursor {
    pub fn advance(&mut self) {
        let token = self.token_stream.next();
    }
}

#[derive(Clone, Debug)]
pub struct Parser {
    token_stream: TokenStream,
}

impl Parser {
    pub fn parse(&mut self) {
        loop {
            let token = self.token_stream.next();
        }
    }
    pub fn parse_stmt(&self) {}
    pub fn parse_expr(&self) {}
    pub fn parse_term(&self) {}
    pub fn parse_factor(&self) {}
}
