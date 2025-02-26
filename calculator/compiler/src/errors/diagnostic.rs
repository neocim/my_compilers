use std::borrow::Cow;

use super::emitter::DynEmitter;

pub trait IntoDiagnostic<'a> {
    fn into_diag(&self, diag_ctxt: &'a DiagnosticCtxt) -> Diagnostic<'a>;
}

/// The main struct for diagnostics. Its used to store errors in `DiagnosticMsgs`
/// and various diagnostic parameters stored in `DiagnosticCtxt`.
pub struct Diagnostic<'a> {
    diag_ctxt: &'a DiagnosticCtxt,
    diag_msgs: DiagnosticMsgs<'a>,
}

/// The struct to store diagnostic parameters such as emitter, etc.
/// (in the future it may be easy to add new parameters if need)
pub struct DiagnosticCtxt {
    emitter: Box<DynEmitter>,
}

/// This structure is the main handler for managing diagnostics,
/// which uses the `DiagnosticCtxt` parameters to correctly output diagnostics to the user.
pub struct DiagnosticHandler<'a> {
    diag_ctxt: &'a DiagnosticCtxt,
}

#[derive(Debug)]
pub struct DiagnosticMsgs<'s> {
    msgs: Vec<DiagnosticMsg<'s>>,
}

#[derive(Debug, Clone)]
pub struct DiagnosticMsg<'s> {
    msg: Cow<'s, str>,
}

impl<'a> Diagnostic<'a> {
    pub fn new(diag_ctxt: &'a DiagnosticCtxt, diag_msgs: DiagnosticMsgs<'a>) -> Self {
        Self {
            diag_ctxt,
            diag_msgs,
        }
    }

    pub fn emit(&self) {
        self.diag_ctxt.emitter.emit_diag(&self.diag_msgs);
    }
}

impl DiagnosticCtxt {
    pub fn new(emitter: Box<DynEmitter>) -> Self {
        Self { emitter }
    }

    pub fn handle<'a>(&'a self) -> DiagnosticHandler<'a> {
        DiagnosticHandler::new(self)
    }
}

impl<'a> DiagnosticHandler<'a> {
    pub fn new(diag_ctxt: &'a DiagnosticCtxt) -> Self {
        Self { diag_ctxt }
    }

    // Creates a `Dianostic` from error and emits it
    pub fn emit_err(self, err: impl IntoDiagnostic<'a>) {
        self.struct_err(err).emit();
    }

    // Create a `Diagnostic` from error that we can emit/return later
    pub fn struct_err(self, err: impl IntoDiagnostic<'a>) -> Diagnostic<'a> {
        err.into_diag(&self.diag_ctxt)
    }
}

impl<'s> DiagnosticMsgs<'s> {
    pub fn new(msgs: Vec<DiagnosticMsg<'s>>) -> Self {
        Self { msgs }
    }

    pub fn get_msgs(&self) -> &Vec<DiagnosticMsg<'s>> {
        &self.msgs
    }
}

impl<'s> DiagnosticMsg<'s> {
    pub fn new(msg: Cow<'s, str>) -> Self {
        Self { msg }
    }

    pub fn get_msg(&self) -> String {
        self.msg.to_string()
    }
}
