use crate::errors::diagnostic::{
    Diagnostic, DiagnosticCtxt, DiagnosticMsg, DiagnosticMsgs, IntoDiagnostic,
};
use calculator_macros::IntoDiagnostic;

#[derive(IntoDiagnostic)]
#[diagnostic("failed to compile program with path: {}")]
pub struct CompileError {
    msg: String,
}

impl CompileError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
