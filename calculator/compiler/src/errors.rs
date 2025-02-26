pub mod diagnostic;
pub mod emitter;

use diagnostic::Diagnostic;
pub type ParseResult<'a, T> = Result<T, Diagnostic<'a>>;
