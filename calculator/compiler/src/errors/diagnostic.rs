use std::borrow::Cow;

use super::emitter::Emitter;

pub trait IntoDiag<'a, E: Emitter> {
    fn into_diag(&self, emitter: &'a E) -> Diag<'a, E>;
}

#[derive(Clone, Debug)]
pub struct Diag<'a, E: Emitter> {
    emitter: &'a E,
    diag_msgs: DiagMessages<'a>,
}

#[derive(Clone, Debug)]
pub struct DiagMessages<'s> {
    messages: Vec<DiagMessage<'s>>,
}

#[derive(Debug, Clone)]
pub struct DiagMessage<'s> {
    msg: Cow<'s, str>,
}

#[derive(Debug, Clone)]
pub struct DiagHandler<'a, E: Emitter> {
    emitter: &'a E,
}

impl<'a, E: Emitter> Diag<'a, E> {
    pub fn new(emitter: &'a E, diag_msgs: DiagMessages<'a>) -> Self {
        Self { emitter, diag_msgs }
    }

    pub fn emit(&self) {
        self.emitter.emit_diag(&self.diag_msgs);
    }

    pub fn handle(&self) -> DiagHandler<'a, E> {
        DiagHandler::new(&self.emitter)
    }
}

impl<'a, E: Emitter> DiagHandler<'a, E> {
    pub fn new(emitter: &'a E) -> Self {
        Self { emitter }
    }

    pub fn struct_err(self, err: impl IntoDiag<'a, E>) -> Diag<'a, E> {
        err.into_diag(self.emitter)
    }

    pub fn emit_err(self, err: impl IntoDiag<'a, E>) {
        self.struct_err(err).emit();
    }
}

impl<'s> DiagMessages<'s> {
    pub fn new(messages: Vec<DiagMessage<'s>>) -> Self {
        Self { messages }
    }

    pub fn get_msgs(&self) -> Vec<DiagMessage<'s>> {
        self.messages.clone()
    }
}

impl<'s> DiagMessage<'s> {
    pub fn new(msg: Cow<'s, str>) -> Self {
        Self { msg }
    }

    pub fn get_msg(&self) -> String {
        self.msg.to_string()
    }
}
