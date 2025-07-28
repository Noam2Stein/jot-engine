use quote::{quote, quote_spanned};
use syn::{Data, DataEnum, DeriveInput, Error, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(SceneEnumType)]
pub fn derive_scene_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);

    let DataEnum {
        enum_token: _,
        brace_token: _,
        variants,
    } = match data {
        Data::Enum(data) => data,

        _ => {
            return Error::new(ident.span(), "`SceneEnumType` is expected to be an enum")
                .into_compile_error()
                .into();
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let variant_fields_result = variants
        .iter()
        .map(|variant| {
            if let [variant_field] = *variant.fields.iter().collect::<Box<[_]>>() {
                Ok(variant_field)
            } else {
                Err(Error::new_spanned(
                    &variant.ident,
                    "`SceneEnum` expects a single field ",
                ))
            }
        })
        .collect::<syn::Result<Vec<_>>>();

    let varaint_fields = match variant_fields_result {
        Ok(varaint_fields) => varaint_fields,
        Err(err) => return err.into_compile_error().into(),
    };

    let update_match_arms =
        variants
            .iter()
            .zip(varaint_fields.iter())
            .map(|(variant, variant_field)| {
                let variant_ident = &variant.ident;
                let variant_member = variant.fields.members().next().unwrap();
                let variant_field_type = &variant_field.ty;

                quote_spanned! {
                    variant_field_type.span() =>

                    Self::#variant_ident { #variant_member: inner_scene }
                        => <#variant_field_type as ::jot::scene::SceneType>::update(inner_scene, delta_time, gpu),
                }
            });

    let event_match_arms = variants.iter().zip(varaint_fields.iter()).map(
        |(variant, variant_field)| {
            let variant_ident = &variant.ident;
            let variant_member = variant.fields.members().next().unwrap();
            let variant_field_type = &variant_field.ty;

            quote_spanned! {
                variant_field_type.span() =>

                Self::#variant_ident { #variant_member: inner_scene }
                    => <#variant_field_type as ::jot::scene::SceneType>::event(inner_scene, event, gpu),
            }
        },
    );

    let draw_match_arms = variants.iter().zip(varaint_fields.iter()).map(
        |(variant, variant_field)| {
            let variant_ident = &variant.ident;
            let variant_member = variant.fields.members().next().unwrap();
            let variant_field_type = &variant_field.ty;

            quote_spanned! {
                variant_field_type.span() =>

                Self::#variant_ident { #variant_member: inner_scene }
                    => <#variant_field_type as ::jot::scene::SceneType>::draw(inner_scene, output, gpu),
            }
        },
    );

    quote! {
        impl #impl_generics ::jot::scene::SceneEnumType for #ident #ty_generics #where_clause {}

        impl #impl_generics ::jot::scene::SceneType for #ident #ty_generics #where_clause {
            type SceneEnum = Self;

            fn update(&mut self, delta_time: f64, gpu: &::jot::gpu::Gpu) -> ::jot::scene::SceneFlow<Self::SceneEnum> {
                match self {
                    #(#update_match_arms)*
                    _ => unreachable!(),
                }
            }

            fn event(&mut self, event: &::jot::game::GameEvent, gpu: &::jot::gpu::Gpu) -> ::jot::scene::SceneFlow<Self::SceneEnum> {
                match self {
                    #(#event_match_arms)*
                    _ => unreachable!(),
                }
            }

            fn draw(&mut self, output: &::jot::gpu::GpuTexture<2>, gpu: &::jot::gpu::Gpu) {
                match self {
                    #(#draw_match_arms)*
                    _ => unreachable!(),
                }
            }
        }
    }
    .into()
}
