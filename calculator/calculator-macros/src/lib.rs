use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Error, Fields, FieldsNamed, LitStr,
};

#[proc_macro_derive(IntoDiagnostic, attributes(diagnostic))]
pub fn into_diag_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match get_fields(input.data) {
        Ok(fields) => fields.named,
        Err(err) => return err.to_compile_error().into(),
    };
    let field_names: Vec<_> = fields.iter().flat_map(|f| f.ident.as_ref()).collect();

    let diagnostic_attr = match get_diag_attr(&input.attrs) {
        Ok(message) => message,
        Err(err) => return err.to_compile_error().into(),
    };

    let implementation = quote! {
        Diagnostic::new(
            diag_ctxt,
            DiagnosticMsgs::new(vec![DiagnosticMsg::new(
                format!(#diagnostic_attr, #(self.#field_names),*).into(),
            )]),
        )
    };

    let gen = quote! {
        impl<'a> IntoDiagnostic<'a> for #name {
            fn into_diag(&self, diag_ctxt: &'a DiagnosticCtxt) -> Diagnostic<'a> {
                #implementation
            }
        }
    };

    proc_macro::TokenStream::from(gen)
}

fn get_diag_attr(attrs: &[Attribute]) -> Result<String, Error> {
    if let Some(attr) = attrs.iter().find(|a| a.path().is_ident("diagnostic")) {
        let message = attr.parse_args::<LitStr>()?.value();
        Ok(message)
    } else {
        Err(Error::new(
            Span::call_site(),
            r#"This macro should only use with attribute `#[diagnostic("your error description: `field 1` is `{}`, `field 2` is `{}`")]`"#,
        ))
    }
}

// Here i didn't have to make a nested match here, but i need to return
// an error that it should only be used with structures with named fields
fn get_fields(input: Data) -> Result<FieldsNamed, Error> {
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
            ))
        }
    }
}
