pub(crate) mod cursor;
mod errors;
#[cfg(test)]
mod tests;
pub(crate) mod token;

use std::fmt::Write as _;

use crate::{
    ast::token::{
        Literal, LiteralKind as AstLitKind, Token as AstToken, TokenKind as AstTokenKind,
    },
    errors::diagnostic::{Diagnostic, DiagnosticCtxt, DiagnosticLevel},
    span::{Pos, Span},
    symbol::Symbol,
};
use cursor::Cursor;
use errors::UnterminatedString;
use token::{LiteralKind as LexerLitKind, Token as LexerToken, TokenKind as LexerTokenKind};

#[derive(Clone)]
pub struct Lexer<'src> {
    src: &'src str,
    pos: Pos,
    token: AstToken,
    dcx: &'src DiagnosticCtxt,
    cursor: Cursor<'src>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str, dcx: &'src DiagnosticCtxt) -> Self {
        Self {
            src,
            pos: Pos::new(1, 1),
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
                LexerTokenKind::Comment => todo!(),
                LexerTokenKind::Ident => todo!(),
                LexerTokenKind::Whitespace => continue,
                LexerTokenKind::OpenParen => todo!(),
                LexerTokenKind::CloseParen => todo!(),
                LexerTokenKind::OpenBrace => todo!(),
                LexerTokenKind::CloseBrace => todo!(),
                LexerTokenKind::OpenBracket => todo!(),
                LexerTokenKind::CloseBracket => todo!(),
                LexerTokenKind::Bang => todo!(),
                LexerTokenKind::Eq => todo!(),
                LexerTokenKind::LessThan => todo!(),
                LexerTokenKind::GreaterThan => todo!(),
                LexerTokenKind::Plus => todo!(),
                LexerTokenKind::Minus => todo!(),
                LexerTokenKind::Slash => todo!(),
                LexerTokenKind::Percent => todo!(),
                LexerTokenKind::Star => todo!(),
                LexerTokenKind::Colon => todo!(),
                LexerTokenKind::SemiColon => todo!(),
                LexerTokenKind::Comma => todo!(),
                LexerTokenKind::And => todo!(),
                LexerTokenKind::Or => todo!(),
                LexerTokenKind::Unknown => todo!(),
                LexerTokenKind::Eof => AstTokenKind::Eof,
            };

            return Ok(AstToken::new(kind, token.span));
        }
    }

    fn literal(&mut self, kind: LexerLitKind, span: Span) -> Result<AstTokenKind, Diagnostic> {
        match kind {
            LexerLitKind::Int => Ok(self.num_lit(kind, span)),
            LexerLitKind::Float => Ok(self.num_lit(kind, span)),
            LexerLitKind::Char { terminated } => self.char_lit(terminated, span),
            LexerLitKind::Str { terminated } => self.str_lit(terminated, span),
            LexerLitKind::Bool => Ok(self.bool_lit(span)),
        }
    }

    /// Returns `AstTokenKind' if the literal is terminated, diagnostic otherwise
    fn str_lit(&mut self, terminated: bool, span: Span) -> Result<AstTokenKind, Diagnostic> {
        if !terminated {
            return Err(self
                .dcx
                .emit_err(UnterminatedString {}, DiagnosticLevel::Error, span));
        }
        let sym = Symbol::intern(self.get_from_src(span));

        Ok(AstTokenKind::Lit(Literal::new(AstLitKind::Str, sym, span)))
    }

    /// Returns `AstTokenKind' if the literal is terminated, diagnostic otherwise
    fn char_lit(&mut self, terminated: bool, span: Span) -> Result<AstTokenKind, Diagnostic> {
        if !terminated {
            return Err(self
                .dcx
                .emit_err(UnterminatedString {}, DiagnosticLevel::Error, span));
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
