mod errors;
#[cfg(test)]
mod tests;

use super::Compile;
use crate::{
    ast_lowering::ast::{Ast, BinOp, BinOpKind, Expr, Lit, LiteralKind, Stmt},
    errors::diagnostic::{Diagnostic, DiagnosticCtxt},
};
use errors::MismatchedTypes;

type CalcRes<'a> = Result<Lit, Diagnostic<'a>>;

pub struct Calculator<'a> {
    root: Ast,
    diag_ctxt: &'a DiagnosticCtxt,
}

impl<'a> Compile for Calculator<'a> {
    type Ret = CalcRes<'a>;

    fn compile(&self) -> Self::Ret {
        self.compile()
    }
}

impl<'a> Calculator<'a> {
    pub fn new(root: Ast, diag_ctxt: &'a DiagnosticCtxt) -> Self {
        Self { root, diag_ctxt }
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
            (Lit { kind: lty }, Lit { kind: rty }) => Err(self.diag_ctxt.handle().struct_err(
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
