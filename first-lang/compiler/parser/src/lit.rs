#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(IntLit),
    Float(FloatLit),
    Char(CharLit),
    Bool(BoolLit),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntLit {
    val: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatLit {
    val: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharLit {
    val: char,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolLit {
    val: bool,
}

impl IntLit {
    pub fn new(val: i32) -> Self {
        IntLit { val }
    }
}

impl FloatLit {
    pub fn new(val: f32) -> Self {
        FloatLit { val }
    }
}

impl CharLit {
    pub fn new(val: char) -> Self {
        CharLit { val }
    }
}

impl BoolLit {
    pub fn new(val: bool) -> Self {
        BoolLit { val }
    }
}
