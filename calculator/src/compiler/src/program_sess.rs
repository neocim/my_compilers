use std::{env, io, process::exit};

use crate::{
    ast_lowering::ast::LiteralKind,
    compile::{Compile, Program, SOURCE_FILE_EXTENSION},
    errors::diagnostic::{Diagnostic, DiagnosticCtxt},
};

pub struct ProgramSess<'a> {
    diag_ctxt: &'a DiagnosticCtxt,
    cur: std::path::PathBuf,
    // if we have a specific file path, it will be `Some()`
    // and `CalcSess::exec_with_display()` will be called. Otherwise, we will go through all the files
    // in the current directory and execute them.
    file_path: Option<std::path::PathBuf>,
    has_program: bool,
}

impl<'a> ProgramSess<'a> {
    pub fn from_path(path: &str, diag_ctxt: &'a DiagnosticCtxt) -> Result<Self, io::Error> {
        let path = std::path::Path::new(path);

        match env::set_current_dir(path) {
            Ok(()) => Ok({
                Self {
                    cur: env::current_dir()?,
                    diag_ctxt,
                    file_path: None,
                    has_program: false,
                }
            }),
            Err(err) if io::ErrorKind::NotADirectory == err.kind() && path.is_file() => {
                // if its a file, then we can be sure that it has a parent
                env::set_current_dir(
                    path.parent()
                        .expect("This is file does not contains a parent directory"),
                )?;

                Ok(Self {
                    cur: env::current_dir()?,
                    diag_ctxt,
                    file_path: Some(env::current_dir()?.join(path.file_name().unwrap())),
                    has_program: false,
                })
            }
            Err(err) => Err(err),
        }
    }

    pub fn run_with_exit(&mut self) {
        let path = &self.file_path;
        match path {
            Some(path) => self.exec_with_exit(path.as_path()),
            None => self.exec_many_with_exit(),
        }
    }

    fn exec_many_with_exit(&mut self) {
        let cur = match self.read_cur_dir() {
            Ok(cur) => cur,
            Err(err) => {
                println!(
                    "Failed to read directory `{}`: {err}",
                    self.get_cur_dir().display()
                );
                exit(1)
            }
        };

        for file in cur {
            let path = match file {
                Ok(file) => file,
                Err(err) => {
                    println!(
                        "Failed to open directory `{}`: {err}",
                        self.get_cur_dir().display()
                    );
                    exit(1)
                }
            }
            .path();

            if self.is_valid_file(path.as_path()) {
                self.has_program = true;
                self.exec_with_exit(path.as_path());
            }
        }

        if self.has_program == false {
            println!(
                "Warning: not a single program was found in the directory `{}`",
                self.get_cur_dir().display()
            );
        }
    }

    fn exec_with_exit(&self, path: &std::path::Path) {
        println!("Compiling program `{}`...", path.display());

        // Why `Err(_)`s? We use `DiagnosticHandler::emit_err()` in the earlier stages of compilation,
        // that displays and returning error, so here we don't need this error for output/something else.
        let program = match self.get_program(path) {
            Ok(program) => program,
            Err(_) => exit(1),
        };
        let res = match program.compile() {
            Ok(program) => program,
            Err(_) => exit(1),
        };

        match res.kind {
            LiteralKind::Int { val } => println!("Result: {val}"),
            LiteralKind::Float { val } => println!("Result: {val}"),
        }
    }

    fn get_program(&self, path: &std::path::Path) -> Result<Program, Diagnostic> {
        Program::from_source_file(path.to_string_lossy().to_string(), self.diag_ctxt)
    }

    fn read_cur_dir(&self) -> Result<std::fs::ReadDir, std::io::Error> {
        self.get_cur_dir().read_dir()
    }

    fn get_cur_dir(&self) -> &std::path::Path {
        self.cur.as_path()
    }

    fn is_valid_file(&self, path: &std::path::Path) -> bool {
        if let Some(ext) = path.extension() {
            ext == SOURCE_FILE_EXTENSION
        } else {
            false
        }
    }
}
