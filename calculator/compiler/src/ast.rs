pub mod token;

use std::collections::VecDeque;

use token::{BinOpKind, LiteralKind, Token};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Ast {
    Stmt(Stmt),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Lit(Lit),
    BinOp(BinOp),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BinOp {
    left: Box<Expr>,
    kind: BinOpKind,
    right: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Lit {
    kind: LiteralKind,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TokenStream(VecDeque<Token>);

impl BinOp {
    pub fn new(left: Expr, kind: BinOpKind, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            kind,
            right: Box::new(right),
        }
    }
}

impl Lit {
    pub fn new(kind: LiteralKind) -> Self {
        Self { kind }
    }
}

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
