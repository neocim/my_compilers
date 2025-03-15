use std::{env, process::exit};

use compile::{Calculator, Compile};
use errors::{diagnostic::DiagnosticCtxt, emitter::StdoutEmitter};

pub mod ast;
pub mod ast_lowering;
pub mod compile;
pub mod errors;
pub mod helpers;
pub mod lexer;
pub mod parser;

fn main() {
    // let cur_dir = env::current_dir().unwrap();
    // let diag_ctxt = DiagnosticCtxt::new(Box::new(StdoutEmitter::new()));
    // let calculator = match Calculator::from_source_file(
    //     "~/Code/my_compilers/calculator/examples/binary_expr.calc",
    //     &diag_ctxt,
    // ) {
    //     Ok(calc) => calc,
    //     Err(err) => {
    //         err.emit();
    //         exit(1)
    //     }
    // };

    // let res = calculator.compile().unwrap();
}
