mod into_diag;

use proc_macro::TokenStream;

use into_diag::into_diag_derive;

/// A macro to generate an implementation of the `IntoDiagnostic` trait from
/// [`compiler::errors::diagnostic`](https://github.com/neocim/my_compilers/blob/master/ulang/src/compiler/src/errors/diagnostic.rs)
/// for compilation errors. It should ONLY be used inside this compiler, because
/// it uses `use` paths starting with `crate`, because im not trying to make
/// this macro used anywhere other than this compiler.
#[proc_macro_derive(IntoDiagnostic, attributes(message))]
pub fn into_diag(input: TokenStream) -> TokenStream {
    into_diag_derive(input)
}
