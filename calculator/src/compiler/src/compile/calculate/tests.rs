use crate::{
    ast_lowering::ast::{Lit, LiteralKind},
    errors::{
        diagnostic::{DiagnosticCtxt, DiagnosticMsg},
        emitter::Emitter,
    },
};

use super::Program;

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
fn test_calculate() {
    let diag_ctxt = DiagnosticCtxt::new(Box::new(MockEmitter));
    let res = Program::from_source("2 + 2 * 2", "".to_string(), &diag_ctxt)
        .unwrap()
        .compile()
        .unwrap();

    assert_eq!(
        res,
        Lit {
            kind: LiteralKind::Int { val: 6 }
        }
    );
}
