#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Lit { kind: LiteralKind },
    BinOp(BinOpKind),
    OpenParen,
    CloseParen,
    Eof,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    Int { val: String },
    Float { val: String },
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
