#![allow(dead_code)]

mod errors;
#[cfg(test)]
mod tests;

use std::{fs, path::Path};

use super::{Compile, SOURCE_FILE_EXTENSION};
use crate::{
    ast_lowering::ast::{Ast, BinOp, BinOpKind, Expr, Lit, LiteralKind, Stmt},
    errors::diagnostic::{Diagnostic, DiagnosticCtxt},
    parser::Parser,
};
use errors::{MismatchedTypes, OpenFileError, WrongFileExtension};

type CalcRes<'a> = Result<Lit, Diagnostic<'a>>;

pub struct Program<'a> {
    root: Ast,
    path: String,
    diag_ctxt: &'a DiagnosticCtxt,
}

impl<'a> Compile for Program<'a> {
    type Ret = CalcRes<'a>;

    fn compile(&self) -> Self::Ret {
        self.compile()
    }
}

impl<'a> Program<'a> {
    pub fn from_source_file(
        path: String,
        diag_ctxt: &'a DiagnosticCtxt,
    ) -> Result<Self, Diagnostic<'a>> {
        let diag_handle = diag_ctxt.handle();
        let file_path = Path::new(&path);

        let file_name = match file_path.file_name() {
            Some(name) => name.to_string_lossy(),
            None => {
                return Err(diag_handle.emit_err(OpenFileError::new(
                    path.to_string(),
                    "wrong file path".to_string(),
                )))
            }
        };

        let ext = match file_path.extension() {
            Some(ext) => ext.to_string_lossy(),
            None => {
                return Err(diag_handle.emit_err(WrongFileExtension::new(
                    None,
                    file_name.into(),
                    SOURCE_FILE_EXTENSION.to_string(),
                )))
            }
        };

        if ext == SOURCE_FILE_EXTENSION {
            let src = match fs::read_to_string(&path) {
                Ok(source) => source,
                Err(err) => {
                    return Err(
                        diag_handle.emit_err(OpenFileError::new(path.to_string(), err.to_string()))
                    )
                }
            };

            Ok(Program::new(
                Parser::from_source(src.as_ref(), diag_ctxt).lowering_parse()?,
                path,
                diag_ctxt,
            ))
        } else {
            Err(diag_handle.emit_err(WrongFileExtension::new(
                Some(ext.into()),
                file_name.into(),
                SOURCE_FILE_EXTENSION.to_string(),
            )))
        }
    }

    pub fn from_source(
        src: &str,
        path: String,
        diag_ctxt: &'a DiagnosticCtxt,
    ) -> Result<Self, Diagnostic<'a>> {
        Ok(Program::new(
            Parser::from_source(src, diag_ctxt).lowering_parse()?,
            path,
            diag_ctxt,
        ))
    }

    fn new(root: Ast, path: String, diag_ctxt: &'a DiagnosticCtxt) -> Self {
        Self {
            root,
            path,
            diag_ctxt,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    fn compile(&self) -> CalcRes<'a> {
        match &self.root {
            Ast::Stmt(stmt) => self.compile_stmt(stmt),
        }
    }

    fn compile_stmt(&self, stmt: &Stmt) -> CalcRes<'a> {
        match stmt {
            Stmt::Expr(expr) => self.compile_expr(expr),
        }
    }

    fn compile_expr(&self, expr: &Expr) -> CalcRes<'a> {
        match expr {
            Expr::Lit(lit) => Ok(*lit),
            Expr::BinOp(binop) => self.compile_binop(binop),
        }
    }

    fn compile_binop(&self, BinOp { lhs, rhs, op }: &BinOp) -> CalcRes<'a> {
        match (self.compile_expr(lhs)?, self.compile_expr(rhs)?) {
            (
                Lit {
                    kind: LiteralKind::Int { val: lhs },
                },
                Lit {
                    kind: LiteralKind::Int { val: rhs },
                },
            ) => Ok(Lit {
                kind: LiteralKind::Int {
                    val: self.apply_binop(lhs, rhs, *op),
                },
            }),
            (
                Lit {
                    kind: LiteralKind::Float { val: lhs },
                },
                Lit {
                    kind: LiteralKind::Float { val: rhs },
                },
            ) => Ok(Lit {
                kind: LiteralKind::Float {
                    val: self.apply_binop(lhs, rhs, *op),
                },
            }),
            (Lit { kind: lty }, Lit { kind: rty }) => Err(self.diag_ctxt.handle().emit_err(
                MismatchedTypes::new(format!("{:?}", lty), format!("{:?}", rty)),
            )),
        }
    }

    fn apply_binop<T>(&self, lhs: T, rhs: T, op: BinOpKind) -> T
    where
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Rem<Output = T>,
    {
        match op {
            BinOpKind::Add => lhs + rhs,
            BinOpKind::Sub => lhs - rhs,
            BinOpKind::Mul => lhs * rhs,
            BinOpKind::Div => lhs / rhs,
            BinOpKind::Mod => lhs % rhs,
        }
    }
}
