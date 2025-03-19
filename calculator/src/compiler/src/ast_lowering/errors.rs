use crate::errors::diagnostic::{Diagnostic, DiagnosticCtxt, DiagnosticMsg, IntoDiagnostic};
use calculator_macros::IntoDiagnostic;

#[derive(IntoDiagnostic)]
#[diagnostic("failed to parse float `{}`: {}")]
pub struct ParseFloatError {
    float: String,
    msg: String,
}

impl ParseFloatError {
    pub fn new(float: String, msg: String) -> Self {
        Self { float, msg }
    }
}

#[derive(IntoDiagnostic)]
#[diagnostic("failed to parse int `{}`: {}")]
pub struct ParseIntError {
    int: String,
    msg: String,
}

impl ParseIntError {
    pub fn new(int: String, msg: String) -> Self {
        Self { int, msg }
    }
}
