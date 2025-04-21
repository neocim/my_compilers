use std::{borrow::Cow, process};

use super::emmiter::DynEmitter;
use crate::span::Span;

#[derive(Debug)]
pub struct Diagnostic<'dcx, 'msg> {
    diag_ctxt: &'dcx DiagnosticCtxt,
    level: DiagnosticLevel,
    diag_msg: DiagnosticMsg<'msg>,
    span: Span,
}

#[derive(Debug)]
pub struct DiagnosticCtxt {
    emitter: Box<DynEmitter>,
    into_console: bool,
    into_file: bool,
    allow_warns: bool,
}

#[derive(Clone, Copy, Debug)]
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
/// has been implemented, which is located in the [`macros`](https://github.com/neocim/my_compilers/tree/master/ulang/src/macros) crate.
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

    pub fn with_span(mut self, span: Span) -> Diagnostic<'dcx, 'msg> {
        self.span = span;
        self
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

    /// Emits and returns diagnostic
    pub fn emit_err(
        &'dcx self,
        err: impl IntoDiagnostic<'dcx, 'msg>,
        level: DiagnosticLevel,
        span: Span,
    ) -> Diagnostic<'dcx, 'msg> {
        let err = self.struct_err(err, level).with_span(span);
        self.emit(&err);
        err
    }

    /// Emits an error and stops compilation.
    pub fn emit_fatal(&'dcx self, err: impl IntoDiagnostic<'dcx, 'msg>, span: Span) {
        self.emit(&self.struct_err(err, DiagnosticLevel::Error).with_span(span));
        process::exit(1);
    }

    /// Creates a new diagnostic and returns it
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
