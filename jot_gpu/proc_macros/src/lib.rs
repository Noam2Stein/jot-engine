use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    Data, DataStruct, DeriveInput, Error, parse_macro_input, spanned::Spanned,
};

#[proc_macro_derive(GpuBindings)]
pub fn derive_gpu_bindings(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_gpu_bindings_inner(input, quote! { ::jot::gpu })
}

#[proc_macro_derive(GpuBindings_Local)]
pub fn derive_gpu_bindings_local(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_gpu_bindings_inner(input, quote! { ::jot_gpu })
}

fn derive_gpu_bindings_inner(
    input: proc_macro::TokenStream,
    jot_gpu: TokenStream,
) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let DataStruct {
        struct_token: _,
        fields,
        semi_token: _,
    } = match data {
        Data::Struct(data) => data,

        Data::Enum(data) => {
            return Error::new_spanned(data.enum_token, "cannot derive `GpuBindings` for enum")
                .into_compile_error()
                .into();
        }

        Data::Union(data) => {
            return Error::new_spanned(data.union_token, "cannot derive `GpuBindings` for union")
                .into_compile_error()
                .into();
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let field_lens = fields.iter().map(|field| {
        let field_type = &field.ty;

        quote_spanned! {
            field_type.span() =>

            <#field_type as #jot_gpu::GpuBindings>::BINDING_COUNT
        }
    });

    let push_field_layout_entries = fields.iter().map(|field| {
        let field_type = &field.ty;

        quote_spanned! {
            field_type.span() =>

            <#field_type as #jot_gpu::GpuBindings>::push_layout_entries(entries, binding);
        }
    });

    let push_field_entries = fields.iter().map(|field| {
        let field_type = &field.ty;
        let field_ident = &field.ident;

        quote_spanned! {
            field_type.span() =>

            <#field_type as #jot_gpu::GpuBindings>::push_entries(&self.#field_ident, entries, binding);
        }
    });

    quote! {
        unsafe impl #impl_generics #jot_gpu::GpuBindings for #ident #ty_generics #where_clause {
            const BINDING_COUNT: usize = 0 #(+ #field_lens)*;
        
            fn push_layout_entries(entries: &mut Vec<#jot_gpu::wgpu::BindGroupLayoutEntry>, binding: &mut u32) {
                #(#push_field_layout_entries)*
            }
        
            fn push_entries<'s>(&'s self, entries: &mut Vec<#jot_gpu::wgpu::BindGroupEntry<'s>>, binding: &mut u32) {
                #(#push_field_entries)*
            }
        }
    }
    .into()

}
