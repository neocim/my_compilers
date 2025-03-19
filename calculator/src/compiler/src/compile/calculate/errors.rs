use crate::errors::diagnostic::{Diagnostic, DiagnosticCtxt, DiagnosticMsg, IntoDiagnostic};
use calculator_macros::IntoDiagnostic;

#[derive(IntoDiagnostic)]
#[diagnostic("failed to open file with path `{}`: {}")]
pub struct OpenFileError {
    path: String,
    msg: String,
}

impl OpenFileError {
    pub fn new(path: String, msg: String) -> Self {
        Self { path, msg }
    }
}

#[derive(IntoDiagnostic)]
#[diagnostic("wrong extension `{:?}` for file `{}`! Expected extension should be `{}`.")]
pub struct WrongFileExtension {
    ext: Option<String>,
    file_name: String,
    expected_ext: String,
}

impl WrongFileExtension {
    pub fn new(ext: Option<String>, file_name: String, expected_ext: String) -> Self {
        Self {
            ext,
            file_name,
            expected_ext,
        }
    }
}

#[derive(IntoDiagnostic)]
#[diagnostic("mismatched types: `{}` and `{}`")]
pub struct MismatchedTypes {
    rty: String,
    lty: String,
}

impl MismatchedTypes {
    pub fn new(lty: String, rty: String) -> Self {
        Self { lty, rty }
    }
}
