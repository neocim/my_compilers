use crate::errors::diagnostic::{
    Diagnostic, DiagnosticCtxt, DiagnosticMsg, DiagnosticMsgs, IntoDiagnostic,
};
use calculator_macros::IntoDiagnostic;

#[derive(IntoDiagnostic)]
#[diagnostic("expected close paren `)`, but found `{}`")]
pub struct ExpectedCloseParen {
    unexpected: String,
}

impl ExpectedCloseParen {
    pub fn new(unexpected: String) -> Self {
        Self { unexpected }
    }
}

#[derive(IntoDiagnostic)]
#[diagnostic("expected expression but found `{}`")]
pub struct ExpectedExpr {
    unexpected: String,
}

impl ExpectedExpr {
    pub fn new(unexpected: String) -> Self {
        Self { unexpected }
    }
}
