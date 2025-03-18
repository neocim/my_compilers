mod errors;

use std::{
    env,
    io::{self, ErrorKind},
};

use errors::CompileError;

use crate::{
    compile::{Compile as _, Program, SOURCE_FILE_EXTENSION},
    errors::diagnostic::{Diagnostic, DiagnosticCtxt},
};

pub struct CalcSess<'a> {
    cur_dir: std::path::PathBuf,
    diag_ctxt: &'a DiagnosticCtxt,
}

impl<'a> CalcSess<'a> {
    pub fn from_path(path: &str, diag_ctxt: &'a DiagnosticCtxt) -> Result<Self, io::Error> {
        let path = std::path::Path::new(path);

        match env::set_current_dir(path) {
            Ok(()) => Ok(Self {
                cur_dir: std::path::PathBuf::from(path),
                diag_ctxt,
            }),
            Err(err) if ErrorKind::NotADirectory == err.kind() && path.is_file() => {
                // if its a file, then we can be sure that it has a parent
                env::set_current_dir(
                    path.parent()
                        .expect("This file is does not contains a parent directory"),
                )?;

                Ok(Self {
                    cur_dir: env::current_dir()?,
                    diag_ctxt,
                })
            }
            Err(err) => Err(err),
        }
    }

    pub fn compile(&self) -> Result<(), Diagnostic<'a>> {
        let programs = self.get_programs()?;
        for program in programs.into_iter() {
            let program = program?;
            let res = program.compile()?;
            println!(
                "Running `{}`..\nOutput of a program: `{}`",
                program.get_path(),
                res.get_int().unwrap()
            );
        }

        Ok(())
    }

    fn get_programs(&self) -> Result<Vec<Result<Program<'a>, Diagnostic<'a>>>, Diagnostic<'a>> {
        let mut programs = Vec::new();

        for src_file in self.get_cur_dir().read_dir().map_err(|err| {
            self.diag_ctxt
                .handle()
                .emit_err(CompileError::new(err.to_string()))
        })? {
            let src_path = src_file
                .map_err(|err| {
                    self.diag_ctxt
                        .handle()
                        .emit_err(CompileError::new(err.to_string()))
                })?
                .path();
            let src_path = src_path.as_path();

            if self.is_correct_source_file(src_path) {
                programs.push(Program::from_source_file(
                    src_path.to_string_lossy().to_string(),
                    self.diag_ctxt,
                ));
            }
        }

        Ok(programs)
    }

    pub fn get_cur_dir(&self) -> &std::path::Path {
        self.cur_dir.as_path()
    }

    fn is_correct_source_file(&self, path: &std::path::Path) -> bool {
        if let Some(ext) = path.extension() {
            ext == SOURCE_FILE_EXTENSION
        } else {
            false
        }
    }
}
