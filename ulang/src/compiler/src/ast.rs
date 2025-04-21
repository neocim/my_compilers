pub mod token;

use crate::{span::Span, symbol::Symbol};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ident {
    span: Span,
    sym: Symbol,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Literal {
    kind: LiteralKind,
    sym: Symbol,
    span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralKind {
    Bool,
    Int,
    Float,
    Str,
    Char,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Punct {
    Colon,     // `:`
    SemiColon, // `;`
    Comma,     // `,`
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Delim {
    Paren,
    Bracket,
    Brace,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenDelim(pub(crate) Delim);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CloseDelim(pub(crate) Delim);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AssignOp {
    Assign,    // `=`
    MulAssign, // `*=`
    ModAssign, // `%=`
    DivAssign, // `/=`
    SubAssign, // `-=`
    AddAssign, // `+=`
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinOp {
    Add,         // `+`
    Sub,         // `-`
    Mul,         // `*`
    Div,         // `/`
    Mod,         // `%`
    LessThan,    // `<`
    LtOrEq,      // `<=`
    GreaterThan, // `>`
    GtOrEq,      // `>=`
    NotEq,       // `!=`
    Eq,          // `==`
    Or,          // `||`
    And,         // `&&`
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnOp {
    Ref,   // `&`
    Deref, // `*`
    Not,   // `!`
    UnAdd, // `++`
    UnSub, // `--`
}

impl Literal {
    pub fn new(kind: LiteralKind, sym: Symbol, span: Span) -> Self {
        Literal { kind, sym, span }
    }
}

impl Ident {
    pub fn new(sym: Symbol, span: Span) -> Self {
        Ident { sym, span }
    }
}

impl std::fmt::Display for OpenDelim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Delim::Paren => write!(f, "("),
            Delim::Bracket => write!(f, "["),
            Delim::Brace => write!(f, "{{"),
        }
    }
}

impl std::fmt::Display for CloseDelim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Delim::Paren => write!(f, ")"),
            Delim::Bracket => write!(f, "]"),
            Delim::Brace => write!(f, "}}"),
        }
    }
}
