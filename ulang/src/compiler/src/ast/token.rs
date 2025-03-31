use crate::span::Span;

use super::Delimiter;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

/// A more generalized token type, unlike `lexer::TokenKind`, which can be more easily
/// converted to the desired AST item (for example, `TokenKind::PlusPlus` to `UnOp::UnAdd`)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Lit(Literal),              // Any literal like `1`, `"hello world"`, `'c'`
    Comment,     // Only `//`. We dont support many lines comments like `/* Comment */`
    Ident,       // `int`, `fn`, `while`, etc.
    Whitespace,  // Any whitespace symbol: `\n`, `\t`, ` `, etc.
    Bang,        // `!`
    Eq,          // `=`
    LessThan,    // `<`
    GreaterThan, // `>`
    Plus,        // `+`
    Minus,       // `-`
    Slash,       // `/`
    Percent,     // `%`
    Star,        // `*`
    StarEq,      // `*=`
    PercentEq,   // `%=`
    SlashEq,     // `/=`
    MinusEq,     // `-=`
    PlusEq,      // `+=`
    BangEq,      // `!=`
    LtEq,        // `<=`
    PlusPlus,    // `++`
    MinusMinus,  // `--`
    GtEq,        // `>=`
    OrOr,        // `||`
    AndAnd,      // `&&`
    OpenDelimiter(Delimiter), // `{`, `[`, `(`
    CloseDelimiter(Delimiter), // `}`, `]`, `)`
    Colon,       // `:`
    SemiColon,   // `;`
    Comma,       // `,`
    And,         // `&`
    Or,          // `|`
    Unknown,
    Eof,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Literal {
    kind: LiteralKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LiteralKind {
    Bool,
    Str,
    Char,
    Float,
    Int,
}
