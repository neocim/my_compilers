mod cursor;
mod errors;
#[cfg(test)]
mod tests;
mod token;

use crate::{
    ast::{
        token::{
            DelimitedStream, Token as AstToken, TokenKind as AstTokenKind, TokenStream, TokenTree,
        },
        CloseDelim, Delim, Ident, Literal, LiteralKind as AstLitKind, OpenDelim,
    },
    errors::diagnostic::{Diagnostic, DiagnosticCtxt, DiagnosticLevel},
    span::Span,
    symbol::{get_from_src, Symbol},
};
use cursor::Cursor;
use errors::{
    EofButCloseDelimIsExpected, MismatchedDelimiters, UnexpectedCloseDelim, UnterminatedChar,
    UnterminatedString,
};
use token::{LiteralKind as LexerLitKind, TokenKind as LexerTokenKind};

#[derive(Clone)]
pub struct Lexer<'src> {
    src: &'src str,
    cur_tok: AstToken,
    dcx: &'src DiagnosticCtxt,
    cursor: Cursor<'src>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str, dcx: &'src DiagnosticCtxt) -> Self {
        Self {
            src,
            cur_tok: AstToken::new(AstTokenKind::InitToken, Span::default()),
            dcx,
            cursor: Cursor::new(src),
        }
    }

    /// Returns the `TokenStream` using the `lexer::Cursor`. Now, if there are errors related to delimiters,
    /// the compiler will emit diagnostic and stop compilation. We need to make a friendlier error message in the future.
    pub fn token_stream(&mut self, is_delimited: bool) -> TokenStream {
        let mut stream = Vec::new();

        loop {
            match self.cur_tok.kind {
                AstTokenKind::OpenDelim(delim) => {
                    stream.push(self.delimited_stream(delim.0));
                }
                AstTokenKind::CloseDelim(delim) => {
                    if is_delimited {
                        return TokenStream::new(stream);
                    } else {
                        self.dcx
                            .emit_fatal(UnexpectedCloseDelim::new(delim), self.cur_tok.span);
                    }
                }
                AstTokenKind::Eof => {
                    if !is_delimited {
                        return TokenStream::new(stream);
                    } else {
                        self.dcx
                            .emit_fatal(EofButCloseDelimIsExpected {}, self.cur_tok.span);
                    }
                }
                _ => stream.push(TokenTree::Token(self.cur_tok)),
            }
        }
    }

    pub fn delimited_stream(&mut self, exp_delim: Delim) -> TokenTree {
        let open_delim_sp = self.cur_tok.span;
        // We only call `delimited_stream()` if see delimiter, therefore, we pass `is_delimited: true`
        let stream = self.token_stream(true);
        let close_delim_sp = self.cur_tok.span;

        match self.cur_tok.kind {
            AstTokenKind::CloseDelim(close_delim) if close_delim.0 == exp_delim => {}
            // Mismatched delimiters. We should raise an error
            AstTokenKind::CloseDelim(close_delim) => {
                self.dcx.emit_fatal(
                    MismatchedDelimiters::new(CloseDelim(exp_delim), close_delim),
                    close_delim_sp,
                );
            }
            AstTokenKind::Eof => {}
            // We call this metod only if we have close delimiter, then this is unreachable
            _ => unreachable!(),
        }

        TokenTree::DelimitedStream(DelimitedStream::new(
            exp_delim,
            open_delim_sp,
            close_delim_sp,
            stream,
        ))
    }

    pub fn advance(&mut self) -> AstToken {
        let token = self.next_from_cursor();
        if let Some(glued) = token.glue(token.kind) {
            // Eat the token that we are glued
            self.next_from_cursor();

            let token = AstToken::new(glued, token.span.increment_hi_col());
            self.cur_tok = token;
            return token;
        }
        token
    }

    fn next_from_cursor(&mut self) -> AstToken {
        // Loop because we want to skip whitespaces
        loop {
            let token = self.cursor.next_token();

            let kind = match token.kind {
                LexerTokenKind::Lit { kind } => self.literal(kind, token.span),
                LexerTokenKind::Ident => self.ident(token.span),
                LexerTokenKind::Comment => AstTokenKind::Comment,
                LexerTokenKind::OpenParen => AstTokenKind::OpenDelim(OpenDelim(Delim::Paren)),
                LexerTokenKind::CloseParen => AstTokenKind::CloseDelim(CloseDelim(Delim::Paren)),
                LexerTokenKind::OpenBrace => AstTokenKind::OpenDelim(OpenDelim(Delim::Brace)),
                LexerTokenKind::CloseBrace => AstTokenKind::CloseDelim(CloseDelim(Delim::Brace)),
                LexerTokenKind::OpenBracket => AstTokenKind::OpenDelim(OpenDelim(Delim::Bracket)),
                LexerTokenKind::CloseBracket => {
                    AstTokenKind::CloseDelim(CloseDelim(Delim::Bracket))
                }
                LexerTokenKind::Bang => AstTokenKind::Bang,
                LexerTokenKind::Eq => AstTokenKind::Eq,
                LexerTokenKind::LessThan => AstTokenKind::LessThan,
                LexerTokenKind::GreaterThan => AstTokenKind::GreaterThan,
                LexerTokenKind::Plus => AstTokenKind::Plus,
                LexerTokenKind::Minus => AstTokenKind::Minus,
                LexerTokenKind::Slash => AstTokenKind::Slash,
                LexerTokenKind::Percent => AstTokenKind::Percent,
                LexerTokenKind::Star => AstTokenKind::Star,
                LexerTokenKind::Colon => AstTokenKind::Colon,
                LexerTokenKind::SemiColon => AstTokenKind::SemiColon,
                LexerTokenKind::Comma => AstTokenKind::Comma,
                LexerTokenKind::And => AstTokenKind::And,
                LexerTokenKind::Or => AstTokenKind::Or,
                LexerTokenKind::Unknown => AstTokenKind::Unknown,
                LexerTokenKind::Eof => AstTokenKind::Eof,
                LexerTokenKind::Whitespace => continue,
            };

            return AstToken::new(kind, token.span);
        }
    }

    fn ident(&self, span: Span) -> AstTokenKind {
        let sym = Symbol::intern(get_from_src(self.src, span));
        AstTokenKind::Ident(Ident::new(sym, span))
    }

    fn literal(&self, kind: LexerLitKind, span: Span) -> AstTokenKind {
        match kind {
            LexerLitKind::Int | LexerLitKind::Float => self.num_lit(kind, span),
            LexerLitKind::Bool => self.bool_lit(span),
            LexerLitKind::Char { terminated } => self.char_lit(terminated, span),
            LexerLitKind::Str { terminated } => self.str_lit(terminated, span),
        }
    }

    /// Returns `AstTokenKind` if the literal is terminated, diagnostic otherwise
    fn str_lit(&self, terminated: bool, span: Span) -> AstTokenKind {
        if !terminated {
            self.dcx
                .emit_err(UnterminatedString {}, DiagnosticLevel::Error, span);
            return AstTokenKind::Error;
        }
        let sym = Symbol::intern(get_from_src(self.src, span));

        AstTokenKind::Lit(Literal::new(AstLitKind::Str, sym, span))
    }

    /// Returns `AstTokenKind` if the literal is terminated, diagnostic otherwise
    fn char_lit(&self, terminated: bool, span: Span) -> AstTokenKind {
        if !terminated {
            self.dcx
                .emit_err(UnterminatedChar {}, DiagnosticLevel::Error, span);
            return AstTokenKind::Error;
        }
        let sym = Symbol::intern(get_from_src(self.src, span));

        AstTokenKind::Lit(Literal::new(AstLitKind::Char, sym, span))
    }

    fn bool_lit(&self, span: Span) -> AstTokenKind {
        let sym = Symbol::intern(get_from_src(self.src, span));
        AstTokenKind::Lit(Literal::new(AstLitKind::Bool, sym, span))
    }

    /// ### PANIC
    /// - Only if we passed a `LiteralKind`, which is not an integer or a floating point number.
    fn num_lit(&self, kind: LexerLitKind, span: Span) -> AstTokenKind {
        let sym = Symbol::intern(get_from_src(self.src, span));
        let kind = match kind {
            LexerLitKind::Int => AstLitKind::Int,
            LexerLitKind::Float => AstLitKind::Float,
            // We should only call this function if we see the number literal
            _ => unreachable!(),
        };
        AstTokenKind::Lit(Literal::new(kind, sym, span))
    }
}
