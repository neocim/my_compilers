mod errors;
#[cfg(test)]
mod tests;

use crate::{
    ast::{
        token::{BinOpKind, Token},
        Ast, BinOp, Expr, Lit, Stmt, TokenStream,
    },
    ast_lowering::{self, Lower},
    errors::{
        diagnostic::{Diagnostic, DiagnosticCtxt, DiagnosticHandler},
        ParseResult,
    },
    lexer::Lexer,
};
use errors::{ExpectedCloseParen, ExpectedExpr};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TokenCursor {
    token_stream: TokenStream,
}

impl TokenCursor {
    pub fn new(token_stream: TokenStream) -> Self {
        Self { token_stream }
    }

    fn advance(&mut self) -> Token {
        self.token_stream.next().unwrap_or(Token::Eof)
    }
}

pub struct Parser<'a> {
    token_cursor: TokenCursor,
    diag_ctxt: &'a DiagnosticCtxt,
    cur_tok: Token,
}

impl<'a> Parser<'a> {
    pub fn new(token_cursor: TokenCursor, diag_ctxt: &'a DiagnosticCtxt) -> Self {
        Self {
            token_cursor,
            diag_ctxt,
            cur_tok: Token::EmptyExpr,
        }
    }

    pub fn from_source(source: &str, diag_ctxt: &'a DiagnosticCtxt) -> Self {
        Parser::new(
            TokenCursor::new(Lexer::new(source).token_stream()),
            diag_ctxt,
        )
    }

    pub fn lowering_parse(&mut self) -> Result<ast_lowering::ast::Ast, Diagnostic<'a>> {
        Lower::new(self.diag_ctxt).lower(self.parse()?)
    }

    pub fn parse(&mut self) -> ParseResult<'a, Ast> {
        let stmt = self.parse_stmt()?;

        Ok(Ast::Stmt(stmt))
    }

    fn parse_stmt(&mut self) -> ParseResult<'a, Stmt> {
        let expr = self.parse_expr()?;

        Ok(Stmt::Expr(expr))
    }

    fn parse_expr(&mut self) -> ParseResult<'a, Expr> {
        let lterm = self.parse_term()?;

        match self.cur_tok.clone() {
            Token::BinOp(kind) if BinOpKind::Sub == kind || BinOpKind::Add == kind => {
                let rexpr = self.parse_expr()?;

                Ok(Expr::BinOp(BinOp::new(lterm, kind, rexpr)))
            }
            Token::Eof => Ok(lterm),
            _ => Ok(lterm),
        }
    }

    fn parse_term(&mut self) -> ParseResult<'a, Expr> {
        let lfactor = self.parse_factor()?;

        match self.advance() {
            Token::BinOp(kind) if BinOpKind::Div == kind || BinOpKind::Mul == kind => {
                let rexpr = self.parse_expr()?;

                Ok(Expr::BinOp(BinOp::new(lfactor, kind, rexpr)))
            }
            Token::Eof => Ok(lfactor),
            _ => Ok(lfactor),
        }
    }

    fn parse_factor(&mut self) -> ParseResult<'a, Expr> {
        match self.advance() {
            Token::Lit { kind } => Ok(Expr::Lit(Lit::new(kind))),
            Token::OpenParen => {
                let expr = self.parse_expr()?;

                if !self.expect(Token::CloseParen) {
                    Err(self
                        .handle()
                        .emit_err(ExpectedCloseParen::new(format!("{:?}", self.cur_tok))))
                } else {
                    Ok(expr)
                }
            }
            _ => Err(self
                .handle()
                .emit_err(ExpectedExpr::new(format!("{:?}", self.cur_tok)))),
        }
    }

    fn advance(&mut self) -> Token {
        let token = self.token_cursor.advance();

        if token != Token::Eof {
            self.cur_tok = token.clone();
        }

        token
    }

    fn expect(&mut self, expected_tok: Token) -> bool {
        self.cur_tok == expected_tok
    }

    fn handle(&self) -> DiagnosticHandler<'a> {
        self.diag_ctxt.handle()
    }
}
