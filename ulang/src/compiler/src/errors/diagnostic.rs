use std::borrow::Cow;

use crate::span::Span;

use super::emmiter::DynEmitter;

pub struct Diagnostic<'dcx, 'msg> {
    diag_ctxt: &'dcx DiagnosticCtxt,
    level: DiagnosticLevel,
    diag_msg: DiagnosticMsg<'msg>,
    span: Span,
}

pub struct DiagnosticCtxt {
    emitter: Box<DynEmitter>,
    into_console: bool,
    into_file: bool,
    allow_warns: bool,
}

#[derive(Clone, Copy)]
pub enum DiagnosticLevel {
    Help,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub struct DiagnosticMsg<'s> {
    msg: Cow<'s, str>,
}

/// A trait that allows you to easily create your own errors and
/// parse them into diagnostics that are used throughout the compiler.
/// To make it easier to create these errors, a [`derive-macro`](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros)
/// has been implemented, which is located in the `ulang-macros` crate.
pub trait IntoDiagnostic<'dcx, 'msg> {
    fn into_diag(
        self,
        diag_ctxt: &'dcx DiagnosticCtxt,
        level: DiagnosticLevel,
    ) -> Diagnostic<'dcx, 'msg>;
}

pub trait IntoDiagMsg {
    fn into_diag_msg<'s>(self) -> DiagnosticMsg<'s>;
}

impl IntoDiagMsg for String {
    fn into_diag_msg<'s>(self) -> DiagnosticMsg<'s> {
        DiagnosticMsg { msg: self.into() }
    }
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

impl<'dcx, 'msg> Diagnostic<'dcx, 'msg> {
    pub fn new(
        diag_ctxt: &'dcx DiagnosticCtxt,
        level: DiagnosticLevel,
        diag_msg: impl IntoDiagMsg,
        span: Span,
    ) -> Self {
        Self {
            diag_ctxt,
            level,
            diag_msg: diag_msg.into_diag_msg(),
            span,
        }
    }
}

impl<'dcx, 'msg> DiagnosticCtxt {
    pub fn new(
        emitter: Box<DynEmitter>,
        into_console: bool,
        into_file: bool,
        allow_warns: bool,
    ) -> Self {
        Self {
            emitter,
            into_console,
            into_file,
            allow_warns,
        }
    }

    pub fn emit_err(
        &'dcx self,
        err: impl IntoDiagnostic<'dcx, 'msg>,
        level: DiagnosticLevel,
    ) -> Diagnostic<'dcx, 'msg> {
        let err = self.struct_err(err, level);
        self.emit(&err);
        err
    }

    pub fn struct_err(
        &'dcx self,
        err: impl IntoDiagnostic<'dcx, 'msg>,
        level: DiagnosticLevel,
    ) -> Diagnostic<'dcx, 'msg> {
        err.into_diag(self, level)
    }

    pub fn emit(&self, diag: &Diagnostic) {
        self.emitter.emit_diag(&diag);
    }
}
