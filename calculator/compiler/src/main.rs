use calc_sess::CalcSess;
use errors::{diagnostic::DiagnosticCtxt, emitter::StdoutEmitter};

pub mod ast;
pub mod ast_lowering;
mod calc_sess;
pub mod compile;
pub mod errors;
pub mod helpers;
pub mod lexer;
pub mod parser;

fn main() {
    let diag_ctxt = DiagnosticCtxt::new(Box::new(StdoutEmitter::new()));
    let sess = match CalcSess::from_path("../examples/", &diag_ctxt) {
        Ok(sess) => sess,
        Err(err) => {
            println!("Compile error: {}", err);
            std::process::exit(1)
        }
    };

    if let Err(err) = sess.compile() {}
}
