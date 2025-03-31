use crate::span::Span;

use super::Delimiter;

/// A token that is almost the same as in `lexer`, but
/// is more high-level and convenient for building an AST
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

/// A more generalized token kind, unlike `lexer::TokenKind`, which can be more easily
/// converted to the desired AST item (for example, `TokenKind::PlusPlus` to `UnOp::UnAdd`)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Lit(Literal),              // Any literal like `1`, `"hello world"`, `'c'`
    Comment,     // Only `//`. We dont support many lines comments like `/* Comment */`
    Ident,       // `int`, `fn`, `while`, etc.
    Whitespace,  // Any whitespace symbol: `\n`, `\t`, ` `, etc.
    Plus,        // `+`
    Minus,       // `-`
    Slash,       // `/`
    Percent,     // `%`
    Star,        // `*`
    Bang,        // `!`
    Eq,          // `=`
    LessThan,    // `<`
    GreaterThan, // `>`
    StarEq,      // `*=`
    PercentEq,   // `%=`
    SlashEq,     // `/=`
    MinusEq,     // `-=`
    PlusEq,      // `+=`
    BangEq,      // `!=`
    LtEq,        // `<=`
    GtEq,        // `>=`
    EqEq,        // `==`
    PlusPlus,    // `++`
    MinusMinus,  // `--`
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
