#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    lo: Pos,
    hi: Pos,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    pub(crate) ln: u16,
    pub(crate) col: u16,
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

impl Default for Span {
    fn default() -> Self {
        Self {
            lo: Pos::new(1, 1),
            hi: Pos::new(1, 1),
        }
    }
}
