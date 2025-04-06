#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    lo: Pos,
    hi: Pos,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    ln: u16,
    col: u16,
}

impl Pos {
    pub fn new(ln: u16, col: u16) -> Self {
        Self { ln, col }
    }
}

impl Span {
    pub fn new(lo: Pos, hi: Pos) -> Self {
        Self { lo, hi }
    }
}
