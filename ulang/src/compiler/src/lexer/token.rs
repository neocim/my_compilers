use crate::span::Span;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Lit { kind: LiteralKind }, // Any literal like `1`, `"hello world"`, `'c'`
    Comment,      // Only `//`. We dont support many lines comments like `/* Comment */`
    Ident,        // `int`, `fn`, `while`, etc.
    Whitespace,   // Any whitespace symbol: `\n`, `\t`, ` `, etc.
    OpenParen,    // `(`
    CloseParen,   // `)`
    OpenBrace,    // `{`
    CloseBrace,   // `}`
    OpenBracket,  // `[`
    CloseBracket, // `]`
    Bang,         // `!`
    Eq,           // `=`
    LessThan,     // `<`
    GreaterThan,  // `>`
    Plus,         // `+`
    Minus,        // `-`
    Slash,        // `/`
    Percent,      // `%`
    Star,         // `*`
    Colon,        // `:`
    SemiColon,    // `;`
    Comma,        // `,`
    And,          // `&`
    Or,           // `|`
    Unknown,      // Any unknown token like `#` or `$`
    Eof,          // Final character in the file, aka `end of file`, `\0`
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralKind {
    Int,
    Float,
    Char { terminated: bool },
    Str { terminated: bool },
    Bool,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
}
