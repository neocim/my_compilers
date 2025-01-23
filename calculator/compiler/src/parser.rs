use crate::ast::TokenStream;

#[derive(Clone, Debug)]
pub struct Parser {
    lexer: TokenStream,
}

impl Parser {
    pub fn parse(&self) {
        todo!()
    }
    pub fn parse_stmt(&self) {}
    pub fn parse_expr(&self) {}
    pub fn parse_term(&self) {}
    pub fn parse_factor(&self) {}
}
