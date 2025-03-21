use compiler::{
    cli_launcher::CliLauncher,
    errors::{diagnostic::DiagnosticCtxt, emitter::StdoutEmitter},
};

fn main() {
    CliLauncher::launch_with_exit(DiagnosticCtxt::new(Box::new(StdoutEmitter::new())));
}
