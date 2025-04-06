use std::borrow::Cow;

use crate::span::Span;

use super::emmiter::DynEmitter;

pub struct Diagnostic<'a> {
    diag_ctxt: &'a DiagnosticCtxt,
    level: DiagnosticLevel,
    diag_msg: DiagnosticMsg<'a>,
    span: Span,
}

pub enum DiagnosticLevel {
    Help,
    Warn,
    Error,
}

impl ToString for DiagnosticLevel {
    fn to_string(&self) -> String {
        match self {
            DiagnosticLevel::Help => String::from("HELP"),
            DiagnosticLevel::Warn => String::from("WARNING"),
            DiagnosticLevel::Error => String::from("ERROR"),
        }
    }
}

pub struct DiagnosticCtxt {
    emitter: Box<DynEmitter>,
}

impl DiagnosticCtxt {
    pub fn new(emitter: Box<DynEmitter>) -> Self {
        Self { emitter }
    }

    pub fn emit_err<'a>(&'a self, err: impl IntoDiagnostic<'a>) -> Diagnostic<'a> {
        let err = self.struct_err(err);
        self.emit(&err);
        err
    }

    pub fn struct_err<'a>(&'a self, err: impl IntoDiagnostic<'a>) -> Diagnostic<'a> {
        err.into_diag(self)
    }

    pub fn emit(&self, diag: &Diagnostic) {
        self.emitter.emit_diag(&diag);
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticMsg<'s> {
    msg: Cow<'s, str>,
}

/// A trait that allows you to easily create your own errors and
/// parse them into diagnostics that are used throughout the compiler.
/// To make it easier to create these errors, a [`derive-macro`](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros)
/// has been implemented, which is located in the `ulang-macros` crate.
pub trait IntoDiagnostic<'d> {
    fn into_diag(&self, diag_ctxt: &'d DiagnosticCtxt) -> Diagnostic<'d>;
}
