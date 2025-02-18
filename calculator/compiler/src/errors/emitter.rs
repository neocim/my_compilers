use crate::errors::diagnostic::DiagMessages;

pub trait Emitter {
    fn emit_diag(&self, diag_msgs: &DiagMessages);
}

#[derive(Clone, Copy)]
pub struct StdoutEmitter;

impl StdoutEmitter {
    pub fn new() -> Self {
        StdoutEmitter
    }

    pub fn emit(&self, diag_msgs: &DiagMessages) {
        for dm in diag_msgs.messages.iter() {
            println!("Error: {}", dm.msg);
        }
    }
}

impl Emitter for StdoutEmitter {
    fn emit_diag(&self, diag_msgs: &DiagMessages) {
        self.emit(diag_msgs);
    }
}
