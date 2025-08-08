use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    Data, DataStruct, DeriveInput, Error, Field, Visibility, parse_macro_input, spanned::Spanned,
};

#[proc_macro_derive(InputType)]
pub fn input_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input_derive_macro_inner(input, quote! { ::jot::input }, quote! { ::jot::scheme })
}

#[proc_macro_derive(InputType_Local)]
pub fn input_derive_macro_local(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input_derive_macro_inner(input, quote! { ::jot_input }, quote! { crate })
}

fn input_derive_macro_inner(
    input: proc_macro::TokenStream,
    jot_input: TokenStream,
    jot_scheme: TokenStream,
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
            return Error::new_spanned(data.enum_token, "cannot derive `InputType` for enum")
                .into_compile_error()
                .into();
        }

        Data::Union(data) => {
            return Error::new_spanned(data.union_token, "cannot derive `InputType` for union")
                .into_compile_error()
                .into();
        }
    };

    if generics.params.len() > 0 || generics.where_clause.is_some() {
        return Error::new_spanned(generics, "cannot derive `InputType` for generic type")
            .into_compile_error()
            .into();
    }

    for field in &fields {
        match field.vis {
            Visibility::Public(_) => {}

            _ => {
                return Error::new_spanned(
                    field,
                    "cannot derive `InputType` for struct with private fields",
                )
                .into_compile_error()
                .into();
            }
        }
    }

    let private_mod_name = format_ident!("private_mod_{ident}");
    let bindings_name = format_ident!("{ident}Bindings");
    let resolver_name = format_ident!("{ident}Resolver");

    let binding_fields = fields
        .iter()
        .map(
            |Field {
                 attrs,
                 vis,
                 mutability: _,
                 ident,
                 colon_token,
                 ty,
             }| {
                quote_spanned! {
                    ty.span() =>

                    #(#attrs)*
                    #vis #ident #colon_token #jot_scheme::Bindings<#ty>
                }
            },
        )
        .collect::<Vec<_>>();

    let resolver_fields = fields
        .iter()
        .map(
            |Field {
                 attrs,
                 vis,
                 mutability: _,
                 ident,
                 colon_token,
                 ty,
             }| {
                quote_spanned! {
                    ty.span() =>

                    #(#attrs)*
                    #vis #ident #colon_token #jot_scheme::Resolver<#ty>
                }
            },
        )
        .collect::<Vec<_>>();

    let new_resolver_fields = fields
        .iter()
        .map(
            |Field {
                 attrs: _,
                 vis: _,
                 mutability: _,
                 ident,
                 colon_token,
                 ty,
             }| {
                quote_spanned! {
                    ty.span() =>

                    #ident #colon_token #jot_scheme::Resolver::<#ty>::new(bindings.#ident)
                }
            },
        )
        .collect::<Vec<_>>();

    let field_events = fields
        .iter()
        .map(
            |Field {
                 attrs: _,
                 vis: _,
                 mutability: _,
                 ident,
                 colon_token: _,
                 ty,
             }| {
                quote_spanned! {
                    ty.span() =>

                    #jot_scheme::Resolver::<#ty>::event(&mut resolver.#ident, event);
                }
            },
        )
        .collect::<Vec<_>>();

    let step_fields = fields
        .iter()
        .map(
            |Field {
                 attrs: _,
                 vis: _,
                 mutability: _,
                 ident,
                 colon_token,
                 ty,
             }| {
                quote_spanned! {
                    ty.span() =>

                    #ident #colon_token #jot_scheme::Resolver::<#ty>::step(&mut resolver.#ident)
                }
            },
        )
        .collect::<Vec<_>>();

    let add_fields = fields
        .iter()
        .map(
            |Field {
                 attrs: _,
                 vis: _,
                 mutability: _,
                 ident,
                 colon_token,
                 ty,
             }| {
                quote_spanned! {
                    ty.span() =>

                    #ident #colon_token &self.#ident + &rhs.#ident
                }
            },
        )
        .collect::<Vec<_>>();

    let add_assign_fields = fields
        .iter()
        .map(
            |Field {
                 attrs: _,
                 vis: _,
                 mutability: _,
                 ident,
                 colon_token: _,
                 ty,
             }| {
                quote_spanned! {
                    ty.span() =>

                    self.#ident += &rhs.#ident;
                }
            },
        )
        .collect::<Vec<_>>();

    let flatten_fields = fields
        .iter()
        .map(
            |Field {
                 attrs: _,
                 vis: _,
                 mutability: _,
                 ident,
                 colon_token: _,
                 ty,
             }| {
                quote_spanned! {
                    ty.span() =>

                    self.#ident.flatten();
                }
            },
        )
        .collect::<Vec<_>>();

    quote! {
        mod #private_mod_name {
            use super::*;

            #[derive(Debug, Clone, PartialEq, Eq, Default)]
            pub struct #bindings_name {#(
                #binding_fields,
            )*}

            #[derive(Debug, Clone, Default)]
            pub struct #resolver_name {#(
                #resolver_fields,
            )*}

            impl #jot_scheme::InputType for #ident {
                type Bindings = #bindings_name;
                type ResolverState = #resolver_name;

                fn new_resolver(bindings: Self::Bindings) -> Self::ResolverState {
                    #resolver_name {#(
                        #new_resolver_fields,
                    )*}
                }

                fn event(resolver: &mut Self::ResolverState, event: &#jot_input::InputDeviceEvent) {
                    #(
                        #field_events
                    )*
                }

                fn step(resolver: &mut Self::ResolverState) -> Self {
                    Self {#(
                        #step_fields,
                    )*}
                }
            }

            impl std::ops::Add for &#bindings_name {
                type Output = #bindings_name;

                fn add(self, rhs: Self) -> Self::Output {
                    #bindings_name {#(
                        #add_fields,
                    )*}
                }
            }
            impl std::ops::AddAssign<&#bindings_name> for #bindings_name {
                fn add_assign(&mut self, rhs: &Self) {
                    #(
                        #add_assign_fields
                    )*
                }
            }

            impl #jot_scheme::BindingsType for #bindings_name {
                fn flatten(&mut self) {
                    #(
                        #flatten_fields
                    )*
                }
            }

            impl<T: Into<#bindings_name>> From<Flat<T>> for #bindings_name {
                fn from(value: Flat<T>) -> Self {
                    value.0.into()
                }
            }
        }
    }
    .into()
}
