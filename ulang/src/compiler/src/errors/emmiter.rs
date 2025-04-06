use super::diagnostic::Diagnostic;

pub type DynEmitter = dyn Emitter;

pub trait Emitter: std::fmt::Debug {
    fn emit_diag(&self, diag_msg: &Diagnostic);
}

/// `NOT IMPLEMENTED YET!`
pub struct OutpEmitter<'d> {
    diags: Vec<Diagnostic<'d>>,
    into_console: bool,
    into_file: bool,
}
