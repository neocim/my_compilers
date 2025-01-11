#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self { kind }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Lit { kind: LiteralKind },
    Op { kind: OpKind },
    OpenParen,
    CloseParen,
    Whitespace,
    Eof,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpKind {
    Star,
    Slash,
    Percent,
    Plus,
    Minus,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    Int { val: String },
    Float { val: String },
}
