use crate::ast::CloseDelim;
use macros::IntoDiagnostic;

#[derive(IntoDiagnostic)]
#[message("unterminated string was found")]
pub struct UnterminatedString {}

#[derive(IntoDiagnostic)]
#[message("unterminated char was found")]
pub struct UnterminatedChar {}

#[derive(IntoDiagnostic)]
#[message("unexpected close delimiter `{delim}` was found")]
pub struct UnexpectedCloseDelim {
    delim: CloseDelim,
}

#[derive(IntoDiagnostic)]
#[message("mismatched delimiters: expected `{expected}`, but found `{found}`")]
pub struct MismatchedDelimiters {
    expected: CloseDelim,
    found: CloseDelim,
}

#[derive(IntoDiagnostic)]
#[message("close delim is expected, but end of file was found")]
pub struct EofButCloseDelimIsExpected {}

impl UnexpectedCloseDelim {
    pub fn new(delim: CloseDelim) -> Self {
        Self { delim }
    }
}

impl MismatchedDelimiters {
    pub fn new(expected: CloseDelim, found: CloseDelim) -> Self {
        Self { expected, found }
    }
}
