use crate::errors::{diagnostic::DiagnosticCtxt, emitter::Emitter};

use super::Program;

#[derive(Debug)]
struct MockEmitter;

impl Emitter for MockEmitter {
    fn emit_diag(&self, diag_msgs: &crate::errors::diagnostic::DiagnosticMsgs) {
        panic!("{:?}", diag_msgs)
    }
}

#[test]
fn test_calculate() {
    let diag_ctxt = DiagnosticCtxt::new(Box::new(MockEmitter));
    let res = Program::from_source("2 + 2 * 2", "".to_string(), &diag_ctxt)
        .unwrap()
        .compile()
        .unwrap()
        .get_int()
        .unwrap();

    assert_eq!(res, 6);
}
