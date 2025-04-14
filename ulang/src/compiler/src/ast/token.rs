use crate::{span::Span, symbol::Symbol};

#[derive(Debug, PartialEq)]
pub struct TokenStream(Vec<Token>);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token {
    pub(crate) kind: TokenKind,
    span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Lit(Literal),
    Ident(Ident),
    OpenDelim(Delim),  // `(`, `[` or `{`
    CloseDelim(Delim), // `)`, `]` or `}`
    Comment,           // Only `//`. We dont support many lines comments like `/* Comment */`
    Whitespace,        // Any whitespace symbol: `\n`, `\t`, ` `, etc.
    Bang,              // `!`
    Eq,                // `=`
    NotEq,             // `!=`
    EqEq,              // `==`
    LessThan,          // `<`
    LtEq,              // `<=`
    GreaterThan,       // `>`
    GtEq,              // `>=`
    Plus,              // `+`
    PlusPlus,          // `++`
    Minus,             // `-`
    MinusMinux,        // `--`
    Slash,             // `/`
    Percent,           // `%`
    Star,              // `*`
    StarEq,            // `*=`
    PlusEq,            // `+=`
    MinusEq,           // `-=`
    SlashEq,           // `/=`
    PercentEq,         // `%=`
    Colon,             // `:`
    SemiColon,         // `;`
    Comma,             // `,`
    And,               // `&`
    AndAnd,            // `&&`
    Or,                // `|`
    OrOr,              // `||`
    Unknown,           // Any unknown token like `#` or `$`
    Eof,               // Final character in the file, aka `end of file`, `\0`
    ZeroToken,
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
pub enum Delim {
    Paren,
    Bracket,
    Brace,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
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
