use std::borrow::Cow;

use super::emitter::Emitter;

pub trait IntoDiag<E: Emitter + Copy> {
    fn into_diag(&self, emitter: E) -> Diag<E>;
}

pub struct Diag<'s, E: Emitter> {
    emitter: E,
    pub diag_msgs: DiagMessages<'s>,
}

pub struct DiagMessages<'s> {
    pub messages: Vec<DiagMessage<'s>>,
}

#[derive(Debug)]
pub struct DiagMessage<'s> {
    pub msg: Cow<'s, str>,
}

impl<'s> DiagMessage<'s> {
    pub fn new(msg: Cow<'s, str>) -> Self {
        Self { msg }
    }
}

impl<'s> DiagMessages<'s> {
    pub fn new(messages: Vec<DiagMessage<'s>>) -> Self {
        Self { messages }
    }
}

impl<'s, E: Emitter + Copy> Diag<'s, E> {
    pub fn new(emitter: E, diag_msgs: DiagMessages<'s>) -> Self {
        Self { emitter, diag_msgs }
    }
    pub fn emit_err(&self, err: impl IntoDiag<E>) {
        err.into_diag(self.emitter).emit();
    }
    pub fn emit(&self) {
        self.emitter.emit_diag(&self.diag_msgs);
    }
}
