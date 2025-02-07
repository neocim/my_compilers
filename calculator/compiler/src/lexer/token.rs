#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Lit { kind: LiteralKind },
    Star,
    Slash,
    Percent,
    Plus,
    Minus,
    OpenParen,
    CloseParen,
    Whitespace,
    Eof,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    Int { val: String },
    Float { val: String },
}
