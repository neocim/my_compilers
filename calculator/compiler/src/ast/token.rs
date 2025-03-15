#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Token {
    Lit { kind: LiteralKind },
    BinOp(BinOpKind),
    OpenParen,
    CloseParen,
    Eof,
    Unknown { content: String },
    // init token (for example, for Parser::new())
    EmptyExpr,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LiteralKind {
    Int { val: String },
    Float { val: String },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
