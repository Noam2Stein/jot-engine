use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned};
use syn::{Ident, Path, Token, Visibility, parse_macro_input};

mod compile;
mod entry_points;
use compile::*;
use entry_points::*;
use uuid::Uuid;

pub fn render_shader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {
        crate_,
        _semi,
        vis,
        ident,
        _colon,
        wgsl,
    } = parse_macro_input!(input as Input);

    let mut errors = Vec::new();
    let module = compile_shader(wgsl.clone()).unwrap_or_else(|e| {
        errors.push(e);
        naga::Module::default()
    });

    let _ = match find_entry_points(&module) {
        Ok((vertex, fragment)) => Ok((vertex, fragment)),
        Err(e) => {
            errors.push(e);

            Err(())
        }
    };

    let mod_ident = format_ident!("shader_{}", Uuid::new_v4().as_u128());

    quote_spanned! {
        ident.span() =>

        #[derive(Debug, Clone, Copy)]
        #[allow(non_camel_case_types)]
        #vis struct #ident;

        impl #crate_::RenderShader for #ident {
            type Vertex = #mod_ident::Vertex;

            const WGSL: &str = stringify!(#wgsl);
        }

        impl #crate_::GpuVertex for #mod_ident::Vertex {

        }

        #[doc(hidden)]
        mod #mod_ident {
            pub struct Vertex {}
        }

        #(#errors)*
    }
    .into()
}

#[derive(Parse)]
struct Input {
    crate_: Path,
    _semi: Token![;],
    vis: Visibility,
    ident: Ident,
    _colon: Token![:],
    wgsl: TokenStream,
}
