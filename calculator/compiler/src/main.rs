use cli_launcher::CliLauncher;

pub mod ast;
pub mod ast_lowering;
mod cli_launcher;
pub mod compile;
pub mod errors;
pub mod helpers;
pub mod lexer;
pub mod parser;
mod program_sess;

fn main() {
    CliLauncher::launch_with_exit();
}
