use crate::{
    ast_lowering::Lower,
    errors::{diagnostic::DiagnosticCtxt, emitter::Emitter},
    lexer::Lexer,
    parser::{Parser, TokenCursor},
};

use super::Calculator;

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
    let lower = Lower::new(&diag_ctxt);

    let mut lexer = Lexer::new("2.1 + 2.0 + 2.3");
    let mut parser = Parser::new(TokenCursor::new(lexer.token_stream()), &diag_ctxt);

    let ast = parser.parse().unwrap();
    let calculator = Calculator::new(lower.lower(ast).unwrap(), &diag_ctxt);

    let res = calculator.compile().unwrap().get_float().unwrap();
    assert_eq!(res, 6.4);
}
