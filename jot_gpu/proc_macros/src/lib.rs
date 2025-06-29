use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;

#[proc_macro]
pub fn shader_errors(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.clone().into();

    let input_str = input.to_string();

    match naga::front::wgsl::parse_str(&input_str) {
        Ok(_) => {
            quote! {}
        }
        Err(e) => {
            let span = e.labels().next().map_or_else(input.span(), |label| Span:: label.0.)
            syn::Error::new_spanned(input, format!("WGSL parse error: {e}")).to_compile_error()
        }
    }
    .into()
}
