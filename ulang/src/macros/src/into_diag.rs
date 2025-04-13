use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Ident, LitStr,
    parse_macro_input,
};

pub fn into_diag_derive(input: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(implement(parse_macro_input!(input as DeriveInput)))
}

fn implement(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let imports = quote! {
        use crate::{
            errors::diagnostic::{Diagnostic, DiagnosticCtxt, DiagnosticLevel, IntoDiagnostic},
        };
        use super::#name;
    };
    let body = get_body(&input);
    let mod_name = Ident::new(
        format!("__impl_IntoDiagnostic_{}", name).as_str(),
        name.span(),
    );
    let implementation = quote! {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        mod #mod_name {
            #imports
            impl<'a, 'b> IntoDiagnostic<'a, 'b> for #name {
                fn into_diag(
                    self,
                    diag_ctxt: &'a DiagnosticCtxt,
                    level: DiagnosticLevel,
                ) -> Diagnostic<'a, 'b> {
                    #body
                }
            }
        }
    };

    proc_macro::TokenStream::from(implementation)
}

fn get_body(input: &DeriveInput) -> proc_macro2::TokenStream {
    let idents: Vec<_> = match get_fields(&input.data) {
        Ok(fields) => fields.named.iter().flat_map(|f| f.ident.as_ref()).collect(),
        Err(err) => return err.to_compile_error().into(),
    };
    let bindings = quote! {
        #(let #idents = self.#idents;)*
    };
    let message = match get_message_attr(&input.attrs) {
        Ok(message) => message,
        Err(err) => return err.to_compile_error().into(),
    };

    let body = quote! {
        #bindings
        Diagnostic::new(diag_ctxt, level, format!(#message), Default::default())
    };

    body
}

fn get_message_attr(attrs: &[Attribute]) -> Result<String, Error> {
    if let Some(attr) = attrs.iter().find(|a| a.path().is_ident("message")) {
        let message = attr.parse_args::<LitStr>()?.value();
        Ok(message)
    } else {
        Err(Error::new(
            Span::call_site(),
            r#"This macro should only use with attribute `#[message(...)]` attribute."#,
        ))
    }
}

// Here i didn't have to make a nested `match`s, but i need to return
// an error if need that it should only be used with structures with named fields
fn get_fields<'a>(input: &'a Data) -> Result<&'a FieldsNamed, Error> {
    match input {
        Data::Struct(DataStruct { fields, .. }) => match fields {
            Fields::Named(fields) => Ok(fields),
            _ => Err(Error::new(
                Span::call_site(),
                "This macro only support `struct`s with named fields",
            )),
        },
        _ => {
            return Err(Error::new(
                Span::call_site(),
                "This macro should only use with `struct`s",
            ));
        }
    }
}
