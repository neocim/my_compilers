use token::{BinOpKind, LiteralKind, Token};

pub mod token;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StmtKind {
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
pub struct TokenStream(pub Vec<Token>);

impl TokenStream {
    pub fn new(stream: Vec<Token>) -> Self {
        Self(stream)
    }
}
