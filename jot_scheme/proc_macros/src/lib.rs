use quote::quote;

#[proc_macro_derive(InputState)]
pub fn input_state_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote! {}.into()
}

#[proc_macro_derive(InputBindings)]
pub fn input_bindings_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote! {}.into()
}
