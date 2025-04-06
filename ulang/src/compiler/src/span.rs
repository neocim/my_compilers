#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    lo: Pos,
    hi: Pos,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    line: u16,
    col: u16,
}

impl Span {
    pub fn new(lo: Pos, hi: Pos) -> Self {
        Self { lo, hi }
    }
}
