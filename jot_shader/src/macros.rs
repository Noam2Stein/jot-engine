#[macro_export]
macro_rules! shader_mod {
    { $vis:vis $ident:ident $(<$($interface:ident: $($interface_seg:ident)::*), * $(,)?>)?: $($tt:tt)* } => {
        $vis mod $ident {
            #[allow(non_snake_case)]
            pub struct _ShaderModType_ $(<$($interface: super::$($interface_seg)::*::_ShaderInterfaceTrait_), *>)? {$($(
                $interface: std::marker::PhantomData<$interface>,
            )*)?}

            $crate::_shader_imports_! {}

            $crate::_shader_mod_items_! { [$($tt)*][$($($interface: super::$($interface_seg)::*::_ShaderInterfaceTrait_), *)?] }
        }
    };
}

#[macro_export]
macro_rules! shader_interface {
    { $vis:vis $ident:ident: $($tt:tt)* } => {
        $vis mod $ident {
            $crate::_shader_imports_! {}

            #[doc(hidden)]
            pub unsafe trait _ShaderInterfaceTrait_ {
                $crate::_shader_interface_items_! { $($tt)* }
            }
        }
    };
}

// Private

#[doc(hidden)]
#[macro_export]
macro_rules! _shader_imports_ {
    () => {
        #[allow(unused_imports)]
        use $crate::_private_::{
            FVec2, FVec3, FVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, fvec2, fvec3, fvec4,
            ivec2, ivec3, ivec4, uvec2, uvec3, uvec4,
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _shader_mod_items_ {
    ([][$($interface:ident: $interface_trait:path), *]) => {};

    ([
        $vis:vis struct $ident:ident {$(
            $field_vis:vis $field_ident:ident: $field_type:ty
        ), * $(,)?}

        $($tt:tt)*
    ][$($interface:ident: $interface_trait:path), *]) => {
        $vis struct $ident<$($interface: $interface_trait), *> {$(
            $field_vis $field_ident: $field_type,
        )*}

        impl $crate::_private_::ShaderType for $ident {
            const INFO: $crate::_private_::ShaderTypeInfo = $crate::_private_::ShaderTypeInfo::Struct {
                id: $crate::_private_::concatcp!(stringify!($ident), $crate::_private_::const_random!(u32)),
                fields: &[$(
                    $crate::_private_::ShaderField {
                        offset: std::mem::offset_of!(Self, $field_ident),
                        type_: <$field_type as $crate::_private_::ShaderType>::INFO,
                    },
                )*],
            };
        }

        $crate::_shader_mod_items_! { $($tt)* }
    };

    (
        $vis:vis fn $ident:ident() {}

        $($tt:tt)*
    ) => {
        #[allow(non_camel_case_types)]
        $vis struct $ident;

        $crate::_shader_mod_items_! { $($tt)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _shader_interface_items_ {
    () => {};

    (
        $vis:vis type $ident:ident;

        $($tt:tt)*
    ) => {
        $vis type $ident;

        $crate::_shader_interface_items_! { $($tt)* }
    };

    (
        $vis:vis fn $ident:ident($($param_ident:ident: $param_type:ty), * $(,)?) $( -> $output:ty)?;

        $($tt:tt)*
    ) => {
        $vis fn $ident($($param_ident: $param_type), *) $( -> $output)?;

        $crate::_shader_interface_items_! { $($tt)* }
    };
}
