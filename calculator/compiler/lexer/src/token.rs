#[derive(Clone, Copy, Debug)]
pub enum Token {
    Lit { kind: LiteralKind },
    Op { kind: OpKind },
}

#[derive(Clone, Copy, Debug)]
pub enum OpKind {
    Star,
    Slash,
    Percent,
    Plus,
    Minux,
}

#[derive(Clone, Copy, Debug)]
pub enum LiteralKind {
    Int { val: i32 },
    Float { val: f32 },
}
