mod cursor;
mod errors;
#[cfg(test)]
mod tests;
mod token;

use std::fmt::Write as _;

use crate::{
    ast::token::{
        Delim, Ident, Literal, LiteralKind as AstLitKind, Token as AstToken,
        TokenKind as AstTokenKind, TokenStream,
    },
    errors::diagnostic::{Diagnostic, DiagnosticCtxt, DiagnosticLevel},
    span::{Pos, Span},
    symbol::{get_from_src, Symbol},
};
use cursor::Cursor;
use errors::{UnterminatedChar, UnterminatedString};
use token::{LiteralKind as LexerLitKind, Token as LexerToken, TokenKind as LexerTokenKind};

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
            cur_tok: AstToken::new(AstTokenKind::ZeroToken, Span::default()),
            dcx,
            cursor: Cursor::new(src),
        }
    }

    fn advance(&mut self) -> AstToken {
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
                LexerTokenKind::OpenParen => AstTokenKind::OpenDelim(Delim::Paren),
                LexerTokenKind::CloseParen => AstTokenKind::CloseDelim(Delim::Paren),
                LexerTokenKind::OpenBrace => AstTokenKind::OpenDelim(Delim::Brace),
                LexerTokenKind::CloseBrace => AstTokenKind::CloseDelim(Delim::Brace),
                LexerTokenKind::OpenBracket => AstTokenKind::OpenDelim(Delim::Bracket),
                LexerTokenKind::CloseBracket => AstTokenKind::CloseDelim(Delim::Paren),
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
            return AstTokenKind::Error(span);
        }
        let sym = Symbol::intern(get_from_src(self.src, span));

        AstTokenKind::Lit(Literal::new(AstLitKind::Str, sym, span))
    }

    /// Returns `AstTokenKind` if the literal is terminated, diagnostic otherwise
    fn char_lit(&self, terminated: bool, span: Span) -> AstTokenKind {
        if !terminated {
            self.dcx
                .emit_err(UnterminatedChar {}, DiagnosticLevel::Error, span);
            return AstTokenKind::Error(span);
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

    fn glue(&self, left_tok: AstTokenKind) -> Option<AstTokenKind> {
        match (left_tok, self.next_ahead().kind) {
            // `!=`
            (AstTokenKind::Bang, LexerTokenKind::Eq) => Some(AstTokenKind::NotEq),
            // `==`
            (AstTokenKind::Eq, LexerTokenKind::Eq) => Some(AstTokenKind::EqEq),
            // `<=`
            (AstTokenKind::LessThan, LexerTokenKind::Eq) => Some(AstTokenKind::LtEq),
            // `>=`
            (AstTokenKind::GreaterThan, LexerTokenKind::Eq) => Some(AstTokenKind::GtEq),
            // `||`
            (AstTokenKind::Or, LexerTokenKind::Or) => Some(AstTokenKind::OrOr),
            // `&&`
            (AstTokenKind::And, LexerTokenKind::And) => Some(AstTokenKind::AndAnd),
            (_, _) => None,
        }
    }

    fn next_ahead(&self) -> LexerToken {
        self.cursor.clone().next_token()
    }
}
