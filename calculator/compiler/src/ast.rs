use std::collections::VecDeque;

use token::{BinOpKind, LiteralKind, Token};

pub mod token;

pub enum Ast {
    Stmt(Stmt),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    Lit { kind: LiteralKind },
    BinOp(BinOp),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BinOp {
    left: Box<Expr>,
    kind: BinOpKind,
    right: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenStream(pub VecDeque<Token>);

impl TokenStream {
    pub fn new(stream: VecDeque<Token>) -> Self {
        Self(stream)
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}
