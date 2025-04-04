use crate::{span::Span, symbol::Symbol};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Colon,        // `:`
    SemiColon,    // `;`
    Comma,        // `,`
    Ident(Ident), // Any ident like keywords or name of variables, etc.
    Lit(Literal),
}

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
pub enum Delimiter {
    Brace,   // `{` or `}`
    Paren,   // `(` or `)`
    Bracket, // `[` or `]`
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
