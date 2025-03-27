#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    kind: TokenKind,
    size: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Comment,      // Only `//`. We dont support many lines comment like `/* Comment */`
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
    Eof,
}

impl Token {
    pub fn new(kind: TokenKind, size: u32) -> Self {
        Token { kind, size }
    }
}
