use macros::IntoDiagnostic;

pub mod ast;
pub mod errors;
pub mod lexer;
pub mod span;
pub mod symbol;

#[derive(IntoDiagnostic)]
#[message("Maxim gay {sfasf}")]
struct Pipiska {
    sfasf: String,
}
