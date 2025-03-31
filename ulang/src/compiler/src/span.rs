#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    lo: u16,
    hi: u16,
}

impl Span {
    pub fn new(lo: u16, hi: u16) -> Self {
        Self { lo, hi }
    }
}
