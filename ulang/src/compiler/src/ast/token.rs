use super::{CloseDelim, Delim, Ident, Literal, OpenDelim};
use crate::span::Span;

#[derive(Debug, PartialEq)]
pub struct TokenStream(pub(crate) Vec<TokenTree>);

/// Why is the division into just a `Token(...)` and `DelimitedStream(...)`?
/// Here, `Token` is a single token, such as `Ident`, `Literal`, etc. The `DelimitedStream`,
/// in turn, stores these `Token`s separated by some kind of separator, such as `[`, `{`, `(`.
/// For example, if we have a function definition, then first in our `TokenStream` (the structure
/// for which this `TokenTree` is made) there will be a keyword and a function name,
/// that is, a pair of `TokenTree::Token(...)`. Then, when we get to the opening parenthesis
/// into which we pass the arguments to the function, we will parse all these arguments into
/// a single `TokenTree::DelimitedStream(...)`, inside which there will be a `TokenStream`
/// with function arguments and information about its separators, such as its kind (in our case, this is parens)
/// and location. This is convenient because we can immediately receive a convenient token stream, which will
/// be convenient to work on in the future. For example, our parser won't need to check where blocks
/// open and close, and it can just work on the syntax. Of course, we can do this in a parser,
/// but then it will be too loaded and inconvenient.
#[derive(Debug, PartialEq)]
pub enum TokenTree {
    Token(Token),
    DelimitedStream(DelimitedStream),
}

#[derive(Debug, PartialEq)]
pub struct DelimitedStream {
    open_delim: Span,
    close_delim: Span,
    delim: Delim,
    stream: TokenStream,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    Lit(Literal),
    Ident(Ident),
    OpenDelim(OpenDelim),   // `(`, `[` or `{`
    CloseDelim(CloseDelim), // `)`, `]` or `}`
    Comment,                // Only `//`. We dont support many lines comments like `/* Comment */`
    Whitespace,             // Any whitespace symbol: `\n`, `\t`, ` `, etc.
    Bang,                   // `!`
    Eq,                     // `=`
    NotEq,                  // `!=`
    EqEq,                   // `==`
    LessThan,               // `<`
    LtEq,                   // `<=`
    GreaterThan,            // `>`
    GtEq,                   // `>=`
    Plus,                   // `+`
    PlusPlus,               // `++`
    Minus,                  // `-`
    MinusMinux,             // `--`
    Slash,                  // `/`
    Percent,                // `%`
    Star,                   // `*`
    StarEq,                 // `*=`
    PlusEq,                 // `+=`
    MinusEq,                // `-=`
    SlashEq,                // `/=`
    PercentEq,              // `%=`
    Colon,                  // `:`
    SemiColon,              // `;`
    Comma,                  // `,`
    And,                    // `&`
    AndAnd,                 // `&&`
    Or,                     // `|`
    OrOr,                   // `||`
    Unknown,                // Any unknown token like `#` or `$`
    Eof,                    // Final character in the file, aka `end of file`, `\0`
    InitToken,              // Initial token
    Error,
}

impl TokenStream {
    pub fn new(stream: Vec<TokenTree>) -> Self {
        Self(stream)
    }
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }

    /// Returns `Some(TokenKind)` if the token was glue, `None` otherwise.
    pub fn glue(&self, left_tok: TokenKind) -> Option<TokenKind> {
        match (self.kind, left_tok) {
            // `!=`
            (TokenKind::Bang, TokenKind::Eq) => Some(TokenKind::NotEq),
            // `==`
            (TokenKind::Eq, TokenKind::Eq) => Some(TokenKind::EqEq),
            // `<=`
            (TokenKind::LessThan, TokenKind::Eq) => Some(TokenKind::LtEq),
            // `>=`
            (TokenKind::GreaterThan, TokenKind::Eq) => Some(TokenKind::GtEq),
            // `||`
            (TokenKind::Or, TokenKind::Or) => Some(TokenKind::OrOr),
            // `&&`
            (TokenKind::And, TokenKind::And) => Some(TokenKind::AndAnd),
            (_, _) => None,
        }
    }
}

impl DelimitedStream {
    pub fn new(delim: Delim, open_delim: Span, close_delim: Span, stream: TokenStream) -> Self {
        Self {
            delim,
            open_delim,
            close_delim,
            stream,
        }
    }
}
