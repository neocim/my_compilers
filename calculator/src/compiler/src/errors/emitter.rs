use super::diagnostic::DiagnosticMsg;

pub type DynEmitter = dyn Emitter;

pub trait Emitter: std::fmt::Debug {
    fn emit_diag(&self, diag_msg: &DiagnosticMsg);
    fn emit_warn(&self, diag_msg: &DiagnosticMsg);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StdoutEmitter;

impl StdoutEmitter {
    pub fn new() -> Self {
        StdoutEmitter
    }

    fn emit(&self, diag_msg: &DiagnosticMsg) {
        println!("Error: {}", diag_msg.get_msg());
    }

    fn emit_warn(&self, diag_msg: &DiagnosticMsg) {
        println!("Warning: {}", diag_msg.get_msg())
    }
}

impl Emitter for StdoutEmitter {
    fn emit_diag(&self, diag_msg: &DiagnosticMsg) {
        self.emit(diag_msg);
    }

    fn emit_warn(&self, diag_msg: &DiagnosticMsg) {
        self.emit_warn(diag_msg);
    }
}
