use macros::IntoDiagnostic;

#[derive(IntoDiagnostic)]
#[message("unterminated string was found")]
pub struct UnterminatedString {}

#[derive(IntoDiagnostic)]
#[message("unterminated char was found")]
pub struct UnterminatedChar {}
