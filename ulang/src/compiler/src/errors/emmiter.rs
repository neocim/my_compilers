use super::diagnostic::Diagnostic;

pub type DynEmitter = dyn Emitter;

pub trait Emitter: std::fmt::Debug {
    fn emit_diag(&self, diag_msg: &Diagnostic);
}
