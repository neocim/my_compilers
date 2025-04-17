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
