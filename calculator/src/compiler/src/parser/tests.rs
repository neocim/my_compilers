use std::fmt::Debug;

use super::Parser;
use crate::{
    ast::{
        token::{BinOpKind, LiteralKind},
        Ast, BinOp, Expr, Lit, Stmt,
    },
    errors::{
        diagnostic::{DiagnosticCtxt, DiagnosticMsg},
        emitter::Emitter,
    },
    helpers::test::DebugHelper,
};

#[derive(Debug)]
struct MockEmitter;

impl Emitter for MockEmitter {
    fn emit_diag(&self, diag_msg: &DiagnosticMsg) {
        panic!("{:?}", diag_msg)
    }

    fn emit_warn(&self, diag_msg: &DiagnosticMsg) {
        panic!("{:?}", diag_msg)
    }
}

#[test]
fn test_binop_parsing() {
    let diag_ctxt = DiagnosticCtxt::new(Box::new(MockEmitter));
    let mut parser = Parser::from_source("1.2345 * (2 + 3)", &diag_ctxt);

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
