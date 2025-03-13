use std::ops::{Add, Div, Mul, Sub};

use super::Compiler;
use crate::ast::{BinOp, BinOpKind, Expr, Lit};

type InterpreterRet = Lit;

/// We only support expressions `Expr`, but if our language was more complex, we
/// could store here something like `Statement`, `Module`, `Program`, `AST`, etc.
pub struct Interpreter {
    root: Expr,
}

impl Interpreter {
    pub fn new(root: Expr) -> Self {
        Self { root }
    }
}

impl Compiler for Interpreter {
    type Ret = InterpreterRet;

    fn compile(&self) -> Self::Ret {
        self.expr(&self.root)
    }
}

impl Interpreter {
    fn expr(&self, expr: &Expr) -> InterpreterRet {
        match expr {
            Expr::BinOp(binop) => self.binop(binop),
            Expr::Lit(lit) => *lit,
        }
    }

    /// Panics if we try to calculate an expression with different types, for example `1 + 2.3'.
    /// Instead of panicking, we should return an error message or use an error recovery feature where we can
    /// convert the types to one.
    fn binop(&self, BinOp { lhs, rhs, op }: &BinOp) -> InterpreterRet {
        match (self.expr(lhs), self.expr(rhs)) {
            (Lit::Int { val: l }, Lit::Int { val: r }) => Lit::Int {
                val: self.apply_binop(l, r, op),
            },
            (Lit::Float { val: l }, Lit::Float { val: r }) => Lit::Float {
                val: self.apply_binop(l, r, op),
            },
            _ => panic!("Types mismatch"),
        }
    }

    fn apply_binop<T>(&self, lhs: T, rhs: T, op: &BinOpKind) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    {
        {
            match op {
                BinOpKind::Add => lhs + rhs,
                BinOpKind::Sub => lhs - rhs,
                BinOpKind::Mul => lhs * rhs,
                BinOpKind::Div => lhs / rhs,
            }
        }
    }
}
