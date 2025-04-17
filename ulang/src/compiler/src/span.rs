#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    pub lo: Pos,
    pub hi: Pos,
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

    pub fn increment_hi_col(&self) -> Span {
        Span::new(self.lo, Pos::new(self.hi.ln, self.hi.col + 1))
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self { ln: 1, col: 1 }
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
