#![allow(dead_code)]

#[derive(Debug, Clone)]
pub enum Expr {
    BinOp(BinOp),
    Lit(Lit),
}

#[derive(Debug, Clone)]
pub struct BinOp {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub op: BinOpKind,
}

#[derive(Debug, Clone, Copy)]
pub enum Lit {
    Int { val: i32 },
    Float { val: f32 },
}

#[derive(Debug, Copy, Clone)]
pub enum BinOpKind {
    Add,
    Sub,
    Div,
    Mul,
}

impl BinOp {
    pub fn new(lhs: Expr, rhs: Expr, op: BinOpKind) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        }
    }
}

impl Lit {
    pub fn int_val(&self) -> Option<i32> {
        match self {
            Lit::Int { val } => Some(*val),
            _ => None,
        }
    }
    pub fn float_val(&self) -> Option<f32> {
        match self {
            Lit::Float { val } => Some(*val),
            _ => None,
        }
    }
}
