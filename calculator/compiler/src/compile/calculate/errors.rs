use crate::errors::diagnostic::{
    Diagnostic, DiagnosticCtxt, DiagnosticMsg, DiagnosticMsgs, IntoDiagnostic,
};
use calculator_macros::IntoDiagnostic;

#[derive(IntoDiagnostic)]
#[diagnostic("mismatched types: `{}` and `{}`")]
pub struct MismatchedTypes {
    lty: String,
    rty: String,
}

impl MismatchedTypes {
    pub fn new(lty: String, rty: String) -> Self {
        Self { lty, rty }
    }
}
