use macros::IntoDiagnostic;

#[derive(IntoDiagnostic)]
#[message("unterminated string was found")]
pub struct UnterminatedString {}
