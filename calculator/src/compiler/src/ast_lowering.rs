use std::str::FromStr as _;

use crate::errors::diagnostic::{Diagnostic, DiagnosticCtxt};
use ast::{Ast, BinOp, BinOpKind, Expr, Lit, LiteralKind, Stmt};

pub mod ast;
pub mod errors;

pub struct Lower<'a> {
    diag_ctxt: &'a DiagnosticCtxt,
}

impl<'a> Lower<'a> {
    pub fn new(diag_ctxt: &'a DiagnosticCtxt) -> Self {
        Self { diag_ctxt }
    }

    pub fn lower(&self, ast: crate::ast::Ast) -> Result<Ast, Diagnostic<'a>> {
        match ast {
            crate::ast::Ast::Stmt(stmt) => Ok(Ast::Stmt(self.lower_stmt(stmt)?)),
        }
    }

    fn lower_stmt(&self, stmt: crate::ast::Stmt) -> Result<Stmt, Diagnostic<'a>> {
        match stmt {
            crate::ast::Stmt::Expr(expr) => Ok(Stmt::Expr(self.lower_expr(expr)?)),
        }
    }

    fn lower_expr(&self, expr: crate::ast::Expr) -> Result<Expr, Diagnostic<'a>> {
        match expr {
            crate::ast::Expr::Lit(lit) => Ok(Expr::Lit(self.lower_lit(lit)?)),
            crate::ast::Expr::BinOp(binop) => Ok(Expr::BinOp(self.lower_binop(binop)?)),
        }
    }

    fn lower_binop(
        &self,
        crate::ast::BinOp {
            left,
            kind: op,
            right,
        }: crate::ast::BinOp,
    ) -> Result<BinOp, Diagnostic<'a>> {
        Ok(BinOp {
            lhs: Box::new(self.lower_expr(*left)?),
            rhs: Box::new(self.lower_expr(*right)?),
            op: self.lower_op(op),
        })
    }

    fn lower_lit(&self, lit: crate::ast::Lit) -> Result<Lit, Diagnostic<'a>> {
        match lit.kind {
            crate::ast::token::LiteralKind::Int { val } => Ok(Lit {
                kind: LiteralKind::Int {
                    val: self.lit_to_i32(val)?,
                },
            }),
            crate::ast::token::LiteralKind::Float { val } => Ok(Lit {
                kind: LiteralKind::Float {
                    val: self.lit_to_f32(val)?,
                },
            }),
        }
    }

    fn lower_op(&self, op: crate::ast::token::BinOpKind) -> BinOpKind {
        match op {
            crate::ast::token::BinOpKind::Add => BinOpKind::Add,
            crate::ast::token::BinOpKind::Sub => BinOpKind::Sub,
            crate::ast::token::BinOpKind::Mul => BinOpKind::Mul,
            crate::ast::token::BinOpKind::Div => BinOpKind::Div,
            crate::ast::token::BinOpKind::Mod => BinOpKind::Mod,
        }
    }

    fn lit_to_i32(&self, maybe_int: String) -> Result<i32, Diagnostic<'a>> {
        match i32::from_str_radix(maybe_int.as_str(), 10) {
            Ok(int_num) => Ok(int_num),
            Err(err) => Err(self
                .diag_ctxt
                .handle()
                .emit_err(errors::ParseIntError::new(maybe_int, err.to_string()))),
        }
    }

    fn lit_to_f32(&self, maybe_float: String) -> Result<f32, Diagnostic<'a>> {
        match f32::from_str(maybe_float.as_str()) {
            Ok(float_num) => Ok(float_num),
            Err(err) => Err(self
                .diag_ctxt
                .handle()
                .emit_err(errors::ParseFloatError::new(maybe_float, err.to_string()))),
        }
    }
}
