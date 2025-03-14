use std::fmt::Debug;

use super::{Parser, TokenCursor};
use crate::{
    ast::{
        token::{BinOpKind, LiteralKind},
        Ast, BinOp, Expr, Lit, Stmt,
    },
    errors::{
        diagnostic::{DiagnosticCtxt, DiagnosticMsgs},
        emitter::Emitter,
    },
    helpers::test::DebugHelper,
    lexer::Lexer,
};

#[derive(Debug)]
struct MockEmitter;

impl Emitter for MockEmitter {
    fn emit_diag(&self, diag_msgs: &DiagnosticMsgs) {
        panic!("{:?}", diag_msgs)
    }
}

#[test]
fn test_binop_parsing() {
    let mut lexer = Lexer::new("1.2345 * (2 + 3)");
    let diag_ctxt = DiagnosticCtxt::new(Box::new(MockEmitter));
    let mut parser = Parser::new(TokenCursor::new(lexer.token_stream()), &diag_ctxt);

    assert_eq!(
        DebugHelper::new_not_iterable(parser.parse().unwrap()),
        DebugHelper::new_not_iterable(Ast::Stmt(Stmt::Expr(Expr::BinOp(BinOp::new(
            Expr::Lit(Lit::new(LiteralKind::Float {
                val: "1.2345".to_string()
            })),
            BinOpKind::Mul,
            Expr::BinOp(BinOp::new(
                Expr::Lit(Lit::new(LiteralKind::Int {
                    val: "2".to_string()
                })),
                BinOpKind::Add,
                Expr::Lit(Lit::new(LiteralKind::Int {
                    val: "3".to_string()
                }))
            ))
        )))))
    );
}
