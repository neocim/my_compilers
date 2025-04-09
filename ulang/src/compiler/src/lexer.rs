pub(crate) mod cursor;
#[cfg(test)]
mod tests;
pub(crate) mod token;

use crate::{
    ast::token::{Token as AstToken, TokenKind as AstTokenKind},
    span::{Pos, Span},
};
use cursor::Cursor;
use token::{Token as LexerToken, TokenKind as LexerTokenKind};

pub struct Lexer<'src> {
    src: &'src str,
    pos: Pos,
    cursor: Cursor<'src>,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Self {
        Self {
            src,
            pos: Pos::new(1, 0),
            cursor: Cursor::new(src),
        }
    }

    pub fn next(&mut self) -> AstToken {
        let token = self.cursor.next_token();

        let kind = match token.kind {
            LexerTokenKind::Lit { kind } => todo!(),
            LexerTokenKind::Comment => todo!(),
            LexerTokenKind::Ident => todo!(),
            LexerTokenKind::Whitespace => todo!(),
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
    }

    fn glue_or_invalid(&mut self, token: LexerTokenKind, start: Pos) -> AstToken {
        let (kind, len) = match token {
            LexerTokenKind::And => {
                if self.next_ahead() == LexerTokenKind::And {
                    self.eat_and_tok(AstTokenKind::AndAnd)
                } else {
                    (AstTokenKind::And, 1)
                }
            }
            LexerTokenKind::Bang => {
                if self.next_ahead() == LexerTokenKind::Eq {
                    self.eat_and_tok(AstTokenKind::NotEq)
                } else {
                    (AstTokenKind::Bang, 1)
                }
            }
        };

        AstToken::new(kind, Span::new(start, self.reset_pos(len)))
    }

    /// Uses in `Lexer::glue_or_invalid()` to shorten the code.
    fn eat_and_tok(&mut self, token: AstTokenKind) -> (AstTokenKind, u16) {
        self.cursor.next_token();
        (token, 2)
    }

    fn next_ahead(&mut self) -> LexerTokenKind {
        self.cursor.clone().next_token().kind
    }

    fn reset_pos(&mut self, tok_len: u16) -> Pos {
        self.pos = Pos::new(self.pos.ln, self.pos.col + tok_len);
        self.pos
    }
}
