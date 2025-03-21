use clap::Parser;

use crate::{errors::diagnostic::DiagnosticCtxt, program_sess::ProgramSess};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliLauncher {
    #[arg(short, long)]
    path: String,
}

impl CliLauncher {
    pub fn launch_with_exit(diag_ctxt: DiagnosticCtxt) {
        let path = CliLauncher::parse().path;
        let mut psess = match ProgramSess::from_path(path.as_str(), &diag_ctxt) {
            Ok(psess) => psess,
            Err(err) => {
                println!("Failed to get `{path}`: {err}",);
                std::process::exit(1)
            }
        };

        psess.run_with_exit();
    }
}
