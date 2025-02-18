use crate::{
    ast::{token::Token, Ast, Expr, TokenStream},
    errors::{diagnostic::Diag, emitter::Emitter},
};

mod errors;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenCursor {
    token_stream: TokenStream,
}

impl TokenCursor {
    pub fn new(token_stream: TokenStream) -> Self {
        Self { token_stream }
    }

    pub fn advance(&mut self) -> Token {
        self.token_stream.next().unwrap_or(Token::Eof)
    }
}

#[derive(Clone, Debug)]
pub struct Parser<'s, E: Emitter> {
    token_cursor: TokenCursor,
    cur_tok: Token,
    diag: Diag<'s, E>,
}

impl<'s, E: Emitter> Parser<'s, E> {
    pub fn parse(&mut self) -> Result<Ast, Diag<'s, E>> {
        loop {
            let stmt = self.parse_stmt();

            if self.cur_tok == Token::Eof {
                break;
            }
        }
        unimplemented!()
    }

    fn parse_stmt(&self) -> Result<Expr, Diag<'s, E>> {
        let expr = self.parse_expr();

        unimplemented!()
    }

    fn parse_expr(&self) -> Result<Expr, Diag<'s, E>> {
        unimplemented!()
    }

    fn parse_term(&self) -> Result<Expr, Diag<'s, E>> {
        unimplemented!()
    }

    fn parse_factor(&mut self) -> Result<Expr, Diag<'s, E>> {
        let token = self.advance();

        let expr = match token {
            Token::Lit { kind } => Expr::Lit { kind },
            _ => unimplemented!(),
        };

        Ok(expr)
    }

    fn advance(&mut self) -> Token {
        let token = self.token_cursor.advance();
        self.cur_tok = token.clone();

        token
    }

    fn expect(&mut self, expected_tok: Token) -> bool {
        self.cur_tok == expected_tok
    }
}
