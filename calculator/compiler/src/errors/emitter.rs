use super::diagnostic::DiagnosticMsgs;

pub type DynEmitter = dyn Emitter;

pub trait Emitter {
    fn emit_diag(&self, diag_msgs: &DiagnosticMsgs);
}

#[derive(Clone, Copy)]
pub struct StdoutEmitter;

impl StdoutEmitter {
    pub fn new() -> Self {
        StdoutEmitter
    }

    pub fn emit(&self, diag_msgs: &DiagnosticMsgs) {
        for dm in diag_msgs.get_msgs().iter() {
            println!("Error: {}", dm.get_msg());
        }
    }
}

impl Emitter for StdoutEmitter {
    fn emit_diag(&self, diag_msgs: &DiagnosticMsgs) {
        self.emit(diag_msgs);
    }
}
