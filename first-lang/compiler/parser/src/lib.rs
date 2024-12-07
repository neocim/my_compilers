pub mod lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Any literal
    LiteralExpr(LiteralExpr),
    /// Any binary expr like `+`, `-`, etc
    BinaryExpr(BinaryExpr),
    /// Expression to store variable
    VariableExpr(VariableExpr),
    /// Expression to call anything
    CallExpr(CallExpr),
    /// Function prototype (name and args)
    FuncPrototype(FuncPrototype),
    /// Function body and it prototype
    Func(Func),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpr {
    kind: LiteralExprKind,
}

impl LiteralExpr {
    pub fn new(kind: LiteralExprKind) -> Self {
        LiteralExpr { kind }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralExprKind {
    IntExpr(IntExpr),
    FloatExpr(FloatExpr),
    CharExpr(CharExpr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntExpr {
    val: i32,
}

impl IntExpr {
    pub fn new(val: i32) -> Self {
        IntExpr { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatExpr {
    val: f32,
}

impl FloatExpr {
    pub fn new(val: f32) -> Self {
        FloatExpr { val }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharExpr {
    val: char,
}

impl CharExpr {
    pub fn new(val: char) -> Self {
        CharExpr { val }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    kind: BinaryExprKind,
    left: LiteralExpr,
    right: LiteralExpr,
}

impl BinaryExpr {
    pub fn new(kind: BinaryExprKind, left: LiteralExpr, right: LiteralExpr) -> Self {
        BinaryExpr { kind, left, right }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryExprKind {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableExpr {
    name: String,
}

impl VariableExpr {
    pub fn new(name: String) -> Self {
        VariableExpr { name }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    call: String,
    args: Vec<Expr>,
}

impl CallExpr {
    pub fn new(call: String, args: Vec<Expr>) -> Self {
        CallExpr { call, args }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncPrototype {
    name: String,
    args: Vec<String>,
}

impl FuncPrototype {
    pub fn new(name: String, args: Vec<String>) -> Self {
        FuncPrototype { name, args }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    prototype: FuncPrototype,
    body: Box<Expr>,
}

impl Func {
    pub fn new(prototype: FuncPrototype, body: Expr) -> Self {
        Func {
            prototype,
            body: Box::new(body),
        }
    }
}
