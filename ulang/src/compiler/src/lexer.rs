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
    span::Span,
    symbol::Symbol,
};
use cursor::Cursor;
use errors::{UnterminatedChar, UnterminatedString};
use token::{LiteralKind as LexerLitKind, TokenKind as LexerTokenKind};

#[derive(Clone)]
pub struct Lexer<'src> {
    src: &'src str,
    token: AstToken,
    dcx: &'src DiagnosticCtxt,
    cursor: Cursor<'src>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str, dcx: &'src DiagnosticCtxt) -> Self {
        Self {
            src,
            token: AstToken::new(AstTokenKind::ZeroToken, Default::default()),
            dcx,
            cursor: Cursor::new(src),
        }
    }

    fn next_from_cursor(&mut self) -> Result<AstToken, Diagnostic> {
        // Loop because we want to skip whitespaces
        loop {
            let token = self.cursor.next_token();

            let kind = match token.kind {
                LexerTokenKind::Lit { kind } => self.literal(kind, token.span)?,
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

            return Ok(AstToken::new(kind, token.span));
        }
    }

    fn ident(&self, span: Span) -> AstTokenKind {
        let sym = Symbol::intern(self.get_from_src(span));
        AstTokenKind::Ident(Ident::new(sym, span))
    }

    fn literal(&self, kind: LexerLitKind, span: Span) -> Result<AstTokenKind, Diagnostic> {
        match kind {
            LexerLitKind::Int | LexerLitKind::Float => Ok(self.num_lit(kind, span)),
            LexerLitKind::Bool => Ok(self.bool_lit(span)),
            LexerLitKind::Char { terminated } => self.char_lit(terminated, span),
            LexerLitKind::Str { terminated } => self.str_lit(terminated, span),
        }
    }

    /// Returns `AstTokenKind' if the literal is terminated, diagnostic otherwise
    fn str_lit(&self, terminated: bool, span: Span) -> Result<AstTokenKind, Diagnostic> {
        if !terminated {
            return Err(self
                .dcx
                .emit_err(UnterminatedString {}, DiagnosticLevel::Error, span));
        }
        let sym = Symbol::intern(self.get_from_src(span));

        Ok(AstTokenKind::Lit(Literal::new(AstLitKind::Str, sym, span)))
    }

    /// Returns `AstTokenKind' if the literal is terminated, diagnostic otherwise
    fn char_lit(&self, terminated: bool, span: Span) -> Result<AstTokenKind, Diagnostic> {
        if !terminated {
            return Err(self
                .dcx
                .emit_err(UnterminatedChar {}, DiagnosticLevel::Error, span));
        }
        let sym = Symbol::intern(self.get_from_src(span));

        Ok(AstTokenKind::Lit(Literal::new(AstLitKind::Str, sym, span)))
    }

    fn bool_lit(&self, span: Span) -> AstTokenKind {
        let sym = Symbol::intern(self.get_from_src(span));
        AstTokenKind::Lit(Literal::new(AstLitKind::Bool, sym, span))
    }

    /// ### PANIC
    /// - Only if we passed a `LiteralKind`, which is not an integer or a floating point number.
    fn num_lit(&self, kind: LexerLitKind, span: Span) -> AstTokenKind {
        let sym = Symbol::intern(self.get_from_src(span));
        let kind = match kind {
            LexerLitKind::Int => AstLitKind::Int,
            LexerLitKind::Float => AstLitKind::Float,
            // We should only call this function if we see the number literal
            _ => unreachable!(),
        };
        AstTokenKind::Lit(Literal::new(kind, sym, span))
    }

    fn glue(&mut self, left_tok: AstTokenKind) -> AstTokenKind {
        let (kind, need_advance) = match (left_tok, self.next_ahead()) {
            // `!=`
            (AstTokenKind::Bang, LexerTokenKind::Eq) => (AstTokenKind::NotEq, true),
            // `==`
            (AstTokenKind::Eq, LexerTokenKind::Eq) => (AstTokenKind::EqEq, true),
            // `<=`
            (AstTokenKind::LessThan, LexerTokenKind::Eq) => (AstTokenKind::LtEq, true),
            // `>=`
            (AstTokenKind::GreaterThan, LexerTokenKind::Eq) => (AstTokenKind::GtEq, true),
            // `||`
            (AstTokenKind::Or, LexerTokenKind::Or) => (AstTokenKind::OrOr, true),
            // `&&`
            (AstTokenKind::And, LexerTokenKind::And) => (AstTokenKind::AndAnd, true),
            (_, _) => (left_tok, false),
        };
        if need_advance {
            self.cursor.next_token();
        }
        kind
    }

    /// Gets string from source text by its span.
    /// ### PANIC
    /// - ONLY if we passed the wrong span, but our `Cursor` ensures that it will be correct.
    /// - Also it can panic if I made a mistakes in the code.
    fn get_from_src(&self, span: Span) -> String {
        let src = self.src;
        let mut result = String::new();
        // Columns
        let l_col = span.lo.col as usize;
        let r_col = span.hi.col as usize;
        // Lines
        let start_l = span.lo.ln as usize;
        let end_l = span.hi.ln as usize;

        // If we need to take several lines, then we will iterate over them.
        if start_l != end_l {
            for (i, line) in src.lines().enumerate().take(end_l.into()).skip(start_l - 1) {
                if i == start_l - 1 {
                    let start_byte = line
                        .char_indices()
                        .nth(l_col - 1)
                        .expect("Failed to get the start byte of the string")
                        .0;
                    writeln!(result, "{}", &line[start_byte..])
                        .expect("Failed to write line into result string");
                } else if i == end_l - 1 {
                    let end_byte = line
                        .char_indices()
                        .nth(r_col - 1)
                        .map(|(end, _)| end)
                        .unwrap_or(line.len());
                    write!(result, "{}", &line[..end_byte])
                        .expect("Failed to write line into result string");
                } else {
                    writeln!(result, "{}", line).expect("Failed to write line into result string");
                }
            }

            return result;
        }
        // If we are here, then we only need to take one line, so we take it in such a simple way.
        let line = src
            .lines()
            .nth(start_l - 1)
            .expect("Failed to get line by start line");
        let start_byte = src
            .char_indices()
            .nth(l_col - 1)
            .expect("Failed to get start byte of the string")
            .0;
        let end_byte = src
            .char_indices()
            .nth(r_col - 1)
            .map(|(end, _)| end)
            .unwrap_or(line.len());
        write!(result, "{}", &line[start_byte..end_byte])
            .expect("Failed to write line into result string");

        result
    }

    fn next_ahead(&mut self) -> LexerTokenKind {
        self.cursor.clone().next_token().kind
    }
}
