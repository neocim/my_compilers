use proc_macro::TokenStream;

mod into_diag;

use into_diag::into_diag_derive;

#[proc_macro_derive(IntoDiagnostic, attributes(message))]
pub fn into_diag(input: TokenStream) -> TokenStream {
    into_diag_derive(input)
}
