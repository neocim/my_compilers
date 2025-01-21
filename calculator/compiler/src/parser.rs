use crate::lexer::Lexer;

#[derive(Clone, Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}
