#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Ast {
    Stmt(Stmt),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Lit(Lit),
    BinOp(BinOp),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BinOp {
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
    pub op: BinOpKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub struct Lit {
    pub kind: LiteralKind,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum LiteralKind {
    Int { val: i32 },
    Float { val: f32 },
}
